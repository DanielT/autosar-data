use std::cmp::Ordering;
use std::hash::Hash;
use std::str::FromStr;

use super::*;

#[derive(Debug, Default, Eq)]
pub(crate) struct ElementSortKey {
    pub(crate) index: Option<u32>,
    pub(crate) item_name_indexed: Option<(String, u32)>,
    pub(crate) item_name: Option<String>,
    pub(crate) definition: Option<String>,
}

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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// if let Some(parent) = element.parent()? {
    ///     // ...
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    pub fn parent(&self) -> Result<Option<Element>, AutosarDataError> {
        self.0.lock().parent()
    }

    pub(crate) fn set_parent(&self, new_parent: ElementOrModel) {
        self.0.lock().set_parent(new_parent);
    }

    /// Get the [`ElementName`] of the element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let element = model.root_element();
    /// let element_name = element.element_name();
    /// assert_eq!(element_name, ElementName::Autosar);
    /// ```
    #[must_use]
    pub fn element_name(&self) -> ElementName {
        self.0.lock().elemname
    }

    /// Get the [`ElementType`] of the element
    ///
    /// The `ElementType` is needed in order to call methods from the autosar-data-specification crate
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// let element_type = element.element_type();
    /// ```
    #[must_use]
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// if let Some(item_name) = element.item_name() {
    ///     // ...
    /// }
    /// ```
    #[must_use]
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// element.set_item_name("NewName");
    /// ```
    ///
    /// # Note
    ///
    /// In order to rename an element *without* updating any references, do this instead:
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// if let Some(short_name) = element.get_sub_element(ElementName::ShortName) {
    ///     short_name.set_character_data(CharacterData::String("the_new_name".to_string()));
    /// }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::ItemNameRequired`] this function was called for an element which is not identifiable
    ///
    pub fn set_item_name(&self, new_name: &str) -> Result<(), AutosarDataError> {
        // a new name is required
        if new_name.is_empty() {
            return Err(AutosarDataError::ItemNameRequired);
        }
        let model = self.model()?;
        let version = self.min_version()?;
        self.0.lock().set_item_name(new_name, &model, version)
    }

    /// Returns true if the element is identifiable
    ///
    /// In order to be identifiable, the specification must require a SHORT-NAME
    /// sub-element and the SHORT-NAME must actually be present.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// if element.is_identifiable() {
    ///     // ...
    /// }
    /// ```
    #[must_use]
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// if element.is_reference() {
    ///     // ex: element.set_reference_target(...)
    /// }
    /// ```
    #[must_use]
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// let path = element.path()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: Th ecurrent element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::ElementNotIdentifiable`]: The current element is not identifiable, so it has no Autosar path
    ///
    pub fn path(&self) -> Result<String, AutosarDataError> {
        self.0.lock().path()
    }

    /// Get a reference to the [`AutosarModel`] containing the current element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    /// let file = element.model()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///
    pub fn model(&self) -> Result<AutosarModel, AutosarDataError> {
        let mut cur_elem = self.clone();
        loop {
            let parent = {
                let element = cur_elem
                    .0
                    .try_lock_for(std::time::Duration::from_millis(10))
                    .ok_or(AutosarDataError::ParentElementLocked)?;
                match &element.parent {
                    ElementOrModel::Element(weak_parent) => {
                        weak_parent.upgrade().ok_or(AutosarDataError::ItemDeleted)?
                    }
                    ElementOrModel::Model(weak_arxmlfile) => {
                        return weak_arxmlfile.upgrade().ok_or(AutosarDataError::ItemDeleted)
                    }
                    ElementOrModel::None => return Err(AutosarDataError::ItemDeleted),
                }
            };
            cur_elem = parent;
        }
    }

    /// Get the [`ContentType`] of the current element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// if element.content_type() == ContentType::CharacterData {
    ///     // ...
    /// }
    /// ```
    #[must_use]
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
    /// The given `ElementName` must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// It is not possible to create named sub elements with this function; use `create_named_sub_element`() for that instead.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let element = model.root_element().create_sub_element(ElementName::ArPackages)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::ItemNameRequired`]: The sub element requires an item name, so you must use `create_named_sub_element`().
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn create_sub_element(&self, element_name: ElementName) -> Result<Element, AutosarDataError> {
        let version = self.min_version()?;
        self.0
            .try_lock()
            .ok_or(AutosarDataError::ParentElementLocked)?
            .create_sub_element(self.downgrade(), element_name, version)
    }

    /// Create a sub element at the specified insertion position
    ///
    /// The given `ElementName` must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// It is not possible to create named sub elements with this function; use `create_named_sub_element_at`() for that instead.
    ///
    /// The specified insertion position will be compared to the range of valid insertion positions; if it falls outside that range then the function fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let element = model.root_element().create_sub_element_at(ElementName::ArPackages, 0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::ItemNameRequired`]: The sub element requires an item name, so you must use `create_named_sub_element_at`().
    ///  - [`AutosarDataError::InvalidPosition`]: This sub element cannot be created at the requested position
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn create_sub_element_at(
        &self,
        element_name: ElementName,
        position: usize,
    ) -> Result<Element, AutosarDataError> {
        let version = self.min_version()?;
        self.0
            .lock()
            .create_sub_element_at(self.downgrade(), element_name, position, version)
    }

    /// Create a named/identifiable sub element at a suitable insertion position
    ///
    /// The given `ElementName` must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    ///
    /// This method can only be used to create identifiable sub elements.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let pkgs_element = model.root_element().create_sub_element(ElementName::ArPackages)?;
    /// let element = pkgs_element.create_named_sub_element(ElementName::ArPackage, "Package")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::ElementNotIdentifiable`]: The sub element does not have an item name, so you must use `create_sub_element`() instead.
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn create_named_sub_element(
        &self,
        element_name: ElementName,
        item_name: &str,
    ) -> Result<Element, AutosarDataError> {
        let model = self.model()?;
        let version = self.min_version()?;
        self.0
            .lock()
            .create_named_sub_element(self.downgrade(), element_name, item_name, &model, version)
    }

    /// Create a named/identifiable sub element at the specified insertion position
    ///
    /// The given `ElementName` must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// The specified insertion position will be compared to the range of valid insertion positions; if it falls outside that range then the function fails.
    ///
    /// This method can only be used to create identifiable sub elements.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let pkgs_element = model.root_element().create_sub_element(ElementName::ArPackages)?;
    /// let element = pkgs_element.create_named_sub_element_at(ElementName::ArPackage, "Package", 0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::ElementNotIdentifiable`]: The sub element does not have an item name, so you must use `create_sub_element`() instead.
    ///  - [`AutosarDataError::InvalidPosition`]: This sub element cannot be created at the requested position.
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn create_named_sub_element_at(
        &self,
        element_name: ElementName,
        item_name: &str,
        position: usize,
    ) -> Result<Element, AutosarDataError> {
        let model = self.model()?;
        let version = self.min_version()?;
        self.0
            .lock()
            .create_named_sub_element_at(self.downgrade(), element_name, item_name, position, &model, version)
    }

    /// Create a deep copy of the given element and insert it as a sub-element
    ///
    /// The other element must be a permissible sub-element in this element and not conflict with any existing sub element.
    /// The other element can originate from any loaded [`AutosarModel`], it does not have to originate from the same model or file as the current element.
    ///
    /// The [`AutosarVersion`] of the other element might differ from the version of the current file;
    /// in this case a partial copy will be performed that omits all incompatible elements.
    ///
    /// If the copied element is identifiable, then the item name might be extended with a numerical suffix, if one is required in order to make the name unique.
    /// For example: An identifiable element "Foo" already exists at the same path; the copied identifiable element will be renamed to "`Foo_1`".
    ///
    /// If the copied element or the hierarchy of elements under it contain any references, then these will need to be adjusted manually after copying.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkgs_element = model.root_element().create_sub_element(ElementName::ArPackages)?;
    /// # let base = pkgs_element.create_named_sub_element(ElementName::ArPackage, "Package")
    /// #    .and_then(|p| p.create_sub_element(ElementName::Elements))?;
    /// # base.create_named_sub_element(ElementName::System, "Path")?;
    /// let other_element = model.get_element_by_path("/Package/Path").unwrap();
    /// let element = base.create_copied_sub_element(&other_element)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn create_copied_sub_element(&self, other: &Element) -> Result<Element, AutosarDataError> {
        if self == other {
            // trying to copy self into self never makes sense, and would deadlock
            return Err(AutosarDataError::InvalidSubElement);
        }
        let model = self.model()?;
        let version = self.min_version()?;
        self.0
            .lock()
            .create_copied_sub_element(self.downgrade(), other, &model, version)
    }

    /// Create a deep copy of the given element and insert it as a sub-element at the given position
    ///
    /// The other element must be a permissible sub-element in this element and not conflict with any existing sub element.
    /// The other element can originate from any loaded [`AutosarModel`], it does not have to originate from the same model or file as the current element.
    ///
    /// The [`AutosarVersion`] of the other element might differ from the version of the current file;
    /// in this case a partial copy will be performed that omits all incompatible elements.
    ///
    /// If the copied element is identifiable, then the item name might be extended with a numerical suffix, if one is required in order to make the name unique.
    /// For example: An identifiable element "Foo" already exists at the same path; the copied identifiable element will be renamed to "`Foo_1`".
    ///
    /// If the copied element or the hierarchy of elements under it contain any references, then these will need to be adjusted manually after copying.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkgs_element = model.root_element().create_sub_element(ElementName::ArPackages)?;
    /// # let base = pkgs_element.create_named_sub_element(ElementName::ArPackage, "Package")
    /// #    .and_then(|pkg| pkg.create_sub_element(ElementName::Elements))?;
    /// # base.create_named_sub_element(ElementName::System, "Path")?;
    /// let other_element = model.get_element_by_path("/Package/Path").unwrap();
    /// let element = base.create_copied_sub_element_at(&other_element, 0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::InvalidPosition`]: This sub element cannot be created at the requested position.
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn create_copied_sub_element_at(&self, other: &Element, position: usize) -> Result<Element, AutosarDataError> {
        if self == other {
            // trying to copy self into self never makes sense, and would deadlock
            return Err(AutosarDataError::InvalidSubElement);
        }
        let model = self.model()?;
        let version = self.min_version()?;
        self.0
            .lock()
            .create_copied_sub_element_at(self.downgrade(), other, position, &model, version)
    }

    /// Take an `element` from it's current location and place it in this element as a sub element
    ///
    /// The moved element can be taken from anywhere - even from a different arxml document that is not part of the same `AutosarModel`
    ///
    /// Restrictions:
    /// 1) The element must have a compatible element type. If it could not have been created here, then it can't be moved either.
    /// 2) The origin document of the element must have exactly the same `AutosarVersion` as the destination.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkgs_element = model.root_element().create_sub_element(ElementName::ArPackages)?;
    /// # let base = pkgs_element.create_named_sub_element(ElementName::ArPackage, "Package")
    /// #    .and_then(|pkg| pkg.create_sub_element(ElementName::Elements))?;
    /// # base.create_named_sub_element(ElementName::System, "Path")?;
    /// let other_element = model.get_element_by_path("/Package/Path").unwrap();
    /// let element = base.move_element_here(&other_element)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::VersionMismatch`]: The Autosar versions of the source and destination are different
    ///  - [`AutosarDataError::ForbiddenMoveToSubElement`]: The destination is a sub element of the source. Moving here is not possible
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn move_element_here(&self, move_element: &Element) -> Result<Element, AutosarDataError> {
        let model_src = move_element.model()?;
        let model = self.model()?;
        let version_src = move_element.min_version()?;
        let version = self.min_version()?;
        if version != version_src {
            return Err(AutosarDataError::VersionMismatch {
                version_cur: version,
                version_new: version_src,
            });
        }
        self.0
            .lock()
            .move_element_here(self.downgrade(), move_element, &model, &model_src, version)
    }

    /// Take an `element` from it's current location and place it at the given position in this element as a sub element
    ///
    /// The moved element can be taken from anywhere - even from a different arxml document that is not part of the same `AutosarModel`
    ///
    /// Restrictions:
    /// 1) The element must have a compatible element type. If it could not have been created here, then it can't be moved either.
    /// 2) The origin document of the element must have exactly the same `AutosarVersion` as the destination.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkgs_element = model.root_element().create_sub_element(ElementName::ArPackages)?;
    /// # let base = pkgs_element.create_named_sub_element(ElementName::ArPackage, "Package")
    /// #    .and_then(|p| p.create_sub_element(ElementName::Elements))?;
    /// # base.create_named_sub_element(ElementName::System, "Path")?;
    /// let other_element = model.get_element_by_path("/Package/Path").unwrap();
    /// let element = base.move_element_here_at(&other_element, 0)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::VersionMismatch`]: The Autosar versions of the source and destination are different
    ///  - [`AutosarDataError::ForbiddenMoveToSubElement`]: The destination is a sub element of the source. Moving here is not possible
    ///  - [`AutosarDataError::InvalidPosition`]: This sub element cannot be created at the requested position.
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn move_element_here_at(&self, move_element: &Element, position: usize) -> Result<Element, AutosarDataError> {
        let model_src = move_element.model()?;
        let model = self.model()?;
        let version_src = move_element.min_version()?;
        let version = self.min_version()?;
        if version != version_src {
            return Err(AutosarDataError::VersionMismatch {
                version_cur: version,
                version_new: version_src,
            });
        }
        self.0
            .lock()
            .move_element_here_at(self.downgrade(), move_element, position, &model, &model_src, version)
    }

    /// Remove the sub element `sub_element`
    ///
    /// The `sub_element` will be unlinked from the hierarchy of elements.
    /// All of the sub-sub-elements nested under the removed element will also be recusively removed.
    ///
    /// Since all elements are reference counted, they might not be deallocated immediately, however they do become invalid and unusable immediately.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let packages = model.root_element().create_sub_element(ElementName::ArPackages)?;
    /// model.root_element().remove_sub_element(packages)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::ElementNotFound`]: The sub element was not found in this element
    ///  - [`AutosarDataError::ShortNameRemovalForbidden`]: It is not permitted to remove the SHORT-NAME of identifiable elements since this would result in invalid data
    pub fn remove_sub_element(&self, sub_element: Element) -> Result<(), AutosarDataError> {
        let model = self.model()?;
        self.0.lock().remove_sub_element(sub_element, &model)
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let elements = model.root_element().create_sub_element(ElementName::ArPackages)
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
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::NotReferenceElement`]: The current element is not a reference, so it is not possible to set a reference target
    ///  - [`AutosarDataError::InvalidReference`]: The target element is not a valid reference target for this reference
    ///  - [`AutosarDataError::ElementNotIdentifiable`]: The target element is not identifiable, so it cannot be referenced by an Autosar path
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn set_reference_target(&self, target: &Element) -> Result<(), AutosarDataError> {
        // the current element must be a reference
        if self.is_reference() {
            // the target element must be identifiable, i.e. it has an autosar path
            let new_ref = target.path()?;
            // it must be possible to use the name of the referenced element name as an enum item in the dest attribute of the reference
            if let Some(enum_item) = EnumItem::from_str(target.element_name().to_str())
                .ok()
                .or(self.element_type().reference_dest_value(&target.element_type()))
            {
                let model = self.model()?;
                let version = self.min_version()?;
                let mut element = self.0.lock();
                // set the DEST attribute first - this could fail if the target element has the wrong type
                if element
                    .set_attribute_internal(AttributeName::Dest, CharacterData::Enum(enum_item), version)
                    .is_ok()
                {
                    // if this reference previously referenced some other element, update
                    if let Some(CharacterData::String(old_ref)) = element.character_data() {
                        model.fix_reference_origins(&old_ref, &new_ref, self.downgrade());
                    } else {
                        // else initialise the new reference
                        model.add_reference_origin(&new_ref, self.downgrade());
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let elements = model.root_element().create_sub_element(ElementName::ArPackages)
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
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::NotReferenceElement`]: The current element is not a reference, so it is not possible to get the reference target
    ///  - [`AutosarDataError::InvalidReference`]: The reference is invalid; there is no element with the referenced Autosar path
    pub fn get_reference_target(&self) -> Result<Element, AutosarDataError> {
        if self.is_reference() {
            if let Some(CharacterData::String(reference)) = self.character_data() {
                let model = self.model()?;
                let target_elem = model
                    .get_element_by_path(&reference)
                    .ok_or(AutosarDataError::InvalidReference)?;

                let dest = self
                    .attribute_value(AttributeName::Dest)
                    .map(|cdata| cdata.to_string())
                    .ok_or(AutosarDataError::InvalidReference)?;
                if dest == target_elem.element_name().to_str() {
                    // common case: the value of DEST is equal to the element name of the target
                    Ok(target_elem)
                } else if let Some(refval) = self.element_type().reference_dest_value(&target_elem.element_type()) {
                    // uncommon: find a correct DEST value in the specification
                    if refval.to_string() == dest {
                        Ok(target_elem)
                    } else {
                        Err(AutosarDataError::InvalidReference)
                    }
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
    /// This method only applies to elements which contain character data, i.e. `element.content_type` == `CharacterData` or Mixed.
    /// On elements with mixed content this function will replace all current content with the single new `CharacterData` item.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))?
    /// #   .get_sub_element(ElementName::ShortName).unwrap();
    /// element.set_character_data(CharacterData::String("value".to_string()))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: Cannot set character data on an element which does not contain character data
    pub fn set_character_data(&self, chardata: CharacterData) -> Result<(), AutosarDataError> {
        let elemtype = self.elemtype();
        if elemtype.content_mode() == ContentMode::Characters || elemtype.content_mode() == ContentMode::Mixed {
            if let Some(cdata_spec) = elemtype.chardata_spec() {
                let model = self.model()?;
                let version = self.min_version()?;
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
                        element.content.clear();
                        element.content.push(ElementContent::CharacterData(chardata));
                    }

                    // short-name: make sure the hashmap in the top-level AutosarModel is updated so that this element can still be found
                    if let Some(prev_path) = prev_path {
                        if let Some(parent) = self.parent()? {
                            let new_path = parent.path()?;
                            model.fix_identifiables(&prev_path, &new_path);
                        }
                    }

                    // reference: update the references hashmap in the top-level AutosarModel
                    if elemtype.is_ref() {
                        if let Some(CharacterData::String(refval)) = self.character_data() {
                            if let Some(old_refval) = old_refval {
                                model.fix_reference_origins(&old_refval, &refval, self.downgrade());
                            } else {
                                model.add_reference_origin(&refval, self.downgrade());
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
    /// This method only applies to elements which contain character data, i.e. `element.content_type` == `CharacterData`
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages)
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
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried. Only relevant when removing references.
    ///  - [`AutosarDataError::ShortNameRemovalForbidden`]: Removing the character content of a SHORT-NAME is forbidden
    ///  - [`AutosarDataError::IncorrectContentType`]: Cannot set character data on an element whoch does not contain character data
    pub fn remove_character_data(&self) -> Result<(), AutosarDataError> {
        let elemtype = self.elemtype();
        if elemtype.content_mode() == ContentMode::Characters {
            if self.element_name() == ElementName::ShortName {
                Err(AutosarDataError::ShortNameRemovalForbidden)
            } else {
                if self.character_data().is_some() {
                    if self.is_reference() {
                        let model = self.model()?;
                        if let Some(CharacterData::String(reference)) = self.character_data() {
                            model.remove_reference_origin(&reference, self.downgrade());
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
    /// This method only applies to elements which contain mixed data, i.e. `element.content_type`() == Mixed.
    /// Use `create_sub_element_at` to add an element instead of a character data item
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages)
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
    ///  - [`AutosarDataError::IncorrectContentType`] the element `content_type` is not Mixed
    ///  - [`AutosarDataError::InvalidPosition`] the position is not valid
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
    /// This method only applies to elements which contain mixed data, i.e. `element.content_type` == Mixed
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages)
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
    ///  - [`AutosarDataError::IncorrectContentType`] the element `content_type` is not Mixed
    ///  - [`AutosarDataError::InvalidPosition`] the position is not valid
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

    /// returns the number of content items in this element
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkg = model.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))?;
    /// assert_eq!(pkg.content_item_count(), 1);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn content_item_count(&self) -> usize {
        self.0.lock().content.len()
    }

    /// Get the character content of the element
    ///
    /// This method only applies to elements which contain character data, i.e. `element.content_type`() == `CharacterData`
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages)
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
    #[must_use]
    pub fn character_data(&self) -> Option<CharacterData> {
        self.0.lock().character_data()
    }

    /// Create an iterator over all of the content of this element
    ///
    /// The iterator can return both sub elements and character data, wrapped as `ElementContent::Element` and `ElementContent::CharacterData`
    ///
    /// This method is intended to be used with elements that contain mixed content.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// for content_item in element.content() {
    ///     match content_item {
    ///         ElementContent::CharacterData(data) => {},
    ///         ElementContent::Element(element) => {},
    ///     }
    /// }
    /// ```
    #[must_use]
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// let weak_element = element.downgrade();
    /// ```
    #[must_use]
    pub fn downgrade(&self) -> WeakElement {
        WeakElement(Arc::downgrade(&self.0))
    }

    /// return the position of this element within the parent element
    ///
    /// None may be returned if the element has been deleted, or for the root element (AUTOSAR) which has no parent.
    /// The returned position can be used with `get_sub_element_at()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let el_ar_packages = model.root_element().create_sub_element(ElementName::ArPackages).unwrap();
    /// let el_pkg1 = el_ar_packages.create_named_sub_element(ElementName::ArPackage, "Pkg1").unwrap();
    /// let el_pkg2 = el_ar_packages.create_named_sub_element(ElementName::ArPackage, "Pkg2").unwrap();
    /// let el_pkg3 = el_ar_packages.create_named_sub_element(ElementName::ArPackage, "Pkg3").unwrap();
    /// let position = el_pkg2.position().unwrap();
    /// assert_eq!(position, 1);
    /// assert_eq!(el_pkg2, el_ar_packages.get_sub_element_at(position).unwrap());
    /// ```
    #[must_use]
    pub fn position(&self) -> Option<usize> {
        if let Ok(Some(parent)) = self.parent() {
            parent
                .0
                .lock()
                .content
                .iter()
                .position(|ec| matches!(ec, ElementContent::Element(elem) if elem == self))
        } else {
            None
        }
    }

    /// Create an iterator over all sub elements of this element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// for sub_element in element.sub_elements() {
    ///     // ...
    /// }
    /// ```
    #[must_use]
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkg = model.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))?;
    /// let element = pkg.get_sub_element(ElementName::ShortName).unwrap();
    /// assert_eq!(element.element_name(), ElementName::ShortName);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
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

    /// Get the sub element at the given position.
    ///
    /// Returns None if no such element exists.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let pkg = model.root_element().create_sub_element(ElementName::ArPackages)
    /// #   .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))?;
    /// let element = pkg.get_sub_element_at(0).unwrap();
    /// assert_eq!(element.element_name(), ElementName::ShortName);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_sub_element_at(&self, position: usize) -> Option<Element> {
        let locked_elem = self.0.lock();
        if let Some(ElementContent::Element(subelem)) = locked_elem.content.get(position) {
            return Some(subelem.clone());
        }
        None
    }

    /// Get or create a sub element
    ///
    /// This is a shorthand for `get_sub_element` followed by `create_cub_element` if getting an existing element fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let element = model.root_element().get_or_create_sub_element(ElementName::ArPackages)?;
    /// let element2 = model.root_element().get_or_create_sub_element(ElementName::ArPackages)?;
    /// assert_eq!(element, element2);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::ItemNameRequired`]: The sub element requires an item name, so you must use `create_named_sub_element`().
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn get_or_create_sub_element(&self, name: ElementName) -> Result<Element, AutosarDataError> {
        let version = self.min_version()?;
        let mut locked_elem = self.0.try_lock().ok_or(AutosarDataError::ParentElementLocked)?;
        for item in &locked_elem.content {
            if let ElementContent::Element(subelem) = item {
                if subelem.element_name() == name {
                    return Ok(subelem.clone());
                }
            }
        }
        locked_elem.create_sub_element(self.downgrade(), name, version)
    }
    /// Get or create a named sub element
    ///
    /// Checks if a matching subelement exists, and returns it if it does.
    /// If no matching subelement exists, tries to create one.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let ar_packages = model.root_element().get_or_create_sub_element(ElementName::ArPackages)?;
    /// let pkg = ar_packages.get_or_create_named_sub_element(ElementName::ArPackage, "Pkg")?;
    /// let pkg_2 = ar_packages.get_or_create_named_sub_element(ElementName::ArPackage, "Pkg")?;
    /// assert_eq!(pkg, pkg_2);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::ItemNameRequired`]: The sub element requires an item name, so you must use `create_named_sub_element`().
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn get_or_create_named_sub_element(
        &self,
        element_name: ElementName,
        item_name: &str,
    ) -> Result<Element, AutosarDataError> {
        let model = self.model()?;
        let version = self.min_version()?;
        let mut locked_elem = self.0.try_lock().ok_or(AutosarDataError::ParentElementLocked)?;
        for item in &locked_elem.content {
            if let ElementContent::Element(subelem) = item {
                if subelem.element_name() == element_name && subelem.item_name().as_deref().unwrap_or("") == item_name {
                    return Ok(subelem.clone());
                }
            }
        }
        locked_elem.create_named_sub_element(self.downgrade(), element_name, item_name, &model, version)
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// for (depth, elem) in element.elements_dfs() {
    ///     // ...
    /// }
    /// ```
    #[must_use]
    pub fn elements_dfs(&self) -> ElementsDfsIterator {
        ElementsDfsIterator::new(self)
    }

    /// Create an iterator over all the attributes in this element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// for attribute in element.attributes() {
    ///     println!("{} = {}", attribute.attrname, attribute.content);
    /// }
    /// ```
    #[must_use]
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let value = model.root_element().attribute_value(AttributeName::xsiSchemalocation);
    /// ```
    #[must_use]
    pub fn attribute_value(&self, attrname: AttributeName) -> Option<CharacterData> {
        self.0.lock().attribute_value(attrname)
    }

    /// Set the value of a named attribute
    ///
    /// If no attribute by that name exists, and the attribute is a valid attribute of the element, then the attribute will be created.
    ///
    /// Returns Ok(()) if the attribute was set, otherwise the Err indicates why setting the attribute failed.
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// let result = element.set_attribute(AttributeName::S, CharacterData::String("1234-5678".to_string()));
    /// # assert!(result.is_ok());
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::InvalidAttribute`]: The `AttributeName` is not valid for this element
    ///  - [`AutosarDataError::InvalidAttributeValue`]: The value is not valid for this attribute in this element
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn set_attribute(&self, attrname: AttributeName, value: CharacterData) -> Result<(), AutosarDataError> {
        let version = self.min_version()?;
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// let result = element.set_attribute_string(AttributeName::T, "2022-01-31T13:59:59Z");
    /// # assert!(result.is_ok());
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::InvalidAttribute`]: The `AttributeName` is not valid for this element
    ///  - [`AutosarDataError::InvalidAttributeValue`]: The value is not valid for this attribute in this element
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn set_attribute_string(&self, attrname: AttributeName, stringvalue: &str) -> Result<(), AutosarDataError> {
        let version = self.min_version()?;
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
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let result = model.root_element().remove_attribute(AttributeName::xsiSchemalocation);
    /// // xsiSchemalocation exists in the AUTOSAR element, but it is mandatory and cannot be removed
    /// assert_eq!(result, false);
    /// ```
    #[must_use]
    pub fn remove_attribute(&self, attrname: AttributeName) -> bool {
        self.0.lock().remove_attribute(attrname)
    }

    /// Recursively sort all sub-elements of this element
    ///
    /// All sub elements of the current element are sorted alphabetically.
    /// If the sub-elements are named, then the sorting is performed according to the item names,
    /// otherwise the serialized form of the sub-elements is used for sorting.
    /// Element attributes are not taken into account while sorting.
    /// The elements are sorted in place, and sorting cannot fail, so there is no return value.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// element.sort();
    /// ```
    pub fn sort(&self) {
        self.0.lock().sort();
    }

    /// create a textual sorting key that can be used to determine the ordering of otherwise equal elements
    pub(crate) fn get_sort_key(&self) -> ElementSortKey {
        let index = if self.get_sub_element(ElementName::DefinitionRef).is_some() {
            self.get_sub_element(ElementName::Index)
                .and_then(|indexelem| indexelem.character_data())
                .and_then(|cdata| cdata.string_value())
                .and_then(|strval| {
                    if let Some(hexstring) = strval.strip_prefix("0x") {
                        u32::from_str_radix(hexstring, 16).ok()
                    } else if let Some(hexstring) = strval.strip_prefix("0X") {
                        u32::from_str_radix(hexstring, 16).ok()
                    } else if let Some(binstring) = strval.strip_prefix("0b") {
                        u32::from_str_radix(binstring, 2).ok()
                    } else if let Some(binstring) = strval.strip_prefix("0B") {
                        u32::from_str_radix(binstring, 2).ok()
                    } else if let Some(octval) = strval
                        .strip_prefix('0')
                        .and_then(|octstring| u32::from_str_radix(octstring, 8).ok())
                    {
                        Some(octval)
                    } else {
                        strval.parse::<u32>().ok()
                    }
                })
        } else {
            None
        };

        let item_name = self.item_name();
        let item_name_indexed = if let Some(item_name) = &item_name {
            // the set of characters for item names is restricted, and none of them takes more than one byte in utf-8
            let bytestr = item_name.as_bytes();
            let mut pos = bytestr.len();
            while pos > 0 && bytestr[pos - 1].is_ascii_digit() {
                pos -= 1;
            }
            if let Ok(index) = item_name[pos..].parse() {
                Some((item_name[0..pos].to_owned(), index))
            } else {
                None
            }
        } else {
            None
        };

        let definition = self
            .get_sub_element(ElementName::DefinitionRef)
            .and_then(|defref| defref.character_data())
            .and_then(|cdata| cdata.string_value());

        ElementSortKey {
            index,
            item_name_indexed,
            item_name,
            definition,
        }
    }

    /// Serialize the element and all of its content to a string
    ///
    /// The serialized text generated for elements below the root element cannot be loaded, but it may be useful for display.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// let text = element.serialize();
    /// ```
    #[must_use]
    pub fn serialize(&self) -> String {
        let mut outstring = String::new();

        self.serialize_internal(&mut outstring, 0, false, None);

        outstring
    }

    pub(crate) fn serialize_internal(
        &self,
        outstring: &mut String,
        indent: usize,
        inline: bool,
        for_file: Option<WeakArxmlFile>,
    ) {
        let element_name = self.element_name().to_str();

        if let Some(comment) = &self.0.lock().comment {
            // put the comment on a separate line
            if !inline {
                self.serialize_newline_indent(outstring, indent);
            }
            outstring.push_str("<!--");
            outstring.push_str(comment);
            outstring.push_str("-->");
        }

        // write the opening tag on a new line and indent it
        if !inline {
            self.serialize_newline_indent(outstring, indent);
        }

        if self.content().count() > 0 {
            outstring.push('<');
            outstring.push_str(element_name);
            self.serialize_attributes(outstring);
            outstring.push('>');

            match self.content_type() {
                ContentType::Elements => {
                    // serialize each sub-element
                    for subelem in self.sub_elements() {
                        if for_file.is_none()
                            || subelem.0.lock().file_membership.is_empty()
                            || subelem.0.lock().file_membership.contains(for_file.as_ref().unwrap())
                        {
                            subelem.serialize_internal(outstring, indent + 1, false, for_file.clone());
                        }
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
                    if let Some(ElementContent::CharacterData(chardata)) = element.content.first() {
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
                                if for_file.is_none()
                                    || subelem.0.lock().file_membership.is_empty()
                                    || subelem.0.lock().file_membership.contains(for_file.as_ref().unwrap())
                                {
                                    subelem.serialize_internal(outstring, indent + 1, true, for_file.clone());
                                }
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
        } else {
            outstring.push('<');
            outstring.push_str(element_name);
            self.serialize_attributes(outstring);
            outstring.push('/');
            outstring.push('>');
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

    // an element might have a diffeent element type depending on the version - as a result of a
    // changed datatype of the CharacterData, or because the element ordering was changed
    fn recalc_element_type(&self, target_version: AutosarVersion) -> ElementType {
        if let Ok(Some(parent)) = self.parent() {
            if let Some((etype, ..)) = parent
                .element_type()
                .find_sub_element(self.element_name(), target_version as u32)
            {
                return etype;
            }
        }

        self.element_type()
    }

    /// check if the sub elements and attributes of this element are compatible with some `target_version`
    pub(crate) fn check_version_compatibility(
        &self,
        file: &WeakArxmlFile,
        target_version: AutosarVersion,
    ) -> (Vec<CompatibilityError>, u32) {
        let mut compat_errors = Vec::new();
        let mut overall_version_mask = u32::MAX;

        // make sure compatibility checks are performed with the element type used in the target version
        let elemtype_new = self.recalc_element_type(target_version);

        // check the compatibility of all the attributes in this element
        {
            let element = self.0.lock();
            for attribute in &element.attributes {
                // find the specification for the current attribute
                if let Some(AttributeSpec {
                    spec: value_spec,
                    version: version_mask,
                    ..
                }) = elemtype_new.find_attribute_spec(attribute.attrname)
                {
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
                                attribute_value: attribute.content.to_string(),
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
            if sub_element.0.lock().file_membership.is_empty() || sub_element.0.lock().file_membership.contains(file) {
                if let Some((_, indices)) = elemtype_new
                    .find_sub_element(sub_element.element_name(), target_version as u32)
                    .or(elemtype_new.find_sub_element(sub_element.element_name(), u32::MAX))
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
                            sub_element.check_version_compatibility(file, target_version);
                        compat_errors.append(&mut sub_element_errors);
                        overall_version_mask &= sub_element_mask;
                    }
                }
            }
        }

        (compat_errors, overall_version_mask)
    }

    /// List all `sub_elements` that are valid in the current element
    ///
    /// The target use case is direct interaction with a user, e.g. through a selection dialog
    ///
    /// # Return Value
    ///
    /// A list of tuples consisting of
    ///     `ElementName` of the potential sub element
    ///     bool: is the sub element named
    ///     bool: can this sub element be inserted considering the current content of the element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// for ValidSubElementInfo{element_name, is_named, is_allowed} in element.list_valid_sub_elements() {
    ///     // ...
    /// }
    /// ```
    #[must_use]
    pub fn list_valid_sub_elements(&self) -> Vec<ValidSubElementInfo> {
        let etype = self.0.lock().elemtype;
        let mut valid_sub_elements = Vec::new();

        if let Ok(version) = self.min_version() {
            for (element_name, _, version_mask, named_mask) in etype.sub_element_spec_iter() {
                if version.compatible(version_mask) {
                    let is_named = version.compatible(named_mask);
                    let is_allowed = self.0.lock().calc_element_insert_range(element_name, version).is_ok();
                    valid_sub_elements.push(ValidSubElementInfo {
                        element_name,
                        is_named,
                        is_allowed,
                    });
                }
            }
        }

        valid_sub_elements
    }

    /// Return the set of files in which the current element is present
    ///
    /// # Return Value
    ///
    /// A tuple (bool, `HashSet`); if the bool value is true, then the file set is stored in this element, otherwise it is inherited from a parent element.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// let (inherited, file_membership) = element.file_membership()?;
    /// # Ok(())
    /// # }
    /// ```
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn file_membership(&self) -> Result<(bool, HashSet<WeakArxmlFile>), AutosarDataError> {
        let mut cur_elem_opt = Some(self.clone());
        while let Some(cur_elem) = &cur_elem_opt {
            let locked_cur_elem = cur_elem
                .0
                .try_lock_for(std::time::Duration::from_millis(10))
                .ok_or(AutosarDataError::ParentElementLocked)?;
            if !locked_cur_elem.file_membership.is_empty() {
                return Ok((cur_elem == self, locked_cur_elem.file_membership.clone()));
            }
            drop(locked_cur_elem);

            cur_elem_opt = cur_elem.parent()?;
        }

        // no file membership info found at any level - this only happens if the model does not contain any files
        Err(AutosarDataError::NoFilesInModel)
    }

    /// return the file membership of this element without trying to get an inherited value
    pub(crate) fn file_membership_local(&self) -> HashSet<WeakArxmlFile> {
        self.0.lock().file_membership.clone()
    }

    /// set the file membership of an element
    ///
    /// The passed set acts as a restriction of the file membership of the parent element.
    /// This means that the set of a child cannot be greater than that of the parent.
    ///
    /// Setting an empty set has a special meaning: it reverts the membership to default,
    /// i.e. inherited from the parent with no additional restriction
    pub(crate) fn set_file_membership(&self, file_membership: HashSet<WeakArxmlFile>) {
        // find out if the parent is splittable. If the parent is unavaliable, assume
        // that the caller knows what they're doing and assume it is splittable
        let parent_splittable = self
            .parent()
            .ok()
            .flatten()
            .map_or(u32::MAX, |p| p.element_type().splittable());
        // can always reset the membership to empty = inherited; otherwise the parent must be splittable
        if file_membership.is_empty() || parent_splittable != 0 {
            self.0.lock().file_membership = file_membership;
        }
    }

    /// add the current element to the given file
    ///
    /// In order to successfully cause the element to appear in the serialized file data, all parent elements
    /// of the current element will also be added if required.
    ///
    /// If the model only has a single file then this function does nothing.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # use std::collections::HashSet;
    /// # let model = AutosarModel::new();
    /// let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// element.add_to_file(&file);
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///
    pub fn add_to_file(&self, file: &ArxmlFile) -> Result<(), AutosarDataError> {
        let parent_splittable = self.parent()?.map_or(true, |p| p.element_type().splittable() != 0);
        if parent_splittable {
            if file.model()? == self.model()? {
                let weak_file = file.downgrade();
                // current_fileset is the set of files which contain the current element
                let (_, current_fileset) = self.file_membership()?;
                // if the model only has a single file or if the element is already in the set then there is nothing to do
                if !current_fileset.contains(&weak_file) {
                    let mut updated_fileset = current_fileset;
                    updated_fileset.insert(weak_file);
                    self.0.lock().file_membership = updated_fileset;

                    // recursively continue with the parent
                    if let Some(parent) = self.parent()? {
                        parent.add_to_file_restricted(file)?;
                    }
                }
                Ok(())
            } else {
                // adding a file from a different model is not permitted
                Err(AutosarDataError::InvalidFile)
            }
        } else {
            Err(AutosarDataError::FilesetModificationForbidden)
        }
    }

    /// add only this element and its direct parents to a file, but not its children
    pub(crate) fn add_to_file_restricted(&self, file: &ArxmlFile) -> Result<(), AutosarDataError> {
        let weak_file = file.downgrade();
        let (local, current_fileset) = self.file_membership().unwrap_or((true, HashSet::new()));

        if !current_fileset.contains(&weak_file) {
            // if the current element is splittable, then all of its subelements are allowed to have their own filesets
            // unless something else is already set, they should get the current unmodified file membership of this element
            // which does not include the new file
            if self.element_type().splittable() != 0 {
                for se in self.sub_elements() {
                    if let Some(mut subelem) = se.0.try_lock() {
                        if subelem.file_membership.is_empty() {
                            subelem.file_membership = current_fileset.clone();
                        }
                    }
                }
            }

            let mut extended_fileset = current_fileset;
            extended_fileset.insert(weak_file);
            // if the parent is splittable, or if the current element already has a fileset, then that fileset should be updated
            let parent_splittable = self.parent()?.map_or(true, |p| p.element_type().splittable() != 0);
            if parent_splittable || local {
                self.0.lock().file_membership = extended_fileset;
            }

            // recursively continue with the parent
            if let Some(parent) = self.parent()? {
                parent.add_to_file_restricted(file)?;
            }
        }

        Ok(())
    }

    /// remove this element from a file
    ///
    /// If the model consists of multiple files, then the set of files in
    /// which this element appears will be restricted.
    /// It may be required to also omit its parent(s), up to the next splittable point.
    ///
    /// If the element is only present in single file then an attempt to delete it will be made instead.
    /// Deleting the element fails if the element is the root AUTOSAR element, or if it is a SHORT-NAME.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # use std::collections::HashSet;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let file2 = model.create_file("test2", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// assert!(model.files().count() > 1);
    /// element.remove_from_file(&file);
    /// ```
    ///
    /// # Possible Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///
    pub fn remove_from_file(&self, file: &ArxmlFile) -> Result<(), AutosarDataError> {
        let parent_splittable = self.parent()?.map_or(true, |p| p.element_type().splittable() != 0);
        if parent_splittable {
            if file.model()? == self.model()? {
                let weak_file = file.downgrade();

                // current_fileset is the set of files which contain the current element
                let (_, current_fileset) = self.file_membership()?;
                let mut restricted_fileset = current_fileset;
                restricted_fileset.remove(&weak_file);
                if restricted_fileset.is_empty() {
                    // the element will no longer be part of any file, so try to delete it
                    if let Some(parent) = self.parent()? {
                        let _ = parent.remove_sub_element(self.to_owned());
                    }
                }
                // this works even if the element was just removed
                self.0.lock().file_membership = restricted_fileset;

                // update all sub elements with non-default file_membership
                let mut to_delete = Vec::new();
                for (_, subelem) in self.elements_dfs() {
                    // only need to care about those where file_membership is not empty. All other inherit from their parent
                    if !subelem.0.lock().file_membership.is_empty() {
                        subelem.0.lock().file_membership.remove(&weak_file);
                        // if the file_membership just went to empty, then subelem should be deleted
                        if subelem.0.lock().file_membership.is_empty() {
                            to_delete.push(subelem);
                        }
                    }
                }
                // delete elements that are no longer in any file
                for delete_elem in to_delete {
                    if let Ok(Some(parent)) = delete_elem.parent() {
                        let _ = parent.remove_sub_element(delete_elem);
                    }
                }

                Ok(())
            } else {
                // adding a file from a different model is not permitted
                Err(AutosarDataError::InvalidFile)
            }
        } else {
            Err(AutosarDataError::FilesetModificationForbidden)
        }
    }

    /// Return a path that includes non-identifiable elements by their xml names
    ///
    /// This function cannot fail completely, it will always collect as much information as possible.
    /// It is intended for display in error messages.
    #[must_use]
    pub fn xml_path(&self) -> String {
        self.0.lock().xml_path()
    }

    /// Find the upper and lower bound on the insert position for a new sub element
    ///
    /// If the sub element is allowed for this element given its current content, this function
    /// returns the lower and upper bound on the position the new sub element could have.
    /// If the sub element is not allowed, then an Err is returned instead.
    ///
    /// The lower and upper bounds are inclusive: lower <= (element insert pos) <= upper.
    /// In many situations lower == upper, this means there is only a single valid position.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # use std::collections::HashSet;
    /// # let model = AutosarModel::new();
    /// # model.create_file("test", AutosarVersion::LATEST).unwrap();
    /// let (lbound, ubound) = model.root_element()
    ///     .calc_element_insert_range(ElementName::ArPackages, AutosarVersion::LATEST)?;
    /// model.root_element().create_sub_element_at(ElementName::ArPackages, lbound)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Possible Errors
    ///
    /// - [`AutosarDataError::ElementInsertionConflict`]: The sub element conflicts with an existing sub element
    /// - [`AutosarDataError::InvalidSubElement`]: The sub element is not valid inside this element
    pub fn calc_element_insert_range(
        &self,
        element_name: ElementName,
        version: AutosarVersion,
    ) -> Result<(usize, usize), AutosarDataError> {
        self.0.lock().calc_element_insert_range(element_name, version)
    }

    /// Return the comment attachd to the element (if any)
    ///
    /// A comment directly preceding the opening tag is considered to be atached and is returned here.
    ///
    /// In the arxml text:
    /// ```xml
    ///     <!--element comment-->
    ///     <ELEMENT> ...
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use std::collections::HashSet;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::LATEST).unwrap();
    /// # let element = model.root_element();
    /// let opt_comment = element.comment();
    /// ```
    pub fn comment(&self) -> Option<String> {
        self.0.lock().comment.clone()
    }

    /// Set or delete the comment attached to the element
    ///
    /// Set None to remove the comment.
    pub fn set_comment(&self, mut opt_comment: Option<String>) {
        if let Some(comment) = &mut opt_comment {
            // make sure the comment we store never contains "--" as this is forbidden by the w3 xml specification
            if comment.contains("--") {
                *comment = comment.replace("--", "__");
            }
        }
        self.0.lock().comment = opt_comment;
    }

    /// find the minumum version of all arxml files which contain this element
    ///
    /// typically this reduces to finding out which single file contains the element and returning this version.
    fn min_version(&self) -> Result<AutosarVersion, AutosarDataError> {
        let (_, files) = self.file_membership()?;
        if files.is_empty() {
            return Err(AutosarDataError::NoFilesInModel);
        }
        let mut ver = AutosarVersion::LATEST;
        for f in files.iter().filter_map(WeakArxmlFile::upgrade) {
            if f.version() < ver {
                ver = f.version();
            }
        }
        Ok(ver)
    }
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbgstruct = f.debug_struct("Element");
        dbgstruct.field("elemname", &self.0.lock().elemname);
        dbgstruct.field("elemtype", &self.0.lock().elemtype);
        dbgstruct.field("parent", &self.0.lock().parent);
        // avoid holding the lock on self.0 while recursing to print all the sub elements
        let content = self.0.lock().content.clone();
        dbgstruct.field("content", &content);
        dbgstruct.field("attributes", &self.0.lock().attributes);
        dbgstruct.field("file_membership", &self.0.lock().file_membership);
        dbgstruct.finish()
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

impl Eq for Element {}

impl Hash for Element {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Arc::as_ptr(&self.0) as usize);
    }
}

impl std::fmt::Debug for WeakElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Element:WeakRef {:p}", Weak::as_ptr(&self.0)))
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

impl Eq for WeakElement {}

impl Hash for WeakElement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Weak::as_ptr(&self.0) as usize);
    }
}

impl ElementContent {
    /// returns the element contained inside this `ElementContent`, or None if the content is `CharacterData`
    #[must_use]
    pub fn unwrap_element(&self) -> Option<Element> {
        if let ElementContent::Element(element) = self {
            Some(element.clone())
        } else {
            None
        }
    }

    /// returns the `CharacterData` inside this `ElementContent`, or None if the content is an Element
    #[must_use]
    pub fn unwrap_cdata(&self) -> Option<CharacterData> {
        if let ElementContent::CharacterData(cdata) = self {
            Some(cdata.clone())
        } else {
            None
        }
    }
}

// custom debug implementation: skip printing the name of the wrapper-enum and directly show the content
impl std::fmt::Debug for ElementOrModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementOrModel::Element(elem) => elem.fmt(f),
            ElementOrModel::Model(model) => model.fmt(f),
            ElementOrModel::None => f.write_str("None"),
        }
    }
}

// custom debug implementation: skip printing the name of the wrapper-enum and directly show the content
impl std::fmt::Debug for ElementContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementContent::Element(elem) => elem.fmt(f),
            ElementContent::CharacterData(cdata) => cdata.fmt(f),
        }
    }
}

