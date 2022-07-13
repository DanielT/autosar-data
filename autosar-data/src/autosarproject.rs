use std::{path::Path, str::FromStr};

use crate::*;

impl AutosarProject {
    /// Create an instance of AutosarData
    ///
    /// Initially it contains no arxml files
    pub fn new() -> AutosarProject {
        AutosarProject(Arc::new(Mutex::new(AutosarProjectRaw {
            files: Vec::new(),
            identifiables: FxHashMap::default(),
            reference_origins: FxHashMap::default(),
        })))
    }

    /// Create a new [ArxmlFile] inside this AutosarData structure
    ///
    /// You must provide a filename for the [ArxmlFile], even if you do not plan to write the data to disk.
    /// You must also specify an [AutosarVersion]. All methods manipulation the data insdie the file will ensure conformity with the version specified here.
    /// The newly created ArxmlFile will be created with a root AUTOSAR element.
    pub fn create_file<P: AsRef<Path>>(
        &self,
        filename: P,
        version: AutosarVersion,
    ) -> Result<ArxmlFile, AutosarDataError> {
        let mut data = self.0.lock();

        if data.files.iter().any(|af| af.filename() == filename.as_ref()) {
            return Err(AutosarDataError::DuplicateFilenameError {
                verb: "create",
                filename: filename.as_ref().to_path_buf(),
            });
        }

        let new_file = ArxmlFile::new(filename, version, self);
        data.files.push(new_file.clone());
        Ok(new_file)
    }

    /// Load a named buffer containig arxml data
    ///
    /// If you have e.g. received arxml data over a network, or decompressed it from an archive, etc, then you may load it with this method.
    ///
    /// Parameters:
    ///  - buffer: The data inside the buffer must be valid utf-8. Optionally it may begin with a UTF-8-BOM, which will be silently ignored.
    ///  - filename: the original filename of the data, or a newly generated name that is unique within the AutosarData instance.
    ///  - strict: toggle strict parsing. Some parsing errors are recoverable and can be issued as warnings.
    ///
    /// This method may be called concurrently on multiple threads to load different buffers
    pub fn load_named_arxml_buffer<P: AsRef<Path>>(
        &self,
        buffer: &[u8],
        filename: P,
        strict: bool,
    ) -> Result<(ArxmlFile, Vec<AutosarDataError>), AutosarDataError> {
        if self.files().any(|file| file.filename() == filename.as_ref()) {
            return Err(AutosarDataError::DuplicateFilenameError {
                verb: "load",
                filename: filename.as_ref().to_path_buf(),
            });
        }

        let mut parser = ArxmlParser::new(filename.as_ref().to_path_buf(), buffer, strict);
        let root_element = parser.parse_arxml()?;

        let arxml_file = ArxmlFile(Arc::new(Mutex::new(ArxmlFileRaw {
            project: self.downgrade(),
            version: parser.get_fileversion(),
            filename: filename.as_ref().to_path_buf(),
            root_element,
        })));
        // graft on the back-link from the root element to the file
        let new_parent = ElementOrFile::File(arxml_file.downgrade());
        arxml_file.root_element().set_parent(new_parent);

        let mut data = self.0.lock();
        data.identifiables.reserve(parser.identifiables.len());
        for (key, value) in parser.identifiables {
            if let Some(existing) = data.identifiables.insert(key, value) {
                if let Some(element) = existing.upgrade() {
                    return Err(AutosarDataError::OverlappingDataError {
                        filename: filename.as_ref().to_path_buf(),
                        path: element.path().unwrap().unwrap_or_else(|| "".to_owned()),
                    });
                }
            }
        }
        data.reference_origins.reserve(parser.references.len());
        for (refpath, referring_element) in parser.references {
            if let Some(xref) = data.reference_origins.get_mut(&refpath) {
                xref.push(referring_element);
            } else {
                data.reference_origins.insert(refpath, vec![referring_element]);
            }
        }
        data.files.push(arxml_file.clone());

        Ok((arxml_file, parser.warnings))
    }

