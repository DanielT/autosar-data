use std::hash::Hash;
use std::path::Path;

use crate::*;

impl ArxmlFile {
    pub(crate) fn new<P: AsRef<Path>>(filename: P, version: AutosarVersion, container: &AutosarProject) -> Self {
        Self(Arc::new(Mutex::new(ArxmlFileRaw {
            version,
            project: container.downgrade(),
            filename: filename.as_ref().to_path_buf(),
            xml_standalone: None,
        })))
    }

    /// Get the filename of this ArxmlFile
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// println!("filename is : {}", file.filename().display());
    /// ```
    pub fn filename(&self) -> PathBuf {
        self.0.lock().filename.clone()
    }

    /// Get the [AutosarVersion] of the file
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let version = file.version();
    /// ```
    pub fn version(&self) -> AutosarVersion {
        self.0.lock().version
    }

    /// Set the [AutosarVersion] of the file
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
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// file.set_version(AutosarVersion::Autosar_00050);
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::VersionIncompatible] the existing data is not compatible with the new version
    ///
    pub fn set_version(&self, new_ver: AutosarVersion) -> Result<(), AutosarDataError> {
        let (compat_errors, _) = self.check_version_compatibility(new_ver);
        if compat_errors.is_empty() {
            let mut file = self.0.lock();
            file.version = new_ver;
            // let attribute_value =
            //     CharacterData::String(format!("http://autosar.org/schema/r4.0 {}", new_ver.filename()));
            // let _ = file.root_element.0.lock().set_attribute_internal(
            //     AttributeName::xsiSchemalocation,
            //     attribute_value,
            //     new_ver,
            // );
            Ok(())
        } else {
            Err(AutosarDataError::VersionIncompatibleData { version: new_ver })
        }
    }

    /// Check if the elements and attributes in this file are compatible with some target_version
    ///
    /// All elements and their attributes will be evaluated against the target version according to the specification.
    /// The output is a list of incompatible elements
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let (error_list, compat_mask) = file.check_version_compatibility(AutosarVersion::Autosar_00050);
    /// ```
    pub fn check_version_compatibility(&self, target_version: AutosarVersion) -> (Vec<CompatibilityError>, u32) {
        if let Ok(project) = self.project() {
            project
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
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// file.set_filename("foo.arxml");
    /// // or
    /// file.set_filename(&Path::new("bar.arxml"));
    /// ```
    pub fn set_filename<P: AsRef<Path>>(&self, new_filename: P) {
        self.0.lock().filename = new_filename.as_ref().to_path_buf();
    }

    /// Get a reference to the [AutosarProject] object that contains this file
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let project = AutosarProject::new();
    /// let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let p2 = file.project()?;
    /// assert_eq!(project, p2);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    /// [AutosarDataError::ItemDeleted]: The project is no longer valid
    ///
    pub fn project(&self) -> Result<AutosarProject, AutosarDataError> {
        let locked_file = self.0.lock();
        // This reference must always be valid, so it is an error if upgrade() fails
        locked_file.project.upgrade().ok_or(AutosarDataError::ItemDeleted)
    }

    /// Create a depth-first search iterator over all [Element]s in this file
    ///
    /// In a multi-file project it will not return any elements from other files.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// for (depth, elem) in file.elements_dfs() {
    ///     // ...
    /// }
    /// ```
    pub fn elements_dfs(&self) -> ArxmlFileElementsDfsIterator {
        ArxmlFileElementsDfsIterator::new(self.downgrade(), &self.project().unwrap().root_element())
    }

    /// Serialize the content of the file to a String
    ///
    /// # Possible errors
    ///
    /// [AutosarDataError::ItemDeleted]: The project is no longer valid
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let text = file.serialize();
    /// ```
    pub fn serialize(&self) -> Result<String, AutosarDataError> {
        let mut outstring = String::with_capacity(1024 * 1024);

        match self.xml_standalone() {
            Some(true) => outstring.push_str("<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>"),
            Some(false) => outstring.push_str("<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"no\"?>"),
            None => outstring.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>"),
        }
        let project = self.project()?;
        project.0.lock().set_version(self.0.lock().version);
        project
            .root_element()
            .serialize_internal(&mut outstring, 0, false, Some(self.downgrade()));

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
    /// let project = AutosarProject::new();
    /// let file_text = r#"<?xml version="1.0" encoding="utf-8" standalone="no"?>
    /// <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    /// </AUTOSAR>"#.as_bytes();
    /// let (file, _warnings) = project.load_named_arxml_buffer(file_text, "filename.arxml", true).unwrap();
    /// assert_eq!(file.xml_standalone(), Some(false));
    /// ```
    pub fn xml_standalone(&self) -> Option<bool> {
        self.0.lock().xml_standalone
    }

    /// Create a weak reference to this ArxmlFile
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
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let weak_file = file.downgrade();
    /// ```
    pub fn downgrade(&self) -> WeakArxmlFile {
        WeakArxmlFile(Arc::downgrade(&self.0))
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

impl WeakArxmlFile {
    /// try to get a strong reference to the [ArxmlFile]
    ///
    /// This succeeds if the ArxmlFile still has any other strong reference to it, otherwise None is returned
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let project = AutosarProject::new();
        let result = project.create_file("test", AutosarVersion::Autosar_4_0_1);
        assert!(result.is_ok());
    }

    #[test]
    fn filename() {
        let project = AutosarProject::new();
        let result = project.create_file("test", AutosarVersion::Autosar_4_0_1);
        let file = result.unwrap();
        let filename = PathBuf::from("newname.arxml");
        file.set_filename(filename.clone());
        assert_eq!(file.filename(), filename);
    }

    #[test]
    fn version() {
        let project = AutosarProject::new();
        let result = project.create_file("test", AutosarVersion::Autosar_4_0_1);
        let file = result.unwrap();
        file.set_version(AutosarVersion::Autosar_00050).unwrap();
        assert_eq!(file.version(), AutosarVersion::Autosar_00050);
    }

    #[test]
    fn references() {
        let project = AutosarProject::new();
        let result = project.create_file("test", AutosarVersion::Autosar_4_0_1);
        let file = result.unwrap();
        let weak_file = file.downgrade();
        let file2 = weak_file.upgrade().unwrap();
        assert_eq!(Arc::strong_count(&file.0), 3); // 3 references are: AutosarProject, file, file2
        assert_eq!(file, file2);
    }

    #[test]
    fn standalone() {
        let project = AutosarProject::new();
        let file_text = r#"<?xml version="1.0" encoding="utf-8" standalone="no"?>
            <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
            </AUTOSAR>"#.as_bytes();
        let (file, _warnings) = project
            .load_named_arxml_buffer(file_text, "filename.arxml", true)
            .unwrap();
        assert_eq!(file.xml_standalone(), Some(false));
    }

    #[test]
    fn serialize() {
        let project = AutosarProject::new();
        let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        assert_eq!(file.project().unwrap(), project);
        assert_eq!(project.root_element().element_name(), ElementName::Autosar);
        let text = file.serialize().unwrap();
        assert_eq!(
            text,
            r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>"#
        );
        file.0.lock().xml_standalone = Some(false);
        let text = file.serialize().unwrap();
        assert_eq!(
            text,
            r#"<?xml version="1.0" encoding="utf-8" standalone="no"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>"#
        );
        file.0.lock().xml_standalone = Some(true);
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

        let project = AutosarProject::new();
        let (file, _) = project
            .load_named_arxml_buffer(FILEBUF_1, "file1.arxml", false)
            .unwrap();
        let proj_elem_count = project.elements_dfs().count();
        let file_elem_count = file.elements_dfs().count();
        assert_eq!(proj_elem_count, file_elem_count);
        project
            .load_named_arxml_buffer(FILEBUF_2, "file2.arxml", false)
            .unwrap();
        let proj_elem_count_2 = project.elements_dfs().count();
        let file_elem_count_2 = file.elements_dfs().count();
        assert!(proj_elem_count < proj_elem_count_2);
        assert_eq!(file_elem_count, file_elem_count_2);
    }

    #[test]
    fn traits() {
        let project = AutosarProject::new();
        let file = project.create_file("filename", AutosarVersion::LATEST).unwrap();
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
