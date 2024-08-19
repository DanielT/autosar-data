use std::hash::Hash;

use crate::*;

impl ArxmlFile {
    pub(crate) fn new<P: AsRef<Path>>(filename: P, version: AutosarVersion, model: &AutosarModel) -> Self {
        ArxmlFileRaw {
            version,
            model: model.downgrade(),
            filename: filename.as_ref().to_path_buf(),
            xml_standalone: None,
        }
        .wrap()
    }

    /// Get the filename of this `ArxmlFile`
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// println!("filename is : {}", file.filename().display());
    /// ```
    #[must_use]
    pub fn filename(&self) -> PathBuf {
        self.0.read().filename.clone()
    }

    /// Get the [`AutosarVersion`] of the file
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let version = file.version();
    /// ```
    #[must_use]
    pub fn version(&self) -> AutosarVersion {
        self.0.read().version
    }

    /// Set the [`AutosarVersion`] of the file
    ///
    /// The compatibility of the data in the file with the new version will be checked before setting the version.
    /// The compatibility check can also be performed manually using the function `check_version_compatibility()`.
    ///
    /// If the data is compatible, then the version is set, otherwise an error is raised.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// file.set_version(AutosarVersion::Autosar_00050);
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::VersionIncompatibleData`] the existing data is not compatible with the new version
    ///
    pub fn set_version(&self, new_ver: AutosarVersion) -> Result<(), AutosarDataError> {
        let (compat_errors, _) = self.check_version_compatibility(new_ver);
        if compat_errors.is_empty() {
            let mut file = self.0.write();
            file.version = new_ver;
            Ok(())
        } else {
            Err(AutosarDataError::VersionIncompatibleData { version: new_ver })
        }
    }

    /// Check if the elements and attributes in this file are compatible with some `target_version`
    ///
    /// All elements and their attributes will be evaluated against the target version according to the specification.
    /// The output is a list of incompatible elements
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let (error_list, compat_mask) = file.check_version_compatibility(AutosarVersion::Autosar_00050);
    /// ```
    #[must_use]
    pub fn check_version_compatibility(&self, target_version: AutosarVersion) -> (Vec<CompatibilityError>, u32) {
        if let Ok(model) = self.model() {
            model
                .root_element()
                .check_version_compatibility(&self.downgrade(), target_version)
        } else {
            (Vec::new(), 0)
        }
    }

    /// Set the filename of this arxml filename
    ///
    /// This will not rename any existing file on disk, but the new filename will be used when writing the data.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::path::Path;
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// file.set_filename("foo.arxml");
    /// // or
    /// file.set_filename(&Path::new("bar.arxml"));
    /// ```
    pub fn set_filename<P: AsRef<Path>>(&self, new_filename: P) -> Result<(), AutosarDataError> {
        let new_filename = new_filename.as_ref().to_path_buf();
        if self
            .model()?
            .files()
            .map(|f| (f.clone(), f.filename()))
            .any(|(file, filename)| file != *self && filename == new_filename)
        {
            Err(AutosarDataError::DuplicateFilenameError {
                verb: "set_filename",
                filename: new_filename,
            })
        } else {
            self.0.write().filename = new_filename;
            Ok(())
        }
    }

    /// Get a reference to the [`AutosarModel`] object that contains this file
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let m2 = file.model()?;
    /// assert_eq!(model, m2);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// [`AutosarDataError::ItemDeleted`]: The model is no longer valid
    ///
    pub fn model(&self) -> Result<AutosarModel, AutosarDataError> {
        let locked_file = self.0.write();
        // This reference must always be valid, so it is an error if upgrade() fails
        locked_file.model.upgrade().ok_or(AutosarDataError::ItemDeleted)
    }

    /// Create a depth-first search iterator over all [Element]s in this file
    ///
    /// In a multi-file model it will not return any elements from other files.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// for (depth, elem) in file.elements_dfs() {
    ///     // ...
    /// }
    /// ```
    #[must_use]
    pub fn elements_dfs(&self) -> ArxmlFileElementsDfsIterator {
        ArxmlFileElementsDfsIterator::new(self, 0)
    }

    /// Create a depth first iterator over all [Element]s in this file, up to a maximum depth
    ///
    /// In a multi-file model it will not return any elements from other files.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// # element.create_sub_element(ElementName::ArPackages).unwrap();
    /// # let sub_elem = element.get_sub_element(ElementName::ArPackages).unwrap();
    /// # sub_elem.create_named_sub_element(ElementName::ArPackage, "test2").unwrap();
    /// for (depth, elem) in file.elements_dfs_with_max_depth(1) {
    ///     assert!(depth <= 1);
    ///     // ...
    /// }
    /// ```
    #[must_use]
    pub fn elements_dfs_with_max_depth(&self, max_depth: usize) -> ArxmlFileElementsDfsIterator {
        ArxmlFileElementsDfsIterator::new(self, max_depth)
    }

