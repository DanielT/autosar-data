use smallvec::smallvec;
use std::{borrow::Cow, str::FromStr, time::Duration};

use crate::iterators::*;

use super::*;

impl Element {
    /// Get the parent element of the current element
    ///
    /// Returns None if the current element is the root, or if it has been deleted from the element hierarchy
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// if let Some(parent) = element.parent()? {
    ///     // ...
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    pub fn parent(&self) -> Result<Option<Element>, AutosarDataError> {
        self.0.lock().parent()
    }

    pub(crate) fn set_parent(&self, new_parent: ElementOrFile) {
        self.0.lock().set_parent(new_parent)
    }

    /// Get the [ElementName] of the element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let element = file.root_element();
    /// let element_name = element.element_name();
    /// assert_eq!(element_name, ElementName::Autosar);
    /// ```
    pub fn element_name(&self) -> ElementName {
        self.0.lock().elemname
    }

    /// Get the [ElementType] of the element
    ///
    /// The ElementType is needed in order to call methods from the autosar-data-specification crate
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// let element_type = element.element_type();
    /// ```
    pub fn element_type(&self) -> ElementType {
        self.0.lock().elemtype
    }

    /// Get the name of an identifiable element
    ///
    /// An identifiable element has a `<SHORT-NAME>` sub element and can be referenced using an autosar path.
    ///
    /// If the element is not identifiable, this function returns None
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// if let Some(item_name) = element.item_name() {
    ///     // ...
    /// }
    /// ```
    pub fn item_name(&self) -> Option<String> {
        self.0.lock().item_name()
    }

    /// Set the item name of this element
    ///
    /// This operation will update all references pointing to the element or its sub-elements so that they remain valid.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// element.set_item_name("NewName");
    /// ```
    ///
    /// # Note
    ///
    /// In order to rename an element *without* updating any references, do this instead:
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// if let Some(short_name) = element.get_sub_element(ElementName::ShortName) {
    ///     short_name.set_character_data(CharacterData::String("the_new_name".to_string()));
    /// }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::ItemNameRequired] this function was called for an element which is not identifiable
    ///
    pub fn set_item_name(&self, new_name: &str) -> Result<(), AutosarDataError> {
        // a new name is required
        if new_name.is_empty() {
            return Err(AutosarDataError::ItemNameRequired);
        }
        let file = self.file()?;
        let project = file.project()?;
        let version = file.version();
        self.0.lock().set_item_name(new_name, &project, version)
    }

    /// Returns true if the element is identifiable
    ///
    /// In order to be identifiable, the specification must require a SHORT-NAME
    /// sub-element and the SHORT-NAME must actually be present.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// if element.is_identifiable() {
    ///     // ...
    /// }
    /// ```
    pub fn is_identifiable(&self) -> bool {
        self.0.lock().is_identifiable()
    }

    /// Returns true if the element should contain a referenct to another element
    ///
    /// The function does not check if the reference is valid
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// if element.is_reference() {
    ///     // ex: element.set_reference_target(...)
    /// }
    /// ```
    pub fn is_reference(&self) -> bool {
        self.elemtype().is_ref()
    }

    /// Get the Autosar path of an identifiable element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// let path = element.path()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: Th ecurrent element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::ElementNotIdentifiable]: The current element is not identifiable, so it has no Autosar path
    ///
    pub fn path(&self) -> Result<String, AutosarDataError> {
        self.0.lock().path()
    }

    /// Get a reference to the [ArxmlFile] containing the current element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// let file = element.file()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///
    pub fn file(&self) -> Result<ArxmlFile, AutosarDataError> {
        let mut cur_elem = self.clone();
        loop {
            let parent = {
                let element = cur_elem
                    .0
                    .try_lock_for(std::time::Duration::from_millis(10))
                    .ok_or(AutosarDataError::ParentElementLocked)?;
                match &element.parent {
                    ElementOrFile::Element(weak_parent) => {
                        weak_parent.upgrade().ok_or(AutosarDataError::ItemDeleted)?
                    }
                    ElementOrFile::File(weak_arxmlfile) => {
                        return weak_arxmlfile.upgrade().ok_or(AutosarDataError::ItemDeleted)
                    }
                    ElementOrFile::None => return Err(AutosarDataError::ItemDeleted),
                }
            };
            cur_elem = parent;
        }
    }

    /// Get the [ContentType] of the current element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// if element.content_type() == ContentType::CharacterData {
    ///     // ...
    /// }
    /// ```
    pub fn content_type(&self) -> ContentType {
        match self.elemtype().content_mode() {
            ContentMode::Sequence => ContentType::Elements,
            ContentMode::Choice => ContentType::Elements,
            ContentMode::Bag => ContentType::Elements,
            ContentMode::Characters => ContentType::CharacterData,
            ContentMode::Mixed => ContentType::Mixed,
        }
    }

    /// Create a sub element at a suitable insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// It is not possible to create named sub elements with this function; use create_named_sub_element() for that instead.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let element = file.root_element().create_sub_element(ElementName::ArPackages)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::IncorrectContentType]: A sub element may not be created in an element with content type CharacterData.
    ///  - [AutosarDataError::ElementInsertionConflict]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [AutosarDataError::InvalidSubElement]: The ElementName is not a valid sub element according to the specification.
    ///  - [AutosarDataError::ItemNameRequired]: The sub element requires an item name, so you must use create_named_sub_element().
    pub fn create_sub_element(&self, element_name: ElementName) -> Result<Element, AutosarDataError> {
        let version = self.file().map(|f| f.version())?;
        self.0
            .lock()
            .create_sub_element(self.downgrade(), element_name, version)
    }

    /// Create a sub element at the specified insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// It is not possible to create named sub elements with this function; use create_named_sub_element_at() for that instead.
    ///
    /// The specified insertion position will be compared to the range of valid insertion positions; if it falls outside that range then the function fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let element = file.root_element().create_sub_element_at(ElementName::ArPackages, 0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::IncorrectContentType]: A sub element may not be created in an element with content type CharacterData.
    ///  - [AutosarDataError::ElementInsertionConflict]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [AutosarDataError::InvalidSubElement]: The ElementName is not a valid sub element according to the specification.
    ///  - [AutosarDataError::ItemNameRequired]: The sub element requires an item name, so you must use create_named_sub_element_at().
    ///  - [AutosarDataError::InvalidPosition]: This sub element cannot be created at the requested position
    pub fn create_sub_element_at(
        &self,
        element_name: ElementName,
        position: usize,
    ) -> Result<Element, AutosarDataError> {
        let version = self.file().map(|f| f.version())?;
        self.0
            .lock()
            .create_sub_element_at(self.downgrade(), element_name, position, version)
    }

    /// Create a named/identifiable sub element at a suitable insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    ///
    /// This method can only be used to create identifiable sub elements.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let pkgs_element = file.root_element().create_sub_element(ElementName::ArPackages)?;
    /// let element = pkgs_element.create_named_sub_element(ElementName::ArPackage, "Package")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::IncorrectContentType]: A sub element may not be created in an element with content type CharacterData.
    ///  - [AutosarDataError::ElementInsertionConflict]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [AutosarDataError::InvalidSubElement]: The ElementName is not a valid sub element according to the specification.
    ///  - [AutosarDataError::ElementNotIdentifiable]: The sub element does not have an item name, so you must use create_sub_element() instead.
    pub fn create_named_sub_element(
        &self,
        element_name: ElementName,
        item_name: &str,
    ) -> Result<Element, AutosarDataError> {
        let file = self.file()?;
        let project = file.project()?;
        let version = file.version();
        self.0
            .lock()
            .create_named_sub_element(self.downgrade(), element_name, item_name, &project, version)
    }

    /// Create a named/identifiable sub element at the specified insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// The specified insertion position will be compared to the range of valid insertion positions; if it falls outside that range then the function fails.
    ///
    /// This method can only be used to create identifiable sub elements.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let pkgs_element = file.root_element().create_sub_element(ElementName::ArPackages)?;
    /// let element = pkgs_element.create_named_sub_element_at(ElementName::ArPackage, "Package", 0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::IncorrectContentType]: A sub element may not be created in an element with content type CharacterData.
    ///  - [AutosarDataError::ElementInsertionConflict]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [AutosarDataError::InvalidSubElement]: The ElementName is not a valid sub element according to the specification.
    ///  - [AutosarDataError::ElementNotIdentifiable]: The sub element does not have an item name, so you must use create_sub_element() instead.
    ///  - [AutosarDataError::InvalidPosition]: This sub element cannot be created at the requested position.
    pub fn create_named_sub_element_at(
        &self,
        element_name: ElementName,
        item_name: &str,
        position: usize,
    ) -> Result<Element, AutosarDataError> {
        let file = self.file()?;
        let project = file.project()?;
        let version = file.version();
        self.0.lock().create_named_sub_element_at(
            self.downgrade(),
            element_name,
            item_name,
            position,
            &project,
            version,
        )
    }

    /// Create a deep copy of the given element and insert it as a sub-element
    ///
    /// The other element must be a permissible sub-element in this element and not conflict with any existing sub element.
    /// The other element can originate from any loaded [AutosarProject], it does not have to originate from the same project or file as the current element.
    ///
    /// The [AutosarVersion] of the other element might differ from the version of the current file;
    /// in this case a partial copy will be performed that omits all incompatible elements.
    ///
    /// If the copied element is identifiable, then the item name might be extended with a numerical suffix, if one is required in order to make the name unique.
    /// For example: An identifiable element "Foo" already exists at the same path; the copied identifiable element will be renamed to "Foo_1".
    ///
    /// If the copied element or the hierarchy of elements under it contain any references, then these will need to be adjusted manually after copying.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkgs_element = file.root_element().create_sub_element(ElementName::ArPackages)?;
    /// # let base = pkgs_element.create_named_sub_element(ElementName::ArPackage, "Package")
    /// #    .and_then(|p| p.create_sub_element(ElementName::Elements))?;
    /// # base.create_named_sub_element(ElementName::System, "Path")?;
    /// let other_element = project.get_element_by_path("/Package/Path").unwrap();
    /// let element = base.create_copied_sub_element(&other_element)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::IncorrectContentType]: A sub element may not be created in an element with content type CharacterData.
    ///  - [AutosarDataError::ElementInsertionConflict]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [AutosarDataError::InvalidSubElement]: The ElementName is not a valid sub element according to the specification.
    pub fn create_copied_sub_element(&self, other: &Element) -> Result<Element, AutosarDataError> {
        let file = self.file()?;
        let project = file.project()?;
        let version = file.version();
        self.0
            .lock()
            .create_copied_sub_element(self.downgrade(), other, &project, version)
    }

    /// Create a deep copy of the given element and insert it as a sub-element at the given position
    ///
    /// The other element must be a permissible sub-element in this element and not conflict with any existing sub element.
    /// The other element can originate from any loaded [AutosarProject], it does not have to originate from the same project or file as the current element.
    ///
    /// The [AutosarVersion] of the other element might differ from the version of the current file;
    /// in this case a partial copy will be performed that omits all incompatible elements.
    ///
    /// If the copied element is identifiable, then the item name might be extended with a numerical suffix, if one is required in order to make the name unique.
    /// For example: An identifiable element "Foo" already exists at the same path; the copied identifiable element will be renamed to "Foo_1".
    ///
    /// If the copied element or the hierarchy of elements under it contain any references, then these will need to be adjusted manually after copying.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkgs_element = file.root_element().create_sub_element(ElementName::ArPackages)?;
    /// # let base = pkgs_element.create_named_sub_element(ElementName::ArPackage, "Package")
    /// #    .and_then(|p| p.create_sub_element(ElementName::Elements))?;
    /// # base.create_named_sub_element(ElementName::System, "Path")?;
    /// let other_element = project.get_element_by_path("/Package/Path").unwrap();
    /// let element = base.create_copied_sub_element_at(&other_element, 0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::IncorrectContentType]: A sub element may not be created in an element with content type CharacterData.
    ///  - [AutosarDataError::ElementInsertionConflict]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [AutosarDataError::InvalidSubElement]: The ElementName is not a valid sub element according to the specification.
    ///  - [AutosarDataError::InvalidPosition]: This sub element cannot be created at the requested position.
    pub fn create_copied_sub_element_at(&self, other: &Element, position: usize) -> Result<Element, AutosarDataError> {
        let file = self.file()?;
        let project = file.project()?;
        let version = file.version();
        self.0
            .lock()
            .create_copied_sub_element_at(self.downgrade(), other, position, &project, version)
    }

    /// Take an `element` from it's current location and place it in this element as a sub element
    ///
    /// The moved element can be taken from anywhere - even from a different arxml document that is not part of the same AutosarProject
    ///
    /// Restrictions:
    /// 1) The element must have a compatible element type. If it could not have been created here, then it can't be moved either.
    /// 2) The origin document of the element must have exactly the same AutosarVersion as the destination.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkgs_element = file.root_element().create_sub_element(ElementName::ArPackages)?;
    /// # let base = pkgs_element.create_named_sub_element(ElementName::ArPackage, "Package")
    /// #    .and_then(|p| p.create_sub_element(ElementName::Elements))?;
    /// # base.create_named_sub_element(ElementName::System, "Path")?;
    /// let other_element = project.get_element_by_path("/Package/Path").unwrap();
    /// let element = base.move_element_here(&other_element)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::IncorrectContentType]: A sub element may not be created in an element with content type CharacterData.
    ///  - [AutosarDataError::ElementInsertionConflict]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [AutosarDataError::InvalidSubElement]: The ElementName is not a valid sub element according to the specification.
    ///  - [AutosarDataError::VersionIncompatible]: The Autosar versions of the source and destination are different
    ///  - [AutosarDataError::ForbiddenMoveToSubElement]: The destination is a sub element of the source. Moving here is not possible
    pub fn move_element_here(&self, move_element: &Element) -> Result<Element, AutosarDataError> {
        let file_src = move_element.file()?;
        let project_src = file_src.project()?;
        let version = self.file().map(|f| f.version())?;
        let project = self.file().and_then(|f| f.project())?;
        let version_src = file_src.version();
        if version != version_src {
            return Err(AutosarDataError::VersionIncompatible);
        }
        self.0
            .lock()
            .move_element_here(self.downgrade(), move_element, &project, &project_src, version)
    }

