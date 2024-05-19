use std::{collections::HashMap, hash::Hash};

use crate::*;

impl AutosarModel {
    /// Create an `AutosarData` model
    ///
    /// Initially it contains no arxml files and only has a default `<AUTOSAR>` element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// let model = AutosarModel::new();
    /// ```
    ///
    #[must_use]
    pub fn new() -> AutosarModel {
        let version = AutosarVersion::LATEST;
        let xsi_schemalocation =
            CharacterData::String(format!("http://autosar.org/schema/r4.0 {}", version.filename()));
        let xmlns = CharacterData::String("http://autosar.org/schema/r4.0".to_string());
        let xmlns_xsi = CharacterData::String("http://www.w3.org/2001/XMLSchema-instance".to_string());
        let root_attributes = smallvec::smallvec![
            Attribute {
                attrname: AttributeName::xsiSchemalocation,
                content: xsi_schemalocation
            },
            Attribute {
                attrname: AttributeName::xmlns,
                content: xmlns
            },
            Attribute {
                attrname: AttributeName::xmlnsXsi,
                content: xmlns_xsi
            },
        ];
        let root_elem = ElementRaw {
            parent: ElementOrModel::None,
            elemname: ElementName::Autosar,
            elemtype: ElementType::ROOT,
            content: SmallVec::new(),
            attributes: root_attributes,
            file_membership: HashSet::with_capacity(0),
            comment: None,
        }
        .wrap();
        let model = AutosarModelRaw {
            files: Vec::new(),
            identifiables: FxIndexMap::default(),
            reference_origins: FxHashMap::default(),
            root_element: root_elem.clone(),
        }
        .wrap();
        root_elem.set_parent(ElementOrModel::Model(model.downgrade()));
        model
    }

    /// Create a new [`ArxmlFile`] inside this `AutosarData` structure
    ///
    /// You must provide a filename for the [`ArxmlFile`], even if you do not plan to write the data to disk.
    /// You must also specify an [`AutosarVersion`]. All methods manipulation the data insdie the file will ensure conformity with the version specified here.
    /// The newly created `ArxmlFile` will be created with a root AUTOSAR element.
    ///
    /// # Parameters
    ///
    ///  - `filename`: A filename for the data from the buffer. It must be unique within the model.
    ///    It will be used by `write()`, and is also used to identify this data in error messages.
    ///  - `version`: The [`AutosarVersion`] that will be used by the data created inside this file
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// let file = model.create_file("filename.arxml", AutosarVersion::Autosar_00050)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::DuplicateFilenameError`]: The model already contains a file with this filename
    ///
    pub fn create_file<P: AsRef<Path>>(
        &self,
        filename: P,
        version: AutosarVersion,
    ) -> Result<ArxmlFile, AutosarDataError> {
        let mut data = self.0.write();

        if data.files.iter().any(|af| af.filename() == filename.as_ref()) {
            return Err(AutosarDataError::DuplicateFilenameError {
                verb: "create",
                filename: filename.as_ref().to_path_buf(),
            });
        }

        let new_file = ArxmlFile::new(filename, version, self);

        data.files.push(new_file.clone());

        // every file contains the root element (but not its children)
        let _ = data.root_element.add_to_file_restricted(&new_file);

        Ok(new_file)
    }

    /// Load a named buffer containing arxml data
    ///
    /// If you have e.g. received arxml data over a network, or decompressed it from an archive, etc, then you may load it with this method.
    ///
    /// # Parameters:
    ///
    ///  - `buffer`: The data inside the buffer must be valid utf-8. Optionally it may begin with a UTF-8-BOM, which will be silently ignored.
    ///  - `filename`: the original filename of the data, or a newly generated name that is unique within the `AutosarData` instance.
    ///  - `strict`: toggle strict parsing. Some parsing errors are recoverable and can be issued as warnings.
    ///
    /// This method may be called concurrently on multiple threads to load different buffers
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// # let buffer = b"";
    /// model.load_buffer(buffer, "filename.arxml", true)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::DuplicateFilenameError`]: The model already contains a file with this filename
    ///  - [`AutosarDataError::OverlappingDataError`]: The new data contains Autosar paths that are already defined by the existing data
    ///  - [`AutosarDataError::ParserError`]: The parser detected an error; the source field gives further details
    ///
    pub fn load_buffer<P: AsRef<Path>>(
        &self,
        buffer: &[u8],
        filename: P,
        strict: bool,
    ) -> Result<(ArxmlFile, Vec<AutosarDataError>), AutosarDataError> {
        self.load_buffer_internal(buffer, filename.as_ref().to_path_buf(), strict)
    }