impl PartialEq for ElementSortKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for ElementSortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // if any item has an <INDEX> element, then all its siblings should too
        // just in case they don't then the elements with an INDEX will be placed first
        match (self.index, other.index) {
            (Some(idx1), Some(idx2)) => match idx1.cmp(&idx2) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
                Ordering::Greater => return Ordering::Greater,
            },
            (None, Some(_)) => return Ordering::Greater,
            (Some(_), None) => return Ordering::Less,
            (None, None) => {}
        }
        // if the item name of an element appears to consist of a prefix followed by a numerical index, then the parts will be available in item_name_indexed
        if let (Some((self_base, self_idx)), Some((other_base, other_idx))) =
            (&self.item_name_indexed, &other.item_name_indexed)
        {
            if self_base == other_base {
                match self_idx.cmp(other_idx) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => {}
                    Ordering::Greater => return Ordering::Greater,
                }
            }
        }
        // compare by definition ref (if any)
        if let (Some(self_def), Some(other_def)) = (&self.definition, &other.definition) {
            match self_def.cmp(other_def) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
                Ordering::Greater => return Ordering::Greater,
            }
        }
        // compare by item name (if any)
        if let (Some(self_item_name), Some(other_item_name)) = (&self.item_name, &other.item_name) {
            // item names are unique among siblings, so this comparison cannot return Equal
            return self_item_name.cmp(other_item_name);
        }
        // sort keys are equal
        Ordering::Equal
    }
}