    /// Serialize the content of the file to a String
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let text = file.serialize();
    /// ```
    ///
    /// # Errors
    ///
    /// [`AutosarDataError::ItemDeleted`]: The model is no longer valid
    /// [`AutosarDataError::EmptyFile`]: The file is empty and cannot be serialized
    pub fn serialize(&self) -> Result<String, AutosarDataError> {
        let model = self.model()?;
        if !model.root_element().file_membership()?.1.contains(&self.downgrade()) {
            return Err(AutosarDataError::EmptyFile);
        }

        let mut outstring = String::with_capacity(1024 * 1024);

        match self.xml_standalone() {
            Some(true) => outstring.push_str("<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>"),
            Some(false) => outstring.push_str("<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"no\"?>"),
            None => outstring.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>"),
        }
        model.0.write().set_version(self.0.read().version);
        model
            .root_element()
            .serialize_internal(&mut outstring, 0, false, &Some(self.downgrade()));

        Ok(outstring)
    }

    /// Return the standalone attribute from the xml header
    ///
    /// Some tools set headers that include the standalone attribute.
    /// This attribute appears to be meaningless for arxml files.
    ///
    /// It is preserved nontheless and can be retrieved with this function.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// let model = AutosarModel::new();
    /// let file_text = r#"<?xml version="1.0" encoding="utf-8" standalone="no"?>
    /// <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    /// </AUTOSAR>"#.as_bytes();
    /// let (file, _warnings) = model.load_buffer(file_text, "filename.arxml", true).unwrap();
    /// assert_eq!(file.xml_standalone(), Some(false));
    /// ```
    #[must_use]
    pub fn xml_standalone(&self) -> Option<bool> {
        self.0.read().xml_standalone
    }

    /// Create a weak reference to this `ArxmlFile`
    ///
    /// A weak reference can be stored without preventing the file from being deallocated.
    /// The weak reference has to be upgraded in order to be used, which can fail if the file no longer exists.
    ///
    /// See the documentation for [Arc]
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let weak_file = file.downgrade();
    /// ```
    #[must_use]
    pub fn downgrade(&self) -> WeakArxmlFile {
        WeakArxmlFile(Arc::downgrade(&self.0))
    }
}

impl ArxmlFileRaw {
    pub(crate) fn wrap(self) -> ArxmlFile {
        ArxmlFile(Arc::new(RwLock::new(self)))
    }
}

impl PartialEq for ArxmlFile {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

impl Eq for ArxmlFile {}

impl Hash for ArxmlFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Arc::as_ptr(&self.0) as usize);
    }
}

impl std::fmt::Debug for ArxmlFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_locked = self.0.read();
        f.debug_struct("ArxmlFile")
            .field("filename", &self_locked.filename)
            .field("version", &self_locked.version)
            .field("model", &self_locked.model)
            .field("xml_standalone", &self_locked.xml_standalone)
            .finish()
    }
}

impl WeakArxmlFile {
    /// try to get a strong reference to the [`ArxmlFile`]
    ///
    /// This succeeds if the `ArxmlFile` still has any other strong reference to it, otherwise None is returned
    pub fn upgrade(&self) -> Option<ArxmlFile> {
        Weak::upgrade(&self.0).map(ArxmlFile)
    }
}

impl PartialEq for WeakArxmlFile {
    fn eq(&self, other: &Self) -> bool {
        Weak::as_ptr(&self.0) == Weak::as_ptr(&other.0)
    }
}

impl Eq for WeakArxmlFile {}

impl Hash for WeakArxmlFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Weak::as_ptr(&self.0) as usize);
    }
}