    /// Load an arxml file
    ///
    /// This function is a wrapper around load_named_arxml_buffer to make the common case of loading a file from disk more convenient
    ///
    /// Parameters:
    ///  - filename: the original filename of the data, or a newly generated name that is unique within the AutosarData instance.
    ///  - strict: toggle strict parsing. Some parsing errors are recoverable and can be issued as warnings.
    pub fn load_arxml_file<P: AsRef<Path>>(
        &self,
        filename: P,
        strict: bool,
    ) -> Result<(ArxmlFile, Vec<AutosarDataError>), AutosarDataError> {
        let filename_buf = filename.as_ref().to_path_buf();
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(error) => {
                return Err(AutosarDataError::IoErrorOpen {
                    filename: filename_buf,
                    ioerror: error,
                })
            }
        };

        let filesize = file.metadata().unwrap().len();
        let mut buffer = Vec::with_capacity(filesize as usize);
        file.read_to_end(&mut buffer)
            .map_err(|err| AutosarDataError::IoErrorRead {
                filename: filename_buf.clone(),
                ioerror: err,
            })?;

        self.load_named_arxml_buffer(&buffer, &filename_buf, strict)
    }

    /// serialize each of the files in the data set
    ///
    /// returns the result in a HashMap of <file_name, file_content>
    pub fn serialize_files(&self) -> FxHashMap<PathBuf, String> {
        let mut result = FxHashMap::default();
        for file in self.files() {
            let data = file.serialize();
            result.insert(file.filename(), data);
        }
        result
    }

    /// create an iterator over all [ArxmlFile]s in this AutosarData object
    pub fn files(&self) -> ArxmlFileIterator {
        ArxmlFileIterator::new(self.clone())
    }

    /// get a named element by its Autosar path
    ///
    /// This is a lookup in a hash table and runs in O(1) time
    pub fn get_element_by_path(&self, path: &str) -> Result<Option<Element>, AutosarDataError> {
        let project = self.0.lock();
        Ok(project.identifiables.get(path).and_then(|element| element.upgrade()))
    }

    /// create a depth-first iterator over all [Element]s in all [ArxmlFile]s
    pub fn elements_dfs(&self) -> AutosarDataElementsDfsIterator {
        AutosarDataElementsDfsIterator::new(self.files())
    }

    /// create an iterator over all identifiable elements
    pub fn identifiable_elements(&self) -> AutosarDataIdentElementsIterator {
        let project = self.0.lock();
        AutosarDataIdentElementsIterator::new(&project.identifiables)
    }

    /// check all Autosar path references and return a list of elements with invalid references
    ///
    /// For each reference: The target must exist and the DEST attribute must correctly specify the type of the target
    ///
    /// If no references are invalid, then the return value is an empty list
    pub fn check_references(&self) -> Vec<WeakElement> {
        let mut broken_refs = Vec::new();

        let project = self.0.lock();
        for (path, element_list) in &project.reference_origins {
            if let Some(target_elem_weak) = project.identifiables.get(path) {
                // reference target exists
                if let Some(target_elem) = target_elem_weak.upgrade() {
                    // the target of the reference exists, but the reference can still be technically invalid
                    // if the content of the DEST attribute on the reference is wrong
                    let target_elemname = target_elem.element_name();
                    // e.g. if the target is a <SYSTEM>, then the reference must have the attribute DEST="SYSTEM".
                    // Converting the ElementName of the target_elem to an EnumItem for use in the DEST attribute
                    // is done by converting ElementName -> str -> EnumItem
                    let required_reftype = EnumItem::from_str(target_elemname.to_str()).unwrap();

                    for referring_elem_weak in element_list {
                        if let Some(referring_elem) = referring_elem_weak.upgrade() {
                            if let Some(CharacterData::Enum(reftype)) =
                                referring_elem.get_attribute(AttributeName::Dest)
                            {
                                if reftype != required_reftype {
                                    // wrong reference type in the DEST attribute
                                    broken_refs.push(referring_elem_weak.clone());
                                }
                            } else {
                                // DEST attribute does not exist - can only happen if broken data was loaded with strict == false
                                broken_refs.push(referring_elem_weak.clone());
                            }
                        }
                    }
                } else {
                    // This case should never happen, possibly panic?
                    // The strong ref count of target_elem can only go to zero if the element is removed,
                    // but remove_element() should also update data.identifiables and data.reference_origins.
                    broken_refs.extend(element_list.iter().cloned());
                }
            } else {
                // reference target does not exist
                broken_refs.extend(element_list.iter().cloned());
            }
        }

        broken_refs
    }

    /// create a weak reference to this data
    pub(crate) fn downgrade(&self) -> WeakAutosarProject {
        WeakAutosarProject(Arc::downgrade(&self.0))
    }

    pub(crate) fn fix_identifiables(&self, old_path: Option<String>, element: &Element) {
        let mut data = self.0.lock();
        let new_path = match element.path() {
            Ok(Some(path)) => path,
            _ => return,
        };
        if let Some(old_path) = old_path {
            data.identifiables.remove(&old_path);

            if element.element_name() == ElementName::ArPackage {
                // a package has been renamed, so it might contain other identifiable elements that are affected by the renaming
                let keys: Vec<String> = data.identifiables.keys().cloned().collect();
                for key in keys {
                    // find keys referring to entries inside the renamed package
                    if key.starts_with(&old_path) {
                        // construct the new element path
                        let (_, suffix) = key.split_at(old_path.len());
                        let new_key = format!("{new_path}{suffix}");
                        // fix the identifiables hashmap
                        let entry = data.identifiables.remove(&key).unwrap();
                        data.identifiables.insert(new_key, entry);
                    }
                }
            }
        }
        // insert under the new name regardless of whether an old name existed or not
        data.identifiables.insert(new_path, element.downgrade());
    }

    pub(crate) fn remove_identifiable(&self, path: &str) {
        let mut data = self.0.lock();
        data.identifiables.remove(path);
    }

    pub(crate) fn add_reference_origin(&self, new_ref: &str, origin: WeakElement) {
        let mut data = self.0.lock();
        // add the new entry
        if let Some(referrer_list) = data.reference_origins.get_mut(new_ref) {
            referrer_list.push(origin);
        } else {
            data.reference_origins.insert(new_ref.to_owned(), vec![origin]);
        }
    }

    pub(crate) fn fix_reference_origins(&self, old_ref: &str, new_ref: &str, origin: WeakElement) {
        if old_ref != new_ref {
            let mut data = self.0.lock();
            // remove the old entry
            if let Some(referrer_list) = data.reference_origins.get_mut(old_ref) {
                if let Some((index, _)) = referrer_list.iter().enumerate().find(|(_, x)| **x == origin) {
                    referrer_list.swap_remove(index);
                }
            }
            // add the new entry
            if let Some(referrer_list) = data.reference_origins.get_mut(new_ref) {
                referrer_list.push(origin);
            } else {
                data.reference_origins.insert(new_ref.to_owned(), vec![origin]);
            }
        }
    }

    pub(crate) fn remove_reference_origin(&self, reference: &str, element: WeakElement) {
        let mut data = self.0.lock();
        if let Some(referrer_list) = data.reference_origins.get_mut(reference) {
            if let Some((index, _)) = referrer_list.iter().enumerate().find(|(_, x)| **x == element) {
                referrer_list.swap_remove(index);
            }
        }
    }
}