    /// Take an `element` from it's current location and place it at the given position in this element as a sub element
    ///
    /// The moved element can be taken from anywhere - even from a different arxml document that is not part of the same AutosarProject
    ///
    /// Restrictions:
    /// 1) The element must have a compatible element type. If it could not have been created here, then it can't be moved either.
    /// 2) The origin document of the element must have exactly the same AutosarVersion as the destination.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkgs_element = file.root_element().create_sub_element(ElementName::ArPackages)?;
    /// # let base = pkgs_element.create_named_sub_element(ElementName::ArPackage, "Package")
    /// #    .and_then(|p| p.create_sub_element(ElementName::Elements))?;
    /// # base.create_named_sub_element(ElementName::System, "Path")?;
    /// let other_element = project.get_element_by_path("/Package/Path").unwrap();
    /// let element = base.move_element_here_at(&other_element, 0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::IncorrectContentType]: A sub element may not be created in an element with content type CharacterData.
    ///  - [AutosarDataError::ElementInsertionConflict]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [AutosarDataError::InvalidSubElement]: The ElementName is not a valid sub element according to the specification.
    ///  - [AutosarDataError::VersionIncompatible]: The Autosar versions of the source and destination are different
    ///  - [AutosarDataError::ForbiddenMoveToSubElement]: The destination is a sub element of the source. Moving here is not possible
    ///  - [AutosarDataError::InvalidPosition]: This sub element cannot be created at the requested position.
    pub fn move_element_here_at(&self, move_element: &Element, position: usize) -> Result<Element, AutosarDataError> {
        let file_src = move_element.file()?;
        let project_src = file_src.project()?;
        let version = self.file().map(|f| f.version())?;
        let project = self.file().and_then(|f| f.project())?;
        let version_src = file_src.version();
        if version != version_src {
            return Err(AutosarDataError::VersionIncompatible);
        }
        self.0.lock().move_element_here_at(
            self.downgrade(),
            move_element,
            position,
            &project,
            &project_src,
            version,
        )
    }

    /// Remove the sub element sub_element
    ///
    /// The sub_element will be unlinked from the hierarchy of elements.
    /// All of the sub-sub-elements nested under the removed element will also be recusively removed.
    ///
    /// Since all elements are reference counted, they might not be deallocated immediately, however they do become invalid and unusable immediately.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let packages = file.root_element().create_sub_element(ElementName::ArPackages)?;
    /// file.root_element().remove_sub_element(packages)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::ElementNotFound]: The sub element was not found in this element
    ///  - [AutosarDataError::ShortNameRemovalForbidden]: It is not permitted to remove the SHORT-NAME of identifiable elements since this would result in invalid data
    pub fn remove_sub_element(&self, sub_element: Element) -> Result<(), AutosarDataError> {
        let project = self.file().and_then(|f| f.project())?;
        self.0.lock().remove_sub_element(sub_element, &project)
    }

    /// Set the reference target for the element to target
    ///
    /// When the reference is updated, the DEST attribute is also updated to match the referenced element.
    /// The current element must be a reference element, otherwise the function fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let elements = file.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))
    /// #   .and_then(|e| e.create_sub_element(ElementName::Elements))?;
    /// # let ref_element = elements.create_named_sub_element(ElementName::System, "System")
    /// #   .and_then(|e| e.create_sub_element(ElementName::FibexElements))
    /// #   .and_then(|e| e.create_sub_element(ElementName::FibexElementRefConditional))
    /// #   .and_then(|e| e.create_sub_element(ElementName::FibexElementRef))?;
    /// let cluster_element = elements.create_named_sub_element(ElementName::CanCluster, "Cluster")?;
    /// ref_element.set_reference_target(&cluster_element)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::NotReferenceElement]: The current element is not a reference, so it is not possible to set a reference target
    ///  - [AutosarDataError::InvalidReference]: The target element is not a valid reference target for this reference
    ///  - [AutosarDataError::ElementNotIdentifiable]: The target element is not identifiable, so it cannot be referenced by an Autosar path
    pub fn set_reference_target(&self, target: &Element) -> Result<(), AutosarDataError> {
        // the current element must be a reference
        if self.is_reference() {
            // the target element must be identifiable, i.e. it has an autosar path
            let new_ref = target.path()?;
            // it must be possible to use the name of the referenced element name as an enum item in the dest attribute of the reference
            if let Ok(enum_item) = EnumItem::from_str(target.element_name().to_str()) {
                let file = self.file()?;
                let project = file.project()?;
                let version = file.version();
                let mut element = self.0.lock();
                // set the DEST attribute first - this could fail if the target element has the wrong type
                if element
                    .set_attribute_internal(AttributeName::Dest, CharacterData::Enum(enum_item), version)
                    .is_ok()
                {
                    // if this reference previously referenced some other element, update
                    if let Some(CharacterData::String(old_ref)) = element.character_data() {
                        project.fix_reference_origins(&old_ref, &new_ref, self.downgrade());
                    } else {
                        // else initialise the new reference
                        project.add_reference_origin(&new_ref, self.downgrade());
                    }
                    element.set_character_data(CharacterData::String(new_ref), version)?;
                    Ok(())
                } else {
                    Err(AutosarDataError::InvalidReference)
                }
            } else {
                Err(AutosarDataError::InvalidReference)
            }
        } else {
            Err(AutosarDataError::NotReferenceElement)
        }
    }

    /// Get the referenced element
    ///
    /// This function will get the reference string from the character data of the element
    /// as well as the destination type from the DEST attribute. Then a lookup of the Autosar
    /// path is performed, and if an element is found at that path, then the type of the
    /// element is compared to the expected type.
    ///
    /// The element is returned if it exists and its type is correct.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let elements = file.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))
    /// #   .and_then(|e| e.create_sub_element(ElementName::Elements))?;
    /// # let ref_element = elements.create_named_sub_element(ElementName::System, "System")
    /// #   .and_then(|e| e.create_sub_element(ElementName::FibexElements))
    /// #   .and_then(|e| e.create_sub_element(ElementName::FibexElementRefConditional))
    /// #   .and_then(|e| e.create_sub_element(ElementName::FibexElementRef))?;
    /// let cluster_element = elements.create_named_sub_element(ElementName::CanCluster, "Cluster")?;
    /// ref_element.set_reference_target(&cluster_element)?;
    /// let ref_target = ref_element.get_reference_target()?;
    /// assert_eq!(cluster_element, ref_target);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::NotReferenceElement]: The current element is not a reference, so it is not possible to get the reference target
    ///  - [AutosarDataError::InvalidReference]: The reference is invalid; there is no element with the referenced Autosar path
    pub fn get_reference_target(&self) -> Result<Element, AutosarDataError> {
        if self.is_reference() {
            if let Some(CharacterData::String(reference)) = self.character_data() {
                let project = self.file().and_then(|file| file.project())?;
                let target_elem = project
                    .get_element_by_path(&reference)
                    .ok_or(AutosarDataError::InvalidReference)?;

                let dest = self
                    .attribute_string(AttributeName::Dest)
                    .ok_or(AutosarDataError::InvalidReference)?;
                if dest == target_elem.element_name().to_str() {
                    Ok(target_elem)
                } else {
                    Err(AutosarDataError::InvalidReference)
                }
            } else {
                Err(AutosarDataError::InvalidReference)
            }
        } else {
            Err(AutosarDataError::NotReferenceElement)
        }
    }