impl PartialOrd for ElementSortKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::{cmp::Ordering, ffi::OsString};

    use super::ElementSortKey;

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
        let model = AutosarModel::new();
        model
            .load_buffer(BASIC_AUTOSAR_FILE.as_bytes(), OsString::from("test.arxml"), true)
            .unwrap();
        let el_autosar = model.root_element();
        let el_ar_package = model.get_element_by_path("/TestPackage").unwrap();

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
        assert_eq!(count, el_elements.content_item_count());

        // inserting another COMPU-METHOD into ELEMENTS hould be allowed at any position
        let (start_pos, end_pos) = el_elements
            .0
            .lock()
            .calc_element_insert_range(ElementName::CompuMethod, AutosarVersion::Autosar_00050)
            .unwrap();
        assert_eq!(start_pos, 0);
        assert_eq!(end_pos, 3); // upper limit is 3 since there are currently 3 elements

        // check if create_named_sub_element correctly registered the element in the hashmap so that it can be found
        let el_compu_method_test = model.get_element_by_path("/TestPackage/TestCompuMethod").unwrap();
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
            .calc_element_insert_range(ElementName::ShortLabel, AutosarVersion::Autosar_00050)
            .unwrap();
        assert_eq!(start_pos, 0);
        assert_eq!(end_pos, 0);

        // COMPU-CONST should only be allowed after DESC inside COMPU-SCALE
        let (start_pos, end_pos) = el_compu_scale
            .calc_element_insert_range(ElementName::CompuConst, AutosarVersion::Autosar_00050)
            .unwrap();
        assert_eq!(start_pos, 1);
        assert_eq!(end_pos, 1);

        // create COMPU-RATIONAL-COEFFS in COMPU-SCALE. It's presence excludes COMPU-CONST from being inserted
        el_compu_scale
            .create_sub_element(ElementName::CompuRationalCoeffs)
            .unwrap();
        // try to insert COMPU-CONST anyway
        let result = el_compu_scale.calc_element_insert_range(ElementName::CompuConst, AutosarVersion::Autosar_00050);
        assert!(result.is_err());
        // it is also not possible to create a second COMPU-RATIONAL-COEFFS
        let result =
            el_compu_scale.calc_element_insert_range(ElementName::CompuRationalCoeffs, AutosarVersion::Autosar_00050);
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
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
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
        drop(model);
        // the reference count of model is now zero, so set_item_name can't get a new reference to it
        assert!(el_ar_package.set_item_name("TestPackage_renamed").is_err());
    }

    #[test]
    fn element_copy() {
        let model = AutosarModel::new();
        model
            .load_buffer(BASIC_AUTOSAR_FILE.as_bytes(), OsString::from("test.arxml"), true)
            .unwrap();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_ar_package = model.get_element_by_path("/TestPackage").unwrap();
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

        let project2 = AutosarModel::new();
        project2
            .create_file("test.arxml", AutosarVersion::Autosar_00044)
            .unwrap();

        // it should not be possible to create an AR-PACKAGE element directly in the AUTOSAR element by copying data
        let result = project2.root_element().create_copied_sub_element(&el_ar_package);
        assert!(result.is_err());

        // create an AR-PACKAGES element and copy the data there. This should succeed.
        // the copied data shoud contain the COMPU-METHOD, but not the DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
        // because the latter was specified in Adaptive 18-03 (Autosar_00045) and is not valid in Autosar_00044
        let el_ar_packages2 = project2
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
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
    fn element_copy_loop() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg")
            .unwrap();

        let result = el_ar_package.create_copied_sub_element(&el_ar_packages);
        assert!(result.is_err());

        // copying an element into itself should return an error and should not deadlock
        let result = el_ar_package.create_copied_sub_element(&el_ar_package);
        assert!(result.is_err());
        let result = el_ar_package.create_copied_sub_element_at(&el_ar_package, 0);
        assert!(result.is_err());
    }

    #[test]
    fn element_deletion() {
        let model = AutosarModel::new();
        model
            .load_buffer(BASIC_AUTOSAR_FILE.as_bytes(), OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = model.get_element_by_path("/TestPackage").unwrap();
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
        let el_ar_package_clone = el_ar_package.clone();
        let el_ar_packages = el_ar_package.parent().unwrap().unwrap();
        let result = el_ar_packages.remove_sub_element(el_ar_package);
        // deleting identifiable elements should also cause the cached references to them to be removed
        assert_eq!(model.0.lock().identifiables.len(), 0);
        assert!(result.is_ok());

        // the removed element may still exist if there were other references to it, but it is no longer usable
        let result = el_ar_package_clone.parent();
        assert!(matches!(result, Err(AutosarDataError::ItemDeleted)));
        let result = el_ar_package_clone.model();
        assert!(matches!(result, Err(AutosarDataError::ItemDeleted)));
        assert_eq!(el_ar_package_clone.position(), None);
    }

    #[test]
    fn element_position() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package1 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg1")
            .unwrap();
        let el_ar_package2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg2")
            .unwrap();
        let el_ar_package3 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg3")
            .unwrap();

        assert_eq!(el_ar_packages.content_item_count(), 3);
        assert_eq!(el_ar_package2.position().unwrap(), 1);
        assert_eq!(el_ar_packages.get_sub_element_at(1).unwrap(), el_ar_package2);
        assert_eq!(el_ar_package3.position().unwrap(), 2);
        assert_eq!(el_ar_packages.get_sub_element_at(2).unwrap(), el_ar_package3);

        // there is no subelement at position 1
        let nonexistent = el_ar_package1.get_sub_element_at(1);
        assert_eq!(nonexistent, None);
    }

    #[test]
    fn element_type() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();

        assert_eq!(el_autosar.element_type(), ElementType::ROOT);
    }

    #[test]
    fn content_type() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
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
        let model = AutosarModel::new();
        model
            .load_buffer(BASIC_AUTOSAR_FILE.as_bytes(), OsString::from("test.arxml"), true)
            .unwrap();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_autosar = model.root_element();
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
        assert!(result);

        // the attribute xmlns is required and cannot be removed
        let result = el_autosar.remove_attribute(AttributeName::xmlns);
        assert!(!result);

        // the attribute ACCESSKEY does not exist in the element AUTOSAR and cannot be removed
        let result = el_autosar.remove_attribute(AttributeName::Accesskey);
        assert!(!result);

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
        let xmlns = el_autosar
            .attribute_value(AttributeName::xmlns)
            .map(|cdata| cdata.to_string())
            .unwrap();
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
        let model = AutosarModel::new();
        model
            .load_buffer(BASIC_AUTOSAR_FILE.as_bytes(), OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = model.get_element_by_path("/TestPackage").unwrap();
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
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
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
        // move an element within the same model
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
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
        assert!(model.get_element_by_path("/Pkg1/EcuInstance").is_some());
        el_pkg2.move_element_here(&el_elements1).unwrap();
        assert_eq!(el_elements1.parent().unwrap().unwrap(), el_pkg2);
        assert!(model.get_element_by_path("/Pkg2/EcuInstance").is_some());
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance);

        // move the unnamed element back using the _at variant
        el_pkg1.move_element_here_at(&el_elements1, 1).unwrap();
        assert_eq!(el_elements1.parent().unwrap().unwrap(), el_pkg1);
        assert!(model.get_element_by_path("/Pkg1/EcuInstance").is_some());
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance);

        // move a named element
        let el_elements2 = el_pkg2.create_sub_element(ElementName::Elements).unwrap();
        el_elements2.move_element_here(&el_ecu_instance).unwrap();
        assert_eq!(el_ecu_instance.parent().unwrap().unwrap(), el_elements2);
        assert!(model.get_element_by_path("/Pkg2/EcuInstance").is_some());
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance);

        // moving an element should automatically resolve name conflicts
        el_elements1
            .create_named_sub_element(ElementName::EcuInstance, "EcuInstance")
            .unwrap();
        el_elements1.move_element_here_at(&el_ecu_instance, 0).unwrap();
        assert_eq!(el_ecu_instance.parent().unwrap().unwrap(), el_elements1);
        assert!(model.get_element_by_path("/Pkg1/EcuInstance_1").is_some());
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance);
    }

    #[test]
    fn move_element_full() {
        // move an element between two projects
        let model1 = AutosarModel::new();
        model1
            .create_file("test1.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar = model1.root_element();
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

        let model2 = AutosarModel::new();
        model2
            .create_file("test2.arxml", AutosarVersion::Autosar_00050)
            .unwrap();
        let el_autosar2 = model2.root_element();
        let el_ar_packages2 = el_autosar2.create_sub_element(ElementName::ArPackages).unwrap();

        // move a named element
        el_ar_packages2.move_element_here(&el_pkg1).unwrap();
        assert!(model1.get_element_by_path("/Pkg1").is_none());
        assert!(model2.get_element_by_path("/Pkg1").is_some());
        el_ar_packages2.move_element_here_at(&el_pkg2, 1).unwrap();
        assert!(model1.get_element_by_path("/Pkg2").is_none());
        assert!(model2.get_element_by_path("/Pkg2").is_some());

        // move an unnamed element
        el_autosar.remove_sub_element(el_ar_packages1).unwrap();
        el_autosar.move_element_here(&el_ar_packages2).unwrap();
        assert!(model1.get_element_by_path("/Pkg1/EcuInstance").is_some());
        assert!(model1.get_element_by_path("/Pkg2/System").is_some());
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance);

        // can't move an element when one of the projects is deleted
        drop(model2);
        assert!(el_autosar2.move_element_here(&el_ar_packages2).is_err());
        assert!(el_autosar2.move_element_here_at(&el_ar_packages2, 0).is_err());

        // can't move between files with different versions
        let project3 = AutosarModel::new();
        project3
            .create_file("test2.arxml", AutosarVersion::Autosar_4_3_0)
            .unwrap();
        let el_autosar3 = project3.root_element();
        assert!(el_autosar3.move_element_here(&el_ar_packages2).is_err());
        assert!(el_autosar3.move_element_here_at(&el_ar_packages2, 0).is_err());
    }

    #[test]
    fn get_set_reference_target() {
        let model = AutosarModel::new();
        model.create_file("text.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
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
        let el_physical_request_ref = el_elements
            .create_named_sub_element(ElementName::DiagnosticConnection, "DiagnosticConnection")
            .and_then(|dc| dc.create_sub_element(ElementName::PhysicalRequestRef))
            .unwrap();
        let el_connection_ident = el_elements
            .create_named_sub_element(ElementName::CanTpConfig, "CanTpConfig")
            .and_then(|ctc| ctc.create_sub_element(ElementName::TpConnections))
            .and_then(|tc: Element| tc.create_sub_element(ElementName::CanTpConnection))
            .and_then(|ctc: Element| ctc.create_named_sub_element(ElementName::Ident, "ConnectionIdent"))
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

        // set a valid reference to <CAN-TP-CONNECTION><IDENT>.
        // This is a complex case, as the correct DEST attribute must be looked up in the specification
        el_physical_request_ref
            .set_reference_target(&el_connection_ident)
            .unwrap();
        assert_eq!(
            el_physical_request_ref.get_reference_target().unwrap(),
            el_connection_ident
        );

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
        // invalid reference: refers to the wrong type of element
        el_fibex_element_ref
            .set_character_data(CharacterData::String("/Package".to_string()))
            .unwrap();
        assert!(el_fibex_element_ref.get_reference_target().is_err());
        // invalid reference: no reference string
        el_fibex_element_ref.remove_character_data().unwrap();
        assert!(el_fibex_element_ref.get_reference_target().is_err());
        el_fibex_element_ref.set_reference_target(&el_ecu_instance2).unwrap();
        // not a reference
        assert!(el_elements.get_reference_target().is_err());
        // model is deleted
        drop(model);
        assert!(el_fibex_element_ref.get_reference_target().is_err());
    }

    #[test]
    fn modify_character_data() {
        let model = AutosarModel::new();
        model.create_file("text.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
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
        model.get_element_by_path("/PackageRenamed").unwrap();

        // set a new reference target, which creates an entry in the reference origin cache
        assert!(el_fibex_element_ref
            .set_character_data(CharacterData::String("/PackageRenamed/EcuInstance1".to_string()))
            .is_ok());
        model
            .0
            .lock()
            .reference_origins
            .get("/PackageRenamed/EcuInstance1")
            .unwrap();

        // modify the reference target, which updates the entry in the reference origin cache
        assert!(el_fibex_element_ref
            .set_character_data(CharacterData::String("/PackageRenamed/EcuInstance2".to_string()))
            .is_ok());
        model
            .0
            .lock()
            .reference_origins
            .get("/PackageRenamed/EcuInstance2")
            .unwrap();
        assert!(model
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
        assert!(model
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

        // operation fails if the model is needed (e.g. reference or short name update), but the model has been deleted
        el_fibex_element_ref
            .set_character_data(CharacterData::String("/PackageRenamed/EcuInstance2".to_string()))
            .unwrap();
        drop(model);
        assert!(el_fibex_element_ref
            .set_character_data(CharacterData::String("/PackageRenamed/EcuInstance1".to_string()))
            .is_err());
        assert!(el_fibex_element_ref.remove_character_data().is_err());
    }

    #[test]
    fn mixed_character_content() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        let el_ar_package = model
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
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
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
    fn get_or_create() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();

        assert_eq!(el_autosar.sub_elements().count(), 0);
        let el_admin_data = el_autosar.get_or_create_sub_element(ElementName::AdminData).unwrap();
        let el_ar_packages = el_autosar.get_or_create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_packages2 = el_autosar.get_or_create_sub_element(ElementName::ArPackages).unwrap();
        assert_ne!(el_admin_data, el_ar_packages);
        assert_eq!(el_ar_packages, el_ar_packages2);

        let el_ar_package = el_ar_packages
            .get_or_create_named_sub_element(ElementName::ArPackage, "Pkg")
            .unwrap();
        let el_ar_package2 = el_ar_packages
            .get_or_create_named_sub_element(ElementName::ArPackage, "Pkg2")
            .unwrap();
        let el_ar_package3 = el_ar_packages
            .get_or_create_named_sub_element(ElementName::ArPackage, "Pkg2")
            .unwrap();
        assert_ne!(el_ar_package, el_ar_package2);
        assert_eq!(el_ar_package2, el_ar_package3);
    }

    #[test]
    fn serialize() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Pkg</SHORT-NAME>
      <DESC>
        <L-2 L="EN">Description<BR/>Description</L-2>
      </DESC>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;
        let model = AutosarModel::new();
        model
            .load_buffer(FILEBUF.as_bytes(), OsString::from("test"), true)
            .unwrap();
        model.files().next().unwrap();
        let el_autosar = model.root_element();

        let mut outstring = String::from(r#"<?xml version="1.0" encoding="utf-8"?>"#);
        el_autosar.serialize_internal(&mut outstring, 0, false, None);

        assert_eq!(FILEBUF, outstring);
    }

    #[test]
    fn list_valid_sub_elements() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_4_3_0).unwrap();
        let el_autosar = model.root_element();
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
        let model = AutosarModel::new();
        let (file, _) = model
            .load_buffer(FILEBUF.as_bytes(), &OsString::from("test"), true)
            .unwrap();
        model.files().next().unwrap();
        let el_autosar = model.root_element();

        let (compat_errors, _) =
            el_autosar.check_version_compatibility(&file.downgrade(), AutosarVersion::Autosar_4_3_0);
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

        // regression test - CompuScales in CompuInternalToPhys was falsely detected as incompatible
        let model = AutosarModel::new();
        let file = model.create_file("filename", AutosarVersion::Autosar_00046).unwrap();
        model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))
            .and_then(|e| e.create_sub_element(ElementName::Elements))
            .and_then(|e| e.create_named_sub_element(ElementName::CompuMethod, "CompuMethod"))
            .and_then(|e| e.create_sub_element(ElementName::CompuInternalToPhys))
            .and_then(|e| e.create_sub_element(ElementName::CompuScales))
            .and_then(|e| e.create_sub_element(ElementName::CompuScale))
            .unwrap();
        let (compat_errors, _) = file.check_version_compatibility(AutosarVersion::Autosar_4_3_0);
        assert!(compat_errors.is_empty());
    }

    #[test]
    fn find_element_insert_pos() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg")
            .unwrap();
        let el_short_name = el_ar_package.get_sub_element(ElementName::ShortName).unwrap();

        // find_element_insert_pos does not operat on CharacterData elements, e.g. SHORT-NAME
        assert!(el_short_name
            .0
            .lock()
            .calc_element_insert_range(ElementName::Desc, AutosarVersion::Autosar_00050)
            .is_err());

        // find_element_insert_pos fails to find a place for a sequence element with multiplicity 0-1
        assert!(el_autosar
            .0
            .lock()
            .calc_element_insert_range(ElementName::ArPackages, AutosarVersion::Autosar_00050)
            .is_err());
    }

    #[test]
    fn sort() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package1 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Z")
            .unwrap();
        let el_ar_package2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "A")
            .unwrap();
        let el_elements = el_ar_package1.create_sub_element(ElementName::Elements).unwrap();
        el_ar_package1.create_sub_element(ElementName::AdminData).unwrap();
        // create some bsw values to sort inside el_ar_package1 "Z"
        let el_emcv = el_elements
            .create_named_sub_element(ElementName::EcucModuleConfigurationValues, "Config")
            .unwrap();
        let el_containers = el_emcv.create_sub_element(ElementName::Containers).unwrap();
        let el_ecv = el_containers
            .create_named_sub_element(ElementName::EcucContainerValue, "ConfigValues")
            .unwrap();
        let el_paramvalues = el_ecv.create_sub_element(ElementName::ParameterValues).unwrap();
        // first bsw value
        let el_value1 = el_paramvalues
            .create_sub_element(ElementName::EcucNumericalParamValue)
            .unwrap();
        let el_defref1 = el_value1.create_sub_element(ElementName::DefinitionRef).unwrap();
        el_defref1
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::EcucBooleanParamDef))
            .unwrap();
        el_defref1
            .set_character_data(CharacterData::String("/DefRef_999".to_string()))
            .unwrap();
        // second bsw value
        let el_value2 = el_paramvalues
            .create_sub_element(ElementName::EcucNumericalParamValue)
            .unwrap();
        let el_defref2 = el_value2.create_sub_element(ElementName::DefinitionRef).unwrap();
        el_defref2
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::EcucBooleanParamDef))
            .unwrap();
        el_defref2
            .set_character_data(CharacterData::String("/DefRef_111".to_string()))
            .unwrap();
        // Create some misc value sto sort inside el_ar_package2 "A"
        let el_elements2 = el_ar_package2.create_sub_element(ElementName::Elements).unwrap();
        let el_system = el_elements2
            .create_named_sub_element(ElementName::System, "System")
            .unwrap();
        let el_fibex_elements = el_system.create_sub_element(ElementName::FibexElements).unwrap();
        let el_fibex_element1 = el_fibex_elements
            .create_sub_element(ElementName::FibexElementRefConditional)
            .unwrap();
        let el_fibex_element_ref1 = el_fibex_element1
            .create_sub_element(ElementName::FibexElementRef)
            .unwrap();
        el_fibex_element_ref1
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::ISignal))
            .unwrap();
        el_fibex_element_ref1
            .set_character_data(CharacterData::String("/ZZZZZ".to_string()))
            .unwrap();
        let el_fibex_element2 = el_fibex_elements
            .create_sub_element(ElementName::FibexElementRefConditional)
            .unwrap();
        let el_fibex_element_ref2 = el_fibex_element2
            .create_sub_element(ElementName::FibexElementRef)
            .unwrap();
        el_fibex_element_ref2
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::ISignal))
            .unwrap();
        el_fibex_element_ref2
            .set_character_data(CharacterData::String("/AAAAA".to_string()))
            .unwrap();

        model.sort();
        // validate that identifiable elements have been sorted
        let mut iter = el_ar_packages.sub_elements();
        let item1 = iter.next().unwrap();
        let item2 = iter.next().unwrap();
        assert_eq!(item1.item_name().unwrap(), "A");
        assert_eq!(item2.item_name().unwrap(), "Z");

        // validate that BSW parameter values have been sorted
        let mut iter = el_paramvalues.sub_elements();
        let item1 = iter.next().unwrap();
        let item2 = iter.next().unwrap();
        assert_eq!(item1, el_value2);
        assert_eq!(item2, el_value1);

        // validate that the misc elements (FIBEX-ELEMENT-REF-CONDITIONAL) without distinguishing features have not been sorted
        let mut iter = el_fibex_elements.sub_elements();
        let item1 = iter.next().unwrap();
        let item2 = iter.next().unwrap();
        assert_eq!(item1, el_fibex_element1);
        assert_eq!(item2, el_fibex_element2);
    }

    fn helper_create_bsw_subelem(
        el_subcontainers: &Element,
        short_name: &str,
        defref: &str,
    ) -> Result<Element, AutosarDataError> {
        let e = el_subcontainers.create_named_sub_element(ElementName::EcucContainerValue, short_name)?;
        let defrefelem = e.create_sub_element(ElementName::DefinitionRef)?;
        defrefelem.set_character_data(CharacterData::String(defref.to_string()))?;
        Ok(e)
    }

    fn helper_create_indexed_bsw_subelem(
        el_subcontainers: &Element,
        short_name: &str,
        indexstr: &str,
        defref: &str,
    ) -> Result<Element, AutosarDataError> {
        let e = helper_create_bsw_subelem(el_subcontainers, short_name, defref)?;
        let indexelem = e.create_sub_element(ElementName::Index)?;
        indexelem.set_character_data(CharacterData::String(indexstr.to_string()))?;
        Ok(e)
    }

    #[test]
    fn sort_bsw_elements() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_subcontainers = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .and_then(|ap| ap.create_named_sub_element(ElementName::ArPackage, "Pkg"))
            .and_then(|ap| ap.create_sub_element(ElementName::Elements))
            .and_then(|elems| elems.create_named_sub_element(ElementName::EcucModuleConfigurationValues, "Config"))
            .and_then(|emcv| emcv.create_sub_element(ElementName::Containers))
            .and_then(|c| c.create_named_sub_element(ElementName::EcucContainerValue, "ConfigValues"))
            .and_then(|ecv| ecv.create_sub_element(ElementName::SubContainers))
            .unwrap();
        let elem1 =
            helper_create_indexed_bsw_subelem(&el_subcontainers, "Aaa", "06", "/Defref/Container/Value").unwrap(); // idx 6
        let elem2 =
            helper_create_indexed_bsw_subelem(&el_subcontainers, "Bbb", "5", "/Defref/Container/Value").unwrap(); // idx 5
        let elem3 =
            helper_create_indexed_bsw_subelem(&el_subcontainers, "Bbb2", "5", "/Defref/Container/Value").unwrap(); // idx 5 duplicate
        let elem4 =
            helper_create_indexed_bsw_subelem(&el_subcontainers, "Ccc", "0X4", "/Defref/Container/Value").unwrap(); // idx 4
        let elem5 = helper_create_bsw_subelem(&el_subcontainers, "Zzz", "/Defref/Container/Value").unwrap();
        let elem6 =
            helper_create_indexed_bsw_subelem(&el_subcontainers, "Ddd", "0b1", "/Defref/Container/Value").unwrap(); // idx 1
        let elem7 =
            helper_create_indexed_bsw_subelem(&el_subcontainers, "Eee", "0x3", "/Defref/Container/Value").unwrap(); // idx 3
        let elem8 =
            helper_create_indexed_bsw_subelem(&el_subcontainers, "Fff", "0B10", "/Defref/Container/Value").unwrap(); // idx 2
        let elem9 =
            helper_create_indexed_bsw_subelem(&el_subcontainers, "Ggg", "0", "/Defref/Container/Value").unwrap(); // idx 0

        let elem10 = helper_create_bsw_subelem(&el_subcontainers, "Mmm_0", "/Defref/Container/Value").unwrap();
        let elem11 = helper_create_bsw_subelem(&el_subcontainers, "Mmm_5", "/Defref/Container/Value").unwrap();
        let elem12 = helper_create_bsw_subelem(&el_subcontainers, "Mmm_10", "/Defref/Container/Value").unwrap();
        let elem13 = helper_create_bsw_subelem(&el_subcontainers, "Mmm_9", "/Defref/Container/Value").unwrap();

        el_subcontainers.sort();
        assert_eq!(elem1.position().unwrap(), 7);
        assert_eq!(elem2.position().unwrap(), 5);
        assert_eq!(elem3.position().unwrap(), 6);
        assert_eq!(elem4.position().unwrap(), 4);
        assert_eq!(elem6.position().unwrap(), 1);
        assert_eq!(elem7.position().unwrap(), 3);
        assert_eq!(elem8.position().unwrap(), 2);
        assert_eq!(elem9.position().unwrap(), 0);
        // elements without indices are sorted behind the indexed elements
        assert_eq!(elem10.position().unwrap(), 8);
        assert_eq!(elem11.position().unwrap(), 9);
        assert_eq!(elem13.position().unwrap(), 10);
        assert_eq!(elem12.position().unwrap(), 11);
        assert_eq!(elem5.position().unwrap(), 12);
    }

    #[test]
    fn file_membership() {
        let model = AutosarModel::new();
        let file1 = model.create_file("test_1", AutosarVersion::Autosar_00050).unwrap();
        let file2 = model.create_file("test_2", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg")
            .unwrap();
        el_ar_package.create_sub_element(ElementName::Elements).unwrap();

        let fm: HashSet<WeakArxmlFile> = vec![file1.downgrade()].iter().cloned().collect();
        // setting the file membership of el_ar_packages should fail
        // its parent is not splittable, so this is not allowed
        el_ar_packages.set_file_membership(fm.clone());
        let (local, _) = el_ar_package.file_membership().unwrap();
        assert!(!local);

        // setting the file membership of el_ar_package should succeed
        // this element is only part of file1, and is only serialized with file1
        el_ar_package.set_file_membership(fm.clone());
        let (local, fm2) = el_ar_package.file_membership().unwrap();
        assert!(local);
        assert_eq!(fm, fm2);
        let filetxt1 = file1.serialize().unwrap();
        let filetxt2 = file2.serialize().unwrap();
        assert_ne!(filetxt1, filetxt2);

        // can't use a file from a different model in add_to_file / remove_from_file
        let model2 = AutosarModel::new();
        let model2_file = model2.create_file("file", AutosarVersion::LATEST).unwrap();
        assert!(el_ar_package.add_to_file(&model2_file).is_err());
        assert!(el_ar_package.remove_from_file(&model2_file).is_err());

        // adding el_ar_package to file1 does nothing, since it is already present in this file
        el_ar_package.add_to_file(&file1).unwrap();
        let (local, fm3) = el_ar_package.file_membership().unwrap();
        assert!(local);
        assert_eq!(fm3.len(), 1);

        // removing el_ar_package from file2 does nothing, it is not present in this file
        el_ar_package.remove_from_file(&file2).unwrap();
        let (local, fm3) = el_ar_package.file_membership().unwrap();
        assert!(local);
        assert_eq!(fm3.len(), 1);

        // adding el_ar_package to file2 succeeds
        el_ar_package.add_to_file(&file2).unwrap();
        let (local, fm3) = el_ar_package.file_membership().unwrap();
        assert!(local);
        assert_eq!(fm3.len(), 2);

        // removing el_ar_package from file1 and file2 causes it to be deleted
        assert!(el_ar_package.get_sub_element(ElementName::Elements).is_some());
        el_ar_package.remove_from_file(&file1).unwrap();
        el_ar_package.remove_from_file(&file2).unwrap();
        assert!(el_ar_package.get_sub_element(ElementName::Elements).is_none());
        assert!(el_ar_package.remove_from_file(&file2).is_err());
    }

    #[test]
    fn min_version() {
        let model = AutosarModel::new();
        let result = model.root_element().min_version();
        assert!(result.is_err());

        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let min_ver = model.root_element().min_version().unwrap();
        assert_eq!(min_ver, AutosarVersion::LATEST);

        model.create_file("test2", AutosarVersion::Autosar_00042).unwrap();
        let min_ver = model.root_element().min_version().unwrap();
        assert_eq!(min_ver, AutosarVersion::Autosar_00042);
    }

    #[test]
    fn element_sort_key() {
        let key_none = ElementSortKey {
            index: None,
            item_name_indexed: None,
            item_name: None,
            definition: None,
        };
        let key_idx_1 = ElementSortKey {
            index: Some(1),
            item_name_indexed: None,
            item_name: None,
            definition: None,
        };

        assert_ne!(key_none, key_idx_1);
        assert!(matches!(key_none.partial_cmp(&key_idx_1), Some(Ordering::Greater)));
        assert!(matches!(key_idx_1.partial_cmp(&key_idx_1), Some(Ordering::Equal)));
        assert!(matches!(key_idx_1.partial_cmp(&key_none), Some(Ordering::Less)));

        let key_partialidx_1 = ElementSortKey {
            index: None,
            item_name_indexed: Some(("base".to_string(), 1)),
            item_name: None,
            definition: None,
        };
        let key_partialidx_2 = ElementSortKey {
            index: None,
            item_name_indexed: Some(("base".to_string(), 2)),
            item_name: None,
            definition: None,
        };

        assert_ne!(key_partialidx_1, key_partialidx_2);
        assert!(matches!(key_partialidx_1.cmp(&key_partialidx_2), Ordering::Less));
        assert!(matches!(key_partialidx_1.cmp(&key_partialidx_1), Ordering::Equal));
        assert!(matches!(key_partialidx_2.cmp(&key_partialidx_1), Ordering::Greater));

        let key_defref_a = ElementSortKey {
            index: None,
            item_name_indexed: None,
            item_name: None,
            definition: Some("/A".to_string()),
        };
        let key_defref_b = ElementSortKey {
            index: None,
            item_name_indexed: None,
            item_name: None,
            definition: Some("/B".to_string()),
        };

        assert_ne!(key_defref_a, key_defref_b);
        assert!(matches!(key_defref_a.cmp(&key_defref_b), Ordering::Less));
        assert!(matches!(key_defref_a.cmp(&key_defref_a), Ordering::Equal));
        assert!(matches!(key_defref_b.cmp(&key_defref_a), Ordering::Greater));
    }

    #[test]
    fn traits() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();

        // traits of elements
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();

        let el_autosar_second_ref = el_autosar.clone();
        assert_eq!(el_autosar, el_autosar_second_ref);
        assert_eq!(format!("{el_autosar:?}"), format!("{el_autosar_second_ref:?}"));
        assert_ne!(el_autosar, el_ar_packages);

        let weak1 = el_autosar.downgrade();
        let weak2 = el_autosar_second_ref.downgrade();
        assert_eq!(weak1, weak2);
        assert_eq!(format!("{weak1:?}"), format!("{weak2:?}"));

        let mut hs = HashSet::new();
        hs.insert(el_autosar);
        hs.insert(el_ar_packages);
        // can't insert el_autosar_second_ref, it is already in the set
        assert!(!hs.insert(el_autosar_second_ref));
        assert_eq!(hs.len(), 2);

        let mut hs2 = HashSet::new();
        hs2.insert(weak1);
        assert_eq!(hs2.insert(weak2), false);
        assert_eq!(hs2.len(), 1);

        // traits of elementcontent
        let ec_elem = ElementContent::Element(model.root_element());
        assert_eq!(format!("{:?}", model.root_element()), format!("{ec_elem:?}"));
        assert_eq!(ec_elem.unwrap_element(), Some(model.root_element()));
        let cdata = CharacterData::String("test".to_string());
        let ec_chars = ElementContent::CharacterData(cdata.clone());
        assert_eq!(format!("{cdata:?}"), format!("{ec_chars:?}"));
        assert_eq!(ec_chars.unwrap_cdata(), Some(cdata));
    }
}