impl Default for AutosarProject {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for AutosarProject {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

impl WeakAutosarProject {
    pub(crate) fn upgrade(&self) -> Option<AutosarProject> {
        Weak::upgrade(&self.0).map(AutosarProject)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_file() {
        let project = AutosarProject::new();
        let file = project.create_file("test", AutosarVersion::Autosar_00050);
        assert!(file.is_ok());
    }

    #[test]
    fn load_buffer() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES></AR-PACKAGES></AUTOSAR>"#;
        let project = AutosarProject::new();
        let result = project.load_named_arxml_buffer(FILEBUF.as_bytes(), "test", true);
        assert!(result.is_ok());
    }

    #[test]
    fn references() {
        let project = AutosarProject::new();
        let weak = project.downgrade();
        let project2 = weak.upgrade();
        assert_eq!(Arc::strong_count(&project.0), 2);
        assert_eq!(project, project2.unwrap());
    }

    #[test]
    fn identifiables_iterator() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>OuterPackage1</SHORT-NAME>
            <AR-PACKAGES>
                <AR-PACKAGE><SHORT-NAME>InnerPackage1</SHORT-NAME></AR-PACKAGE>
                <AR-PACKAGE><SHORT-NAME>InnerPackage2</SHORT-NAME></AR-PACKAGE>
            </AR-PACKAGES>
        </AR-PACKAGE>
        <AR-PACKAGE><SHORT-NAME>OuterPackage2</SHORT-NAME>
            <AR-PACKAGES>
                <AR-PACKAGE><SHORT-NAME>InnerPackage1</SHORT-NAME></AR-PACKAGE>
                <AR-PACKAGE><SHORT-NAME>InnerPackage2</SHORT-NAME></AR-PACKAGE>
            </AR-PACKAGES>
        </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(FILEBUF.as_bytes(), "test", true)
            .unwrap();
        let mut iter = project.identifiable_elements();
        assert_eq!(iter.next().unwrap().0, "/OuterPackage1");
        assert_eq!(iter.next().unwrap().0, "/OuterPackage1/InnerPackage1");
        assert_eq!(iter.next().unwrap().0, "/OuterPackage1/InnerPackage2");
        assert_eq!(iter.next().unwrap().0, "/OuterPackage2");
        assert_eq!(iter.next().unwrap().0, "/OuterPackage2/InnerPackage1");
        assert_eq!(iter.next().unwrap().0, "/OuterPackage2/InnerPackage2");
    }