impl std::fmt::Debug for WeakArxmlFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ArxmlFile:WeakRef {:p}", Weak::as_ptr(&self.0)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let model = AutosarModel::new();
        let result = model.create_file("test", AutosarVersion::Autosar_4_0_1);
        assert!(result.is_ok());
    }

    #[test]
    fn filename() {
        let model = AutosarModel::new();
        let result = model.create_file("test", AutosarVersion::Autosar_4_0_1);
        let file = result.unwrap();
        let filename = PathBuf::from("newname.arxml");
        file.set_filename(filename.clone()).unwrap();
        assert_eq!(file.filename(), filename);
    }

    #[test]
    fn version() {
        let model: AutosarModel = AutosarModel::new();
        let file = model.create_file("test", AutosarVersion::Autosar_00051).unwrap();

        let el_elements = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .and_then(|arpkgs| arpkgs.create_named_sub_element(ElementName::ArPackage, "Pkg"))
            .and_then(|arpkg| arpkg.create_sub_element(ElementName::Elements))
            .unwrap();
        let incompatible_elem = el_elements
            .create_named_sub_element(ElementName::AdaptiveApplicationSwComponentType, "incompatible")
            .unwrap();

        let result = file.set_version(AutosarVersion::Autosar_4_0_1);
        assert!(result.is_err());

        el_elements.remove_sub_element(incompatible_elem).unwrap();

        file.set_version(AutosarVersion::Autosar_4_0_1).unwrap();
        assert_eq!(file.version(), AutosarVersion::Autosar_4_0_1);
    }

    #[test]
    fn references() {
        let model = AutosarModel::new();
        let result = model.create_file("test", AutosarVersion::Autosar_4_0_1);
        let file = result.unwrap();
        let weak_file = file.downgrade();
        let file2 = weak_file.upgrade().unwrap();
        assert_eq!(Arc::strong_count(&file.0), 3); // 3 references are: AutosarModel, file, file2
        assert_eq!(file, file2);
    }

    #[test]
    fn standalone() {
        let model = AutosarModel::new();
        let file_text = r#"<?xml version="1.0" encoding="utf-8" standalone="no"?>
            <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
            </AUTOSAR>"#.as_bytes();
        let (file, _warnings) = model.load_buffer(file_text, "filename.arxml", true).unwrap();
        assert_eq!(file.xml_standalone(), Some(false));
    }

    #[test]
    fn serialize() {
        let model = AutosarModel::new();
        let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        assert_eq!(file.model().unwrap(), model);
        assert_eq!(model.root_element().element_name(), ElementName::Autosar);
        let text = file.serialize().unwrap();
        assert_eq!(
            text,
            r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>"#
        );
        file.0.write().xml_standalone = Some(false);
        let text = file.serialize().unwrap();
        assert_eq!(
            text,
            r#"<?xml version="1.0" encoding="utf-8" standalone="no"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>"#
        );
        file.0.write().xml_standalone = Some(true);
        let text = file.serialize().unwrap();
        assert_eq!(
            text,
            r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>"#
        );
    }

    #[test]
    fn elements_dfs_iterator() {
        const FILEBUF_1: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE>
            <SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
              <SYSTEM><SHORT-NAME>System</SHORT-NAME></SYSTEM>
            </ELEMENTS>
          </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#.as_bytes();
        const FILEBUF_2: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE>
            <SHORT-NAME>Pkg2</SHORT-NAME>
            <ELEMENTS>
            <APPLICATION-PRIMITIVE-DATA-TYPE><SHORT-NAME>DataType</SHORT-NAME></APPLICATION-PRIMITIVE-DATA-TYPE>
            </ELEMENTS>
          </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#.as_bytes();

        let model = AutosarModel::new();
        let (file, _) = model.load_buffer(FILEBUF_1, "file1.arxml", false).unwrap();
        let proj_elem_count = model.elements_dfs().count();
        let file_elem_count = file.elements_dfs().count();
        assert_eq!(proj_elem_count, file_elem_count);
        model.load_buffer(FILEBUF_2, "file2.arxml", false).unwrap();
        let proj_elem_count_2 = model.elements_dfs().count();
        let file_elem_count_2 = file.elements_dfs().count();
        assert!(proj_elem_count < proj_elem_count_2);
        assert_eq!(file_elem_count, file_elem_count_2);
    }

    #[test]
    fn elements_dfs_with_max_depth() {
        const FILEBUF: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
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
        let model = AutosarModel::new();
        let (file, _) = model.load_buffer(FILEBUF, "test1", true).unwrap();
        let all_count = file.elements_dfs().count();
        let lvl2_count = file.elements_dfs_with_max_depth(2).count();
        assert!(all_count > lvl2_count);
        for elem in file.elements_dfs_with_max_depth(2) {
            assert!(elem.0 <= 2);
        }
    }

    #[test]
    fn multiple_files_1() {
        // two files are created, both contain AUTOSAR. Then a child element of AUTOSAR is created, which is present in both files
        let model = AutosarModel::new();
        let file_a = model.create_file("a", AutosarVersion::LATEST).unwrap();
        let file_b = model.create_file("b", AutosarVersion::LATEST).unwrap();
        let el_a_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        // el_a_packages is part of both file_a and file_b, because both files automatically contain the
        // root AUTOSAR element, and AR_PACKAGES inherits this
        let (_, fs) = el_a_packages.file_membership().unwrap();
        assert!(fs.contains(&file_a.downgrade()));
        assert!(fs.contains(&file_b.downgrade()));
    }

    #[test]
    fn multiple_files_2() {
        // one file is created, which contains AUTOSAR. Then a child element is created, which is automatically part of this file.
        // then a second file is created, but the element is NOT part of the new file
        let model = AutosarModel::new();
        let file_a = model.create_file("a", AutosarVersion::LATEST).unwrap();
        let el_a_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let file_b = model.create_file("b", AutosarVersion::LATEST).unwrap();
        // el_a_packages is only part of file_a
        let (_, fs) = el_a_packages.file_membership().unwrap();
        assert!(fs.contains(&file_a.downgrade()));
        assert!(!fs.contains(&file_b.downgrade()));
    }

    #[test]
    fn multiple_files_3() {
        // a file is created with multiple sub elements. A pat of this hierarchy is added to a second file
        let model = AutosarModel::new();
        let file_a = model.create_file("a", AutosarVersion::LATEST).unwrap();
        let el_ar_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let el_pkg1 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg1")
            .unwrap();
        let el_pkg2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg2")
            .unwrap();
        let file_b = model.create_file("b", AutosarVersion::LATEST).unwrap();
        let (_, fs) = el_pkg1.file_membership().unwrap();
        assert!(fs.contains(&file_a.downgrade())); // el_pkg1 is part of file_a
        assert!(!fs.contains(&file_b.downgrade())); // el_pkg1 is not part of file_b
        let (_, fs) = el_pkg2.file_membership().unwrap();
        assert!(fs.contains(&file_a.downgrade())); // el_pkg2 is part of file_a
        assert!(!fs.contains(&file_b.downgrade())); // el_pkg2 is not part of file_b

        // add el_pkg2 to file_b
        el_pkg2.add_to_file(&file_b).unwrap();
        let (_, fs) = el_pkg1.file_membership().unwrap();
        assert!(fs.contains(&file_a.downgrade())); // el_pkg1 is part of file_a
        assert!(!fs.contains(&file_b.downgrade())); // el_pkg1 is not part of file_b
        let (_, fs) = el_pkg2.file_membership().unwrap();
        assert!(fs.contains(&file_a.downgrade())); // el_pkg2 is part of file_a
        assert!(fs.contains(&file_b.downgrade())); // el_pkg2 is part of file_b

        // el_ar_packages was automatically added to file_b, in order to add el_pkg2
        let (_, fs) = el_ar_packages.file_membership().unwrap();
        assert!(fs.contains(&file_a.downgrade())); // el_ar_packages is part of file_a
        assert!(fs.contains(&file_b.downgrade())); // el_ar_packages is part of file_b

        // remove el_pkg2 from file_a
        let (_, fs) = el_pkg2.file_membership().unwrap();
        assert!(fs.contains(&file_a.downgrade())); // el_pkg2 is part of file_a
        assert!(fs.contains(&file_b.downgrade())); // el_pkg2 is part of file_b
        el_pkg2.remove_from_file(&file_a).unwrap();
        let (_, fs) = el_pkg2.file_membership().unwrap();
        assert!(!fs.contains(&file_a.downgrade())); // el_pkg2 is part of file_a
        assert!(fs.contains(&file_b.downgrade())); // el_pkg2 is part of file_b

        // add_to_file / remove_from_file cannot be called on all elements
        let el_elements = el_pkg1.create_sub_element(ElementName::Elements).unwrap();
        let result = el_elements.add_to_file(&file_a);
        assert!(matches!(result, Err(AutosarDataError::FilesetModificationForbidden)));
        let result: Result<(), AutosarDataError> = el_elements.remove_from_file(&file_a);
        assert!(matches!(result, Err(AutosarDataError::FilesetModificationForbidden)));

        // serializing a single file
        let text_before = file_a.serialize().unwrap();
        model.remove_file(&file_b);
        assert!(model.get_element_by_path("/Pkg2").is_none());
        let text_after = file_a.serialize().unwrap();
        assert_eq!(text_before, text_after);
    }

    #[test]
    fn traits() {
        let model = AutosarModel::new();
        let file = model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let weak_file = file.downgrade();
        let file_cloned = file.clone();
        assert_eq!(file, file_cloned);
        assert_eq!(format!("{file:#?}"), format!("{file_cloned:#?}"));
        let mut hashset = HashSet::<ArxmlFile>::new();
        hashset.insert(file);
        let inserted = hashset.insert(file_cloned);
        assert!(!inserted);

        let weak_file_cloned = weak_file.clone();
        assert_eq!(weak_file, weak_file_cloned);
        assert_eq!(format!("{weak_file:#?}"), format!("{weak_file_cloned:#?}"));
        let mut hashset = HashSet::<WeakArxmlFile>::new();
        hashset.insert(weak_file);
        let inserted = hashset.insert(weak_file_cloned);
        assert!(!inserted);
    }
}