    fn load_buffer_internal(
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
        let version = parser.get_fileversion();
        let arxml_file = ArxmlFileRaw {
            version,
            model: self.downgrade(),
            filename: filename.clone(),
            xml_standalone: parser.get_standalone(),
        }
        .wrap();

        if self.0.read().files.is_empty() {
            root_element.set_parent(ElementOrModel::Model(self.downgrade()));
            root_element.0.write().file_membership.insert(arxml_file.downgrade());
            self.0.write().root_element = root_element;
        } else {
            let result = self.merge_file_data(&root_element, arxml_file.downgrade());
            if let Err(error) = result {
                let _ = self.root_element().remove_from_file(&arxml_file);
                return Err(error);
            }
        }

        let mut data = self.0.write();
        data.identifiables.reserve(parser.identifiables.len());
        for (key, value) in parser.identifiables {
            // the same identifiables can be present in multiple files
            // in this case we only keep the first one
            if let Some(existing_element) = data.identifiables.get(&key).and_then(WeakElement::upgrade) {
                // present in both
                if let Some(new_element) = value.upgrade() {
                    if existing_element.element_name() != new_element.element_name() {
                        // referenced element is different on both sides
                        return Err(AutosarDataError::OverlappingDataError {
                            filename,
                            path: new_element.xml_path(),
                        });
                    }
                }
            } else {
                data.identifiables.insert(key, value);
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

    // Merge the elements from an incoming arxml file into the overall model
    //
    // The Autosar standard specifies that the data can be split across multiple arxml files
    // It states that each ARXML file can represent an "AUTOSAR Partial Model".
    // The possible partitioning is marked in the meta model, where some elements have the attribute "splitable".
    // These are the points where the overall elements can be split into different arxml files, or, while loading, merged.
    // Unfortunately, the standard says nothing about how this should be done, so the algorithm here is just a guess.
    // In the wild, only merging at the AR-PACKAGES and at the ELEMENTS level exists. Everything else seems like a bad idea anyway.
    fn merge_file_data(&self, new_root: &Element, new_file: WeakArxmlFile) -> Result<(), AutosarDataError> {
        let root = self.root_element();
        let files: HashSet<WeakArxmlFile> = self.files().map(|f| f.downgrade()).collect();

        AutosarModel::merge_element(&root, &files, new_root, &new_file)?;
        self.root_element().0.write().file_membership.insert(new_file);

        Ok(())
    }

    fn merge_element(
        parent_a: &Element,
        files: &HashSet<WeakArxmlFile>,
        parent_b: &Element,
        new_file: &WeakArxmlFile,
    ) -> Result<(), AutosarDataError> {
        let mut iter_a = parent_a.sub_elements().enumerate();
        let mut iter_b = parent_b.sub_elements();
        let mut item_a = iter_a.next();
        let mut item_b = iter_b.next();
        let mut elements_a_only = Vec::<Element>::new();
        let mut elements_b_only = Vec::<(Element, usize)>::new();
        let mut elements_merge = Vec::<(Element, Element)>::new();
        let min_ver_a = files
            .iter()
            .filter_map(|weak| weak.upgrade().map(|f| f.version()))
            .min()
            .unwrap_or(AutosarVersion::LATEST);
        let min_ver_b = new_file.upgrade().map_or(AutosarVersion::LATEST, |f| f.version());
        let version = std::cmp::min(min_ver_a, min_ver_b);
        let splitable = parent_a.element_type().splittable_in(version);

        while let (Some((pos_a, elem_a)), Some(elem_b)) = (&item_a, &item_b) {
            if elem_a.element_name() == elem_b.element_name() {
                if elem_a.is_identifiable() {
                    if elem_a.item_name() == elem_b.item_name() {
                        // equal
                        // advance both iterators
                        elements_merge.push((elem_a.clone(), elem_b.clone()));
                        item_a = iter_a.next();
                        item_b = iter_b.next();
                    } else {
                        // assume that the ordering on both sides is different
                        // find a match for a among the siblings of b
                        if let Some(sibling) = parent_b
                            .sub_elements()
                            .find(|e| e.element_name() == elem_a.element_name() && e.item_name() == elem_a.item_name())
                        {
                            // matching item found
                            elements_merge.push((elem_a.clone(), sibling.clone()));
                        } else {
                            // element is unique in a
                            if splitable {
                                elements_a_only.push(elem_a.clone());
                            } else {
                                return Err(AutosarDataError::InvalidFileMerge {
                                    path: parent_a.xml_path(),
                                });
                            }
                        }
                        item_a = iter_a.next();
                    }
                } else {
                    // special case for BSW parameters - many elements used here don't have a SHORT-NAME, but they do have a DEFINITION-REF
                    let defref_a = elem_a
                        .get_sub_element(ElementName::DefinitionRef)
                        .and_then(|dr| dr.character_data())
                        .and_then(|cdata| cdata.string_value());
                    let defref_b = elem_b
                        .get_sub_element(ElementName::DefinitionRef)
                        .and_then(|dr| dr.character_data())
                        .and_then(|cdata| cdata.string_value());
                    // defref_a and _b are simply none for all other elements which don't have a definition-ref
                    if defref_a == defref_b {
                        // either: defrefs exist and are identical, OR they are both None
                        // if they are None, then there is nothing else that can be compared, so we just assume the elements are identical
                        // advance both iterators
                        elements_merge.push((elem_a.clone(), elem_b.clone()));
                        item_a = iter_a.next();
                        item_b = iter_b.next();
                    } else {
                        // check if a sibling of elem_b has the same definiton-ref as elem_a
                        // this handles the case where the the elements on both sides are ordered differently
                        if let Some(sibling) = parent_b
                            .sub_elements()
                            .filter(|e| e.element_name() == elem_a.element_name())
                            .find(|e| {
                                e.get_sub_element(ElementName::DefinitionRef)
                                    .and_then(|dr| dr.character_data())
                                    .and_then(|cdata| cdata.string_value())
                                    == defref_a
                            })
                        {
                            // a match for item_a exists
                            elements_merge.push((elem_a.clone(), sibling.clone()));
                        } else {
                            // element is unique in a
                            if splitable {
                                elements_a_only.push(elem_a.clone());
                            } else {
                                return Err(AutosarDataError::InvalidFileMerge {
                                    path: parent_a.xml_path(),
                                });
                            }
                        }
                        item_a = iter_a.next();
                    }
                }
            } else {
                // a and b are different kinds of elements. This is only allowed if parent is splittable
                let parent_type = parent_a.element_type();
                // The following check does not work, real examples still fail:
                // if !parent_type.splittable_in(self.version()) && parent_a.element_name() != ElementName::ArPackage {
                //     return Err(AutosarDataError::InvalidFileMerge { path: parent_a.xml_path() });
                // }

                let (_, indices_a) = parent_type.find_sub_element(elem_a.element_name(), u32::MAX).unwrap();
                let (_, indices_b) = parent_type.find_sub_element(elem_b.element_name(), u32::MAX).unwrap();
                if indices_a < indices_b {
                    // elem_a comes before elem_b, advance only a
                    // a: <parent> | <a = child 1> <child 2>
                    // b: <parent> |               <b = child 2>
                    elements_a_only.push(elem_a.clone());
                    item_a = iter_a.next();
                } else {
                    // elem_b comes before elem_a, advance only b
                    // a: <parent> |               <a = child 2>
                    // b: <parent> | <b = child 1> <child 2>
                    if !elements_merge.iter().any(|(_, merge_b)| merge_b == elem_b) {
                        elements_b_only.push((elem_b.clone(), *pos_a));
                        item_b = iter_b.next();
                    }
                }
            }
        }
        // at least one of the two iterators has reached the end
        // make sure the other one also reaches the end
        if let Some((_, elem_a)) = item_a {
            elements_a_only.push(elem_a);
            for (_, elem_a) in iter_a {
                elements_a_only.push(elem_a);
            }
        }
        if let Some(elem_b) = item_b {
            let elem_count = parent_a.0.read().content.len();
            if !elements_merge.iter().any(|(_, merge_b)| merge_b == &elem_b) {
                elements_b_only.push((elem_b, elem_count));
            }
            for elem_b in iter_b {
                if !elements_merge.iter().any(|(_, merge_b)| merge_b == &elem_b) {
                    elements_b_only.push((elem_b, elem_count));
                }
            }
        }

        // elements in elements_a_only are already present in the model, so they only need to be restricted
        for element in elements_a_only {
            // files contains the permisions of the parent
            let mut elem_locked = element.0.write();
            if elem_locked.file_membership.is_empty() {
                files.clone_into(&mut elem_locked.file_membership);
            }
        }
        // elements in elements_b_only are not present in the model. They need to be moved over and inserted at a reasonable position
        let mut parent_a_locked = parent_a.0.write();
        for (idx, (new_element, insert_pos)) in elements_b_only.into_iter().enumerate() {
            new_element.set_parent(ElementOrModel::Element(parent_a.downgrade()));
            // restrict new_element, it is only present in new_file
            new_element.0.write().file_membership.insert(new_file.clone());
            // add the new_element (from side b) to the content of parent_a
            // to do this, first check valid element insertion positions
            let (first_pos, last_pos) = parent_a_locked
                .calc_element_insert_range(new_element.element_name(), min_ver_b)
                .map_err(|_| AutosarDataError::InvalidFileMerge {
                    path: new_element.element_name().to_string(),
                })?;
            // idx number of elements have already been inserted, so the destination position must be adjusted
            let dest = insert_pos + idx;
            // clamp dest, so that first_pos <= dest <= last_pos
            let dest = dest.max(first_pos).min(last_pos);
            parent_a_locked
                .content
                .insert(dest, ElementContent::Element(new_element));
        }
        drop(parent_a_locked);

        // recurse for elements that need to be merged
        for (elem_a, elem_b) in elements_merge {
            let files = if !elem_a.0.read().file_membership.is_empty() {
                elem_a.0.read().file_membership.clone()
            } else {
                files.clone()
            };
            AutosarModel::merge_element(&elem_a, &files, &elem_b, new_file)?;
            let mut elem_a_locked = elem_a.0.write();
            if !elem_a_locked.file_membership.is_empty() {
                elem_a_locked.file_membership.insert(new_file.clone());
            }
        }

        Ok(())
    }

    /// Load an arxml file
    ///
    /// This function is a wrapper around `load_buffer` to make the common case of loading a file from disk more convenient
    ///
    /// # Parameters:
    ///
    ///  - `filename`: the original filename of the data, or a newly generated name that is unique within the `AutosarData` instance.
    ///  - `strict`: toggle strict parsing. Some parsing errors are recoverable and can be issued as warnings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// model.load_file("filename.arxml", true)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::IoErrorOpen`]: The file could not be opened
    ///  - [`AutosarDataError::IoErrorRead`]: There was an error while reading the file
    ///  - [`AutosarDataError::DuplicateFilenameError`]: The model already contains a file with this filename
    ///  - [`AutosarDataError::OverlappingDataError`]: The new data contains Autosar paths that are already defined by the existing data
    ///  - [`AutosarDataError::ParserError`]: The parser detected an error; the source field gives further details
    ///
    pub fn load_file<P: AsRef<Path>>(
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

        self.load_buffer(&buffer, &filename_buf, strict)
    }

    /// remove a file from the model
    ///
    /// # Parameters:
    ///
    ///  - `file`: The file that will be removed from the model
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// let file = model.create_file("filename.arxml", AutosarVersion::Autosar_00050)?;
    /// model.remove_file(&file);
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_file(&self, file: &ArxmlFile) {
        let mut locked_model = self.0.write();
        let find_result = locked_model
            .files
            .iter()
            .enumerate()
            .find(|(_, f)| *f == file)
            .map(|(pos, _)| pos);
        // find_result is stored first so that the lock on model is dropped
        if let Some(pos) = find_result {
            locked_model.files.swap_remove(pos);
            if locked_model.files.is_empty() {
                // no other files remain in the model, so it reverts to being empty
                locked_model.root_element.0.write().content.clear();
                locked_model.root_element.set_file_membership(HashSet::new());
                locked_model.identifiables.clear();
                locked_model.reference_origins.clear();
            } else {
                drop(locked_model);
                // other files still contribute elements, so only the elements specifically associated with this file should be removed
                let _ = self.root_element().remove_from_file(file);
                // self.unmerge_file(&file.downgrade());
            }
        }
    }

    /// serialize each of the files in the model
    ///
    /// returns the result in a `HashMap` of <`file_name`, `file_content`>
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// for (pathbuf, file_content) in model.serialize_files() {
    ///     // do something with it
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[must_use]
    pub fn serialize_files(&self) -> HashMap<PathBuf, String> {
        let mut result = HashMap::new();
        for file in self.files() {
            if let Ok(data) = file.serialize() {
                result.insert(file.filename(), data);
            }
        }
        result
    }

    /// write all files in the model
    ///
    /// This is a wrapper around `serialize_files`. The current filename of each file will be used to write the serialized data.
    ///
    /// If any of the individual files cannot be written, then `write()` will abort and return the error.
    /// This may result in a situation where some files have been written and others have not.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// // load or create files
    /// model.write()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::IoErrorWrite`]: There was an error while writing a file
    pub fn write(&self) -> Result<(), AutosarDataError> {
        for (pathbuf, filedata) in self.serialize_files() {
            std::fs::write(pathbuf.clone(), filedata).map_err(|err| AutosarDataError::IoErrorWrite {
                filename: pathbuf,
                ioerror: err,
            })?;
        }
        Ok(())
    }

    /// create an iterator over all [`ArxmlFile`]s in this `AutosarData` object
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// // load or create files
    /// for file in model.files() {
    ///     // do something with the file
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn files(&self) -> ArxmlFileIterator {
        ArxmlFileIterator::new(self.clone())
    }

    /// Get a reference to the root ```<AUTOSAR ...>``` element of this model
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let _file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let autosar_element = model.root_element();
    /// ```
    #[must_use]
    pub fn root_element(&self) -> Element {
        let locked_model = self.0.read();
        locked_model.root_element.clone()
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
    /// let model = AutosarModel::new();
    /// // [...]
    /// if let Some(element) = model.get_element_by_path("/Path/To/Element") {
    ///     // use the element
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_element_by_path(&self, path: &str) -> Option<Element> {
        let model = self.0.read();
        model.identifiables.get(path).and_then(WeakElement::upgrade)
    }

    /// Duplicate the model
    ///
    /// This creates a second, fully independent model.
    /// The original model and the duplicate are not linked in any way and can be modified independently.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// // [...]
    /// let model_copy = model.duplicate()?;
    /// assert!(model != model_copy);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn duplicate(&self) -> Result<AutosarModel, AutosarDataError> {
        let copy = Self::new();
        let mut filemap = HashMap::new();

        for orig_file in self.files() {
            let filename = orig_file.filename();
            let new_file = copy.create_file(filename.clone(), orig_file.version())?;
            new_file.0.write().xml_standalone = orig_file.0.read().xml_standalone;
            filemap.insert(filename, new_file.downgrade());
        }

        // by inserting copies of the sub elements of <AUTOSAR>, we automatically
        // get up-to-date identifiables and reference_origins
        for element in self.root_element().sub_elements() {
            copy.root_element().create_copied_sub_element(&element)?;
        }

        // `create_copied_sub_element` does not transfer information about file membership
        // this needs to be added back
        let orig_iter = self.elements_dfs();
        let copy_iter = copy.elements_dfs();
        let combined = std::iter::zip(orig_iter, copy_iter);
        for ((_, orig_elem), (_, copy_elem)) in combined {
            let mut locked_copy = copy_elem.0.try_write().ok_or(AutosarDataError::ParentElementLocked)?;
            locked_copy.file_membership.clear();

            for orig_file in orig_elem.0.read().file_membership.iter().filter_map(|w| w.upgrade()) {
                if let Some(copy_file) = filemap.get(&orig_file.filename()) {
                    locked_copy.file_membership.insert(copy_file.clone());
                }
            }
        }

        Ok(copy)
    }

    /// create a depth-first iterator over all [Element]s in the model
    ///
    /// The iterator returns all elements from the merged model, consisting of
    /// data from all arxml files loaded in this model.
    ///
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
    /// # let model = AutosarModel::new();
    /// for (depth, element) in model.elements_dfs() {
    ///     // [...]
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn elements_dfs(&self) -> ElementsDfsIterator {
        self.root_element().elements_dfs()
    }

    /// Recursively sort all elements in the model. This is exactly identical to calling `sort()` on the root element of the model.
    ///
    /// All sub elements of the root element are sorted alphabetically.
    /// If the sub-elements are named, then the sorting is performed according to the item names,
    /// otherwise the serialized form of the sub-elements is used for sorting.
    ///
    /// Element attributes are not taken into account while sorting.
    /// The elements are sorted in place, and sorting cannot fail, so there is no return value.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// model.sort();
    /// ```
    pub fn sort(&self) {
        self.root_element().sort();
    }

    /// Create an iterator over the list of the Autosar paths of all identifiable elements
    ///
    /// The list contains the full Autosar path of each element. It is not sorted.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// for (path, _) in model.identifiable_elements() {
    ///     let element = model.get_element_by_path(&path).unwrap();
    ///     // [...]
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn identifiable_elements(&self) -> IdentifiablesIterator {
        IdentifiablesIterator::new(self)
    }

    /// return all elements referring to the given target path
    ///
    /// It returns [`WeakElement`]s which must be upgraded to get usable [Element]s.
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
    /// # let model = AutosarModel::new();
    /// for weak_element in model.get_references_to("/Path/To/Element") {
    ///     // [...]
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_references_to(&self, target_path: &str) -> Vec<WeakElement> {
        if let Some(origins) = self.0.read().reference_origins.get(target_path) {
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
    /// # let model = AutosarModel::new();
    /// for broken_ref_weak in model.check_references() {
    ///     if let Some(broken_ref) = broken_ref_weak.upgrade() {
    ///         // update or delete ref?
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn check_references(&self) -> Vec<WeakElement> {
        let mut broken_refs = Vec::new();

        let model = self.0.read();
        for (path, element_list) in &model.reference_origins {
            if let Some(target_elem_weak) = model.identifiables.get(path) {
                // reference target exists
                if let Some(target_elem) = target_elem_weak.upgrade() {
                    // the target of the reference exists, but the reference can still be technically invalid
                    // if the content of the DEST attribute on the reference is wrong
                    for referring_elem_weak in element_list {
                        if let Some(referring_elem) = referring_elem_weak.upgrade() {
                            if let Some(CharacterData::Enum(dest_value)) =
                                referring_elem.attribute_value(AttributeName::Dest)
                            {
                                if target_elem.element_type().verify_reference_dest(dest_value) {
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
    pub(crate) fn downgrade(&self) -> WeakAutosarModel {
        WeakAutosarModel(Arc::downgrade(&self.0))
    }

    // add an identifiable element to the cache
    pub(crate) fn add_identifiable(&self, new_path: String, elem: WeakElement) {
        let mut model = self.0.write();
        model.identifiables.insert(new_path, elem);
    }

    // fix a single identifiable element or tree of elements in the cache which has been moved/renamed
    pub(crate) fn fix_identifiables(&self, old_path: &str, new_path: &str) {
        let mut model = self.0.write();

        // the renamed element might contain other identifiable elements that are affected by the renaming
        let keys: Vec<String> = model.identifiables.keys().cloned().collect();
        for key in keys {
            // find keys referring to entries inside the renamed package
            if let Some(suffix) = key.strip_prefix(old_path) {
                if suffix.is_empty() || suffix.starts_with('/') {
                    let new_key = format!("{new_path}{suffix}");
                    // fix the identifiables hashmap
                    if let Some(entry) = model.identifiables.swap_remove(&key) {
                        model.identifiables.insert(new_key, entry);
                    }
                }
            }
        }
    }

    // remove a deleted element from the cache
    pub(crate) fn remove_identifiable(&self, path: &str) {
        let mut model = self.0.write();
        model.identifiables.swap_remove(path);
    }

    pub(crate) fn add_reference_origin(&self, new_ref: &str, origin: WeakElement) {
        let mut data = self.0.write();
        // add the new entry
        if let Some(referrer_list) = data.reference_origins.get_mut(new_ref) {
            referrer_list.push(origin);
        } else {
            data.reference_origins.insert(new_ref.to_owned(), vec![origin]);
        }
    }

    pub(crate) fn fix_reference_origins(&self, old_ref: &str, new_ref: &str, origin: WeakElement) {
        if old_ref != new_ref {
            let mut data = self.0.write();
            let mut remove_list = false;
            // remove the old entry
            if let Some(referrer_list) = data.reference_origins.get_mut(old_ref) {
                if let Some(index) = referrer_list.iter().position(|x| *x == origin) {
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
        let mut data = self.0.write();
        let mut count = 1;
        if let Some(referrer_list) = data.reference_origins.get_mut(reference) {
            if let Some(index) = referrer_list.iter().position(|x| *x == element) {
                referrer_list.swap_remove(index);
            }
            count = referrer_list.len();
        }
        if count == 0 {
            data.reference_origins.remove(reference);
        }
    }
}

impl AutosarModelRaw {
    pub(crate) fn set_version(&mut self, new_ver: AutosarVersion) {
        let attribute_value = CharacterData::String(format!("http://autosar.org/schema/r4.0 {}", new_ver.filename()));
        let _ = self.root_element.0.write().set_attribute_internal(
            AttributeName::xsiSchemalocation,
            attribute_value,
            new_ver,
        );
    }

    pub(crate) fn wrap(self) -> AutosarModel {
        AutosarModel(Arc::new(RwLock::new(self)))
    }
}

impl std::fmt::Debug for AutosarModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let model = self.0.read();
        // instead of the usual f.debug_struct().field().field() ...
        // this is disassembled here, in order to hold self.0.lock() as briefly as possible
        let rootelem = model.root_element.clone();
        let mut dbgstruct = f.debug_struct("AutosarModel");
        dbgstruct.field("root_element", &rootelem);
        dbgstruct.field("files", &model.files);
        dbgstruct.field("identifiables", &model.identifiables);
        dbgstruct.field("reference_origins", &model.reference_origins);
        dbgstruct.finish()
    }
}

impl Default for AutosarModel {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for AutosarModel {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

impl Eq for AutosarModel {}

impl Hash for AutosarModel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Arc::as_ptr(&self.0) as usize);
    }
}

impl WeakAutosarModel {
    pub(crate) fn upgrade(&self) -> Option<AutosarModel> {
        Weak::upgrade(&self.0).map(AutosarModel)
    }
}

impl std::fmt::Debug for WeakAutosarModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("AutosarModel:WeakRef {:p}", Weak::as_ptr(&self.0)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn create_file() {
        let model = AutosarModel::new();
        let file = model.create_file("test", AutosarVersion::Autosar_00050);
        assert!(file.is_ok());
        // error: duplicate file name
        let file = model.create_file("test", AutosarVersion::Autosar_00050);
        assert!(file.is_err());
    }

    #[test]
    fn load_buffer() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE>
            <SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
              <SYSTEM><SHORT-NAME>Thing</SHORT-NAME></SYSTEM>
            </ELEMENTS>
          </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const FILEBUF2: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE><SHORT-NAME>OtherPkg</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const FILEBUF3: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE>
            <SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
            <APPLICATION-PRIMITIVE-DATA-TYPE><SHORT-NAME>Thing</SHORT-NAME></APPLICATION-PRIMITIVE-DATA-TYPE>
            </ELEMENTS>
          </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const NON_ARXML: &str = "The quick brown fox jumps over the lazy dog";
        let model = AutosarModel::new();
        // succefully load a buffer
        let result = model.load_buffer(FILEBUF.as_bytes(), "test", true);
        assert!(result.is_ok());
        // succefully load a second buffer
        let result = model.load_buffer(FILEBUF2.as_bytes(), "other", true);
        assert!(result.is_ok());
        // error: duplicate file name
        let result = model.load_buffer(FILEBUF.as_bytes(), "test", true);
        assert!(result.is_err());
        // error: overlapping autosar paths
        let result = model.load_buffer(FILEBUF3.as_bytes(), "test2", true);
        assert!(result.is_err());
        // error: not arxml data
        let result = model.load_buffer(NON_ARXML.as_bytes(), "nonsense", true);
        assert!(result.is_err());
    }

    #[test]
    fn load_file() {
        let dir = tempdir().unwrap();

        let model = AutosarModel::new();
        let filename = dir.path().with_file_name("nonexistent.arxml");
        assert!(model.load_file(&filename, true).is_err());

        let filename = dir.path().with_file_name("test.arxml");
        model.create_file(&filename, AutosarVersion::LATEST).unwrap();
        model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .and_then(|ap| ap.create_named_sub_element(ElementName::ArPackage, "Pkg"))
            .unwrap();
        model.write().unwrap();

        assert!(filename.exists());

        // careate a new model without data
        let model = AutosarModel::new();
        model.load_file(&filename, true).unwrap();
        let el_pkg = model.get_element_by_path("/Pkg");
        assert!(el_pkg.is_some());
    }

    #[test]
    fn data_merge() {
        const FILEBUF1: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE><SHORT-NAME>Pkg_A</SHORT-NAME><ELEMENTS>
            <ECUC-MODULE-CONFIGURATION-VALUES><SHORT-NAME>BswModule</SHORT-NAME><CONTAINERS><ECUC-CONTAINER-VALUE>
              <SHORT-NAME>BswModuleValues</SHORT-NAME>
              <PARAMETER-VALUES>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_A</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_B</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_C</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
              </PARAMETER-VALUES>
            </ECUC-CONTAINER-VALUE></CONTAINERS></ECUC-MODULE-CONFIGURATION-VALUES>
          </ELEMENTS></AR-PACKAGE>
          <AR-PACKAGE><SHORT-NAME>Pkg_B</SHORT-NAME></AR-PACKAGE>
          <AR-PACKAGE><SHORT-NAME>Pkg_C</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#.as_bytes();
        const FILEBUF2: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE><SHORT-NAME>Pkg_B</SHORT-NAME></AR-PACKAGE>
          <AR-PACKAGE><SHORT-NAME>Pkg_A</SHORT-NAME><ELEMENTS>
            <ECUC-MODULE-CONFIGURATION-VALUES><SHORT-NAME>BswModule</SHORT-NAME><CONTAINERS><ECUC-CONTAINER-VALUE>
              <SHORT-NAME>BswModuleValues</SHORT-NAME>
              <PARAMETER-VALUES>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_B</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_A</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
              </PARAMETER-VALUES>
            </ECUC-CONTAINER-VALUE></CONTAINERS></ECUC-MODULE-CONFIGURATION-VALUES>
          </ELEMENTS></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#.as_bytes();
        // test with re-ordered identifiable elements and re-ordered BSW parameter values
        // file2 is a subset of file1, so the total number of elements does not increase
        let model = AutosarModel::new();
        let (file1, _) = model.load_buffer(FILEBUF1, "test1", true).unwrap();
        let file1_elemcount = file1.elements_dfs().count();
        let (file2, _) = model.load_buffer(FILEBUF2, "test2", true).unwrap();
        let file2_elemcount = file2.elements_dfs().count();
        let model_elemcount = model.elements_dfs().count();
        assert_eq!(file1_elemcount, model_elemcount);
        assert!(file1_elemcount > file2_elemcount);
        // verify file membership after merging
        let (local, fileset) = model.root_element().file_membership().unwrap();
        assert_eq!(local, true);
        assert_eq!(fileset.len(), 2);

        let el_pkg_c = model.get_element_by_path("/Pkg_C").unwrap();
        let (local, fileset) = el_pkg_c.file_membership().unwrap();
        assert_eq!(local, true);
        assert_eq!(fileset.len(), 1);
        let el_npv2 = model
            .get_element_by_path("/Pkg_A/BswModule/BswModuleValues")
            .and_then(|bmv| bmv.get_sub_element(ElementName::ParameterValues))
            .and_then(|pv| pv.get_sub_element_at(2))
            .unwrap();
        let (loc, fm) = el_npv2.file_membership().unwrap();
        assert_eq!(loc, true);
        assert_eq!(fm.len(), 1);

        // the following two files diverge on the TIMING-RESOURCE element
        // this is not permitted, because SYSTEM-TIMING is not splittable
        const ERRFILE1: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME>
          <ELEMENTS>
            <SYSTEM-TIMING>
              <SHORT-NAME>SystemTimings</SHORT-NAME>
              <CATEGORY>CAT</CATEGORY>
              <TIMING-RESOURCE>
                <SHORT-NAME>Name_One</SHORT-NAME>
              </TIMING-RESOURCE>
            </SYSTEM-TIMING>
          </ELEMENTS>
        </AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#.as_bytes();
        const ERRFILE2: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME>
          <ELEMENTS>
            <SYSTEM-TIMING>
              <SHORT-NAME>SystemTimings</SHORT-NAME>
              <TIMING-RESOURCE>
                <SHORT-NAME>Name_Two</SHORT-NAME>
              </TIMING-RESOURCE>
            </SYSTEM-TIMING>
          </ELEMENTS>
        </AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#.as_bytes();
        let model = AutosarModel::new();
        let result = model.load_buffer(ERRFILE1, "test1", true);
        assert!(result.is_ok());
        let result = model.load_buffer(ERRFILE2, "test2", true);
        let error = result.unwrap_err();
        assert!(matches!(error, AutosarDataError::InvalidFileMerge { .. }));
    }

    #[test]
    fn remove_file() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const FILEBUF2: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00049.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME>
        <ELEMENTS><CAN-CLUSTER><SHORT-NAME>CAN_Cluster</SHORT-NAME></CAN-CLUSTER></ELEMENTS>
        </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const FILEBUF3: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00048.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>Package2</SHORT-NAME>
        <ELEMENTS><SYSTEM><SHORT-NAME>System</SHORT-NAME>
        <FIBEX-ELEMENTS><FIBEX-ELEMENT-REF-CONDITIONAL>
            <FIBEX-ELEMENT-REF DEST="CAN-CLUSTER">/Package/CAN_Cluster</FIBEX-ELEMENT-REF>
        </FIBEX-ELEMENT-REF-CONDITIONAL></FIBEX-ELEMENTS>
        </SYSTEM></ELEMENTS></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        // easy case: remove the only file
        let model = AutosarModel::new();
        let (file, _) = model.load_buffer(FILEBUF.as_bytes(), "test", true).unwrap();
        assert_eq!(model.files().count(), 1);
        assert_eq!(model.identifiable_elements().count(), 1);
        model.remove_file(&file);
        assert_eq!(model.files().count(), 0);
        assert_eq!(model.identifiable_elements().count(), 0);
        // complicated: remove one of several files
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF.as_bytes(), "test1", true).unwrap();
        assert_eq!(model.files().count(), 1);
        let modeltxt_1 = model.root_element().serialize();
        let (file2, _) = model.load_buffer(FILEBUF2.as_bytes(), "test2", true).unwrap();
        assert_eq!(model.files().count(), 2);
        let modeltxt_1_2 = model.root_element().serialize();
        assert_ne!(modeltxt_1, modeltxt_1_2);
        let (file3, _) = model.load_buffer(FILEBUF3.as_bytes(), "test3", true).unwrap();
        assert_eq!(model.files().count(), 3);
        let modeltxt_1_2_3 = model.root_element().serialize();
        assert_ne!(modeltxt_1_2, modeltxt_1_2_3);
        model.get_element_by_path("/Package2/System").unwrap();
        model.remove_file(&file3);
        // the serialized text of the model after deleting file 3 should be the same as it was before loading file 3
        let modeltxt_1_2_x = model.root_element().serialize();
        assert_eq!(modeltxt_1_2, modeltxt_1_2_x);
        model.remove_file(&file2);
        // the serialized text of the model after deleting files 2 and 3 should be the same as it was before loading files 2 and 3
        let modeltxt_1_x_x = model.root_element().serialize();
        assert_eq!(modeltxt_1, modeltxt_1_x_x);
        assert_eq!(model.files().count(), 1);
    }

    #[test]
    fn refcount() {
        let model = AutosarModel::default();
        let weak = model.downgrade();
        let project2 = weak.upgrade();
        assert_eq!(Arc::strong_count(&model.0), 2);
        assert_eq!(model, project2.unwrap());
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
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF.as_bytes(), "test", true).unwrap();
        let mut identifiable_elements = model.identifiable_elements().collect::<Vec<_>>();
        identifiable_elements.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(identifiable_elements[0].0, "/OuterPackage1");
        assert_eq!(identifiable_elements[1].0, "/OuterPackage1/InnerPackage1");
        assert_eq!(identifiable_elements[2].0, "/OuterPackage1/InnerPackage2");
        assert_eq!(identifiable_elements[3].0, "/OuterPackage2");
        assert_eq!(identifiable_elements[4].0, "/OuterPackage2/InnerPackage1");
        assert_eq!(identifiable_elements[5].0, "/OuterPackage2/InnerPackage2");
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
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF.as_bytes(), "test", true).unwrap();
        let el_fibex_elements = model
            .get_element_by_path("/Pkg/System")
            .and_then(|sys| sys.get_sub_element(ElementName::FibexElements))
            .unwrap();
        let el_fibex_element_ref = el_fibex_elements
            .create_sub_element(ElementName::FibexElementRefConditional)
            .and_then(|ferc| ferc.create_sub_element(ElementName::FibexElementRef))
            .unwrap();
        el_fibex_element_ref
            .set_character_data(CharacterData::String("/Pkg/System".to_string()))
            .unwrap();
        let invalid_refs = model.check_references();
        assert_eq!(invalid_refs.len(), 3);
        let ref0 = invalid_refs[0].upgrade().unwrap();
        assert_eq!(ref0.element_name(), ElementName::FibexElementRef);
        let refpath = ref0.character_data().and_then(|cdata| cdata.string_value()).unwrap();
        // there is no defined order in which the references will be checked, so any of the three broken refs could be returned first
        assert!(refpath == "/Pkg/System" || refpath == "/Some/Invalid/Path");

        model.get_element_by_path("/Pkg/EcuInstance").unwrap();
        let refs = model.get_references_to("/Pkg/EcuInstance");
        assert_eq!(refs.len(), 1);
        let refs = model.get_references_to("nonexistent");
        assert!(refs.is_empty());
    }

    #[test]
    fn serialize_files() {
        let model = AutosarModel::default();
        let file1 = model.create_file("filename1", AutosarVersion::Autosar_00042).unwrap();
        let file2 = model.create_file("filename2", AutosarVersion::Autosar_00042).unwrap();

        let result = model.serialize_files();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result.get(&PathBuf::from("filename1")).unwrap(),
            &file1.serialize().unwrap()
        );
        assert_eq!(
            result.get(&PathBuf::from("filename2")).unwrap(),
            &file2.serialize().unwrap()
        );
    }

    #[test]
    fn duplicate() {
        let model = AutosarModel::new();
        let file1 = model.create_file("filename1", AutosarVersion::Autosar_00042).unwrap();
        let file2 = model.create_file("filename2", AutosarVersion::Autosar_00042).unwrap();
        let el_ar_packages = model.root_element().create_sub_element(ElementName::ArPackages).unwrap();
        let el_pkg1 = el_ar_packages.create_named_sub_element(ElementName::ArPackage, "pkg1").unwrap();
        let el_pkg2 = el_ar_packages.create_named_sub_element(ElementName::ArPackage, "pkg2").unwrap();

        assert_eq!(el_ar_packages.file_membership().unwrap().1.len(), 2);
        el_pkg1.remove_from_file(&file2).unwrap();
        assert_eq!(el_pkg1.file_membership().unwrap().1.len(), 1);
        el_pkg2.remove_from_file(&file1).unwrap();
        assert_eq!(el_pkg2.file_membership().unwrap().1.len(), 1);

        let model2 = model.duplicate().unwrap();
        assert_eq!(model2.files().count(), 2);
        let mut files_iter = model2.files();
        // get the files out of model 2
        let mut model2_file1 = files_iter.next().unwrap();
        let mut model2_file2 = files_iter.next().unwrap();
        // the iterator could return the files in any order - make sure that model2_file1 corresponds to file1
        if model2_file1.filename() != file1.filename() {
            std::mem::swap(&mut model2_file1, &mut model2_file2);
        }

        assert_eq!(file1.filename(), model2_file1.filename());
        assert_eq!(file2.filename(), model2_file2.filename());
        assert_eq!(file1.serialize().unwrap(), model2_file1.serialize().unwrap());
        assert_eq!(file2.serialize().unwrap(), model2_file2.serialize().unwrap());
    }

    #[test]
    fn write() {
        let model = AutosarModel::default();
        model.write().unwrap();
    }

    #[test]
    fn traits() {
        // AutosarModel: Debug, Clone, Hash
        let model = AutosarModel::new();
        let model_cloned = model.clone();
        assert_eq!(model, model_cloned);
        assert_eq!(format!("{model:#?}"), format!("{model_cloned:#?}"));
        let mut hashset = HashSet::<AutosarModel>::new();
        hashset.insert(model);
        let inserted = hashset.insert(model_cloned);
        assert!(!inserted);

        // CharacterData
        let cdata = CharacterData::String("x".to_string());
        let cdata2 = cdata.clone();
        assert_eq!(cdata, cdata2);
        assert_eq!(format!("{cdata:#?}"), format!("{cdata2:#?}"));

        // ContentType
        let ct: ContentType = ContentType::Elements;
        let ct2 = ct.clone();
        assert_eq!(ct, ct2);
        assert_eq!(format!("{ct:#?}"), format!("{ct2:#?}"));
    }
}
