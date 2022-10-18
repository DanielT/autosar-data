use std::{borrow::Cow, collections::HashMap, path::Path, str::FromStr};

use crate::*;

impl AutosarProject {
    /// Create an instance of AutosarData
    ///
    /// Initially it contains no arxml files
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// let project = AutosarProject::new();
    /// ```
    ///
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
    ///
    /// # Parameters
    ///
    ///  - `filename`: A filename for the data from the buffer. It must be unique within the project.
    ///    It will be used by write(), and is also used to identify this data in error messages.
    ///  - `version`: The [AutosarVersion] that will be used by the data created inside this file
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let project = AutosarProject::new();
    /// let file = project.create_file("filename.arxml", AutosarVersion::Autosar_00050)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::DuplicateFilenameError]: The project already contains a file with this filename
    ///
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

    /// Load a named buffer containing arxml data
    ///
    /// If you have e.g. received arxml data over a network, or decompressed it from an archive, etc, then you may load it with this method.
    ///
    /// # Parameters:
    ///
    ///  - `buffer`: The data inside the buffer must be valid utf-8. Optionally it may begin with a UTF-8-BOM, which will be silently ignored.
    ///  - `filename`: the original filename of the data, or a newly generated name that is unique within the AutosarData instance.
    ///  - `strict`: toggle strict parsing. Some parsing errors are recoverable and can be issued as warnings.
    ///
    /// This method may be called concurrently on multiple threads to load different buffers
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let project = AutosarProject::new();
    /// # let buffer = b"";
    /// project.load_named_arxml_buffer(buffer, "filename.arxml", true)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::DuplicateFilenameError]: The project already contains a file with this filename
    ///  - [AutosarDataError::OverlappingDataError]: The new data contains Autosar paths that are already defined by the existing data
    ///  - [AutosarDataError::ParserError]: The parser detected an error; the source field gives further details
    ///
    pub fn load_named_arxml_buffer<P: AsRef<Path>>(
        &self,
        buffer: &[u8],
        filename: P,
        strict: bool,
    ) -> Result<(ArxmlFile, Vec<AutosarDataError>), AutosarDataError> {
        self.load_named_arxml_buffer_internal(buffer, filename.as_ref().to_path_buf(), strict)
    }

