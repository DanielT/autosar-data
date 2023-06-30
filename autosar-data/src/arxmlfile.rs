use std::path::Path;

use crate::*;

impl ArxmlFile {
    pub(crate) fn new<P: AsRef<Path>>(filename: P, version: AutosarVersion, container: &AutosarProject) -> Self {
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
        let root_element = Element(Arc::new(Mutex::new(ElementRaw {
            parent: ElementOrFile::None,
            elemname: ElementName::Autosar,
            elemtype: ElementType::ROOT,
            content: SmallVec::new(),
            attributes: root_attributes,
        })));
        let file = Self(Arc::new(Mutex::new(ArxmlFileRaw {
            project: container.downgrade(),
            root_element,
            version,
            filename: filename.as_ref().to_path_buf(),
        })));
        let new_parent = ElementOrFile::File(file.downgrade());
        file.root_element().set_parent(new_parent);
        file
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
            let attribute_value =
                CharacterData::String(format!("http://autosar.org/schema/r4.0 {}", new_ver.filename()));
            let _ = file.root_element.0.lock().set_attribute_internal(
                AttributeName::xsiSchemalocation,
                attribute_value,
                new_ver,
            );
            Ok(())
        } else {
            Err(AutosarDataError::VersionIncompatible)
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
        self.root_element().check_version_compatibility(target_version)
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

    /// Get a referenct to the root ```<AUTOSAR ...>``` element of this file
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let autosar_element = file.root_element();
    /// ```
    pub fn root_element(&self) -> Element {
        let file = self.0.lock();
        file.root_element.clone()
    }

    /// Create a depth-first search iterator over all [Element]s in this file
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
    pub fn elements_dfs(&self) -> ElementsDfsIterator {
        let file = self.0.lock();
        file.root_element.elements_dfs()
    }

    /// Serialize the content of the file to a String
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let text = file.serialize();
    /// ```
    pub fn serialize(&self) -> String {
        let mut outstring = String::with_capacity(1024 * 1024);

        outstring.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>");
        self.root_element().serialize_internal(&mut outstring, 0, false);

        outstring
    }

    /// Recursively sort all elements in the file. This is exactly identical to calling sort() on the root element of the file.
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
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// file.sort();
    /// ```
    pub fn sort(&self) {
        self.root_element().sort()
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
    fn serialize() {
        let project = AutosarProject::new();
        let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        let text = file.serialize();
        assert_eq!(
            text,
            r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
</AUTOSAR>"#
        );
    }
}