    /// Set the character data of this element
    ///
    /// This method only applies to elements which contain character data, i.e. element.content_type == CharacterData
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))?
    /// #   .get_sub_element(ElementName::ShortName).unwrap();
    /// element.set_character_data(CharacterData::String("value".to_string()))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [AutosarDataError::IncorrectContentType]: Cannot set character data on an element whoch does not contain character data
    pub fn set_character_data(&self, chardata: CharacterData) -> Result<(), AutosarDataError> {
        let elemtype = self.elemtype();
        if elemtype.content_mode() == ContentMode::Characters {
            if let Some(cdata_spec) = elemtype.chardata_spec() {
                let project = self.file().and_then(|file| file.project())?;
                let version = self.file().map(|f| f.version())?;
                if CharacterData::check_value(&chardata, cdata_spec, version) {
                    // if this is a SHORT-NAME element a whole lot of handling is needed in order to unbreak all the cross references
                    let mut prev_path = None;
                    if self.element_name() == ElementName::ShortName {
                        // this SHORT-NAME element might be newly created, in which case there is no previous path
                        if self.character_data().is_some() {
                            if let Some(parent) = self.parent()? {
                                prev_path = Some(parent.path()?);
                            }
                        }
                    };

                    // if this is a reference, then some extra effort is needed there too
                    let old_refval = if elemtype.is_ref() {
                        self.character_data().and_then(|cdata| cdata.string_value())
                    } else {
                        None
                    };

                    // update the character data
                    {
                        let mut element = self.0.lock();
                        if element.content.is_empty() {
                            element.content.push(ElementContent::CharacterData(chardata));
                        } else {
                            element.content[0] = ElementContent::CharacterData(chardata);
                        }
                    }

                    // short-name: make sure the hashmap in the top-level AutosarProject is updated so that this element can still be found
                    if let Some(prev_path) = prev_path {
                        if let Some(parent) = self.parent()? {
                            let new_path = parent.path()?;
                            project.fix_identifiables(&prev_path, &new_path);
                        }
                    }

                    // reference: update the references hashmap in the top-level AutosarProject
                    if elemtype.is_ref() {
                        if let Some(CharacterData::String(refval)) = self.character_data() {
                            if let Some(old_refval) = old_refval {
                                project.fix_reference_origins(&old_refval, &refval, self.downgrade());
                            } else {
                                project.add_reference_origin(&refval, self.downgrade())
                            }
                        }
                    }

                    return Ok(());
                }
            }
        }
        Err(AutosarDataError::IncorrectContentType)
    }

    /// Remove the character data of this element
    ///
    /// This method only applies to elements which contain character data, i.e. element.content_type == CharacterData
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))
    /// #   .and_then(|e| e.create_sub_element(ElementName::Elements))
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::System, "System"))
    /// #   .and_then(|e| e. create_sub_element(ElementName::PncVectorLength))
    /// #   .unwrap();
    /// element.remove_character_data()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::ParentElementLocked]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried. Only relevant when removing references.
    ///  - [AutosarDataError::ShortNameRemovalForbidden]: Removing the character content of a SHORT-NAME is forbidden
    ///  - [AutosarDataError::IncorrectContentType]: Cannot set character data on an element whoch does not contain character data
    pub fn remove_character_data(&self) -> Result<(), AutosarDataError> {
        let elemtype = self.elemtype();
        if elemtype.content_mode() == ContentMode::Characters {
            if self.element_name() == ElementName::ShortName {
                Err(AutosarDataError::ShortNameRemovalForbidden)
            } else {
                if self.character_data().is_some() {
                    if self.is_reference() {
                        let project = self.file().and_then(|file| file.project())?;
                        if let Some(CharacterData::String(reference)) = self.character_data() {
                            project.remove_reference_origin(&reference, self.downgrade())
                        }
                    }
                    self.0.lock().content.clear();
                }
                Ok(())
            }
        } else {
            Err(AutosarDataError::IncorrectContentType)
        }
    }

    /// Insert a character data item into the content of this element
    ///
    /// This method only applies to elements which contain mixed data, i.e. element.content_type() == Mixed.
    /// Use create_sub_element_at to add an element instead of a character data item
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))?;
    /// // mixed content elements are primarily used for documentation and description
    /// let desc = element.create_sub_element(ElementName::Desc)?;
    /// let l2 = desc.create_sub_element(ElementName::L2)?;
    /// l2.insert_character_content_item("descriptive text", 0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::IncorrectContentType] the element content_type is not Mixed
    ///  - [AutosarDataError::InvalidPosition] the position is not valid
    pub fn insert_character_content_item(&self, chardata: &str, position: usize) -> Result<(), AutosarDataError> {
        let mut element = self.0.lock();
        if let ContentMode::Mixed = element.elemtype.content_mode() {
            if position <= element.content.len() {
                element.content.insert(
                    position,
                    ElementContent::CharacterData(CharacterData::String(chardata.to_owned())),
                );
                Ok(())
            } else {
                Err(AutosarDataError::InvalidPosition)
            }
        } else {
            Err(AutosarDataError::IncorrectContentType)
        }
    }

    /// Remove a character data item from the content of this element
    ///
    /// This method only applies to elements which contain mixed data, i.e. element.content_type == Mixed
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))
    /// #   .and_then(|e| e.create_sub_element(ElementName::Desc))
    /// #   .and_then(|e| e.create_sub_element(ElementName::L2))?;
    /// element.insert_character_content_item("descriptive text", 0)?;
    /// element.remove_character_content_item(0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::IncorrectContentType] the element content_type is not Mixed
    ///  - [AutosarDataError::InvalidPosition] the position is not valid
    pub fn remove_character_content_item(&self, position: usize) -> Result<(), AutosarDataError> {
        let mut element = self.0.lock();
        if let ContentMode::Mixed = element.elemtype.content_mode() {
            if position < element.content.len() {
                if let ElementContent::CharacterData(_) = element.content[position] {
                    element.content.remove(position);
                    return Ok(());
                }
            }
            Err(AutosarDataError::InvalidPosition)
        } else {
            Err(AutosarDataError::IncorrectContentType)
        }
    }

    /// Get the character content of the element
    ///
    /// This method only applies to elements which contain character data, i.e. element.content_type() == CharacterData
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))?
    /// #   .get_sub_element(ElementName::ShortName).unwrap();
    /// match element.character_data() {
    ///     Some(CharacterData::String(stringval)) => {},
    ///     Some(CharacterData::Enum(enumval)) => {},
    ///     Some(CharacterData::UnsignedInteger(intval)) => {},
    ///     Some(CharacterData::Double(dblval)) => {},
    ///     None => {},
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn character_data(&self) -> Option<CharacterData> {
        self.0.lock().character_data()
    }

    /// Create an iterator over all of the content of this element
    ///
    /// The iterator can return both sub elements and character data, wrapped as ElementContent::Element and ElementContent::CharacterData
    ///
    /// This method is intended to be used with elements that contain mixed content.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// for content_item in element.content() {
    ///     match content_item {
    ///         ElementContent::CharacterData(data) => {},
    ///         ElementContent::Element(element) => {},
    ///     }
    /// }
    /// ```
    pub fn content(&self) -> ElementContentIterator {
        ElementContentIterator::new(self)
    }

    /// Create a weak reference to this element
    ///
    /// A weak reference can be stored without preventing the element from being deallocated.
    /// The weak reference has to be upgraded in order to be used, which can fail if the element no longer exists.
    ///
    /// See the documentation for [Arc]
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// let weak_element = element.downgrade();
    /// ```
    pub fn downgrade(&self) -> WeakElement {
        WeakElement(Arc::downgrade(&self.0))
    }

    /// Create an iterator over all sub elements of this element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// for sub_element in element.sub_elements() {
    ///     // ...
    /// }
    /// ```
    pub fn sub_elements(&self) -> ElementsIterator {
        ElementsIterator::new(self.clone())
    }

    /// Get the sub element with the given element name
    ///
    /// Returns None if no such element exists. if there are multiple sub elements with the requested name, then only the first is returned
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkg = file.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))?;
    /// let element = pkg.get_sub_element(ElementName::ShortName).unwrap();
    /// assert_eq!(element.element_name(), ElementName::ShortName);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_sub_element(&self, name: ElementName) -> Option<Element> {
        let locked_elem = self.0.lock();
        for item in &locked_elem.content {
            if let ElementContent::Element(subelem) = item {
                if subelem.element_name() == name {
                    return Some(subelem.clone());
                }
            }
        }
        None
    }

    /// Create a depth first iterator over this element and all of its sub elements
    ///
    /// Each step in the iteration returns the depth and an element. Due to the nature of a depth first search,
    /// the returned depth can remain the same, increase by one, or decrease by an arbitrary number in each step.
    ///
    /// The dfs iterator will always return this element as the first item.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// for (depth, elem) in element.elements_dfs() {
    ///     // ...
    /// }
    /// ```
    pub fn elements_dfs(&self) -> ElementsDfsIterator {
        ElementsDfsIterator::new(self)
    }

    /// Create an iterator over all the attributes in this element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// for attribute in element.attributes() {
    ///     println!("{} = {}", attribute.attrname, attribute.content);
    /// }
    /// ```
    pub fn attributes(&self) -> AttributeIterator {
        AttributeIterator {
            element: self.clone(),
            index: 0,
        }
    }

    /// Get the value of an attribute by name
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let value = file.root_element().attribute_value(AttributeName::xsiSchemalocation);
    /// ```
    pub fn attribute_value(&self, attrname: AttributeName) -> Option<CharacterData> {
        self.0.lock().attribute_value(attrname)
    }

    /// Get the content of an attribute as a string
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// let value = element.attribute_string(AttributeName::Dest);
    /// ```
    pub fn attribute_string(&self, attrname: AttributeName) -> Option<String> {
        self.0.lock().attribute_string(attrname)
    }

    /// Set the value of a named attribute
    ///
    /// If no attribute by that name exists, and the attribute is a valid attribute of the element, then the attribute will be created.
    ///
    /// Returns Ok(()) if the attribute was set, otherwise the Err indicates why setting the attribute failed.
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// let result = element.set_attribute(AttributeName::S, CharacterData::String("1234-5678".to_string()));
    /// # assert!(result.is_ok());
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::InvalidAttribute]: The AttributeName is not valid for this element
    ///  - [AutosarDataError::InvalidAttributeValue]: The value is not valid for this attribute in this element
    pub fn set_attribute(&self, attrname: AttributeName, value: CharacterData) -> Result<(), AutosarDataError> {
        let version = self.file().map(|f| f.version())?;
        self.0.lock().set_attribute_internal(attrname, value, version)
    }

    /// Set the value of a named attribute from a string
    ///
    /// The function tries to convert the string to the correct data type for the attribute
    ///
    /// Returns Ok(()) if the attribute was set, otherwise the Err indicates why setting the attribute failed.
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// let result = element.set_attribute_string(AttributeName::T, "2022-01-31T13:59:59Z");
    /// # assert!(result.is_ok());
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [AutosarDataError::ItemDeleted]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [AutosarDataError::InvalidAttribute]: The AttributeName is not valid for this element
    ///  - [AutosarDataError::InvalidAttributeValue]: The value is not valid for this attribute in this element
    pub fn set_attribute_string(&self, attrname: AttributeName, stringvalue: &str) -> Result<(), AutosarDataError> {
        let version = self.file().map(|f| f.version())?;
        self.0.lock().set_attribute_string(attrname, stringvalue, version)
    }

    /// Remove an attribute from the element
    ///
    /// Returns true if the attribute existed and could be removed.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let result = file.root_element().remove_attribute(AttributeName::xsiSchemalocation);
    /// // xsiSchemalocation exists in the AUTOSAR element, but it is mandatory and cannot be removed
    /// assert_eq!(result, false);
    /// ```
    pub fn remove_attribute(&self, attrname: AttributeName) -> bool {
        self.0.lock().remove_attribute(attrname)
    }

    /// Serialize the element and all of its content to a string
    ///
    /// The serialized text generated for elements below the root element cannot be loaded, but it may be useful for display.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// let text = element.serialize();
    /// ```
    pub fn serialize(&self) -> String {
        let mut outstring = String::new();

        self.serialize_internal(&mut outstring, 0, false);

        outstring
    }

    pub(crate) fn serialize_internal(&self, outstring: &mut String, indent: usize, inline: bool) {
        let element_name = self.element_name().to_str();

        // write the opening tag on a new line and indent it
        if !inline {
            self.serialize_newline_indent(outstring, indent);
        }
        outstring.push('<');
        outstring.push_str(element_name);
        self.serialize_attributes(outstring);
        outstring.push('>');

        match self.content_type() {
            ContentType::Elements => {
                // serialize each sub-element
                for subelem in self.sub_elements() {
                    subelem.serialize_internal(outstring, indent + 1, false);
                }
                // put the closing tag on a new line and indent it
                self.serialize_newline_indent(outstring, indent);
                outstring.push_str("</");
                outstring.push_str(element_name);
                outstring.push('>');
            }
            ContentType::CharacterData => {
                // write the character data on the same line as the opening tag
                let element = self.0.lock();
                if let Some(ElementContent::CharacterData(chardata)) = element.content.get(0) {
                    chardata.serialize_internal(outstring);
                }

                // write the closing tag on the same line
                outstring.push_str("</");
                outstring.push_str(element_name);
                outstring.push('>');
            }
            ContentType::Mixed => {
                for item in self.content() {
                    match item {
                        ElementContent::Element(subelem) => {
                            subelem.serialize_internal(outstring, indent + 1, true);
                        }
                        ElementContent::CharacterData(chardata) => {
                            chardata.serialize_internal(outstring);
                        }
                    }
                }
                // write the closing tag on the same line
                outstring.push_str("</");
                outstring.push_str(element_name);
                outstring.push('>');
            }
        }
    }

    fn serialize_newline_indent(&self, outstring: &mut String, indent: usize) {
        outstring.push('\n');
        for _ in 0..indent {
            outstring.push_str("  ");
        }
    }

    fn serialize_attributes(&self, outstring: &mut String) {
        let element = self.0.lock();
        if !element.attributes.is_empty() {
            for attribute in &element.attributes {
                outstring.push(' ');
                outstring.push_str(attribute.attrname.to_str());
                outstring.push_str("=\"");
                attribute.content.serialize_internal(outstring);
                outstring.push('"');
            }
        }
    }

    pub(crate) fn elemtype(&self) -> ElementType {
        self.0.lock().elemtype
    }

    /// check if the sub elements and attributes of this element are compatible with some target_version
    pub(crate) fn check_version_compatibility(&self, target_version: AutosarVersion) -> (Vec<CompatibilityError>, u32) {
        let mut compat_errors = Vec::new();
        let mut overall_version_mask = u32::MAX;

        // check the compatibility of all the attributes in this element
        {
            let element = self.0.lock();
            for attribute in &element.attributes {
                // find the specification for the current attribute
                if let Some((value_spec, _, version_mask)) = element.elemtype.find_attribute_spec(attribute.attrname) {
                    overall_version_mask &= version_mask;
                    // check if the attribute is allowed at all
                    if !target_version.compatible(version_mask) {
                        compat_errors.push(CompatibilityError::IncompatibleAttribute {
                            element: self.clone(),
                            attribute: attribute.attrname,
                            version_mask,
                        });
                    } else {
                        let (is_compatible, value_version_mask) = attribute
                            .content
                            .check_version_compatibility(value_spec, target_version);
                        if !is_compatible {
                            compat_errors.push(CompatibilityError::IncompatibleAttributeValue {
                                element: self.clone(),
                                attribute: attribute.attrname,
                                version_mask: value_version_mask,
                            });
                        }
                        overall_version_mask &= value_version_mask;
                    }
                }
            }
        }

        // check the compatibility of all sub-elements
        for sub_element in self.sub_elements() {
            if let Some((_, indices)) = self
                .element_type()
                .find_sub_element(sub_element.element_name(), u32::MAX)
            {
                let version_mask = self.element_type().get_sub_element_version_mask(&indices).unwrap();
                overall_version_mask &= version_mask;
                if !target_version.compatible(version_mask) {
                    compat_errors.push(CompatibilityError::IncompatibleElement {
                        element: sub_element.clone(),
                        version_mask,
                    });
                } else {
                    let (mut sub_element_errors, sub_element_mask) =
                        sub_element.check_version_compatibility(target_version);
                    compat_errors.append(&mut sub_element_errors);
                    overall_version_mask &= sub_element_mask;
                }
            }
        }

        (compat_errors, overall_version_mask)
    }

    /// List all sub_elements that are valid in the current element
    ///
    /// The target use case is direct interaction with a user, e.g. through a selection dialog
    ///
    /// # Return Value
    ///
    /// A list of tuples consisting of
    ///     ElementName of the potential sub element
    ///     bool: is the sub element named
    ///     bool: can this sub element be inserted considering the current content of the element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let project = AutosarProject::new();
    /// # let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = file.root_element();
    /// for (element_name, is_named, is_allowed) in element.list_valid_sub_elements() {
    ///     // ...
    /// }
    /// ```
    pub fn list_valid_sub_elements(&self) -> Vec<(ElementName, bool, bool)> {
        let etype = self.0.lock().elemtype;
        let mut valid_sub_elements = Vec::new();

        if let Ok(version) = self.file().map(|f| f.version()) {
            for (element_name, _, version_mask, named_mask) in etype.sub_element_spec_iter() {
                if version.compatible(version_mask) {
                    let named = version.compatible(named_mask);
                    let available = self.0.lock().find_element_insert_pos(element_name, version).is_ok();
                    valid_sub_elements.push((element_name, named, available));
                }
            }
        }

        valid_sub_elements
    }
}

/// ElementRaw provides the internal implementation of (almost) all Element operations
///
/// To get an ElementRaw the Element operation must lock the element, so all of these operations run with at least one lock held.
///
/// Note regarding deadlock avoidance:
/// Consider the case where two element operations are started in parallel on different threads.
/// One calls file() or path() and traverses the hierarchy of elements upward
/// root <- element <- element <- current element (locked)
///
/// The other calls e.g. create_copied_sub_element() or remove_sub_element() and wants to lock all of its sub elements
/// root -> current element (locked) -> element -> element
///
/// These two operations could deadlock if they operate on the same tree of elements.
/// To avoid this, parent element locks can only be acquired with try_lock(). If the lock is not acquired within a
/// reasonable time (10ms is used here), then the operation aborts with a ParentElementLocked error.
impl ElementRaw {
    /// get the parent element of the current element
    pub(crate) fn parent(&self) -> Result<Option<Element>, AutosarDataError> {
        match &self.parent {
            ElementOrFile::Element(parent) => {
                // for items that should have a parent, getting it is not allowed to return None
                let parent = parent.upgrade().ok_or(AutosarDataError::ItemDeleted)?;
                Ok(Some(parent))
            }
            ElementOrFile::File(_) => Ok(None),
            ElementOrFile::None => Err(AutosarDataError::ItemDeleted),
        }
    }

    pub(crate) fn set_parent(&mut self, new_parent: ElementOrFile) {
        self.parent = new_parent;
    }

    /// get the [ElementName] of the element
    pub(crate) fn element_name(&self) -> ElementName {
        self.elemname
    }

    /// get the name of an identifiable element
    pub(crate) fn item_name(&self) -> Option<String> {
        // is this element named in any autosar version? - If it's not named here then we'll simply fail in the next step
        if self.elemtype.is_named() {
            // if an item is named, then the SHORT-NAME sub element that contains the name is always the first sub element
            if let Some(ElementContent::Element(subelem)) = self.content.get(0) {
                // why use try_lock? It is possible that we're calling path() while already holding the lock on subelem.
                // In this case path() descends to the parent, then calls item_name() and would deadlock.
                // This case happens when subelem is *not* a ShortName but elemtype.is_named() returns true because there
                // is a name in some other version, so it is acceptable to return None when locking fails
                if let Some(subelem_locked) = subelem.0.try_lock_for(Duration::from_millis(10)) {
                    if subelem_locked.elemname == ElementName::ShortName {
                        if let Some(CharacterData::String(name)) = subelem_locked.character_data() {
                            return Some(name);
                        }
                    }
                }
            }
        }
        None
    }