    fn load_named_arxml_buffer_internal(
        &self,
        buffer: &[u8],
        filename: PathBuf,
        strict: bool,
    ) -> Result<(ArxmlFile, Vec<AutosarDataError>), AutosarDataError> {
        if self.files().any(|file| file.filename() == filename) {
            return Err(AutosarDataError::DuplicateFilenameError { verb: "load", filename });
        }

        let mut parser = ArxmlParser::new(filename.clone(), buffer, strict);
        let root_element = parser.parse_arxml()?;

        let arxml_file = ArxmlFile(Arc::new(Mutex::new(ArxmlFileRaw {
            project: self.downgrade(),
            version: parser.get_fileversion(),
            filename: filename.clone(),
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
                        filename,
                        path: element.path().unwrap_or_else(|_| "".to_owned()),
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
    /// # Parameters:
    ///
    ///  - `filename`: the original filename of the data, or a newly generated name that is unique within the AutosarData instance.
    ///  - `strict`: toggle strict parsing. Some parsing errors are recoverable and can be issued as warnings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let project = AutosarProject::new();
    /// project.load_arxml_file("filename.arxml", true)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::IoErrorOpen]: The file could not be opened
    ///  - [AutosarDataError::IoErrorRead]: There was an error while reading the file
    ///  - [AutosarDataError::DuplicateFilenameError]: The project already contains a file with this filename
    ///  - [AutosarDataError::OverlappingDataError]: The new data contains Autosar paths that are already defined by the existing data
    ///  - [AutosarDataError::ParserError]: The parser detected an error; the source field gives further details
    ///
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

    /// remove a file from the project
    ///
    /// # Parameters:
    ///
    ///  - `file`: The file that will be removed from the project
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let project = AutosarProject::new();
    /// let file = project.create_file("filename.arxml", AutosarVersion::Autosar_00050)?;
    /// project.remove_file(&file);
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_file(&self, file: &ArxmlFile) {
        let find_result = self
            .0
            .lock()
            .files
            .iter()
            .enumerate()
            .find(|(_, f)| *f == file)
            .map(|(pos, _)| pos);
        // find_result is stored first so that the lock on project is dropped
        if let Some(pos) = find_result {
            self.0.lock().files.swap_remove(pos);
            let root_elem = file.root_element();
            root_elem
                .0
                .lock()
                .remove_internal(root_elem.downgrade(), self, Cow::from(""));
        }
    }

    /// serialize each of the files in the project
    ///
    /// returns the result in a HashMap of <file_name, file_content>
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let project = AutosarProject::new();
    /// for (pathbuf, file_content) in project.serialize_files() {
    ///     // do something with it
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn serialize_files(&self) -> HashMap<PathBuf, String> {
        let mut result = HashMap::new();
        for file in self.files() {
            let data = file.serialize();
            result.insert(file.filename(), data);
        }
        result
    }

    /// write all files in the project
    ///
    /// This is a wrapper around serialize_files. The current filename of each file will be used to write the serialized data.
    ///
    /// If any of the individual files cannot be written, then write() will abort and return the error.
    /// This may result in a situation where some files have been written and others have not.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let project = AutosarProject::new();
    /// // load or create files
    /// project.write()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible errors
    ///
    ///  - [AutosarDataError::IoErrorWrite]: There was an error while writing a file
    ///
    pub fn write(&self) -> Result<(), AutosarDataError> {
        for (pathbuf, filedata) in self.serialize_files() {
            std::fs::write(pathbuf.clone(), filedata).map_err(|err| AutosarDataError::IoErrorWrite {
                filename: pathbuf,
                ioerror: err,
            })?;
        }
        Ok(())
    }

    /// create an iterator over all [ArxmlFile]s in this AutosarData object
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let project = AutosarProject::new();
    /// // load or create files
    /// for file in project.files() {
    ///     // do something with the file
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn files(&self) -> ArxmlFileIterator {
        ArxmlFileIterator::new(self.clone())
    }

    /// get a named element by its Autosar path
    ///
    /// This is a lookup in a hash table and runs in O(1) time
    ///
    /// # Parameters
    ///
    ///  - `path`: The Autosar path to look up
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let project = AutosarProject::new();
    /// // [...]
    /// if let Some(element) = project.get_element_by_path("/Path/To/Element") {
    ///     // use the element
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_element_by_path(&self, path: &str) -> Option<Element> {
        let project = self.0.lock();
        project.identifiables.get(path).and_then(|element| element.upgrade())
    }

    /// create a depth-first iterator over all [Element]s in all [ArxmlFile]s
    ///
    /// The AUTOSAR elements in each file will appear at depth = 0.
    /// Directly printing the return values could show something like this:
    ///
    /// <pre>
    /// 0: AUTOSAR
    /// 1: AR-PACKAGES
    /// 2: AR-PACKAGE
    /// ...
    /// 2: AR-PACKAGE
    /// </pre>
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// for (depth, element) in project.elements_dfs() {
    ///     // [...]
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn elements_dfs(&self) -> AutosarDataElementsDfsIterator {
        AutosarDataElementsDfsIterator::new(self.files())
    }

    /// create an iterator over all identifiable elements
    ///
    /// It returns the full Autosar path of each element together with a reference to the element.
    /// The returned values are sorted alphabetically by the path value.
    ///
    /// The iterator created by identifiable_elements() returns values from an internal list
    /// that is built when identifiable_elements() is called. This means that the iteration can
    /// never show any changes that were made to the data after the iterator was created.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// for (path, element) in project.identifiable_elements() {
    ///     // [...]
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn identifiable_elements(&self) -> AutosarDataIdentElementsIterator {
        let project = self.0.lock();
        AutosarDataIdentElementsIterator::new(&project.identifiables)
    }

    /// return all elements referring to the given target path
    ///
    /// It returns [WeakElement]s which must be upgraded to get usable [Element]s.
    ///
    /// This is effectively the reverse operation of `get_element_by_path()`
    ///
    /// # Parameters
    ///
    ///  - `target_path`: The path whose references should be returned
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// for weak_element in project.get_references_to("/Path/To/Element") {
    ///     // [...]
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_references_to(&self, target_path: &str) -> Vec<WeakElement> {
        if let Some(origins) = self.0.lock().reference_origins.get(target_path) {
            origins.clone()
        } else {
            Vec::new()
        }
    }

    /// check all Autosar path references and return a list of elements with invalid references
    ///
    /// For each reference: The target must exist and the DEST attribute must correctly specify the type of the target
    ///
    /// If no references are invalid, then the return value is an empty list
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// for broken_ref_weak in project.check_references() {
    ///     if let Some(broken_ref) = broken_ref_weak.upgrade() {
    ///         // update or delete ref?
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
                                referring_elem.attribute_value(AttributeName::Dest)
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

    // add an identifiable element to the cache
    pub(crate) fn add_identifiable(&self, new_path: String, elem: WeakElement) {
        let mut project = self.0.lock();
        project.identifiables.insert(new_path, elem);
    }

    // fix a single identifiable element or tree of elements in the cache which has been moved/renamed
    pub(crate) fn fix_identifiables(&self, old_path: &str, new_path: &str) {
        let mut project = self.0.lock();

        // the renamed element might contain other identifiable elements that are affected by the renaming
        let keys: Vec<String> = project.identifiables.keys().cloned().collect();
        for key in keys {
            // find keys referring to entries inside the renamed package
            if let Some(suffix) = key.strip_prefix(old_path) {
                if suffix.is_empty() || suffix.starts_with('/') {
                    let new_key = format!("{new_path}{suffix}");
                    // fix the identifiables hashmap
                    let entry = project.identifiables.remove(&key).unwrap();
                    project.identifiables.insert(new_key, entry);
                }
            }
        }
    }

    // remove a deleted element from the cache
    pub(crate) fn remove_identifiable(&self, path: &str) {
        let mut project = self.0.lock();
        project.identifiables.remove(path);
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
            let mut remove_list = false;
            // remove the old entry
            if let Some(referrer_list) = data.reference_origins.get_mut(old_ref) {
                if let Some((index, _)) = referrer_list.iter().enumerate().find(|(_, x)| **x == origin) {
                    referrer_list.swap_remove(index);
                    remove_list = referrer_list.is_empty();
                }
            }
            if remove_list {
                data.reference_origins.remove(old_ref);
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
        let mut count = 1;
        if let Some(referrer_list) = data.reference_origins.get_mut(reference) {
            if let Some((index, _)) = referrer_list.iter().enumerate().find(|(_, x)| **x == element) {
                referrer_list.swap_remove(index);
            }
            count = referrer_list.len();
        }
        if count == 0 {
            data.reference_origins.remove(reference);
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
        // error: duplicate file name
        let file = project.create_file("test", AutosarVersion::Autosar_00050);
        assert!(file.is_err());
    }

    #[test]
    fn load_buffer() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const FILEBUF2: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE><SHORT-NAME>OtherPkg</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        let project = AutosarProject::new();
        // succefully load a buffer
        let result = project.load_named_arxml_buffer(FILEBUF.as_bytes(), "test", true);
        assert!(result.is_ok());
        // succefully load a second buffer
        let result = project.load_named_arxml_buffer(FILEBUF2.as_bytes(), "other", true);
        assert!(result.is_ok());
        // error: duplicate file name
        let result = project.load_named_arxml_buffer(FILEBUF.as_bytes(), "test", true);
        assert!(result.is_err());
        // error: overlapping autosar paths
        let result = project.load_named_arxml_buffer(FILEBUF.as_bytes(), "test2", true);
        assert!(result.is_err());
    }

    #[test]
    fn load_file() {
        let project = AutosarProject::new();
        assert!(project.load_arxml_file("nonexistent", true).is_err());
    }

    #[test]
    fn remove_file() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        let project = AutosarProject::new();
        let (file, _) = project
            .load_named_arxml_buffer(FILEBUF.as_bytes(), "test", true)
            .unwrap();
        assert_eq!(project.files().count(), 1);
        assert_eq!(project.identifiable_elements().count(), 1);
        project.remove_file(&file);
        assert_eq!(project.files().count(), 0);
        assert_eq!(project.identifiable_elements().count(), 0);
    }

    #[test]
    fn refcount() {
        let project = AutosarProject::default();
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
        let refpath = ref0.character_data().and_then(|cdata| cdata.string_value()).unwrap();
        if refpath != "/Pkg/System" && refpath != "/Some/Invalid/Path" {
            panic!("unexpected path: {refpath}");
        }
        project.get_element_by_path("/Pkg/EcuInstance").unwrap();
        let refs = project.get_references_to("/Pkg/EcuInstance");
        assert_eq!(refs.len(), 1);
        let refs = project.get_references_to("nonexistent");
        assert!(refs.is_empty());
    }

    #[test]
    fn serialize_files() {
        let project = AutosarProject::default();
        let file1 = project.create_file("filename1", AutosarVersion::Autosar_00042).unwrap();
        let file2 = project.create_file("filename2", AutosarVersion::Autosar_00042).unwrap();

        let result = project.serialize_files();
        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&PathBuf::from("filename1")).unwrap(), &file1.serialize());
        assert_eq!(result.get(&PathBuf::from("filename2")).unwrap(), &file2.serialize());
    }
}