    #[test]
    fn check_references() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
                <SYSTEM><SHORT-NAME>System</SHORT-NAME>
                    <FIBEX-ELEMENTS>
                        <FIBEX-ELEMENT-REF-CONDITIONAL>
                            <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE">/Pkg/EcuInstance</FIBEX-ELEMENT-REF>
                        </FIBEX-ELEMENT-REF-CONDITIONAL>
                        <FIBEX-ELEMENT-REF-CONDITIONAL>
                            <FIBEX-ELEMENT-REF DEST="I-SIGNAL-I-PDU">/Some/Invalid/Path</FIBEX-ELEMENT-REF>
                        </FIBEX-ELEMENT-REF-CONDITIONAL>
                        <FIBEX-ELEMENT-REF-CONDITIONAL>
                            <FIBEX-ELEMENT-REF DEST="I-SIGNAL">/Pkg/System</FIBEX-ELEMENT-REF>
                        </FIBEX-ELEMENT-REF-CONDITIONAL>
                    </FIBEX-ELEMENTS>
                </SYSTEM>
                <ECU-INSTANCE><SHORT-NAME>EcuInstance</SHORT-NAME></ECU-INSTANCE>
            </ELEMENTS>
        </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(FILEBUF.as_bytes(), "test", true)
            .unwrap();
        let invalid_refs = project.check_references();
        assert_eq!(invalid_refs.len(), 2);
        let ref0 = invalid_refs[0].upgrade().unwrap();
        assert_eq!(ref0.element_name(), ElementName::FibexElementRef);
        if let CharacterData::String(refpath) = ref0.character_data().unwrap() {
            if refpath != "/Pkg/System" && refpath != "/Some/Invalid/Path" {
                panic!("unexpected path: {refpath}");
            }
        } else {
            panic!("did not get a reference path where it was expected")
        }
    }
}