    /// Set the item name of this element
    pub(crate) fn set_item_name(
        &self,
        new_name: &str,
        project: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<(), AutosarDataError> {
        if let Some(current_name) = self.item_name() {
            // bail out early if the name is actually the same
            if current_name == new_name {
                return Ok(());
            }

            let old_path = self.path()?;
            let new_path = format!("{}{new_name}", old_path.strip_suffix(&current_name).unwrap());
            if project.get_element_by_path(&new_path).is_some() {
                return Err(AutosarDataError::DuplicateItemName);
            }

            // if an item is named, then the SHORT-NAME sub element that contains the name is always the first sub element
            if let Some(ElementContent::Element(subelem_wrapped)) = self.content.get(0) {
                let mut subelem = subelem_wrapped.0.lock();
                if subelem.element_name() == ElementName::ShortName {
                    subelem.set_character_data(CharacterData::String(new_name.to_owned()), version)?;
                    project.fix_identifiables(&old_path, &new_path);
                    let new_prefix = new_path;
                    let mut project_locked = project.0.lock();

                    // check all references and update those that point to this element or its sub elements
                    let refpaths = project_locked
                        .reference_origins
                        .keys()
                        .cloned()
                        .collect::<Vec<String>>();
                    for refpath in refpaths {
                        // if the existing reference has the old path as a prefix, then it needs to be updated
                        if let Some(partial_path) = refpath.strip_prefix(&old_path) {
                            // prevent ref updates from being applied to e.g. /package10 while renaming /package1
                            if partial_path.is_empty() || partial_path.starts_with('/') {
                                if let Some(reflist) = project_locked.reference_origins.remove(&refpath) {
                                    let refpath_new = format!("{new_prefix}{partial_path}");

                                    for weak_ref_elem in &reflist {
                                        if let Some(ref_elem) = weak_ref_elem.upgrade() {
                                            let mut ref_elem_locked = ref_elem.0.lock();
                                            // can't use .set_character_data() here, because the project is locked
                                            ref_elem_locked.content[0] = ElementContent::CharacterData(
                                                CharacterData::String(refpath_new.clone()),
                                            );
                                        }
                                    }
                                    project_locked.reference_origins.insert(refpath_new, reflist);
                                }
                            }
                        }
                    }
                }
            }

            Ok(())
        } else {
            Err(AutosarDataError::ElementNotIdentifiable)
        }
    }

    /// returns true if the element is identifiable
    pub(crate) fn is_identifiable(&self) -> bool {
        // is this element named in any autosar version? - If it's not named here then we'll simply fail in the next step
        if self.elemtype.is_named() {
            // if an item is named, then the SHORT-NAME sub element that contains the name is always the first sub element
            if let Some(ElementContent::Element(subelem)) = self.content.get(0) {
                if subelem.element_name() == ElementName::ShortName {
                    return true;
                }
            }
        }
        false
    }

    /// get the Autosar path of an identifiable element
    ///
    /// returns Some(path) if the element is identifiable, None otherwise
    pub(crate) fn path(&self) -> Result<String, AutosarDataError> {
        if self.is_identifiable() {
            self.path_unchecked()
        } else {
            Err(AutosarDataError::ElementNotIdentifiable)
        }
    }

    fn path_unchecked(&self) -> Result<String, AutosarDataError> {
        let mut path_components = vec![];

        if let Some(name) = self.item_name() {
            path_components.push(name);
        }

        let mut cur_elem_opt = self.parent()?;
        while let Some(cur_elem) = &cur_elem_opt {
            if let Some(name) = cur_elem
                .0
                .try_lock_for(std::time::Duration::from_millis(10))
                .ok_or(AutosarDataError::ParentElementLocked)?
                .item_name()
            {
                path_components.push(name);
            }
            cur_elem_opt = cur_elem.parent()?;
        }
        path_components.push(String::new());
        path_components.reverse();
        let path = path_components.join("/");
        Ok(path)
    }

    /// Create a sub element at a suitable insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// It is not possible to create named sub elements with this function; use create_named_sub_element() for that instead.
    pub(crate) fn create_sub_element(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let (_start_pos, end_pos) = self.find_element_insert_pos(element_name, version)?;
        self.create_sub_element_inner(self_weak, element_name, end_pos, version)
    }

    /// Create a sub element at the specified insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// It is not possible to create named sub elements with this function; use create_named_sub_element_at() for that instead.
    /// The specified insertion position will be compared to the range of valid insertion positions; if it falls ooutside that range then the function fails.
    pub(crate) fn create_sub_element_at(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        position: usize,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let (start_pos, end_pos) = self.find_element_insert_pos(element_name, version)?;
        if start_pos <= position && position <= end_pos {
            self.create_sub_element_inner(self_weak, element_name, position, version)
        } else {
            Err(AutosarDataError::InvalidPosition)
        }
    }

    /// helper function for create_sub_element and create_sub_element_at
    fn create_sub_element_inner(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        position: usize,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let (elemtype, _) = self
            .elemtype
            .find_sub_element(element_name, version as u32)
            .ok_or(AutosarDataError::InvalidSubElement)?;
        if elemtype.is_named_in_version(version) {
            Err(AutosarDataError::ItemNameRequired)
        } else {
            let sub_element = Element(Arc::new(Mutex::new(ElementRaw {
                parent: ElementOrFile::Element(self_weak),
                elemname: element_name,
                elemtype,
                content: smallvec![],
                attributes: smallvec![],
            })));
            self.content
                .insert(position, ElementContent::Element(sub_element.clone()));
            Ok(sub_element)
        }
    }

    /// create a named/identifiable sub element at a suitable insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// This method can only be used to create identifiable sub elements.
    pub(crate) fn create_named_sub_element(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        item_name: &str,
        project: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let (_start_pos, end_pos) = self.find_element_insert_pos(element_name, version)?;
        self.create_named_sub_element_inner(self_weak, element_name, item_name, end_pos, project, version)
    }

    /// create a named/identifiable sub element at the specified insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// The specified insertion position will be compared to the range of valid insertion positions; if it falls ooutside that range then the function fails.
    /// This method can only be used to create identifiable sub elements.
    pub(crate) fn create_named_sub_element_at(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        item_name: &str,
        position: usize,
        project: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let (start_pos, end_pos) = self.find_element_insert_pos(element_name, version)?;
        if start_pos <= position && position <= end_pos {
            self.create_named_sub_element_inner(self_weak, element_name, item_name, position, project, version)
        } else {
            Err(AutosarDataError::InvalidPosition)
        }
    }

    /// helper function for create_named_sub_element and create_named_sub_element_at
    fn create_named_sub_element_inner(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        item_name: &str,
        position: usize,
        project: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        if item_name.is_empty() {
            return Err(AutosarDataError::ItemNameRequired);
        }
        let item_name_cdata = CharacterData::String(item_name.to_owned());

        let (elemtype, _) = self
            .elemtype
            .find_sub_element(element_name, version as u32)
            .ok_or(AutosarDataError::InvalidSubElement)?;

        if elemtype.is_named_in_version(version) {
            // verify that the given item_name is actually a valid name
            if !elemtype
                .find_sub_element(ElementName::ShortName, version as u32)
                .and_then(|(se_type, _)| se_type.chardata_spec())
                .map(|cdata_spec| CharacterData::check_value(&item_name_cdata, cdata_spec, version))
                .unwrap_or(false)
            {
                return Err(AutosarDataError::IncorrectContentType);
            }

            let parent_path = self.path_unchecked()?;
            let path = format!("{parent_path}/{item_name}");
            // verify that the name is unique: there must be no existing element that already has this autosar path
            if project.get_element_by_path(&path).is_some() {
                return Err(AutosarDataError::DuplicateItemName);
            }

            // create the new element
            let sub_element = Element(Arc::new(Mutex::new(ElementRaw {
                parent: ElementOrFile::Element(self_weak),
                elemname: element_name,
                elemtype,
                content: smallvec![],
                attributes: smallvec![],
            })));
            self.content
                .insert(position, ElementContent::Element(sub_element.clone()));
            // create a SHORT-NAME for the sub element
            let shortname_element =
                sub_element
                    .0
                    .lock()
                    .create_sub_element(sub_element.downgrade(), ElementName::ShortName, version)?;
            let _ = shortname_element.0.lock().set_character_data(item_name_cdata, version);
            project.add_identifiable(path, sub_element.downgrade());
            Ok(sub_element)
        } else {
            Err(AutosarDataError::ElementNotIdentifiable)
        }
    }

    /// create a deep copy of the given element and insert it as a sub-element
    pub(crate) fn create_copied_sub_element(
        &mut self,
        self_weak: WeakElement,
        other: &Element,
        project: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let other_elemname = {
            // need to do this inside its own scope to limit the lifetime of the mutex
            let other_element = other.0.lock();
            other_element.elemname
        };
        let (_, end) = self.find_element_insert_pos(other_elemname, version)?;
        self.create_copied_sub_element_inner(self_weak, other, end, project, version)
    }

    /// create a deep copy of the given element and insert it as a sub-element at the given position
    pub(crate) fn create_copied_sub_element_at(
        &mut self,
        self_weak: WeakElement,
        other: &Element,
        position: usize,
        project: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let other_elemname = {
            // need to do this inside its own scope to limit the lifetime of the mutex
            let other_element = other.0.lock();
            other_element.elemname
        };
        let (start_pos, end_pos) = self.find_element_insert_pos(other_elemname, version)?;
        if start_pos <= position && position <= end_pos {
            self.create_copied_sub_element_inner(self_weak, other, position, project, version)
        } else {
            Err(AutosarDataError::InvalidPosition)
        }
    }

    fn create_copied_sub_element_inner(
        &mut self,
        self_weak: WeakElement,
        other: &Element,
        position: usize,
        project: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        // Arc overrides clone() so that it only manipulates the reference count, so a separate deep_copy operation is needed here.
        // Additionally, implementing this manually provides the opportunity to filter out
        // elements that ae not compatible with the version of the current file.
        let newelem = other.0.lock().deep_copy(version)?;
        let path = self.path_unchecked()?;

        // set the parent of the newelem - the methods path(), containing_file(), etc become available on newelem
        newelem.set_parent(ElementOrFile::Element(self_weak));
        if newelem.is_identifiable() {
            newelem.0.lock().make_unique_item_name(project, &path)?;
        }

        let mut path_parts: Vec<Option<String>> = vec![Some(path)];
        for (depth, sub_elem) in newelem.elements_dfs() {
            while path_parts.len() > depth + 1 {
                path_parts.pop();
            }
            // add each identifiable sub element to the identifiables hashmap
            if sub_elem.is_identifiable() {
                path_parts.push(sub_elem.item_name());
                let sub_elem_path = path_parts
                    .iter()
                    .filter_map(|x| x.clone())
                    .collect::<Vec<String>>()
                    .join("/");
                project.add_identifiable(sub_elem_path, sub_elem.downgrade());
            } else {
                path_parts.push(None);
            }
            // add all references to the reference_origins hashmap
            if sub_elem.is_reference() {
                if let Some(CharacterData::String(reference)) = sub_elem.character_data() {
                    project.add_reference_origin(&reference, sub_elem.downgrade())
                }
            }
        }

        self.content.insert(position, ElementContent::Element(newelem.clone()));

        Ok(newelem)
    }

    /// perform a deep copy of an element, but keep only those sub elements etc, which are compatible with target_version
    fn deep_copy(&self, target_version: AutosarVersion) -> Result<Element, AutosarDataError> {
        let copy_wrapped = Element(Arc::new(Mutex::new(ElementRaw {
            elemname: self.elemname,
            elemtype: self.elemtype,
            content: SmallVec::with_capacity(self.content.len()),
            attributes: SmallVec::with_capacity(self.attributes.len()),
            parent: ElementOrFile::None,
        })));

        {
            let mut copy = copy_wrapped.0.lock();
            // copy all the attributes
            for attribute in &self.attributes {
                // get the specification of the attribute
                let (cdataspec, required, attr_version_mask) = self
                    .elemtype
                    .find_attribute_spec(attribute.attrname)
                    .ok_or(AutosarDataError::VersionIncompatible)?;
                // check if the attribute is compatible with the target version
                if target_version.compatible(attr_version_mask)
                    && attribute
                        .content
                        .check_version_compatibility(cdataspec, target_version)
                        .0
                {
                    copy.attributes.push(attribute.clone());
                } else if required {
                    return Err(AutosarDataError::VersionIncompatible);
                } else {
                    // no action, the attribute is not compatible, but it's not required either
                }
            }

            // copy all content: sub elements and text items
            for content_item in &self.content {
                match content_item {
                    ElementContent::Element(sub_elem) => {
                        let sub_elem_name = sub_elem.element_name();
                        // since find_sub_element already considers the version, finding the element also means it's valid in the target_version
                        if self
                            .elemtype
                            .find_sub_element(sub_elem_name, target_version as u32)
                            .is_some()
                        {
                            if let Ok(copied_sub_elem) = sub_elem.0.lock().deep_copy(target_version) {
                                copied_sub_elem.0.lock().parent = ElementOrFile::Element(copy_wrapped.downgrade());
                                copy.content.push(ElementContent::Element(copied_sub_elem));
                            }
                        }
                    }
                    ElementContent::CharacterData(cdata) => {
                        copy.content.push(ElementContent::CharacterData(cdata.clone()));
                    }
                }
            }
        }

        Ok(copy_wrapped)
    }

    /// make_unique_item_name ensures that a copied element has a unique name
    fn make_unique_item_name(&self, project: &AutosarProject, parent_path: &str) -> Result<String, AutosarDataError> {
        let orig_name = self.item_name().ok_or(AutosarDataError::ElementNotIdentifiable)?;
        let mut name = orig_name.clone();
        let mut counter = 1;

        let mut path = format!("{parent_path}/{orig_name}");
        while project.get_element_by_path(&path).is_some() {
            name = format!("{orig_name}_{counter}");
            counter += 1;
            path = format!("{parent_path}/{name}");
        }

        if counter > 1 {
            // set the name directly by modifying the character content of the short name element
            // note: the method set_character_data is not suitable here, because it updates the identifiables hashmap
            if let Some(ElementContent::Element(short_name_elem)) = self.content.get(0) {
                // the SHORT-NAME at index 0 is guaranteed to exist, because the earlier is_identifiable check succeeded
                let mut sn_element = short_name_elem.0.lock();
                sn_element.content.clear();
                sn_element
                    .content
                    .push(ElementContent::CharacterData(CharacterData::String(name.clone())));
            }
        }

        Ok(name)
    }

    /// take an `element` from it's current location and place it in this element as a sub element
    ///
    /// The moved element can be taken from anywhere - even from a different arxml document that is not part of the same AutosarProject
    ///
    /// Restrictions:
    /// 1) The element must have a compatible element type. If it could not have been created here, then it can't be moved either.
    /// 2) The origin document of the element must have exactly the same AutosarVersion as the destination.
    pub(crate) fn move_element_here(
        &mut self,
        self_weak: WeakElement,
        move_element: &Element,
        project: &AutosarProject,
        project_src: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let move_element_name = move_element.element_name();
        let (_, end_pos) = self.find_element_insert_pos(move_element_name, version)?;

        if project == project_src {
            let src_parent = move_element.parent()?.ok_or(AutosarDataError::InvalidSubElement)?;
            if src_parent.downgrade() == self_weak {
                Ok(move_element.clone())
            } else {
                // move the element within the same project
                self.move_element_local(self_weak, move_element, end_pos, project, version)
            }
        } else {
            // move the element between different projects
            self.move_element_full(self_weak, move_element, end_pos, project, project_src, version)
        }
    }

    /// take an `element` from it's current location and place it at the given position in this element as a sub element
    ///
    /// The moved element can be taken from anywhere - even from a different arxml document that is not part of the same AutosarProject
    ///
    /// Restrictions:
    /// 1) The element must have a compatible element type. If it could not have been created here, then it can't be moved either.
    /// 2) The origin document of the element must have exactly the same AutosarVersion as the destination.
    pub(crate) fn move_element_here_at(
        &mut self,
        self_weak: WeakElement,
        move_element: &Element,
        position: usize,
        project: &AutosarProject,
        project_src: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let move_element_name = move_element.element_name();
        let (start_pos, end_pos) = self.find_element_insert_pos(move_element_name, version)?;

        if start_pos <= position && position <= end_pos {
            if project == project_src {
                let src_parent = move_element.parent()?.ok_or(AutosarDataError::InvalidSubElement)?;
                if src_parent.downgrade() == self_weak {
                    // move new_element to a different position within the current element
                    self.move_element_position(move_element, position)
                } else {
                    // move the element within the same project
                    self.move_element_local(self_weak, move_element, position, project, version)
                }
            } else {
                // move the element between different projects
                self.move_element_full(self_weak, move_element, position, project, project_src, version)
            }
        } else {
            Err(AutosarDataError::InvalidPosition)
        }
    }

    /// move a sub element within the current element to a different position
    fn move_element_position(&mut self, move_element: &Element, position: usize) -> Result<Element, AutosarDataError> {
        // need to check self.content.len() here, because find_element_insert_pos() will allow values up to len()+1
        // that's correct when adding elements to self.content, but not strict enough here
        if position < self.content.len() {
            let current_position = self
                .content
                .iter()
                .enumerate()
                .find(|(_, item)| {
                    if let ElementContent::Element(elem) = item {
                        *elem == *move_element
                    } else {
                        false
                    }
                })
                .map(|(idx, _)| idx)
                .unwrap();

            if current_position < position {
                // the first element in the subslice is moved to the last position by rotate_left
                self.content[current_position..=position].rotate_left(1);
            } else {
                // the last element in the subslice is moved to the first position by rotate_right
                self.content[position..=current_position].rotate_right(1);
            }

            Ok(move_element.clone())
        } else {
            Err(AutosarDataError::InvalidPosition)
        }
    }

    // remove an element from its current parent and make it a sub element of this element
    // the current location must be within the same project
    // All references to the moved sub element will be updated to refer to the new path
    fn move_element_local(
        &mut self,
        self_weak: WeakElement,
        move_element: &Element,
        position: usize,
        project: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        // check if self (target of the move) is a sub element of new_element
        // if it is, then the move is not allowed
        let mut wrapped_parent = self.parent.clone();
        while let ElementOrFile::Element(weak_parent) = wrapped_parent {
            let parent = weak_parent.upgrade().ok_or(AutosarDataError::ItemDeleted)?;
            if parent == *move_element {
                return Err(AutosarDataError::ForbiddenMoveToSubElement);
            }
            wrapped_parent = parent.0.lock().parent.clone();
        }

        let src_parent = move_element.parent()?.ok_or(AutosarDataError::InvalidSubElement)?;

        // collect the paths of all identifiable elements under new_element before moving it
        let original_paths: Vec<String> = move_element.elements_dfs().filter_map(|(_, e)| e.path().ok()).collect();

        let src_path_prefix = move_element.0.lock().path_unchecked()?;
        let dest_path_prefix = self.path_unchecked()?;

        // limit the lifetime of the mutex on src_parent
        {
            // lock the source parent element and remove the move_element from its content list
            let mut src_parent_locked = src_parent
                .0
                .try_lock_for(Duration::from_millis(10))
                .ok_or(AutosarDataError::ParentElementLocked)?;
            let idx = src_parent_locked
                .content
                .iter()
                .enumerate()
                .find(|(_, item)| {
                    if let ElementContent::Element(elem) = item {
                        *elem == *move_element
                    } else {
                        false
                    }
                })
                .map(|(idx, _)| idx)
                .unwrap();
            src_parent_locked.content.remove(idx);
        }

        // set the parent of the new element to the current element
        move_element.0.lock().parent = ElementOrFile::Element(self_weak);
        let dest_path = if move_element.is_identifiable() {
            let new_name = move_element
                .0
                .lock()
                .make_unique_item_name(project, &dest_path_prefix)?;
            format!("{dest_path_prefix}/{new_name}")
        } else {
            dest_path_prefix
        };

        // fix the identifiables cache
        if move_element.is_identifiable() {
            // simple case: the moved element is identifiable; fix_identifiables automatically handles the sub-elements
            project.fix_identifiables(&src_path_prefix, &dest_path);
        } else {
            // the moved element is not identifiable, so its identifiable sub-elements must be fixed individually
            for orig_path in &original_paths {
                if let Some(suffix) = orig_path.strip_prefix(&src_path_prefix) {
                    let updated_path = format!("{dest_path}{suffix}");
                    project.fix_identifiables(orig_path, &updated_path);
                }
            }
        }

        // the move_element was moved within this autosar project, so we can update all other references pointing to it
        let mut project_locked = project.0.lock();
        for orig_ref in &original_paths {
            if let Some(suffix) = orig_ref.strip_prefix(&src_path_prefix) {
                // e.g. orig_ref = "/Pkg/Foo/Sub/Element" and src_path_prefix = "/Pkg/Foo" then suffix = "/Sub/Element"
                // strip prefix can't fail, because all original_paths have the src_path_prefix
                if let Some(ref_elements) = project_locked.reference_origins.remove(orig_ref) {
                    let refstr = format!("{dest_path}{suffix}");
                    for ref_element_weak in &ref_elements {
                        if let Some(ref_element) = ref_element_weak.upgrade() {
                            ref_element
                                .0
                                .lock()
                                .set_character_data(CharacterData::String(refstr.clone()), version)?;
                        }
                    }
                    project_locked.reference_origins.insert(refstr, ref_elements);
                }
            }
        }

        // insert move_element
        self.content
            .insert(position, ElementContent::Element(move_element.clone()));

        Ok(move_element.clone())
    }

    // remove an element from its current parent and make it a sub element of this element
    // This is a move between two different projects
    // If the moved sub element contains its own tree of sub elements, then references within that tree will be updated
    fn move_element_full(
        &mut self,
        self_weak: WeakElement,
        move_element: &Element,
        position: usize,
        project: &AutosarProject,
        project_src: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let src_path_prefix = move_element.0.lock().path_unchecked()?;
        let dest_path_prefix = self.path_unchecked()?;
        let src_parent = move_element.parent()?.ok_or(AutosarDataError::InvalidSubElement)?;

        // collect the paths of all identifiable elements under move_element before moving it
        let original_paths: FxHashMap<String, Element> = move_element
            .elements_dfs()
            .filter_map(|(_, e)| e.path().ok().map(|path| (path, e)))
            .collect();
        // collect all reference targets and referring elements under move_element
        let original_refs: Vec<(String, Element)> = move_element
            .elements_dfs()
            .filter(|(_, e)| e.is_reference())
            .filter_map(|(_, e)| e.character_data().map(|data| (data.to_string(), e)))
            .collect();

        // limit the lifetime of the mutex on src_parent
        {
            // lock the parent of the new element and remove it from the parent's content list
            let mut src_parent_locked = src_parent
                .0
                .try_lock_for(Duration::from_millis(10))
                .ok_or(AutosarDataError::ParentElementLocked)?;
            let idx = src_parent_locked
                .content
                .iter()
                .enumerate()
                .find(|(_, item)| {
                    if let ElementContent::Element(elem) = item {
                        *elem == *move_element
                    } else {
                        false
                    }
                })
                .map(|(idx, _)| idx)
                .unwrap();
            src_parent_locked.content.remove(idx);
        }

        // remove all cached references for elements under move_element - they all become invalid as a result of moving it
        for path in original_paths.keys() {
            project_src.remove_identifiable(path);
        }
        // delete all reference origin info for elements under move_element
        for (path, elem) in &original_refs {
            project_src.remove_reference_origin(path, elem.downgrade());
        }

        // set the parent of the new element to the current element
        move_element.0.lock().parent = ElementOrFile::Element(self_weak);
        let dest_path = if move_element.is_identifiable() {
            let new_name = move_element
                .0
                .lock()
                .make_unique_item_name(project, &dest_path_prefix)?;
            format!("{dest_path_prefix}/{new_name}")
        } else {
            dest_path_prefix
        };

        // cache references to all the identifiable elements in move_element
        for (orig_path, identifiable_element) in &original_paths {
            if let Some(suffix) = orig_path.strip_prefix(&src_path_prefix) {
                let path = format!("{dest_path}{suffix}");
                project.add_identifiable(path, identifiable_element.downgrade());
            }
        }
        // cache all newly added reference origins under move_element
        for (old_ref, ref_element) in original_refs {
            // if the reference points to a known old path, then update it to use the new path instead
            if original_paths.contains_key(&old_ref) {
                let mut refstr = old_ref.clone();
                if let Some(suffix) = old_ref.strip_prefix(&src_path_prefix) {
                    refstr = format!("{dest_path}{suffix}");
                    ref_element
                        .0
                        .lock()
                        .set_character_data(CharacterData::String(refstr.clone()), version)?;
                }
                project.add_reference_origin(&refstr, ref_element.downgrade());
            }
        }

        // insert move_element
        self.content
            .insert(position, ElementContent::Element(move_element.clone()));

        Ok(move_element.clone())
    }

    /// find the upper and lower bound on the insert position for a sub element
    fn find_element_insert_pos(
        &self,
        element_name: ElementName,
        version: AutosarVersion,
    ) -> Result<(usize, usize), AutosarDataError> {
        let elemtype = self.elemtype;
        if self.elemtype.content_mode() == ContentMode::Characters {
            // cant't insert at all, only character data is permitted
            return Err(AutosarDataError::IncorrectContentType);
        }

        if let Some((_, new_element_indices)) = elemtype.find_sub_element(element_name, version as u32) {
            let mut start_pos = 0;
            let mut end_pos = 0;
            for (idx, content_item) in self.content.iter().enumerate() {
                if let ElementContent::Element(subelement) = content_item {
                    let (_, existing_element_indices) = elemtype
                        .find_sub_element(subelement.element_name(), version as u32)
                        .unwrap();
                    let group_type = elemtype.find_common_group(&new_element_indices, &existing_element_indices);
                    match group_type.content_mode() {
                        ContentMode::Sequence => {
                            // decide where to insert
                            match new_element_indices.cmp(&existing_element_indices) {
                                std::cmp::Ordering::Less => {
                                    // new element is smaller than the existing one
                                    // this means that all plausible insert positions have already been seen
                                    break;
                                }
                                std::cmp::Ordering::Equal => {
                                    // new element is not smaller than the current one, so set the end position
                                    end_pos = idx + 1;
                                    // are identical elements of this type allowed at all?
                                    if let Some(multiplicity) =
                                        elemtype.get_sub_element_multiplicity(&new_element_indices)
                                    {
                                        if multiplicity != ElementMultiplicity::Any {
                                            // the new element is identical to an existing one, but repetitions are not allowed
                                            return Err(AutosarDataError::ElementInsertionConflict);
                                        }
                                    }
                                }
                                std::cmp::Ordering::Greater => {
                                    // new element is greater (i.e. later in the sequence) than the current one
                                    // the erliest possible insert position is aftet the current element
                                    start_pos = idx + 1;
                                    end_pos = idx + 1;
                                }
                            }
                        }
                        ContentMode::Choice => {
                            // can insert elements that ar identical to the existing element, if more than one of this element is allowed
                            // can't insert anything else
                            if new_element_indices == existing_element_indices {
                                if let Some(multiplicity) = elemtype.get_sub_element_multiplicity(&new_element_indices)
                                {
                                    if multiplicity != ElementMultiplicity::Any {
                                        // the new element is identical to an existing one, but repetitions are not allowed
                                        return Err(AutosarDataError::ElementInsertionConflict);
                                    }
                                }
                                // the existing element and the new element are equal in positioning
                                // start_pos remains at its current value (< current position), while
                                // end_pos is increased to allow inserting before or after this element
                                end_pos = idx + 1;
                            } else {
                                return Err(AutosarDataError::ElementInsertionConflict);
                            }
                        }
                        ContentMode::Bag | ContentMode::Mixed => {
                            // can insert before or after the current element
                            end_pos = idx + 1;
                        }
                        ContentMode::Characters => {
                            panic!(); // impossible on sub-groups
                        }
                    }
                } else if let ElementContent::CharacterData(_) = content_item {
                    end_pos = idx + 1;
                }
            }

            Ok((start_pos, end_pos))
        } else {
            Err(AutosarDataError::InvalidSubElement)
        }
    }

    /// remove the sub element sub_element
    ///
    /// The sub_element will be unlinked from the hierarchy of elements. All of the sub-sub-elements nested under the removed element will also be recusively removed.
    /// Since all elements are reference counted, they might not be deallocated immediately, however they do become invalid and unusable immediately.
    pub(crate) fn remove_sub_element(
        &mut self,
        sub_element: Element,
        project: &AutosarProject,
    ) -> Result<(), AutosarDataError> {
        let path = Cow::from(self.path_unchecked()?);
        let mut sub_element_locked = sub_element.0.lock();
        // find the position of sub_element in the parent element first to verify that sub_element actuall *is* a sub element
        let pos = self
            .content
            .iter()
            .enumerate()
            .find(|(_, item)| {
                if let ElementContent::Element(elem) = item {
                    *elem == sub_element
                } else {
                    false
                }
            })
            .map(|(idx, _)| idx)
            .ok_or(AutosarDataError::ElementNotFound)?;
        if self.elemtype.is_named() && sub_element_locked.elemname == ElementName::ShortName {
            // may not remove the SHORT-NAME, because that would leave the data in an invalid state
            return Err(AutosarDataError::ShortNameRemovalForbidden);
        }
        sub_element_locked.remove_internal(sub_element.downgrade(), project, path);
        self.content.remove(pos);
        Ok(())
    }

    // remove all of the content of an element
    pub(crate) fn remove_internal(&mut self, self_weak: WeakElement, project: &AutosarProject, mut path: Cow<str>) {
        if self.is_identifiable() {
            if let Some(name) = self.item_name() {
                let mut new_path = String::with_capacity(path.len() + name.len() + 1);
                new_path.push_str(&path);
                new_path.push('/');
                new_path.push_str(&name);
                path = Cow::from(new_path.clone());

                project.remove_identifiable(&path);
            }
        }
        if self.elemtype.is_ref() {
            if let Some(CharacterData::String(reference)) = self.character_data() {
                // remove the references-reference (ugh. terminology???)
                project.remove_reference_origin(&reference, self_weak);
            }
        }
        for item in &self.content {
            if let ElementContent::Element(sub_element) = item {
                sub_element
                    .0
                    .lock()
                    .remove_internal(sub_element.downgrade(), project, Cow::from(path.as_ref()));
            }
        }
        self.content.clear();
        self.parent = ElementOrFile::None;
    }

    /// set the character data of this element
    ///
    /// This method only applies to elements which contain character data, i.e. element.content_type == CharacterData
    pub(crate) fn set_character_data(
        &mut self,
        chardata: CharacterData,
        version: AutosarVersion,
    ) -> Result<(), AutosarDataError> {
        if self.elemtype.content_mode() == ContentMode::Characters {
            if let Some(cdata_spec) = self.elemtype.chardata_spec() {
                if CharacterData::check_value(&chardata, cdata_spec, version) {
                    // update the character data
                    if self.content.is_empty() {
                        self.content.push(ElementContent::CharacterData(chardata));
                    } else {
                        self.content[0] = ElementContent::CharacterData(chardata);
                    }
                    return Ok(());
                }
            }
        }
        Err(AutosarDataError::IncorrectContentType)
    }

    /// get the character content of the element
    ///
    /// This method only applies to elements which contain character data, i.e. element.content_type == CharacterData
    pub(crate) fn character_data(&self) -> Option<CharacterData> {
        if let ContentMode::Characters = self.elemtype.content_mode() {
            if let Some(ElementContent::CharacterData(cdata)) = self.content.get(0) {
                return Some(cdata.clone());
            }
        }
        None
    }

    /// get a single attribute by name
    pub(crate) fn attribute_value(&self, attrname: AttributeName) -> Option<CharacterData> {
        if let Some(attr) = self.attributes.iter().find(|attr| attr.attrname == attrname) {
            return Some(attr.content.clone());
        }
        None
    }

    /// get the content of a named attribute as a string
    pub(crate) fn attribute_string(&self, attrname: AttributeName) -> Option<String> {
        if let Some(chardata) = self.attribute_value(attrname) {
            match chardata {
                CharacterData::String(stringval) => return Some(stringval),
                other => return Some(other.to_string()),
            }
        }
        None
    }

    pub(crate) fn set_attribute_internal(
        &mut self,
        attrname: AttributeName,
        value: CharacterData,
        file_version: AutosarVersion,
    ) -> Result<(), AutosarDataError> {
        // find the attribute specification in the item type
        if let Some((spec, _, _)) = self.elemtype.find_attribute_spec(attrname) {
            // the existing attribute gets updated
            if CharacterData::check_value(&value, spec, file_version) {
                // find the attribute the element's attribute list
                if let Some(attr) = self.attributes.iter_mut().find(|attr| attr.attrname == attrname) {
                    attr.content = value;
                } else {
                    self.attributes.push(Attribute {
                        attrname,
                        content: value,
                    });
                }
                Ok(())
            } else {
                Err(AutosarDataError::InvalidAttributeValue)
            }
        } else {
            Err(AutosarDataError::InvalidAttribute)
        }
    }

    pub(crate) fn set_attribute_string(
        &mut self,
        attrname: AttributeName,
        stringvalue: &str,
        version: AutosarVersion,
    ) -> Result<(), AutosarDataError> {
        if let Some((character_data_spec, _, _)) = self.elemtype.find_attribute_spec(attrname) {
            if let Some(value) = CharacterData::parse(stringvalue, character_data_spec, version) {
                if let Some(attr) = self.attributes.iter_mut().find(|attr| attr.attrname == attrname) {
                    attr.content = value;
                } else {
                    self.attributes.push(Attribute {
                        attrname,
                        content: value,
                    });
                }
                Ok(())
            } else {
                Err(AutosarDataError::InvalidAttributeValue)
            }
        } else {
            Err(AutosarDataError::InvalidAttribute)
        }
    }

    /// remove an attribute from the element
    pub(crate) fn remove_attribute(&mut self, attrname: AttributeName) -> bool {
        // find the index of the attribute in the attribute list of the element
        for idx in 0..self.attributes.len() {
            if self.attributes[idx].attrname == attrname {
                // find the definition of this attribute in the specification
                if let Some((_, required, _)) = self.elemtype.find_attribute_spec(attrname) {
                    // the attribute can only be removed if it is optional
                    if !required {
                        self.attributes.remove(idx);
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

impl WeakElement {
    /// try to get a strong reference to the [Element]
    pub fn upgrade(&self) -> Option<Element> {
        Weak::upgrade(&self.0).map(Element)
    }
}

impl PartialEq for WeakElement {
    fn eq(&self, other: &Self) -> bool {
        Weak::as_ptr(&self.0) == Weak::as_ptr(&other.0)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::ffi::OsString;

    const BASIC_AUTOSAR_FILE: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
            <AR-PACKAGE>
                <SHORT-NAME>TestPackage</SHORT-NAME>
            </AR-PACKAGE>
        </AR-PACKAGES>
    </AUTOSAR>"#;

    #[test]
    fn element_creation() {
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_autosar = project.files().next().unwrap().root_element();
        let el_ar_package = project.get_element_by_path("/TestPackage").unwrap();

        let el_elements = el_ar_package.create_sub_element(ElementName::Elements).unwrap();
        let el_compu_method = el_elements
            .create_named_sub_element(ElementName::CompuMethod, "TestCompuMethod")
            .unwrap();
        el_elements
            .create_named_sub_element(ElementName::CompuMethod, "TestCompuMethod2")
            .unwrap();
        el_elements
            .create_named_sub_element(ElementName::CompuMethod, "TestCompuMethod3")
            .unwrap();
        // elements with duplicate names are not allowed
        assert!(el_elements
            .create_named_sub_element(ElementName::CompuMethod, "TestCompuMethod3")
            .is_err());

        let count = el_elements.sub_elements().count();
        assert_eq!(count, 3);

        // inserting another COMPU-METHOD into ELEMENTS hould be allowed at any position
        let (start_pos, end_pos) = el_elements
            .0
            .lock()
            .find_element_insert_pos(ElementName::CompuMethod, AutosarVersion::Autosar_00050)
            .unwrap();
        assert_eq!(start_pos, 0);
        assert_eq!(end_pos, 3); // upper limit is 3 since there are currently 3 elements

        // check if create_named_sub_element correctly registered the element in the hashmap so that it can be found
        let el_compu_method_test = project.get_element_by_path("/TestPackage/TestCompuMethod").unwrap();
        assert_eq!(el_compu_method, el_compu_method_test);

        // create more hierarchy
        let el_compu_internal_to_phys = el_compu_method
            .create_sub_element(ElementName::CompuInternalToPhys)
            .unwrap();
        let el_compu_scales = el_compu_internal_to_phys
            .create_sub_element(ElementName::CompuScales)
            .unwrap();
        let el_compu_scale = el_compu_scales.create_sub_element(ElementName::CompuScale).unwrap();
        el_compu_scale.create_sub_element(ElementName::Desc).unwrap();

        // SHORT-LABEL should only be allowed before DESC inside COMPU-SCALE
        let (start_pos, end_pos) = el_compu_scale
            .0
            .lock()
            .find_element_insert_pos(ElementName::ShortLabel, AutosarVersion::Autosar_00050)
            .unwrap();
        assert_eq!(start_pos, 0);
        assert_eq!(end_pos, 0);

        // COMPU-CONST should only be allowed after DESC inside COMPU-SCALE
        let (start_pos, end_pos) = el_compu_scale
            .0
            .lock()
            .find_element_insert_pos(ElementName::CompuConst, AutosarVersion::Autosar_00050)
            .unwrap();
        assert_eq!(start_pos, 1);
        assert_eq!(end_pos, 1);

        // create COMPU-RATIONAL-COEFFS in COMPU-SCALE. It's presence excludes COMPU-CONST from being inserted
        el_compu_scale
            .create_sub_element(ElementName::CompuRationalCoeffs)
            .unwrap();
        // try to insert COMPU-CONST anyway
        let result = el_compu_scale
            .0
            .lock()
            .find_element_insert_pos(ElementName::CompuConst, AutosarVersion::Autosar_00050);
        assert!(result.is_err());
        // it is also not possible to create a second COMPU-RATIONAL-COEFFS
        let result = el_compu_scale
            .0
            .lock()
            .find_element_insert_pos(ElementName::CompuRationalCoeffs, AutosarVersion::Autosar_00050);
        assert!(result.is_err());

        // creating a sub element at an invalid position fails
        assert!(el_elements
            .create_named_sub_element_at(ElementName::System, "System", 99)
            .is_err());
        assert!(el_autosar.create_sub_element_at(ElementName::AdminData, 99).is_err());

        // an identifiable element cannot be created without a name
        assert!(el_elements.create_sub_element(ElementName::System).is_err());
        // the name for an identifiable element must be valid according to the rules
        assert!(el_elements.create_named_sub_element(ElementName::System, "").is_err());
        assert!(el_elements
            .create_named_sub_element(ElementName::System, "abc def")
            .is_err());

        // a non-identifiable element cannot be created with a name
        assert!(el_autosar
            .create_named_sub_element(ElementName::AdminData, "AdminData")
            .is_err());

        // only valid sub-elements can be created
        assert!(el_autosar
            .create_named_sub_element(ElementName::Autosar, "Autosar")
            .is_err());
        assert!(el_autosar
            .create_named_sub_element_at(ElementName::Autosar, "Autosar", 0)
            .is_err());
        assert!(el_autosar.create_sub_element(ElementName::Autosar).is_err());
        assert!(el_autosar.create_sub_element_at(ElementName::Autosar, 0).is_err());

        // creating a sub element fails when any parent element in the hierarchy is locked
        let el_autosar_locked = el_autosar.0.lock();
        assert!(el_elements
            .create_named_sub_element(ElementName::System, "System")
            .is_err());
        assert!(el_elements
            .create_named_sub_element_at(ElementName::System, "System", 0)
            .is_err());
        assert!(el_autosar.create_sub_element(ElementName::AdminData).is_err());
        assert!(el_autosar.create_sub_element_at(ElementName::AdminData, 0).is_err());
        drop(el_autosar_locked);
    }

    #[test]
    fn element_rename() {
        let project = AutosarProject::new();
        let file = project
            .create_file("test.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar = file.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Package")
            .unwrap();
        let el_elements = el_ar_package.create_sub_element(ElementName::Elements).unwrap();
        let el_can_cluster = el_elements
            .create_named_sub_element_at(ElementName::CanCluster, "CanCluster", 0)
            .unwrap();
        let el_can_physical_channel = el_can_cluster
            .create_sub_element(ElementName::CanClusterVariants)
            .and_then(|ccv| ccv.create_sub_element(ElementName::CanClusterConditional))
            .and_then(|ccc| ccc.create_sub_element(ElementName::PhysicalChannels))
            .and_then(|pc| pc.create_named_sub_element(ElementName::CanPhysicalChannel, "CanPhysicalChannel"))
            .unwrap();

        let el_can_frame_triggering = el_can_physical_channel
            .create_sub_element_at(ElementName::FrameTriggerings, 1)
            .and_then(|ft| ft.create_named_sub_element(ElementName::CanFrameTriggering, "CanFrameTriggering"))
            .unwrap();

        let el_ar_package2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Package2")
            .unwrap();
        let el_can_frame = el_ar_package2
            .create_sub_element(ElementName::Elements)
            .and_then(|e| e.create_named_sub_element(ElementName::CanFrame, "CanFrame"))
            .unwrap();
        let el_frame_ref = el_can_frame_triggering
            .create_sub_element(ElementName::FrameRef)
            .unwrap();
        let _ = el_frame_ref.set_reference_target(&el_can_frame);

        // initial value of the reference
        let refstr = el_frame_ref.character_data().unwrap().string_value().unwrap();
        assert_eq!(refstr, "/Package2/CanFrame");

        // empty name, renaming should fail
        let result = el_ar_package.set_item_name("");
        assert!(result.is_err());

        // rename 1. package
        el_ar_package.set_item_name("NewPackage").unwrap();
        // setting the current name again - should be a no-op
        el_ar_package.set_item_name("NewPackage").unwrap();

        // duplicate name for Package2, renaming should fail
        let result = el_ar_package2.set_item_name("NewPackage");
        assert!(result.is_err());

        // rename package 2 with a valid name
        el_ar_package2.set_item_name("OtherPackage").unwrap();
        let refstr = el_frame_ref.character_data().unwrap().string_value().unwrap();
        assert_eq!(refstr, "/OtherPackage/CanFrame");

        // make sure get_reference_target still works after renaming
        let el_can_frame2 = el_frame_ref.get_reference_target().unwrap();
        assert_eq!(el_can_frame, el_can_frame2);

        // rename the CanFrame as well
        el_can_frame.set_item_name("CanFrame_renamed").unwrap();
        let refstr = el_frame_ref.character_data().unwrap().string_value().unwrap();
        assert_eq!(refstr, "/OtherPackage/CanFrame_renamed");

        // invalid element
        assert!(el_autosar.set_item_name("Autosar").is_err());

        // invalid preconditions
        let el_autosar_locked = el_autosar.0.lock();
        // fails because a parent element is locked
        assert!(el_ar_package.set_item_name("TestPackage_renamed").is_err());
        drop(el_autosar_locked);
        drop(project);
        // the reference count of project is now zero, so set_item_name can't get a new reference to it
        assert!(el_ar_package.set_item_name("TestPackage_renamed").is_err());
    }

    #[test]
    fn element_copy() {
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = project.get_element_by_path("/TestPackage").unwrap();
        el_ar_package
            .set_attribute(AttributeName::Uuid, CharacterData::String("0123456".to_string()))
            .unwrap();
        let el_elements = el_ar_package.create_sub_element(ElementName::Elements).unwrap();
        let el_compu_method = el_elements
            .create_named_sub_element(ElementName::CompuMethod, "CompuMethod")
            .unwrap();
        el_elements
            .create_named_sub_element(ElementName::DdsServiceInstanceToMachineMapping, "ApItem")
            .unwrap();
        el_elements
            .create_named_sub_element(ElementName::AclObjectSet, "AclObjectSet")
            .and_then(|el| el.create_sub_element(ElementName::DerivedFromBlueprintRefs))
            .and_then(|el| el.create_sub_element(ElementName::DerivedFromBlueprintRef))
            .and_then(|el| {
                el.set_attribute(
                    AttributeName::Dest,
                    CharacterData::Enum(EnumItem::AbstractImplementationDataType),
                )
            })
            .unwrap();
        el_elements
            .create_named_sub_element(ElementName::System, "System")
            .and_then(|el| el.create_sub_element(ElementName::FibexElements))
            .and_then(|el| el.create_sub_element(ElementName::FibexElementRefConditional))
            .and_then(|el| el.create_sub_element(ElementName::FibexElementRef))
            .and_then(|el| el.set_character_data(CharacterData::String("/invalid".to_string())))
            .unwrap();

        let project2 = AutosarProject::new();
        let file = project2
            .create_file(OsString::from("test.arxml"), AutosarVersion::Autosar_00044)
            .unwrap();

        // it should not be possible to create an AR-PACKAGE element directly in the AUTOSAR element by copying data
        let result = file.root_element().create_copied_sub_element(&el_ar_package);
        assert!(result.is_err());

        // create an AR-PACKAGES element and copy the data there. This should succeed.
        // the copied data shoud contain the COMPU-METHOD, but not the DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
        // because the latter was specified in Adaptive 18-03 (Autosar_00045) and is not valid in Autosar_00044
        let el_ar_packages2 = file.root_element().create_sub_element(ElementName::ArPackages).unwrap();
        el_ar_packages2.create_copied_sub_element(&el_ar_package).unwrap();

        // it should be possible to look up the copied compu-method by its path
        let el_compu_method_2 = project2.get_element_by_path("/TestPackage/CompuMethod").unwrap();

        // the copy should not refer to the same memory as the original
        assert_ne!(el_compu_method, el_compu_method_2);
        // the copy should serialize to exactly the same string as the original
        assert_eq!(el_compu_method.serialize(), el_compu_method_2.serialize());

        // verify that the DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING element was not copied
        let result = project2.get_element_by_path("/TestPackage/ApItem");
        assert!(result.is_none());

        // make sure the element ordering constraints are considered when copying with the _at() variant
        let el_ar_package2 = el_ar_packages2
            .create_named_sub_element(ElementName::ArPackage, "Package2")
            .unwrap();
        let result = el_ar_package2.create_copied_sub_element_at(&el_elements, 0);
        assert!(result.is_err()); // position 0 is already used by the SHORT-NAME
        let el_elements2 = el_ar_package2.create_sub_element(ElementName::Elements).unwrap();
        let result = el_elements2.create_copied_sub_element_at(&el_compu_method, 99);
        assert!(result.is_err()); // position 99 is not valid
        let result = el_elements2.create_copied_sub_element_at(&el_compu_method, 0);
        assert!(result.is_ok()); // position 0 is valid

        // can't copy an element that is not a valid sub element here
        let result = el_ar_package2.create_copied_sub_element_at(&el_compu_method, 0);
        assert!(result.is_err()); // COMPU-METHOS id not a valid sub-element of AR-PACKAGE
    }

    #[test]
    fn element_deletion() {
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = project.get_element_by_path("/TestPackage").unwrap();
        let el_short_name = el_ar_package.get_sub_element(ElementName::ShortName).unwrap();
        el_ar_package
            .create_sub_element(ElementName::Elements)
            .and_then(|el| el.create_named_sub_element(ElementName::System, "System"))
            .and_then(|el| el.create_sub_element(ElementName::FibexElements))
            .and_then(|el| el.create_sub_element(ElementName::FibexElementRefConditional))
            .and_then(|el| el.create_sub_element(ElementName::FibexElementRef))
            .and_then(|el| el.set_character_data(CharacterData::String("/invalid".to_string())))
            .unwrap();

        // removing the SHORT-NAME of an identifiable element is forbidden
        let result = el_ar_package.remove_sub_element(el_short_name);
        if let Err(AutosarDataError::ShortNameRemovalForbidden) = result {
            // correct
        } else {
            panic!("Removing the SHORT-NAME was not prohibited");
        }
        let el_ar_packages = el_ar_package.parent().unwrap().unwrap();
        let result = el_ar_packages.remove_sub_element(el_ar_package);
        // deleting identifiable elements should also cause the cached references to them to be removed
        assert_eq!(project.0.lock().identifiables.len(), 0);
        assert!(result.is_ok());
    }

    #[test]
    fn element_type() {
        let project = AutosarProject::new();
        let file = project
            .create_file("test.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar = file.root_element();

        assert_eq!(el_autosar.element_type(), ElementType::ROOT);
    }

    #[test]
    fn file() {
        let project = AutosarProject::new();
        let file = project
            .create_file("test.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar = file.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();

        // ok case
        assert!(el_autosar.file().is_ok());

        // invalid reference to a file
        drop(project);
        drop(file);
        assert!(el_autosar.file().is_err());

        // no reference to a file
        el_autosar.0.lock().parent = ElementOrFile::None;
        assert!(el_autosar.file().is_err());

        // invalid parent
        drop(el_autosar);
        assert!(el_ar_packages.file().is_err());
    }

    #[test]
    fn content_type() {
        let project = AutosarProject::new();
        let file = project
            .create_file("test.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar = file.root_element();
        let el_ar_package = el_autosar
            .create_sub_element(ElementName::ArPackages)
            .and_then(|ar_pkgs| ar_pkgs.create_named_sub_element(ElementName::ArPackage, "Package"))
            .unwrap();
        let el_short_name = el_ar_package.get_sub_element(ElementName::ShortName).unwrap();

        let el_l4 = el_ar_package
            .create_sub_element(ElementName::LongName)
            .and_then(|ln| ln.create_sub_element(ElementName::L4))
            .unwrap();

        let el_elements = el_ar_package.create_sub_element(ElementName::Elements).unwrap();
        let el_debounce_algo = el_elements
            .create_named_sub_element(ElementName::DiagnosticContributionSet, "DCS")
            .and_then(|dcs| dcs.create_sub_element(ElementName::CommonProperties))
            .and_then(|cp| cp.create_sub_element(ElementName::DiagnosticCommonPropsVariants))
            .and_then(|dcpv| dcpv.create_sub_element(ElementName::DiagnosticCommonPropsConditional))
            .and_then(|dcpc| dcpc.create_sub_element(ElementName::DebounceAlgorithmPropss))
            .and_then(|dap| dap.create_named_sub_element(ElementName::DiagnosticDebounceAlgorithmProps, "ddap"))
            .and_then(|ddap| ddap.create_sub_element(ElementName::DebounceAlgorithm))
            .unwrap();

        assert_eq!(el_autosar.element_type().content_mode(), ContentMode::Sequence);
        assert_eq!(el_autosar.content_type(), ContentType::Elements);
        assert_eq!(el_elements.element_type().content_mode(), ContentMode::Bag);
        assert_eq!(el_elements.content_type(), ContentType::Elements);
        assert_eq!(el_debounce_algo.element_type().content_mode(), ContentMode::Choice);
        assert_eq!(el_debounce_algo.content_type(), ContentType::Elements);
        assert_eq!(el_short_name.element_type().content_mode(), ContentMode::Characters);
        assert_eq!(el_short_name.content_type(), ContentType::CharacterData);
        assert_eq!(el_l4.element_type().content_mode(), ContentMode::Mixed);
        assert_eq!(el_l4.content_type(), ContentType::Mixed);
    }

    #[test]
    fn attributes() {
        let project = AutosarProject::new();
        let (file, _) = project
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_autosar = file.root_element();
        let el_ar_packages = el_autosar.get_sub_element(ElementName::ArPackages).unwrap();

        let count = el_autosar.attributes().count();
        assert_eq!(count, 3);

        // set the attribute S on the element AUTOSAR
        el_autosar
            .set_attribute(AttributeName::S, CharacterData::String(String::from("something")))
            .unwrap();

        // AUTOSAR has no DEST attribute, so this should fail
        assert!(el_autosar
            .set_attribute(AttributeName::Dest, CharacterData::String(String::from("something")))
            .is_err());

        // The attribute S exists and is optional, so it can be removed
        let result = el_autosar.remove_attribute(AttributeName::S);
        assert_eq!(result, true);

        // the attribute xmlns is required and cannot be removed
        let result = el_autosar.remove_attribute(AttributeName::xmlns);
        assert_eq!(result, false);

        // the attribute ACCESSKEY does not exist in the element AUTOSAR and cannot be removed
        let result = el_autosar.remove_attribute(AttributeName::Accesskey);
        assert_eq!(result, false);

        // the attribute T is permitted on AUTOSAR and the string is a valid value
        el_autosar
            .set_attribute_string(AttributeName::T, "2022-01-31T13:00:59Z")
            .unwrap();

        // update an existing attribute
        el_autosar
            .set_attribute_string(AttributeName::T, "2022-01-31T14:00:59Z")
            .unwrap();

        // fail set an attribute due to data validation
        assert!(el_autosar.set_attribute_string(AttributeName::T, "abc").is_err());

        // can't set unknown attributes with set_attribute_string
        assert!(el_ar_packages
            .set_attribute_string(AttributeName::xmlns, "abc")
            .is_err());

        // directly return an attribute as a string
        let xmlns = el_autosar.attribute_string(AttributeName::xmlns).unwrap();
        assert_eq!(xmlns, "http://autosar.org/schema/r4.0".to_string());

        // attribute operation fails when a parent element is locked
        let lock = el_autosar.0.lock();
        assert!(el_ar_packages
            .set_attribute(AttributeName::Uuid, CharacterData::String(String::from("1234")))
            .is_err());
        assert!(el_ar_packages
            .set_attribute_string(AttributeName::Uuid, "1234")
            .is_err());
        drop(lock);
    }

    #[test]
    fn mixed_content() {
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = project.get_element_by_path("/TestPackage").unwrap();
        let el_long_name = el_ar_package.create_sub_element(ElementName::LongName).unwrap();
        assert_eq!(el_long_name.content_type(), ContentType::Elements);
        let el_l_4 = el_long_name.create_sub_element(ElementName::L4).unwrap();
        assert_eq!(el_l_4.content_type(), ContentType::Mixed);

        el_l_4.create_sub_element(ElementName::E).unwrap();
        el_l_4.insert_character_content_item("foo", 1).unwrap();
        el_l_4.create_sub_element(ElementName::Sup).unwrap();
        el_l_4.insert_character_content_item("bar", 0).unwrap();
        assert_eq!(el_l_4.content().count(), 4);

        // character data item "foo" is now in position 2 and gets removed
        assert!(el_l_4.remove_character_content_item(2).is_ok());
        assert_eq!(el_l_4.content().count(), 3);
        // character data item "bar" should be in postion 0
        let item = el_l_4.content().next().unwrap();
        if let ElementContent::CharacterData(CharacterData::String(content)) = item {
            assert_eq!(content, "bar");
        } else {
            panic!("unexpected content in <L-4>: {item:?}");
        }
    }

    #[test]
    fn move_element_position() {
        // move an element to a different position within its parent
        let project = AutosarProject::new();
        let file = project
            .create_file("test.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar = file.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let pkg1 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg1")
            .unwrap();
        let pkg2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg2")
            .unwrap();
        let pkg3 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg3")
            .unwrap();

        // "moving" an element inside its parent without actually giving a position is a no-op
        el_ar_packages.move_element_here(&pkg3).unwrap();

        // moving to an invalid position fails
        assert!(el_ar_packages.move_element_here_at(&pkg1, 99).is_err());
        assert!(el_ar_packages.move_element_here_at(&pkg1, 3).is_err()); // special boundary case

        // move an element forward
        el_ar_packages.move_element_here_at(&pkg2, 0).unwrap();
        // move an element backward
        el_ar_packages.move_element_here_at(&pkg1, 2).unwrap();
        // check the new ordering
        let mut packages_iter = el_ar_packages.sub_elements();
        assert_eq!(packages_iter.next().unwrap(), pkg2);
        assert_eq!(packages_iter.next().unwrap(), pkg3);
        assert_eq!(packages_iter.next().unwrap(), pkg1);

        // moving elements should also work with mixed content
        let el_l_4 = pkg1
            .create_sub_element(ElementName::LongName)
            .and_then(|el| el.create_sub_element(ElementName::L4))
            .unwrap();
        el_l_4.create_sub_element(ElementName::E).unwrap();
        el_l_4.insert_character_content_item("foo", 1).unwrap();
        let el_sup = el_l_4.create_sub_element(ElementName::Sup).unwrap();
        el_l_4.insert_character_content_item("bar", 0).unwrap();
        el_l_4.move_element_here_at(&el_sup, 0).unwrap();
        let mut iter = el_l_4.sub_elements();
        assert_eq!(iter.next().unwrap(), el_sup);
    }

    #[test]
    fn move_element_local() {
        // move an element within the same project
        let project = AutosarProject::new();
        let file = project
            .create_file("test.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar = file.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_pkg1 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg1")
            .unwrap();
        let el_elements1 = el_pkg1.create_sub_element(ElementName::Elements).unwrap();
        let el_ecu_instance = el_elements1
            .create_named_sub_element(ElementName::EcuInstance, "EcuInstance")
            .unwrap();
        let el_pkg2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg2")
            .unwrap();
        let el_pkg3 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg3")
            .unwrap();
        let el_fibex_element_ref = el_pkg3
            .create_sub_element(ElementName::Elements)
            .and_then(|el| el.create_named_sub_element(ElementName::System, "System"))
            .and_then(|el| el.create_sub_element(ElementName::FibexElements))
            .and_then(|el| el.create_sub_element(ElementName::FibexElementRefConditional))
            .and_then(|el| el.create_sub_element(ElementName::FibexElementRef))
            .unwrap();
        el_fibex_element_ref.set_reference_target(&el_ecu_instance).unwrap();

        // can't move an element of the wrong type
        assert!(el_ar_packages.move_element_here(&el_autosar).is_err());
        assert!(el_ar_packages.move_element_here_at(&el_autosar, 0).is_err());

        // moving an element into its own sub element (creating a loop) is forbidden
        assert!(el_pkg1.move_element_here(&el_ar_packages).is_err());
        assert!(el_pkg1.move_element_here_at(&el_ar_packages, 1).is_err());

        // move an unnamed element
        assert!(project.get_element_by_path("/Pkg1/EcuInstance").is_some());
        el_pkg2.move_element_here(&el_elements1).unwrap();
        assert_eq!(el_elements1.parent().unwrap().unwrap(), el_pkg2);
        assert!(project.get_element_by_path("/Pkg2/EcuInstance").is_some());
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance);

        // move the unnamed element back using the _at variant
        el_pkg1.move_element_here_at(&el_elements1, 1).unwrap();
        assert_eq!(el_elements1.parent().unwrap().unwrap(), el_pkg1);
        assert!(project.get_element_by_path("/Pkg1/EcuInstance").is_some());
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance);

        // move a named element
        let el_elements2 = el_pkg2.create_sub_element(ElementName::Elements).unwrap();
        el_elements2.move_element_here(&el_ecu_instance).unwrap();
        assert_eq!(el_ecu_instance.parent().unwrap().unwrap(), el_elements2);
        assert!(project.get_element_by_path("/Pkg2/EcuInstance").is_some());
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance);

        // moving an element should automatically resolve name conflicts
        el_elements1
            .create_named_sub_element(ElementName::EcuInstance, "EcuInstance")
            .unwrap();
        el_elements1.move_element_here_at(&el_ecu_instance, 0).unwrap();
        assert_eq!(el_ecu_instance.parent().unwrap().unwrap(), el_elements1);
        assert!(project.get_element_by_path("/Pkg1/EcuInstance_1").is_some());
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance);
    }

    #[test]
    fn move_element_full() {
        // move an element between two projects
        let project1 = AutosarProject::new();
        let file = project1
            .create_file("test1.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar = file.root_element();
        let el_ar_packages1 = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_pkg1 = el_ar_packages1
            .create_named_sub_element(ElementName::ArPackage, "Pkg1")
            .unwrap();
        let el_elements1 = el_pkg1.create_sub_element(ElementName::Elements).unwrap();
        let el_ecu_instance = el_elements1
            .create_named_sub_element(ElementName::EcuInstance, "EcuInstance")
            .unwrap();
        let el_pkg2 = el_ar_packages1
            .create_named_sub_element(ElementName::ArPackage, "Pkg2")
            .unwrap();
        let el_fibex_element_ref = el_pkg2
            .create_sub_element(ElementName::Elements)
            .and_then(|el| el.create_named_sub_element(ElementName::System, "System"))
            .and_then(|el| el.create_sub_element(ElementName::FibexElements))
            .and_then(|el| el.create_sub_element(ElementName::FibexElementRefConditional))
            .and_then(|el| el.create_sub_element(ElementName::FibexElementRef))
            .unwrap();
        el_fibex_element_ref.set_reference_target(&el_ecu_instance).unwrap();

        let project2 = AutosarProject::new();
        let file2 = project2
            .create_file("test2.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar2 = file2.root_element();
        let el_ar_packages2 = el_autosar2.create_sub_element(ElementName::ArPackages).unwrap();

        // move a named element
        el_ar_packages2.move_element_here(&el_pkg1).unwrap();
        assert!(project1.get_element_by_path("/Pkg1").is_none());
        assert!(project2.get_element_by_path("/Pkg1").is_some());
        el_ar_packages2.move_element_here_at(&el_pkg2, 1).unwrap();
        assert!(project1.get_element_by_path("/Pkg2").is_none());
        assert!(project2.get_element_by_path("/Pkg2").is_some());

        // move an unnamed element
        el_autosar.remove_sub_element(el_ar_packages1).unwrap();
        el_autosar.move_element_here(&el_ar_packages2).unwrap();
        assert!(project1.get_element_by_path("/Pkg1/EcuInstance").is_some());
        assert!(project1.get_element_by_path("/Pkg2/System").is_some());
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance);

        // can't move an element when one of the projects is deleted
        drop(project2);
        assert!(el_autosar2.move_element_here(&el_ar_packages2).is_err());
        assert!(el_autosar2.move_element_here_at(&el_ar_packages2, 0).is_err());

        // can't move between files with different versions
        let project3 = AutosarProject::new();
        let file3 = project3
            .create_file("test2.arxml", AutosarVersion::Autosar_4_3_0)
            .unwrap();
        let el_autosar3 = file3.root_element();
        assert!(el_autosar3.move_element_here(&el_ar_packages2).is_err());
        assert!(el_autosar3.move_element_here_at(&el_ar_packages2, 0).is_err());
    }

    #[test]
    fn get_set_reference_target() {
        let project = AutosarProject::new();
        let file = project
            .create_file("text.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar = file.root_element();
        let el_ar_package = el_autosar
            .create_sub_element(ElementName::ArPackages)
            .and_then(|arpkgs| arpkgs.create_named_sub_element(ElementName::ArPackage, "Package"))
            .unwrap();
        let el_elements = el_ar_package.create_sub_element(ElementName::Elements).unwrap();
        let el_ecu_instance1 = el_elements
            .create_named_sub_element(ElementName::EcuInstance, "EcuInstance1")
            .unwrap();
        let el_ecu_instance2 = el_elements
            .create_named_sub_element(ElementName::EcuInstance, "EcuInstance2")
            .unwrap();
        let el_req_result = el_elements
            .create_named_sub_element(ElementName::DiagnosticRoutine, "DiagRoutine")
            .and_then(|dr| dr.create_named_sub_element(ElementName::RequestResult, "RequestResult"))
            .unwrap();
        let el_fibex_element_ref = el_elements
            .create_named_sub_element(ElementName::System, "System")
            .and_then(|sys| sys.create_sub_element(ElementName::FibexElements))
            .and_then(|fe| fe.create_sub_element(ElementName::FibexElementRefConditional))
            .and_then(|ferc| ferc.create_sub_element(ElementName::FibexElementRef))
            .unwrap();

        // set_reference_target does not work for elements which are not references
        assert!(el_elements.set_reference_target(&el_ar_package).is_err());
        // element AUTOSAR is not identifiable, and a reference to it cannot be set
        assert!(el_fibex_element_ref.set_reference_target(&el_autosar).is_err());
        // element AR-PACKAGE is identifiable, but not a valid reference target for a FIBEX-ELEMENT-REF
        assert!(el_fibex_element_ref.set_reference_target(&el_ar_package).is_err());
        // element REQUEST-RESULT is identifiable, but cannot be referenced by any other element as here is no valid DEST enum entry for it
        assert!(el_fibex_element_ref.set_reference_target(&el_req_result).is_err());

        // set a valid reference and verify that the reference can be used
        el_fibex_element_ref.set_reference_target(&el_ecu_instance1).unwrap();
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance1);
        // update with a different valid reference and verify that the reference can be used
        el_fibex_element_ref.set_reference_target(&el_ecu_instance2).unwrap();
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance2);

        // invalid reference: bad DEST attribute
        el_fibex_element_ref
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::ISignal))
            .unwrap();
        assert!(el_fibex_element_ref.get_reference_target().is_err());
        // invalid reference: no DEST attribute
        el_fibex_element_ref.0.lock().attributes.clear(); // remove the DEST attribute
        assert!(el_fibex_element_ref.get_reference_target().is_err());
        el_fibex_element_ref.set_reference_target(&el_ecu_instance2).unwrap();
        // invalid reference: bad reference string
        el_fibex_element_ref
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::EcuInstance))
            .unwrap();
        el_fibex_element_ref
            .set_character_data(CharacterData::String("/does/not/exist".to_string()))
            .unwrap();
        assert!(el_fibex_element_ref.get_reference_target().is_err());
        // invalid reference: no reference string
        el_fibex_element_ref.remove_character_data().unwrap();
        assert!(el_fibex_element_ref.get_reference_target().is_err());
        el_fibex_element_ref.set_reference_target(&el_ecu_instance2).unwrap();
        // not a reference
        assert!(el_elements.get_reference_target().is_err());
        // project is deleted
        drop(project);
        assert!(el_fibex_element_ref.get_reference_target().is_err());
    }

    #[test]
    fn modify_character_data() {
        let project = AutosarProject::new();
        let file = project
            .create_file("text.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar = file.root_element();
        let el_ar_package = el_autosar
            .create_sub_element(ElementName::ArPackages)
            .and_then(|arpkgs| arpkgs.create_named_sub_element(ElementName::ArPackage, "Package"))
            .unwrap();
        let el_short_name = el_ar_package.get_sub_element(ElementName::ShortName).unwrap();
        let el_elements = el_ar_package.create_sub_element(ElementName::Elements).unwrap();
        let el_system = el_elements
            .create_named_sub_element(ElementName::System, "System")
            .unwrap();
        let el_fibex_element_ref = el_system
            .create_sub_element(ElementName::FibexElements)
            .and_then(|fe| fe.create_sub_element(ElementName::FibexElementRefConditional))
            .and_then(|ferc| ferc.create_sub_element(ElementName::FibexElementRef))
            .unwrap();
        let el_pnc_vector_length = el_system.create_sub_element(ElementName::PncVectorLength).unwrap();

        // set character data on an "ordinary" element that has no special handling
        assert!(el_pnc_vector_length
            .set_character_data(CharacterData::String("2".to_string()))
            .is_ok());

        // set a new SHORT-NAME, this also updates path cache
        assert!(el_short_name
            .set_character_data(CharacterData::String("PackageRenamed".to_string()))
            .is_ok());
        assert_eq!(
            el_short_name.character_data().unwrap().string_value().unwrap(),
            "PackageRenamed"
        );
        project.get_element_by_path("/PackageRenamed").unwrap();

        // set a new reference target, which creates an entry in the reference origin cache
        assert!(el_fibex_element_ref
            .set_character_data(CharacterData::String("/PackageRenamed/EcuInstance1".to_string()))
            .is_ok());
        project
            .0
            .lock()
            .reference_origins
            .get("/PackageRenamed/EcuInstance1")
            .unwrap();

        // modify the reference target, which updates the entry in the reference origin cache
        assert!(el_fibex_element_ref
            .set_character_data(CharacterData::String("/PackageRenamed/EcuInstance2".to_string()))
            .is_ok());
        project
            .0
            .lock()
            .reference_origins
            .get("/PackageRenamed/EcuInstance2")
            .unwrap();
        assert!(project
            .0
            .lock()
            .reference_origins
            .get("/PackageRenamed/EcuInstance1")
            .is_none());

        // can only set character data that are specified with ContentMode::Characters
        assert!(el_autosar
            .set_character_data(CharacterData::String("text".to_string()))
            .is_err());

        // can't set a value that doesn't match the target spec
        assert!(el_short_name
            .set_character_data(CharacterData::UnsignedInteger(0))
            .is_err());

        // remove character data
        assert!(el_pnc_vector_length.remove_character_data().is_ok());

        // remove the character data of a reference
        assert!(el_fibex_element_ref.remove_character_data().is_ok());
        assert!(project
            .0
            .lock()
            .reference_origins
            .get("/PackageRenamed/EcuInstance2")
            .is_none());

        // remove on an element whose character data has already been removed is not an error
        assert!(el_fibex_element_ref.remove_character_data().is_ok());

        // can't remove SHORT-NAME
        assert!(el_short_name.remove_character_data().is_err());

        // can't remove from elements which do not contain character data
        assert!(el_autosar.remove_character_data().is_err());

        // slightly different behavior for the internal version that is used for locked elements
        assert!(el_autosar
            .0
            .lock()
            .set_character_data(CharacterData::UnsignedInteger(0), AutosarVersion::Autosar_00050)
            .is_err());
        assert!(el_fibex_element_ref
            .0
            .lock()
            .set_character_data(CharacterData::UnsignedInteger(0), AutosarVersion::Autosar_00050)
            .is_err());

        // operation fails if the project is needed (e.g. reference or short name update), but the project has been deleted
        el_fibex_element_ref
            .set_character_data(CharacterData::String("/PackageRenamed/EcuInstance2".to_string()))
            .unwrap();
        drop(project);
        assert!(el_fibex_element_ref
            .set_character_data(CharacterData::String("/PackageRenamed/EcuInstance1".to_string()))
            .is_err());
        assert!(el_fibex_element_ref.remove_character_data().is_err());
    }

    #[test]
    fn mixed_character_content() {
        let project = AutosarProject::new();
        let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        let el_ar_package = file
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))
            .unwrap();
        let el_desc = el_ar_package.create_sub_element(ElementName::Desc).unwrap();
        let el_l2 = el_desc.create_sub_element(ElementName::L2).unwrap();

        // ok: add a character content item to a vaild element at a valid position
        el_l2.insert_character_content_item("descriptive text", 0).unwrap();

        // ok: add an element to the mixed item as well
        el_l2.create_sub_element(ElementName::Br).unwrap();

        // not ok: add a character content item to a valid element at an invalid position
        assert!(el_l2.insert_character_content_item("more text", 99).is_err());

        // not ok: add a character content item to an invalid element
        assert!(el_desc.insert_character_content_item("text", 0).is_err());

        // not ok: remove character content from an invalid position
        assert!(el_l2.remove_character_content_item(99).is_err());

        // not ok: remove character content from an invalid element
        assert!(el_desc.remove_character_content_item(0).is_err());

        // not ok: remove a sub-element
        assert!(el_l2.remove_character_content_item(1).is_err());

        // ok: remove character content from a valid element at a valid position
        el_l2.remove_character_content_item(0).unwrap();
    }

    #[test]
    fn get_sub_element() {
        let project = AutosarProject::new();
        let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = file.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Package")
            .unwrap();
        let el_desc = el_ar_package.create_sub_element(ElementName::Desc).unwrap();
        let el_l2 = el_desc.create_sub_element(ElementName::L2).unwrap();

        el_l2.insert_character_content_item("descriptive text", 0).unwrap();
        el_l2.create_sub_element(ElementName::Br).unwrap();

        assert_eq!(
            el_autosar.get_sub_element(ElementName::ArPackages).unwrap(),
            el_ar_packages
        );
        assert!(el_autosar.get_sub_element(ElementName::Abs).is_none());
        assert!(el_l2.get_sub_element(ElementName::Br).is_some());
    }

    #[test]
    fn serialize() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Pkg</SHORT-NAME>
      <DESC>
        <L-2 L="EN">Description<BR>
          </BR>Description</L-2>
      </DESC>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(FILEBUF.as_bytes(), &OsString::from("test"), true)
            .unwrap();
        let file = project.files().next().unwrap();
        let el_autosar = file.root_element();

        let mut outstring = String::from(r#"<?xml version="1.0" encoding="utf-8"?>"#);
        el_autosar.serialize_internal(&mut outstring, 0, false);

        assert_eq!(FILEBUF, outstring);
    }

    #[test]
    fn list_valid_sub_elements() {
        let project = AutosarProject::new();
        let file = project
            .create_file("test.arxml", AutosarVersion::Autosar_4_3_0)
            .unwrap();
        let el_autosar = file.root_element();
        let el_elements = el_autosar
            .create_sub_element(ElementName::ArPackages)
            .and_then(|el| el.create_named_sub_element(ElementName::ArPackage, "Package"))
            .and_then(|el| el.create_sub_element(ElementName::Elements))
            .unwrap();
        let result = el_elements.list_valid_sub_elements();
        assert!(!result.is_empty());
    }

    #[test]
    fn check_version_compatibility() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Pkg</SHORT-NAME>
      <ELEMENTS>
        <ACL-OBJECT-SET UUID="012345">
          <SHORT-NAME BLUEPRINT-VALUE="xyz">AclObjectSet</SHORT-NAME>
          <DERIVED-FROM-BLUEPRINT-REFS>
            <DERIVED-FROM-BLUEPRINT-REF DEST="ABSTRACT-IMPLEMENTATION-DATA-TYPE">/invalid</DERIVED-FROM-BLUEPRINT-REF>
          </DERIVED-FROM-BLUEPRINT-REFS>
        </ACL-OBJECT-SET>
        <ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE>
          <SHORT-NAME>AdaptiveApplicationSwComponentType</SHORT-NAME>
        </ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(FILEBUF.as_bytes(), &OsString::from("test"), true)
            .unwrap();
        let file = project.files().next().unwrap();
        let el_autosar = file.root_element();

        let (compat_errors, _) = el_autosar.check_version_compatibility(AutosarVersion::Autosar_4_3_0);
        assert_eq!(compat_errors.len(), 3);

        for ce in compat_errors {
            match ce {
                CompatibilityError::IncompatibleElement { element, .. } => {
                    assert_eq!(element.element_name(), ElementName::AdaptiveApplicationSwComponentType);
                }
                CompatibilityError::IncompatibleAttribute { element, attribute, .. } => {
                    assert_eq!(element.element_name(), ElementName::ShortName);
                    assert_eq!(attribute, AttributeName::BlueprintValue);
                }
                CompatibilityError::IncompatibleAttributeValue { element, attribute, .. } => {
                    assert_eq!(element.element_name(), ElementName::DerivedFromBlueprintRef);
                    assert_eq!(attribute, AttributeName::Dest);
                }
            }
        }
    }

    #[test]
    fn find_element_insert_pos() {
        let project = AutosarProject::new();
        let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = file.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg")
            .unwrap();
        let el_short_name = el_ar_package.get_sub_element(ElementName::ShortName).unwrap();

        // find_element_insert_pos does not operat on CharacterData elements, e.g. SHORT-NAME
        assert!(el_short_name
            .0
            .lock()
            .find_element_insert_pos(ElementName::Desc, AutosarVersion::Autosar_00050)
            .is_err());

        // find_element_insert_pos fails to find a place for a sequence element with multiplicity 0-1
        assert!(el_autosar
            .0
            .lock()
            .find_element_insert_pos(ElementName::ArPackages, AutosarVersion::Autosar_00050)
            .is_err());
    }
}
