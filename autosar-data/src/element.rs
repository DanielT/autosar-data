use std::cmp::Ordering;
use std::hash::Hash;
use std::str::FromStr;

use super::*;

static LOCK_CONTENTION_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(10);

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
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    pub fn parent(&self) -> Result<Option<Element>, AutosarDataError> {
        self.0.read().parent()
    }

    /// Get the next named parent (or grandparent, etc) of the current element
    ///
    /// This function steps through the hierarchy until an identifiable element is found.
    /// It never returns the current element, even if the current element is identifiable.
    ///
    /// The function returns a suitable element if one is found, or None if the root is reached.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050)?;
    /// # let el_arpackage = model.root_element().create_sub_element(ElementName::ArPackages)?.create_named_sub_element(ElementName::ArPackage, "Pkg")?;
    /// let el_elements = el_arpackage.create_sub_element(ElementName::Elements)?;
    /// let el_system = el_elements.create_named_sub_element(ElementName::System, "Sys")?;
    /// let named_parent = el_elements.named_parent()?.unwrap();
    /// let named_parent2 = el_system.named_parent()?.unwrap();
    /// assert_eq!(named_parent, el_arpackage);
    /// assert_eq!(named_parent2, el_arpackage);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    pub fn named_parent(&self) -> Result<Option<Element>, AutosarDataError> {
        let mut cur_elem_opt = self.parent()?;
        while let Some(parent) = cur_elem_opt {
            if parent.is_identifiable() {
                return Ok(Some(parent));
            }
            cur_elem_opt = parent.parent()?;
        }

        Ok(None)
    }

    pub(crate) fn set_parent(&self, new_parent: ElementOrModel) {
        self.0.write().set_parent(new_parent);
    }

    /// point this element and all of its sub elements at the given model
    ///
    /// Every element caches the model it belongs to next to its parent reference, so a subtree that
    /// was built outside of a model, by `deep_copy` or in a different model or by a move between
    /// models, has to be updated when it is attached. The element itself already has the right
    /// parent reference at this point, so setting it again is harmless.
    pub(crate) fn set_model_recursive(&self, model: &WeakAutosarModel) {
        for (_, element) in self.elements_dfs() {
            if let ElementOrModel::Element(_, element_model) = &mut element.0.write().parent {
                *element_model = model.clone();
            }
        }
    }

    /// detach this element and all of its sub elements from their model
    ///
    /// The elements keep their content, so handles to any of them remain usable, but they no
    /// longer have a parent or a model.
    pub(crate) fn detach_recursive(&self) {
        for (_, element) in self.elements_dfs() {
            element.0.write().parent = ElementOrModel::None;
        }
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
        self.0.read().elemname
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
        self.0.read().elemtype
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
        self.0.read().item_name()
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
    ///     short_name.set_character_data("the_new_name");
    /// }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::ItemNameRequired`] this function was called for an element which is not identifiable
    ///
    pub fn set_item_name(&self, new_name: &str) -> Result<(), AutosarDataError> {
        // a new name is required
        if new_name.is_empty() {
            return Err(AutosarDataError::ItemNameRequired {
                element: self.element_name(),
            });
        }
        let (model, version) = self.model_and_version()?;
        self.0.write().set_item_name(new_name, &model, version)
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
        self.0.read().is_identifiable()
    }

    /// Returns true if the element should contain a reference to another element
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
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: Th ecurrent element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::ElementNotIdentifiable`]: The current element is not identifiable, so it has no Autosar path
    ///
    pub fn path(&self) -> Result<String, AutosarDataError> {
        // path() is frequently called on parent elements while a child lock is held,
        // so a blocking read() here could deadlock
        self.0
            .try_read_for(LOCK_CONTENTION_TIMEOUT)
            .ok_or(AutosarDataError::ParentElementLocked)?
            .path()
    }

    /// Get the package element containing the current element
    ///
    /// It never returns the current element, even if the current element is a package, but always
    /// goes up the hierarchy to find the nearest parent package.
    ///
    /// Returns
    ///   Ok(Some(Element)) if a parent package is found
    ///   Ok(None) at the top level.
    ///   Err if the action cannot be completed
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element().create_sub_element(ElementName::ArPackages)?.create_named_sub_element(ElementName::ArPackage, "Pkg")?;
    /// let package = element.package()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    /// - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///   The operation was aborted to avoid a deadlock, but can be retried.
    pub fn package(&self) -> Result<Option<Element>, AutosarDataError> {
        let mut cur_elem = self.clone();
        loop {
            let parent = {
                let element = cur_elem
                    .0
                    .try_read_for(LOCK_CONTENTION_TIMEOUT)
                    .ok_or(AutosarDataError::ParentElementLocked)?;
                match &element.parent {
                    ElementOrModel::Element(weak_parent, _) => {
                        let parent = weak_parent.upgrade().ok_or(AutosarDataError::ItemDeleted)?;
                        // avoid deadlocking if the parent is locked, because we're already holding a child lock here
                        let parent_name = {
                            let parent_lock = parent
                                .0
                                .try_read_for(LOCK_CONTENTION_TIMEOUT)
                                .ok_or(AutosarDataError::ParentElementLocked)?;
                            parent_lock.element_name()
                        };

                        if parent_name == ElementName::ArPackage {
                            return Ok(Some(parent));
                        }
                        parent
                    }
                    ElementOrModel::Model(_) => {
                        return Ok(None);
                    }
                    ElementOrModel::None => return Err(AutosarDataError::ItemDeleted),
                }
            };
            cur_elem = parent;
        }
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
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///
    pub fn model(&self) -> Result<AutosarModel, AutosarDataError> {
        self.0
            .try_read_for(LOCK_CONTENTION_TIMEOUT)
            .ok_or(AutosarDataError::ParentElementLocked)?
            .model()
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
        self.elemtype().content_mode().into()
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
    /// # Errors
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
            .try_write()
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
    /// # Errors
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
            .write()
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
    /// # Errors
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
        let (model, version) = self.model_and_version()?;
        self.0
            .write()
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
    /// # Errors
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
        let (model, version) = self.model_and_version()?;
        self.0
            .write()
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
    /// # Errors
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
            return Err(AutosarDataError::InvalidSubElement {
                parent: self.element_name(),
                element: self.element_name(),
            });
        }
        let (model, version) = self.model_and_version()?;
        let copy = self
            .0
            .write()
            .create_copied_sub_element(self.downgrade(), other, &model, version);
        // the copy may contain relative references and REFERENCE-BASE declarations, neither of which
        // could be resolved while the element was being copied
        model.resolve_relative_references();
        copy
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
    /// # Errors
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
            return Err(AutosarDataError::InvalidSubElement {
                parent: self.element_name(),
                element: self.element_name(),
            });
        }
        let (model, version) = self.model_and_version()?;
        let copy = self
            .0
            .write()
            .create_copied_sub_element_at(self.downgrade(), other, position, &model, version);
        // the copy may contain relative references and REFERENCE-BASE declarations, neither of which
        // could be resolved while the element was being copied
        model.resolve_relative_references();
        copy
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
    /// # Errors
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
        if self == move_element {
            // trying to move self into self never makes sense, and would deadlock
            return Err(AutosarDataError::ForbiddenMoveToSubElement);
        }
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
        let moved = self
            .0
            .write()
            .move_element_here(self.downgrade(), move_element, &model, &model_src, version);
        // the moved elements may contain relative references which are now in the scope of different
        // reference bases, and they may declare reference bases themselves
        model_src.resolve_relative_references();
        model.resolve_relative_references();
        moved
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
    /// # Errors
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
        if self == move_element {
            // trying to move self into self never makes sense, and would deadlock
            return Err(AutosarDataError::ForbiddenMoveToSubElement);
        }
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
        let moved =
            self.0
                .write()
                .move_element_here_at(self.downgrade(), move_element, position, &model, &model_src, version);
        // the moved elements may contain relative references which are now in the scope of different
        // reference bases, and they may declare reference bases themselves
        model_src.resolve_relative_references();
        model.resolve_relative_references();
        moved
    }

    /// Reorder the existing sub elements to match `desired`.
    ///
    /// `desired` must be a permutation of the current sub elements; any
    /// element it does not mention, or any element it mentions that is not
    /// a current child, makes the call a no-op returning `false`.
    ///
    /// This is the bulk form of repeated [`Element::move_element_here_at`]:
    /// the autocomposition save passes permute a whole container at a time,
    /// and the per-element API costs a sibling scan plus a borrow for each
    /// of them (quadratic in the container size; packed AUTOSAR outputs have
    /// `ELEMENTS` lists with thousands of entries).
    ///
    /// The character data of an element with mixed content is not part of
    /// the permutation: it keeps its position between the sub elements, so
    /// only the sub elements themselves change places.
    ///
    /// Unlike `move_element_here_at`, this does not use the insert range of
    /// each individual element, but it still rejects an order that the
    /// specification does not allow: in a `Sequence` the sub elements have to
    /// stay sorted by their position in the schema, and in a `Choice` a
    /// different alternative may not be moved in front of the selected one.
    /// `Bag` and `Mixed` containers accept any order.
    ///
    /// Returns `true` when the content was reordered, `false` when `desired`
    /// was not a permutation of the current sub elements or would have
    /// produced an invalid order.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let root = model.root_element();
    /// let pkgs = root.create_sub_element(ElementName::ArPackages).unwrap();
    /// let b = pkgs.create_named_sub_element(ElementName::ArPackage, "b").unwrap();
    /// let a = pkgs.create_named_sub_element(ElementName::ArPackage, "a").unwrap();
    /// assert!(pkgs.reorder_sub_elements(&[a.clone(), b.clone()]));
    /// ```
    pub fn reorder_sub_elements(&self, desired: &[Element]) -> bool {
        let Some(version) = self.min_version().ok() else {
            return false;
        };
        let mut locked = self.0.write();
        let mut current: Vec<Element> = Vec::with_capacity(locked.content.len());
        for item in &locked.content {
            if let ElementContent::Element(elem) = item {
                current.push(elem.clone());
            }
        }
        if current.len() != desired.len() {
            return false;
        }
        // `desired` must be exactly the current children, each once. Elements are compared by
        // identity, so their allocation address is used as the key: the contents of an element can
        // be modified through a lock, which disqualifies `Element` itself as a `HashSet` key.
        let current_ids: HashSet<usize> = current.iter().map(|elem| Arc::as_ptr(&elem.0) as usize).collect();
        let mut desired_ids = HashSet::with_capacity(desired.len());
        for want in desired {
            let id = Arc::as_ptr(&want.0) as usize;
            if !current_ids.contains(&id) || !desired_ids.insert(id) {
                return false;
            }
        }
        if current.iter().zip(desired).all(|(have, want)| have == want) {
            return false;
        }
        if !locked.is_valid_sub_element_order(desired, version) {
            return false;
        }
        // fill the element slots with the new order; the character data of a mixed content element
        // stays in its position between them
        let mut wanted = desired.iter();
        let mut next: Vec<ElementContent> = Vec::with_capacity(locked.content.len());
        for item in &locked.content {
            match item {
                ElementContent::Element(_) => match wanted.next() {
                    Some(want) => next.push(ElementContent::Element(want.clone())),
                    None => return false,
                },
                ElementContent::CharacterData(cdata) => next.push(ElementContent::CharacterData(cdata.clone())),
            }
        }
        locked.content = next.into();
        true
    }

    /// Remove the sub element `sub_element`
    ///
    /// The `sub_element` will be unlinked from the hierarchy of elements.
    /// All of the sub-sub-elements nested under the removed element will also be recursively removed.
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
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::ElementNotFound`]: The sub element was not found in this element
    ///  - [`AutosarDataError::ShortNameRemovalForbidden`]: It is not permitted to remove the SHORT-NAME of identifiable elements since this would result in invalid data
    pub fn remove_sub_element(&self, sub_element: Element) -> Result<(), AutosarDataError> {
        if *self == sub_element {
            // an element is never a sub element of itself; without this check the operation would deadlock
            return Err(AutosarDataError::ElementNotFound {
                target: self.element_name(),
                parent: self.element_name(),
            });
        }
        let model = self.model()?;
        let result = self.0.write().remove_sub_element(sub_element, &model);
        // removing a REFERENCE-BASE changes what the relative references in its scope resolve to
        model.resolve_relative_references();
        result
    }

    /// Remove a sub element identified by an ElementName
    ///
    /// If multiple sub elements with the same ElementName exist, only the first one will be removed.
    ///
    /// A sub element with the given ElementName will be unlinked from the hierarchy of elements.
    /// All of the sub-sub-elements nested under the removed element will also be recursively removed.
    ///
    /// This is a convenience function that is equivalent to calling `get_sub_element()` followed by `remove_sub_element()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let packages = model.root_element().create_sub_element(ElementName::ArPackages)?;
    /// model.root_element().remove_sub_element_kind(ElementName::ArPackages)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::ElementNotFound`]: The sub element was not found in this element
    ///  - [`AutosarDataError::ShortNameRemovalForbidden`]: It is not permitted to remove the SHORT-NAME of identifiable elements since this would result in invalid data
    pub fn remove_sub_element_kind(&self, element_name: ElementName) -> Result<(), AutosarDataError> {
        let Some(sub_element) = self.get_sub_element(element_name) else {
            return Err(AutosarDataError::ElementNotFound {
                target: element_name,
                parent: self.element_name(),
            });
        };
        self.remove_sub_element(sub_element)
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
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::NotReferenceElement`]: The current element is not a reference, so it is not possible to set a reference target
    ///  - [`AutosarDataError::InvalidReference`]: The target element is not a valid reference target for this reference
    ///  - [`AutosarDataError::ElementNotIdentifiable`]: The target element is not identifiable, so it cannot be referenced by an Autosar path
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn set_reference_target(&self, target: &Element) -> Result<(), AutosarDataError> {
        let result = self.set_reference_target_internal(target, None);
        // the reference is registered while this element is locked, so it can only be resolved here
        if let Ok(model) = self.model() {
            model.resolve_relative_references();
        }
        result
    }

    /// Set the reference target using a relative path and explicit BASE label
    ///
    /// This method behaves like [`Self::set_reference_target`], but it writes
    /// a relative reference path and sets the BASE attribute to `base_label`.
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::NotReferenceElement`]: The current element is not a reference, so it is not possible to set a reference target
    ///  - [`AutosarDataError::InvalidReference`]: The target element is not a valid reference target for this reference
    ///  - [`AutosarDataError::InvalidReferenceBase`]: The target element uses an invalid base label
    ///  - [`AutosarDataError::ElementNotIdentifiable`]: The target element is not identifiable, so it cannot be referenced by an Autosar path
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn set_relative_reference_target(&self, target: &Element, base_label: &str) -> Result<(), AutosarDataError> {
        let result = self.set_reference_target_internal(target, Some(base_label));
        // the reference is registered while this element is locked, so it can only be resolved here
        if let Ok(model) = self.model() {
            model.resolve_relative_references();
        }
        result
    }

    /// Resolve the reference base named `label`, for a relative reference located in this element
    ///
    /// A REFERENCE-BASE is declared by an AR-PACKAGE, and it is in scope for that package as well as for
    /// everything nested inside it. The applicable declaration is therefore found by walking up the
    /// ancestor packages of the reference and taking the first one which declares `label`.
    ///
    /// The PACKAGE-REF of a REFERENCE-BASE may carry a BASE attribute of its own, i.e. it can be
    /// relative to another reference base. It is then resolved in the scope of the package which
    /// declares it, and the two paths are joined. Invalid data can make this chain cyclic, so the
    /// declarations which have already been visited are tracked in order to avoid looping forever.
    ///
    /// The result is `None` if the label is not in scope, if this element is not inside a package, or if
    /// an element lock could not be acquired: this reads the element tree while the caller may hold the
    /// model lock, so every element lock is taken with a timeout.
    pub(crate) fn resolve_reference_base(&self, label: &str) -> Option<String> {
        let mut search_package = self.package().ok().flatten()?;
        let mut label = label.to_owned();
        // the path components contributed by the chained reference bases, innermost first
        let mut relative_parts: Vec<String> = Vec::new();
        let mut visited: Vec<(Element, String)> = Vec::new();

        loop {
            let (declaring_package, package_ref, package_ref_base) =
                Self::find_reference_base_declaration(&search_package, &label)?;
            if visited.contains(&(declaring_package.clone(), label.clone())) {
                // a cycle in the chain of reference bases - the base cannot be resolved
                return None;
            }
            visited.push((declaring_package.clone(), label));

            let Some(outer_label) = package_ref_base else {
                // the end of the chain: this PACKAGE-REF contains an absolute path
                let mut resolved = package_ref;
                for part in relative_parts.iter().rev() {
                    resolved.push('/');
                    resolved.push_str(part);
                }
                return Some(resolved);
            };
            relative_parts.push(package_ref);
            // the BASE attribute of the PACKAGE-REF is resolved in the scope of the package which
            // declares this reference base
            search_package = declaring_package;
            label = outer_label;
        }
    }

    /// Resolve this relative reference to the absolute path of the element it refers to
    ///
    /// This is the definition of what a relative reference means, and therefore of the key it is
    /// registered under in the model. The result is `None` if this is not a usable relative reference:
    /// it has no character data or no BASE attribute, it is not inside a package, or its reference base
    /// is not in scope.
    pub(crate) fn resolve_relative_target(&self) -> Option<String> {
        let reference = self.character_data()?.string_value()?;
        let base_label = self.attribute_value(AttributeName::Base)?.string_value()?;
        let base_path = self.resolve_reference_base(&base_label)?;
        Some(format!("{base_path}/{reference}"))
    }

    /// Find the nearest declaration of the reference base `label`: `package` itself is searched first,
    /// followed by its ancestor packages
    ///
    /// The result is the declaring package, the character data of its PACKAGE-REF, and the BASE
    /// attribute of that PACKAGE-REF.
    fn find_reference_base_declaration(package: &Element, label: &str) -> Option<(Element, String, Option<String>)> {
        let mut current_package = Some(package.clone());
        while let Some(package) = current_package {
            if let Some(reference_bases) = package.try_get_sub_element(ElementName::ReferenceBases) {
                let declarations: Vec<Element> = reference_bases
                    .0
                    .try_read_for(LOCK_CONTENTION_TIMEOUT)?
                    .content
                    .iter()
                    .filter_map(|item| match item {
                        ElementContent::Element(sub_element) => Some(sub_element.clone()),
                        ElementContent::CharacterData(_) => None,
                    })
                    .collect();
                for declaration in declarations {
                    if let Some((declared_label, package_ref, package_ref_base)) = declaration
                        .0
                        .try_read_for(LOCK_CONTENTION_TIMEOUT)?
                        .reference_base_declaration()
                        && declared_label == label
                    {
                        return Some((package, package_ref, package_ref_base));
                    }
                }
            }
            current_package = package.package().ok().flatten();
        }
        None
    }

    /// Get the first sub element with the given name, acquiring every element lock with a timeout
    ///
    /// Unlike [`Self::get_sub_element`] this can be used while the model lock is held.
    fn try_get_sub_element(&self, name: ElementName) -> Option<Element> {
        let locked_element = self.0.try_read_for(LOCK_CONTENTION_TIMEOUT)?;
        for item in &locked_element.content {
            if let ElementContent::Element(sub_element) = item
                && sub_element.0.try_read_for(LOCK_CONTENTION_TIMEOUT)?.elemname == name
            {
                return Some(sub_element.clone());
            }
        }
        None
    }

    fn set_reference_target_internal(
        &self,
        target: &Element,
        base_label: Option<&str>,
    ) -> Result<(), AutosarDataError> {
        // the current element must be a reference
        if !self.is_reference() {
            return Err(AutosarDataError::NotReferenceElement);
        }

        // the target element must be identifiable, i.e. it has an autosar path
        let new_ref = target.path()?;
        // it must be possible to use the name of the referenced element name as an enum item in the dest attribute of the reference
        let Some(enum_item) = EnumItem::from_str(target.element_name().to_str())
            .ok()
            .or(self.element_type().reference_dest_value(&target.element_type()))
        else {
            return Err(AutosarDataError::InvalidReference);
        };

        let model = self.model()?;
        // build a target string, which is either the absolute path or the relative path depending on whether a base label was provided
        let target_string = if let Some(base_label) = base_label {
            // a relative reference can only be resolved if the reference element is inside a package;
            // this is not always the case, e.g. references inside AUTOSAR > ADMIN-DATA are outside any package
            let base_path = self
                .resolve_reference_base(base_label)
                .ok_or(AutosarDataError::InvalidReferenceBase)?;
            let mut trimmed_target_path = new_ref
                .strip_prefix(&base_path)
                .ok_or(AutosarDataError::InvalidReferenceBase)?;
            if let Some(no_leading_slash) = trimmed_target_path.strip_prefix('/') {
                trimmed_target_path = no_leading_slash;
            }
            trimmed_target_path.to_string()
        } else {
            new_ref
        };

        let version = self.min_version()?;

        let mut element = self.0.write();
        // set the DEST attribute first - this could fail if the target element has the wrong type
        if element
            .set_attribute_internal(AttributeName::Dest, CharacterData::Enum(enum_item), version)
            .is_err()
        {
            return Err(AutosarDataError::InvalidReference);
        }

        let opt_old_ref = element.character_data().and_then(|cdata| cdata.string_value());

        // if this reference previously referenced some other element, update
        if let Some(old_ref) = opt_old_ref {
            model.fix_reference_origins(&old_ref, &target_string, base_label, self.downgrade());
        } else {
            // else initialise the new reference
            model.add_reference_origin(&target_string, base_label, self.downgrade());
        }

        // do the update - Note: The update is infallible in practice, since we've constructed a valid target string
        element.set_character_data(CharacterData::String(target_string), version)?;
        if let Some(base_label) = base_label {
            element.set_attribute_internal(
                AttributeName::Base,
                CharacterData::String(base_label.to_string()),
                version,
            )?;
        } else {
            // ensure BASE does not remain when retargeting from a relative reference to an absolute reference
            element.remove_attribute(AttributeName::Base);
        }

        Ok(())
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
    /// # Errors
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

                let full_path = if self.attribute_value(AttributeName::Base).is_some() {
                    // The target path of a relative reference is already known: it is the key that the
                    // reference is registered under, so its reference base does not have to be resolved
                    // again here. The path is absent while the base is not in scope, which happens e.g.
                    // for a reference inside AUTOSAR > ADMIN-DATA, outside of any package.
                    model
                        .relative_reference_target(&self.downgrade())
                        .ok_or(AutosarDataError::InvalidReference)?
                } else {
                    reference
                };
                let target_elem = model
                    .get_element_by_path(&full_path)
                    .ok_or(AutosarDataError::InvalidReference)?;

                let dest_value = self
                    .attribute_value(AttributeName::Dest)
                    .and_then(|cdata| cdata.enum_value())
                    .ok_or(AutosarDataError::InvalidReference)?;
                if target_elem.element_type().verify_reference_dest(dest_value) {
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
    /// element.set_character_data("value")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: Cannot set character data on an element which does not contain character data
    ///  - [`AutosarDataError::InvalidCharacterData`]: The character data is not valid for this element - wrong type, or does not match the
    ///    pattern or string restrictions of the element type
    pub fn set_character_data<T: Into<CharacterData>>(&self, value: T) -> Result<(), AutosarDataError> {
        let chardata: CharacterData = value.into();
        self.set_character_data_internal(chardata)
    }

    // internal function to set the character data - separated out since it doesn't need to be generic
    fn set_character_data_internal(&self, mut chardata: CharacterData) -> Result<(), AutosarDataError> {
        let elemtype = self.elemtype();
        if (elemtype.content_mode() == ContentMode::Characters || elemtype.content_mode() == ContentMode::Mixed)
            && let Some(cdata_spec) = elemtype.chardata_spec()
        {
            let (model, version) = self.model_and_version()?;
            let mut compatible_value = CharacterData::check_value(&chardata, cdata_spec, version);
            if !compatible_value
                && matches!(
                    cdata_spec,
                    CharacterDataSpec::Pattern { .. } | CharacterDataSpec::String { .. }
                )
            {
                chardata = CharacterData::String(chardata.to_string());
                compatible_value = CharacterData::check_value(&chardata, cdata_spec, version);
            }
            if compatible_value {
                // if this is a SHORT-NAME element a whole lot of handling is needed in order to unbreak all the cross references
                let mut prev_path = None;
                if self.element_name() == ElementName::ShortName {
                    // this SHORT-NAME element might be newly created, in which case there is no previous path
                    if self.character_data().is_some()
                        && let Some(parent) = self.parent()?
                    {
                        prev_path = Some(parent.path()?);
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
                    let mut element = self.0.write();
                    element.content.clear();
                    element.content.push(ElementContent::CharacterData(chardata));
                }

                // short-name: make sure the hashmap in the top-level AutosarModel is updated so that this element can still be found
                if let Some(prev_path) = prev_path
                    && let Some(parent) = self.parent()?
                {
                    let new_path = parent.path()?;
                    model.fix_element_paths(&PathRemap::single(prev_path, new_path));
                    // note: set_character_data explicitly does not adapt references to the renamed element, unlike set_item_name
                }

                // reference: update the references hashmap in the top-level AutosarModel
                if elemtype.is_ref()
                    && let Some(CharacterData::String(refval)) = self.character_data()
                {
                    let base = self
                        .attribute_value(AttributeName::Base)
                        .and_then(|cdata| cdata.string_value());
                    if let Some(old_refval) = old_refval {
                        model.fix_reference_origins(&old_refval, &refval, base.as_deref(), self.downgrade());
                    } else {
                        model.add_reference_origin(&refval, base.as_deref(), self.downgrade());
                    }
                }

                // This may have been a reference, or the SHORT-LABEL or PACKAGE-REF of a
                // REFERENCE-BASE, which changes what the relative references in its scope resolve to.
                model.resolve_relative_references();

                Ok(())
            } else {
                Err(AutosarDataError::InvalidCharacterData {
                    element: self.element_name(),
                    value: chardata.to_string(),
                })
            }
        } else {
            Err(AutosarDataError::IncorrectContentType {
                element: self.element_name(),
            })
        }
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
    /// # Errors
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
                        // the model is required in order to de-register the reference
                        let model = self.model()?;
                        if let Some(CharacterData::String(reference)) = self.character_data() {
                            model.remove_reference_origin(&reference, self.downgrade());
                        }
                    }
                    self.0.write().content.clear();
                    // this may have been the SHORT-LABEL or PACKAGE-REF of a REFERENCE-BASE, which the
                    // relative references in its scope resolve through
                    if let Ok(model) = self.model() {
                        model.resolve_relative_references();
                    }
                }
                Ok(())
            }
        } else {
            Err(AutosarDataError::IncorrectContentType {
                element: self.element_name(),
            })
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
    /// # Errors
    ///
    ///  - [`AutosarDataError::IncorrectContentType`] the element `content_type` is not Mixed
    ///  - [`AutosarDataError::InvalidPosition`] the position is not valid
    pub fn insert_character_content_item(&self, chardata: &str, position: usize) -> Result<(), AutosarDataError> {
        let mut element = self.0.write();
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
            Err(AutosarDataError::IncorrectContentType {
                element: element.element_name(),
            })
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
    /// # Errors
    ///
    ///  - [`AutosarDataError::IncorrectContentType`] the element `content_type` is not Mixed
    ///  - [`AutosarDataError::InvalidPosition`] the position is not valid
    pub fn remove_character_content_item(&self, position: usize) -> Result<(), AutosarDataError> {
        let mut element = self.0.write();
        if let ContentMode::Mixed = element.elemtype.content_mode() {
            if position < element.content.len()
                && let ElementContent::CharacterData(_) = element.content[position]
            {
                element.content.remove(position);
                return Ok(());
            }
            Err(AutosarDataError::InvalidPosition)
        } else {
            Err(AutosarDataError::IncorrectContentType {
                element: element.element_name(),
            })
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
        self.0.read().content.len()
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
    ///     Some(CharacterData::Float(floatval)) => {},
    ///     None => {},
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn character_data(&self) -> Option<CharacterData> {
        self.0.read().character_data()
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
                .read()
                .content
                .iter()
                .position(|ec| matches!(ec, ElementContent::Element(elem) if elem == self))
        } else {
            None
        }
    }

    /// Create an iterator over all sub elements of this element
    ///
    /// If the element is modified while the iterator is in use, the iterator can skip sub elements or return duplicates.
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
    /// The first child whose item name equals `name`, ignoring `except`
    ///
    /// The comparison happens under the child lock against the stored
    /// short name, so no `String` is allocated per child. This is the
    /// lookup a caller needs when it wants to find an existing sibling
    /// with a given name before inserting another — doing it by walking
    /// [`Element::sub_elements`] and calling [`Element::item_name`] costs
    /// one lock plus one allocation per child, which is quadratic over a
    /// container that grows to tens of thousands of entries.
    ///
    /// Document order is preserved, so when several children share the
    /// name the first one is returned.
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
    /// # let other = base.create_named_sub_element(ElementName::System, "Path")?;
    /// assert_eq!(base.find_named_sub_element("Path", None), Some(other));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn find_named_sub_element(&self, name: &str, except: Option<&Element>) -> Option<Element> {
        self.0.read().find_named_sub_element(name, except)
    }

    /// Iterate over the sub-elements of this element
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

    /// Number of raw content entries of this element
    ///
    /// Counts both character data and sub elements — the index space of
    /// [`Element::content_at`].
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// for index in 0..element.content_len() {
    ///     let _ = element.content_at(index);
    /// }
    /// ```
    #[must_use]
    pub fn content_len(&self) -> usize {
        self.0.read().content.len()
    }

    /// Get a single raw content entry by index
    ///
    /// The index counts entries of the element's content list — both
    /// character data and sub elements — so it is *not* a child ordinal.
    /// This gives positional access to a sub element without walking the
    /// container from the start, which matters for containers holding tens
    /// of thousands of children.
    ///
    /// Returns `None` when the index is out of range or the element has
    /// been deleted.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// if let Some(content) = element.content_at(0) {
    ///     let _ = content.unwrap_element();
    /// }
    /// ```
    #[must_use]
    pub fn content_at(&self, index: usize) -> Option<ElementContent> {
        self.0.read().content.get(index).cloned()
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
        let locked_elem = self.0.read();
        for item in &locked_elem.content {
            if let ElementContent::Element(subelem) = item
                && subelem.element_name() == name
            {
                return Some(subelem.clone());
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
        let locked_elem = self.0.read();
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
    /// # Errors
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
        let mut locked_elem = self.0.try_write().ok_or(AutosarDataError::ParentElementLocked)?;
        for item in &locked_elem.content {
            if let ElementContent::Element(subelem) = item
                && subelem.element_name() == name
            {
                return Ok(subelem.clone());
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
    /// # Errors
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
        let (model, version) = self.model_and_version()?;
        let mut locked_elem = self.0.try_write().ok_or(AutosarDataError::ParentElementLocked)?;
        for item in &locked_elem.content {
            if let ElementContent::Element(subelem) = item
                && subelem.element_name() == element_name
                && subelem.item_name().as_deref().unwrap_or("") == item_name
            {
                return Ok(subelem.clone());
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
        ElementsDfsIterator::new(self, 0)
    }

    /// Create a depth first iterator over this element and all of its sub elements up to a maximum depth
    ///
    /// Each step in the iteration returns the depth and an element. Due to the nature of a depth first search,
    /// the returned depth can remain the same, increase by one, or decrease by an arbitrary number in each step.
    ///
    /// The dfs iterator will always return this element as the first item. A `max_depth` of `0` returns all
    /// child elements, regardless of depth (like `elements_dfs` does).
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
    /// for (depth, elem) in element.elements_dfs_with_max_depth(1) {
    ///     assert!(depth <= 1);
    ///     // ...
    /// }
    /// ```
    #[must_use]
    pub fn elements_dfs_with_max_depth(&self, max_depth: usize) -> ElementsDfsIterator {
        ElementsDfsIterator::new(self, max_depth)
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
        self.0.read().attribute_value(attrname)
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
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::InvalidAttribute`]: The `AttributeName` is not valid for this element
    ///  - [`AutosarDataError::InvalidAttributeValue`]: The value is not valid for this attribute in this element
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    This happens while determining the Autosar version of the element, so the attribute has not been set in this case.
    pub fn set_attribute<T: Into<CharacterData>>(
        &self,
        attrname: AttributeName,
        value: T,
    ) -> Result<(), AutosarDataError> {
        let version = self.min_version()?;
        let (old_base, reference_value) = self.base_attribute_info(attrname);

        self.0.write().set_attribute_internal(attrname, value.into(), version)?;

        self.base_attribute_fixup(attrname, old_base, reference_value);
        Ok(())
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
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::InvalidAttribute`]: The `AttributeName` is not valid for this element
    ///  - [`AutosarDataError::InvalidAttributeValue`]: The value is not valid for this attribute in this element
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    This happens while determining the Autosar version of the element, so the attribute has not been set in this case.
    pub fn set_attribute_string(&self, attrname: AttributeName, stringvalue: &str) -> Result<(), AutosarDataError> {
        let version = self.min_version()?;
        let (old_base, reference_value) = self.base_attribute_info(attrname);

        self.0.write().set_attribute_string(attrname, stringvalue, version)?;

        // post-change fixup for BASE attribute changes only
        self.base_attribute_fixup(attrname, old_base, reference_value);
        Ok(())
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
        let (old_base, reference_value) = self.base_attribute_info(attrname);

        let removed = self.0.write().remove_attribute(attrname);

        if removed {
            // fix the cache of reference origins if a BASE attribute was removed
            self.base_attribute_fixup(attrname, old_base, reference_value);
        }
        removed
    }

    /// helper function to get the old BASE attribute value and the reference value before changing or removing the attribute
    fn base_attribute_info(&self, attrname: AttributeName) -> (Option<String>, Option<String>) {
        if !self.is_reference() || attrname != AttributeName::Base {
            return (None, None);
        }
        let old_base = self
            .attribute_value(AttributeName::Base)
            .and_then(|cdata| cdata.string_value());
        let reference_value = self.character_data().and_then(|cdata| cdata.string_value());
        (old_base, reference_value)
    }

    /// Fix the caches affected by a reference element's BASE attribute being changed or removed
    fn base_attribute_fixup(&self, attrname: AttributeName, old_base: Option<String>, reference_value: Option<String>) {
        if attrname != AttributeName::Base {
            return;
        }
        if let Some(reference_value) = reference_value
            && let Ok(model) = self.model()
        {
            let new_base = self
                .attribute_value(AttributeName::Base)
                .and_then(|cdata| cdata.string_value());
            if old_base != new_base {
                model.fix_reference_origins(
                    &reference_value,
                    &reference_value,
                    new_base.as_deref(),
                    self.downgrade(),
                );
            }
        }
        // The BASE attribute of the PACKAGE-REF of a REFERENCE-BASE selects the reference base that
        // the PACKAGE-REF is itself relative to, so this can change what the relative references in the
        // scope of that REFERENCE-BASE resolve to.
        if let Ok(model) = self.model() {
            model.resolve_relative_references();
        }
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
        self.0.write().sort();
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

        self.0
            .read()
            .serialize_internal(&mut outstring, 0, false, &None, None)
            .expect("writing to a String never fails");

        outstring
    }

    pub(crate) fn elemtype(&self) -> ElementType {
        self.0.read().elemtype
    }

    // an element might have a diffeent element type depending on the version - as a result of a
    // changed datatype of the CharacterData, or because the element ordering was changed
    fn recalc_element_type(&self, target_version: AutosarVersion) -> ElementType {
        if let Ok(Some(parent)) = self.parent()
            && let Some((etype, ..)) = parent
                .element_type()
                .find_sub_element(self.element_name(), target_version as u32)
        {
            return etype;
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
            let element = self.0.read();
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
            if sub_element.0.read().is_in_file(file)
                && let Some((_, indices)) = elemtype_new
                    .find_sub_element(sub_element.element_name(), target_version as u32)
                    .or(elemtype_new.find_sub_element(sub_element.element_name(), u32::MAX))
                && let Some(version_mask) = elemtype_new.get_sub_element_version_mask(&indices)
            {
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
        let etype = self.0.read().elemtype;
        let mut valid_sub_elements = Vec::new();

        if let Ok(version) = self.min_version() {
            for (element_name, _, version_mask, named_mask) in etype.sub_element_spec_iter() {
                if version.compatible(version_mask) {
                    let is_named = version.compatible(named_mask);
                    let is_allowed = self.0.read().calc_element_insert_range(element_name, version).is_ok();
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
    /// # Errors
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
                .try_read_for(LOCK_CONTENTION_TIMEOUT)
                .ok_or(AutosarDataError::ParentElementLocked)?;
            if let Some(files) = locked_cur_elem.file_membership.as_deref() {
                return Ok((cur_elem == self, files.clone()));
            }
            let parent = locked_cur_elem.parent()?;
            drop(locked_cur_elem);

            cur_elem_opt = parent;
        }

        // no file membership info found at any level - this only happens if the model does not contain any files
        Err(AutosarDataError::NoFilesInModel)
    }

    /// Number of files in this element's (inherited) file membership, without
    /// materialising the set.
    ///
    /// Answers the "is this element shared between files" question that
    /// [`Element::file_membership`] answers by cloning the whole set.
    ///
    /// Returns `None` when no enclosing element restricts the membership (the
    /// model does not contain any files).
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// let model = AutosarModel::new();
    /// let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let root = model.root_element();
    /// assert_eq!(root.inherited_file_count(), Some(1));
    /// ```
    #[must_use]
    pub fn inherited_file_count(&self) -> Option<usize> {
        let mut cur_elem_opt = Some(self.clone());
        while let Some(cur_elem) = &cur_elem_opt {
            let locked_cur_elem = cur_elem.0.try_read_for(LOCK_CONTENTION_TIMEOUT)?;
            if let Some(files) = locked_cur_elem.file_membership.as_deref() {
                return Some(files.len());
            }
            let parent = locked_cur_elem.parent().ok()?;
            drop(locked_cur_elem);
            cur_elem_opt = parent;
        }
        None
    }

    /// Test whether `file` is part of this element's (inherited) file
    /// membership, without materialising the set.
    ///
    /// [`Element::file_membership`] clones the whole
    /// `HashSet<WeakArxmlFile>`, which on a model with hundreds of files
    /// means allocating and copying hundreds of entries for a single
    /// membership test. Callers that test membership per element while
    /// walking the tree should use this instead.
    ///
    /// Returns `None` when no enclosing element restricts the membership
    /// (the model does not contain any files) — the caller decides what
    /// that means.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// let model = AutosarModel::new();
    /// let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let root = model.root_element();
    /// assert_eq!(root.is_member_of_file(&file), Some(true));
    /// ```
    #[must_use]
    pub fn is_member_of_file(&self, file: &ArxmlFile) -> Option<bool> {
        let target = file.downgrade();
        let mut cur_elem_opt = Some(self.clone());
        while let Some(cur_elem) = &cur_elem_opt {
            let locked_cur_elem = cur_elem.0.try_read_for(LOCK_CONTENTION_TIMEOUT)?;
            if let Some(files) = locked_cur_elem.file_membership.as_deref() {
                return Some(files.contains(&target));
            }
            let parent = locked_cur_elem.parent().ok()?;
            drop(locked_cur_elem);
            cur_elem_opt = parent;
        }
        None
    }

    /// true if this element is a member of `file`, without copying the set
    ///
    /// The membership test the file iterator needs, borrowing under the element
    /// lock instead of cloning the set. It runs for every element of every file
    /// walk, so the clone it replaces dominated a whole-model scan: the pack
    /// phase walked ~45M elements per class over 27 classes.
    pub(crate) fn is_in_file_cheap(&self, file: &WeakArxmlFile) -> bool {
        self.0.read().is_in_file(file)
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
            self.0.write().set_file_membership(file_membership);
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
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///
    pub fn add_to_file(&self, file: &ArxmlFile) -> Result<(), AutosarDataError> {
        let parent_splittable = self.parent()?.is_none_or(|p| p.element_type().splittable() != 0);
        if parent_splittable {
            if file.model()? == self.model()? {
                let weak_file = file.downgrade();
                // current_fileset is the set of files which contain the current element
                let (_, current_fileset) = self.file_membership()?;
                // if the model only has a single file or if the element is already in the set then there is nothing to do
                if !current_fileset.contains(&weak_file) {
                    let mut updated_fileset = current_fileset;
                    updated_fileset.insert(weak_file);
                    self.0.write().set_file_membership(updated_fileset);

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
                    if let Some(mut subelem) = se.0.try_write_for(LOCK_CONTENTION_TIMEOUT)
                        && subelem.file_membership.is_none()
                    {
                        subelem.set_file_membership(current_fileset.clone());
                    }
                }
            }

            let mut extended_fileset = current_fileset;
            extended_fileset.insert(weak_file);
            // if the parent is splittable, or if the current element already has a fileset, then that fileset should be updated
            let parent_splittable = self.parent()?.is_none_or(|p| p.element_type().splittable() != 0);
            if parent_splittable || local {
                self.0.write().set_file_membership(extended_fileset);
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
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::InvalidFile`]: The file is part of a different model
    ///  - [`AutosarDataError::FilesetModificationForbidden`]: The parent of the current element is not splittable, so the fileset of the current element cannot be modified
    ///  - [`AutosarDataError::RootElementRemovalForbidden`]: The current element is the AUTOSAR root element, which cannot be removed from the last file containing it
    ///  - [`AutosarDataError::ShortNameRemovalForbidden`]: The current element is a SHORT-NAME that would have to be deleted, which is not permitted
    ///
    pub fn remove_from_file(&self, file: &ArxmlFile) -> Result<(), AutosarDataError> {
        let parent_splittable = self.parent()?.is_none_or(|p| p.element_type().splittable() != 0);
        if parent_splittable {
            if file.model()? == self.model()? {
                let weak_file = file.downgrade();

                // current_fileset is the set of files which contain the current element
                let (_, current_fileset) = self.file_membership()?;
                if !current_fileset.contains(&weak_file) {
                    // the file does not contain this element, so there is nothing to remove. Bailing
                    // out here skips the subtree walk below, which is otherwise a no-op that visits
                    // every descendant of a large moved element for nothing.
                    return Ok(());
                }
                let mut restricted_fileset = current_fileset;
                restricted_fileset.remove(&weak_file);
                if restricted_fileset.is_empty() {
                    // the element would no longer be part of any file, so it must be deleted instead.
                    let Some(parent) = self.parent()? else {
                        return Err(AutosarDataError::RootElementRemovalForbidden);
                    };
                    parent.remove_sub_element(self.to_owned())
                } else {
                    self.0.write().set_file_membership(restricted_fileset);

                    // update all sub elements with non-default file_membership
                    let mut to_delete = Vec::new();
                    for (_, subelem) in self.elements_dfs() {
                        // only need to care about those where file_membership is not empty. All other inherit from their parent
                        if subelem.0.read().file_membership.is_some() {
                            subelem.0.write().remove_file_membership(&weak_file);
                            // removing the last file resets the membership to inherited, which means
                            // that subelem is no longer in any file and has to be deleted
                            if subelem.0.read().file_membership.is_none() {
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
                }
            } else {
                // adding a file from a different model is not permitted
                Err(AutosarDataError::InvalidFile)
            }
        } else {
            Err(AutosarDataError::FilesetModificationForbidden)
        }
    }

    /// Remove this element from several files at once
    ///
    /// Semantically identical to calling [`Element::remove_from_file`] for
    /// every file in `files`, but visits the subtree ONCE instead of once per
    /// file. Callers that move an element between resources must strip the
    /// source membership, and a model merged from N inputs gives every
    /// element a membership set of up to N entries: the per-file loop then
    /// walks the whole moved subtree N times. On the PWT corpus that was
    /// 47 494 moves x 755 files x subtree = the dominant cost of the pack.
    ///
    /// The membership sets are shrunk by the whole batch in one pass, and the
    /// elements left without any file are deleted at the end, exactly like the
    /// single-file version.
    ///
    /// # Errors
    ///
    /// Same as [`Element::remove_from_file`]; the first failure aborts the
    /// batch, so callers that tolerate individual failures should keep the
    /// per-file loop.
    pub fn remove_from_files(&self, files: &[ArxmlFile]) -> Result<(), AutosarDataError> {
        if files.is_empty() {
            return Ok(());
        }
        let model = self.model()?;
        let parent_splittable = self.parent()?.is_none_or(|p| p.element_type().splittable() != 0);
        if !parent_splittable {
            return Err(AutosarDataError::FilesetModificationForbidden);
        }
        for file in files {
            if file.model()? != model {
                return Err(AutosarDataError::InvalidFile);
            }
        }
        let targets: HashSet<WeakArxmlFile> = files.iter().map(ArxmlFile::downgrade).collect();
        let (_, current_fileset) = self.file_membership()?;
        if !current_fileset.iter().any(|f| targets.contains(f)) {
            return Ok(());
        }
        let mut restricted = current_fileset;
        restricted.retain(|f| !targets.contains(f));
        if restricted.is_empty() {
            let Some(parent) = self.parent()? else {
                return Err(AutosarDataError::RootElementRemovalForbidden);
            };
            return parent.remove_sub_element(self.to_owned()).map(|_| ());
        }
        self.0.write().set_file_membership(restricted);

        // One walk for the whole batch: only elements with an EXPLICIT
        // membership need attention (the rest inherit from their parent).
        let mut to_delete = Vec::new();
        for (_, subelem) in self.elements_dfs() {
            if subelem.0.read().file_membership.is_some() {
                {
                    let mut locked = subelem.0.write();
                    if let Some(owned) = locked.file_membership.as_mut() {
                        owned.retain(|f| !targets.contains(f));
                        if owned.is_empty() {
                            locked.file_membership = None;
                        }
                    }
                }
                if subelem.0.read().file_membership.is_none() {
                    to_delete.push(subelem);
                }
            }
        }
        for delete_elem in to_delete {
            if let Ok(Some(parent)) = delete_elem.parent() {
                let _ = parent.remove_sub_element(delete_elem);
            }
        }
        Ok(())
    }

    /// Return a path that includes non-identifiable elements by their xml names
    ///
    /// This function cannot fail completely, it will always collect as much information as possible.
    /// It is intended for display in error messages.
    #[must_use]
    pub fn xml_path(&self) -> String {
        self.0.read().xml_path()
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
    /// # Errors
    ///
    /// - [`AutosarDataError::ElementInsertionConflict`]: The sub element conflicts with an existing sub element
    /// - [`AutosarDataError::InvalidSubElement`]: The sub element is not valid inside this element
    pub fn calc_element_insert_range(
        &self,
        element_name: ElementName,
        version: AutosarVersion,
    ) -> Result<(usize, usize), AutosarDataError> {
        self.0.read().calc_element_insert_range(element_name, version)
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
    #[must_use]
    pub fn comment(&self) -> Option<String> {
        self.0.read().comment.clone()
    }

    /// Set or delete the comment attached to the element
    ///
    /// Set None to remove the comment.
    ///
    /// If the new comment value contains "--", then this is replaced with "__", because "--" is forbidden inside XML comments.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use std::collections::HashSet;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::LATEST).unwrap();
    /// # let element = model.root_element();
    /// # let string = "".to_string();
    /// element.set_comment(Some(string));
    /// ```
    pub fn set_comment(&self, mut opt_comment: Option<String>) {
        if let Some(comment) = &mut opt_comment {
            // make sure the comment we store never contains "--" as this is forbidden by the w3 xml specification
            if comment.contains("--") {
                *comment = comment.replace("--", "__");
            }
        }
        self.0.write().comment = opt_comment;
    }

    /// find the minumum version of all arxml files which contain this element
    ///
    /// Typically this reduces to finding out which single file contains the element and returning this version.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use std::collections::HashSet;
    /// # let model = AutosarModel::new();
    /// let file1 = model.create_file("file1", AutosarVersion::LATEST).unwrap();
    /// let file2 = model.create_file("file2", AutosarVersion::Autosar_4_3_0).unwrap();
    /// let version = model.root_element().min_version().unwrap();
    /// assert_eq!(version, AutosarVersion::Autosar_4_3_0);
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn min_version(&self) -> Result<AutosarVersion, AutosarDataError> {
        let model = self.model()?;
        self.min_version_in(&model)
    }

    /// get the model of this element together with its minimum Autosar version
    ///
    /// The element creation and modification functions need both values, and getting the version
    /// requires the model, so getting them separately would look up the model twice.
    pub(crate) fn model_and_version(&self) -> Result<(AutosarModel, AutosarVersion), AutosarDataError> {
        let model = self.model()?;
        let version = self.min_version_in(&model)?;
        Ok((model, version))
    }

    /// get the minimum Autosar version of this element, given the model it belongs to
    fn min_version_in(&self, model: &AutosarModel) -> Result<AutosarVersion, AutosarDataError> {
        if let Some(version) = model.single_file_version() {
            // every element of a model with a single file belongs to that file, so its version
            // applies without looking up the file membership
            return Ok(version);
        }

        // the model is split across several files, so the file membership of the element decides.
        // It is inherited from the closest parent element that has one.
        let mut cur_elem_opt = Some(self.clone());
        while let Some(cur_elem) = &cur_elem_opt {
            let locked_cur_elem = cur_elem
                .0
                .try_read_for(LOCK_CONTENTION_TIMEOUT)
                .ok_or(AutosarDataError::ParentElementLocked)?;
            if let Some(files) = locked_cur_elem.file_membership.as_deref() {
                let ver = files
                    .iter()
                    .filter_map(WeakArxmlFile::upgrade)
                    .map(|f| f.version())
                    .min()
                    .unwrap_or(AutosarVersion::LATEST);

                return Ok(ver);
            }
            // read the parent while the lock of the current element is still held: going through
            // Element::parent() instead would acquire the same lock a second time on every level
            let parent = locked_cur_elem.parent()?;
            drop(locked_cur_elem);

            cur_elem_opt = parent;
        }

        Err(AutosarDataError::NoFilesInModel)
    }
}

impl Ord for Element {
    /// compare the content of two elements
    ///
    /// This function compares the content of two elements, returning a cmp::Ordering value.
    /// The purpose of this function is to allow sorting of elements based on their content.
    ///
    /// The comparison is performed in the following order:
    /// 1. Element name
    /// 2. Index (if present)
    /// 3. Item name (if present)
    /// 4. Definition reference (if present)
    /// 5. DEST attribute (if present)
    /// 6. Content of the element
    /// 7. Attributes of the element
    ///
    /// If the comparison returns `Ordering::Equal`, then the two elements are identical, but this does not imply that they are the same object.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::LATEST).unwrap();
    /// # let element1 = model.root_element();
    /// # let element2 = model.root_element();
    /// let ordering = element1.cmp(&element2);
    /// ```
    fn cmp(&self, other: &Element) -> std::cmp::Ordering {
        // Sort by the element name first. This test prevents the other comparisons from being performed when they don't make sense.
        match self.element_name().to_str().cmp(other.element_name().to_str()) {
            Ordering::Equal => {}
            other => return other,
        }

        // if both elements have an index, then compare the index; if only one has an index, then it comes first
        // if neither has an index, then continue and compare other criteria
        let index1 = self
            .get_sub_element(ElementName::Index)
            .and_then(|indexelem| indexelem.character_data())
            .and_then(|cdata| cdata.parse_integer::<u64>());
        let index2 = other
            .get_sub_element(ElementName::Index)
            .and_then(|indexelem| indexelem.character_data())
            .and_then(|cdata| cdata.parse_integer::<u64>());
        match (index1, index2) {
            (Some(idx1), Some(idx2)) => {
                let result = idx1.cmp(&idx2);
                if result != Ordering::Equal {
                    return result;
                }
            }
            (Some(_), None) => return std::cmp::Ordering::Less,
            (None, Some(_)) => return std::cmp::Ordering::Greater,
            (None, None) => {}
        }

        // sort by item name if present
        if let (Some(name1), Some(name2)) = (self.item_name(), other.item_name()) {
            // both items have a name - try to decompose the name into a base and an index
            // this allows for a more natural sorting of indexed items (e.g. "item2" < "item10")
            if let (Some((base1, idx1)), Some((base2, idx2))) =
                (decompose_item_name(&name1), decompose_item_name(&name2))
                && base1 == base2
            {
                let result = idx1.cmp(&idx2);
                if result != Ordering::Equal {
                    return result;
                }
            }
            // if the decomposition fails, then just compare the full item names
            let result = name1.cmp(&name2);
            if result != Ordering::Equal {
                return result;
            }
        }

        // for BSW values: compare the definition references
        let definition1 = self
            .get_sub_element(ElementName::DefinitionRef)
            .and_then(|defref| defref.character_data())
            .and_then(|cdata| cdata.string_value());
        let definition2 = other
            .get_sub_element(ElementName::DefinitionRef)
            .and_then(|defref| defref.character_data())
            .and_then(|cdata| cdata.string_value());
        if let (Some(def1), Some(def2)) = (definition1, definition2) {
            let result = def1.cmp(&def2);
            if result != Ordering::Equal {
                return result;
            }
        }

        // for references: compare the DEST attribute
        let dest1 = self
            .attribute_value(AttributeName::Dest)
            .and_then(|cdata| cdata.enum_value());
        let dest2 = other
            .attribute_value(AttributeName::Dest)
            .and_then(|cdata| cdata.enum_value());
        match (dest1, dest2) {
            (Some(dest1), Some(dest2)) => {
                let result = dest1.to_str().cmp(dest2.to_str());
                if result != Ordering::Equal {
                    return result;
                }
            }
            (Some(_), None) => return std::cmp::Ordering::Less,
            (None, Some(_)) => return std::cmp::Ordering::Greater,
            (None, None) => {}
        }

        // if all else fails, compare the content of the elements
        let locked_self = self.0.read();
        let locked_other = other.0.read();
        locked_self
            .content
            .cmp(&locked_other.content)
            .then(locked_self.attributes.cmp(&locked_other.attributes))
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Element) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// decompose an item name into a base name and an index
/// The index is expected to be a decimal number at the end of the string
///
/// E.g. "item123" -> ("item", 123)
fn decompose_item_name(name: &str) -> Option<(String, u64)> {
    let bytestr = name.as_bytes();
    let mut pos = bytestr.len();
    while pos > 0 && bytestr[pos - 1].is_ascii_digit() {
        pos -= 1;
    }
    if let Ok(index) = name[pos..].parse() {
        Some((name[0..pos].to_owned(), index))
    } else {
        None
    }
}

// a helper that provides compact debug output for the content of an element
struct ElementContentFormatter<'a>(&'a SmallVec<[ElementContent; 2]>);
impl std::fmt::Debug for ElementContentFormatter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list_fmt = f.debug_list();
        for item in self.0.iter() {
            match item {
                ElementContent::Element(elem) => list_fmt.entry(&elem.element_name()),
                ElementContent::CharacterData(cdata) => list_fmt.entry(&cdata),
            };
        }
        list_fmt.finish()
    }
}

// A custom type is needed in order to print a custom value in the Debug implementation without double quoting
struct DebugDisplay<'a>(&'a str);
impl std::fmt::Debug for DebugDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

// custom debug implementation: print the content instead of only showing the pointer of the Arc
impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elem = self.0.read();
        let mut dbgstruct = f.debug_struct("Element");
        if let Some(name) = elem.item_name() {
            dbgstruct.field("name", &name);
        }
        dbgstruct.field("elemname", &elem.elemname);
        dbgstruct.field("elemtype", &elem.elemtype);
        dbgstruct.field("parent", &elem.parent);
        dbgstruct.field("content", &ElementContentFormatter(&elem.content));
        dbgstruct.field("attributes", &elem.attributes);
        // only print the file membership if the element is splittable
        // elements that are not splittable may not modify their file membership
        if elem.elemtype.splittable() != 0 {
            if elem.file_membership.is_none() {
                dbgstruct.field("file_membership", &DebugDisplay("(inherited)"));
            } else {
                dbgstruct.field("file_membership", &elem.file_membership);
            }
        }
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
        if let Some(elem) = self.upgrade() {
            // write!(f, "Element:WeakRef {:p}", Arc::as_ptr(&elem.0));
            f.write_fmt(format_args!("Element:WeakRef ({})", elem.element_name()))
        } else {
            f.write_fmt(format_args!("Element:WeakRef {:p} (invalid)", Weak::as_ptr(&self.0)))
        }
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

// custom debug implementation: skip printing any content, since the content is only "WeakRef(0x...)"
impl std::fmt::Debug for ElementOrModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementOrModel::Element(..) => f.write_str("Element"),
            ElementOrModel::Model(_) => f.write_str("Model"),
            ElementOrModel::None => f.write_str("None/Invalid"),
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

#[cfg(test)]
mod test {
    use crate::*;
    use std::ffi::OsString;

    /// `remove_from_files` must be exactly the per-file loop: same surviving
    /// membership sets, same deletions (an element whose set empties is
    /// deleted), same early-out when no target file is present.
    ///
    /// The batch exists because a model merged from N inputs gives an element
    /// up to N membership entries, so the loop walked the moved subtree N
    /// times (251 s of the PWT pack). A batching bug would surface as a
    /// surviving element that the loop deleted, or a stale file entry.
    #[test]
    fn remove_from_files_matches_per_file_loop() {
        fn build() -> (AutosarModel, ArxmlFile, ArxmlFile, ArxmlFile, Element) {
            let model = AutosarModel::new();
            let f1 = model.create_file("a.arxml", AutosarVersion::Autosar_4_2_2).unwrap();
            let f2 = model.create_file("b.arxml", AutosarVersion::Autosar_4_2_2).unwrap();
            let f3 = model.create_file("c.arxml", AutosarVersion::Autosar_4_2_2).unwrap();
            let pkgs = model
                .root_element()
                .create_sub_element(ElementName::ArPackages)
                .unwrap();
            let pkg = pkgs.create_named_sub_element(ElementName::ArPackage, "P").unwrap();
            let elems = pkg.get_or_create_sub_element(ElementName::Elements).unwrap();
            let elt = elems
                .create_named_sub_element(ElementName::ImplementationDataType, "T")
                .unwrap();
            elt.create_sub_element(ElementName::SubElements).unwrap();
            // give the element (and one explicit child) a wide membership
            for f in [&f1, &f2, &f3] {
                elt.add_to_file(f).unwrap();
            }
            let child = elt
                .get_sub_element(ElementName::SubElements)
                .unwrap()
                .create_named_sub_element(ElementName::ImplementationDataTypeElement, "C")
                .unwrap();
            for f in [&f1, &f2] {
                child.add_to_file(f).unwrap();
            }
            (model, f1, f2, f3, elt)
        }

        // batch: strip f1 + f2, keeping f3
        let (_m1, f1, f2, f3, elt) = build();
        elt.remove_from_files(&[f1, f2]).unwrap();
        let (local_batch, set_batch) = elt.file_membership().unwrap();
        let batch_len = set_batch.len();
        assert!(local_batch);
        assert_eq!(batch_len, 1);
        assert!(set_batch.contains(&f3.downgrade()));
        // the child's set emptied -> it is deleted
        let child_alive = elt
            .get_sub_element(ElementName::SubElements)
            .map(|s| s.sub_elements().count())
            .unwrap_or(0);
        assert_eq!(child_alive, 1, "child still holds f3, so it survives");

        // loop: same operations, one file at a time
        let (_m2, f1, f2, f3, elt) = build();
        elt.remove_from_file(&f1).unwrap();
        elt.remove_from_file(&f2).unwrap();
        let (local_loop, set_loop) = elt.file_membership().unwrap();
        assert!(local_loop);
        assert_eq!(set_loop.len(), batch_len);
        assert!(set_loop.contains(&f3.downgrade()));
        let child_alive_loop = elt
            .get_sub_element(ElementName::SubElements)
            .map(|s| s.sub_elements().count())
            .unwrap_or(0);
        assert_eq!(child_alive_loop, child_alive);

        // unknown file: no-op, matching the loop's early-out
        let (_m3, _f1, _f2, f3, elt) = build();
        elt.remove_from_files(&[]).unwrap();
        elt.remove_from_files(&[f3]).unwrap();
        let (_, after) = elt.file_membership().unwrap();
        assert_eq!(after.len(), 2);
    }

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
        assert!(
            el_elements
                .create_named_sub_element(ElementName::CompuMethod, "TestCompuMethod3")
                .is_err()
        );

        let count = el_elements.sub_elements().count();
        assert_eq!(count, 3);
        assert_eq!(count, el_elements.content_item_count());

        // inserting another COMPU-METHOD into ELEMENTS hould be allowed at any position
        let (start_pos, end_pos) = el_elements
            .0
            .read()
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
        assert!(
            el_elements
                .create_named_sub_element_at(ElementName::System, "System", 99)
                .is_err()
        );
        assert!(el_autosar.create_sub_element_at(ElementName::AdminData, 99).is_err());

        // an identifiable element cannot be created without a name
        assert!(el_elements.create_sub_element(ElementName::System).is_err());
        // the name for an identifiable element must be valid according to the rules
        assert!(el_elements.create_named_sub_element(ElementName::System, "").is_err());
        assert!(
            el_elements
                .create_named_sub_element(ElementName::System, "abc def")
                .is_err()
        );

        // a non-identifiable element cannot be created with a name
        assert!(
            el_autosar
                .create_named_sub_element(ElementName::AdminData, "AdminData")
                .is_err()
        );

        // only valid sub-elements can be created
        assert!(
            el_autosar
                .create_named_sub_element(ElementName::Autosar, "Autosar")
                .is_err()
        );
        assert!(
            el_autosar
                .create_named_sub_element_at(ElementName::Autosar, "Autosar", 0)
                .is_err()
        );
        assert!(el_autosar.create_sub_element(ElementName::Autosar).is_err());
        assert!(el_autosar.create_sub_element_at(ElementName::Autosar, 0).is_err());

        // creating a sub element fails when any parent element in the hierarchy is locked for writing
        let el_autosar_locked = el_autosar.0.write();
        assert!(
            el_elements
                .create_named_sub_element(ElementName::System, "System")
                .is_err()
        );
        assert!(
            el_elements
                .create_named_sub_element_at(ElementName::System, "System", 0)
                .is_err()
        );
        assert!(el_autosar.create_sub_element(ElementName::AdminData).is_err());
        assert!(el_autosar.create_sub_element_at(ElementName::AdminData, 0).is_err());
        drop(el_autosar_locked);
    }

    #[test]
    fn element_creation_after_nonstrict_load() {
        // a non-strict load retains elements which are not valid in the version of the file:
        // PNC-VECTOR-LENGTH does not exist in Autosar 4.0.1
        let file_content = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_4-0-1.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
            <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME><ELEMENTS>
                <SYSTEM><SHORT-NAME>Sys</SHORT-NAME><PNC-VECTOR-LENGTH>8</PNC-VECTOR-LENGTH></SYSTEM>
            </ELEMENTS></AR-PACKAGE></AR-PACKAGES>
        </AUTOSAR>"#;
        let model = AutosarModel::new();
        let (_, warnings) = model
            .load_buffer(file_content.as_bytes(), OsString::from("test.arxml"), false)
            .unwrap();
        assert!(!warnings.is_empty());

        // creating a new sub element must position it relative to the retained
        // version-incompatible sub element instead of panicking
        let el_system = model.get_element_by_path("/Pkg/Sys").unwrap();
        el_system.create_sub_element(ElementName::FibexElements).unwrap();
    }

    #[test]
    fn parent() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Package")
            .unwrap();
        let el_elements = el_ar_package.create_sub_element(ElementName::Elements).unwrap();
        let el_system = el_elements
            .create_named_sub_element(ElementName::System, "Sys")
            .unwrap();
        let el_fibex = el_system.create_sub_element(ElementName::FibexElements).unwrap();
        let el_fibex_cond = el_fibex
            .create_sub_element(ElementName::FibexElementRefConditional)
            .unwrap();

        let parent = el_fibex_cond.parent().unwrap().unwrap();
        assert_eq!(parent, el_fibex);
        let named_parent = el_fibex_cond.named_parent().unwrap().unwrap();
        assert_eq!(named_parent, el_system);

        let named_parent = el_system.named_parent().unwrap().unwrap();
        assert_eq!(named_parent, el_ar_package);

        let named_parent = el_autosar.named_parent().unwrap();
        assert!(named_parent.is_none());

        // trying to get the named parent of a removed element should fail
        el_autosar.remove_sub_element(el_ar_packages).unwrap();
        assert!(el_ar_package.named_parent().is_err());
    }

    #[test]
    fn package() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Package")
            .unwrap();
        let el_ar_packages_2 = el_ar_package.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package_2 = el_ar_packages_2
            .create_named_sub_element(ElementName::ArPackage, "SubPackage")
            .unwrap();

        assert_eq!(el_ar_package.package().unwrap(), None); // top level package has no parent package
        assert_eq!(el_ar_packages_2.package().unwrap().unwrap(), el_ar_package);
        assert_eq!(el_ar_package_2.package().unwrap().unwrap(), el_ar_package); // SubPackage -> Package

        drop(el_autosar);
        drop(model);
        assert!(el_ar_package.package().is_err()); // the model is dropped, so the package cannot be accessed anymore
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
        let el_autosar_locked = el_autosar.0.write();
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
            .and_then(|el| el.set_character_data("/invalid"))
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
    fn element_self_reference_loop() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg")
            .unwrap();

        // removing an element from itself should return an error and should not deadlock
        let result = el_ar_package.remove_sub_element(el_ar_package.clone());
        assert!(matches!(result, Err(AutosarDataError::ElementNotFound { .. })));

        // moving an element into itself should return an error and should not deadlock
        let result = el_ar_packages.move_element_here(&el_ar_packages.clone());
        assert!(matches!(result, Err(AutosarDataError::ForbiddenMoveToSubElement)));
        let result = el_ar_packages.move_element_here_at(&el_ar_packages.clone(), 0);
        assert!(matches!(result, Err(AutosarDataError::ForbiddenMoveToSubElement)));
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
            .and_then(|el| el.set_character_data("/invalid"))
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
        assert_eq!(model.0.read().identifiables.len(), 0);
        assert!(result.is_ok());

        // alternative: remove_sub_element_kind
        el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "SecondPackage")
            .unwrap();
        assert_eq!(el_ar_packages.content_item_count(), 1);
        let result = el_ar_packages.remove_sub_element_kind(ElementName::ArPackage);
        assert!(result.is_ok());
        assert_eq!(el_ar_packages.content_item_count(), 0);
        let result = el_ar_packages.remove_sub_element_kind(ElementName::ArPackage);
        assert!(result.is_err());

        // the removed element may still exist if there were other references to it, but it is no longer usable
        let result = el_ar_package_clone.parent();
        assert!(matches!(result, Err(AutosarDataError::ItemDeleted)));
        let result = el_ar_package_clone.model();
        assert!(matches!(result, Err(AutosarDataError::ItemDeleted)));
        assert_eq!(el_ar_package_clone.position(), None);
    }

    #[test]
    fn reorder_sub_elements() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let el_a = el_packages
            .create_named_sub_element(ElementName::ArPackage, "a")
            .unwrap();
        let el_b = el_packages
            .create_named_sub_element(ElementName::ArPackage, "b")
            .unwrap();
        let el_c = el_packages
            .create_named_sub_element(ElementName::ArPackage, "c")
            .unwrap();

        // a permutation of all children is applied
        assert!(el_packages.reorder_sub_elements(&[el_c.clone(), el_a.clone(), el_b.clone()]));
        let names: Vec<String> = el_packages.sub_elements().filter_map(|e| e.item_name()).collect();
        assert_eq!(names, ["c", "a", "b"]);

        // repeating the current order is not a change
        assert!(!el_packages.reorder_sub_elements(&[el_c.clone(), el_a.clone(), el_b.clone()]));

        // anything which is not exactly a permutation of the children is rejected, and changes nothing
        assert!(!el_packages.reorder_sub_elements(&[el_a.clone(), el_b.clone()]));
        assert!(!el_packages.reorder_sub_elements(&[el_a.clone(), el_a.clone(), el_b.clone()]));
        let el_other = el_packages
            .create_named_sub_element(ElementName::ArPackage, "d")
            .unwrap();
        // a child of a different parent, which is not part of this content at all
        let el_other_parent = el_other
            .create_sub_element(ElementName::Elements)
            .and_then(|elements| elements.create_named_sub_element(ElementName::System, "s"))
            .unwrap();
        // a child that is not part of the permutation
        assert!(!el_packages.reorder_sub_elements(&[el_a.clone(), el_b.clone(), el_c.clone()]));
        // an element that is a child of a different parent is rejected
        assert!(!el_packages.reorder_sub_elements(&[el_a.clone(), el_b.clone(), el_c.clone(), el_other_parent]));
        // a duplicated element is rejected
        assert!(!el_packages.reorder_sub_elements(&[el_a.clone(), el_a.clone(), el_b.clone(), el_c.clone()]));
        assert_eq!(el_packages.content_item_count(), 4);
        let names: Vec<String> = el_packages.sub_elements().filter_map(|e| e.item_name()).collect();
        assert_eq!(names, ["c", "a", "b", "d"]);
    }

    #[test]
    fn reorder_sub_elements_keeps_mixed_content() {
        // L-4 is a mixed content element: its character data sits between the sub elements, and a
        // reorder may not drop it
        let filebuf = br#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>Pkg</SHORT-NAME>
      <LONG-NAME>
        <L-4 L="FOR-ALL">head<E>aa</E><E>bb</E>tail</L-4>
      </LONG-NAME>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;
        let model = AutosarModel::new();
        model.load_buffer(filebuf, "test", true).unwrap();
        let el_l4 = model
            .get_element_by_path("/Pkg")
            .and_then(|pkg| pkg.get_sub_element(ElementName::LongName))
            .and_then(|ln| ln.get_sub_element(ElementName::L4))
            .unwrap();
        assert_eq!(el_l4.content_item_count(), 4);

        let mut reversed: Vec<Element> = el_l4.sub_elements().collect();
        reversed.reverse();
        assert!(el_l4.reorder_sub_elements(&reversed));

        // both text items are still there, in their positions, and only the elements swapped
        let text = el_l4.serialize();
        assert!(
            text.contains("head<E>bb</E><E>aa</E>tail"),
            "unexpected content: {text}"
        );
        assert_eq!(el_l4.content_item_count(), 4);
    }

    #[test]
    fn reorder_sub_elements_checks_schema_order() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let el_package = el_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg")
            .unwrap();
        let el_short_name = el_package.get_sub_element(ElementName::ShortName).unwrap();
        let el_elements = el_package.create_sub_element(ElementName::Elements).unwrap();

        // AR-PACKAGE is a sequence: SHORT-NAME has to stay in front of ELEMENTS. Reordering them into
        // an order the schema does not allow is rejected, exactly as move_element_here_at rejects it
        assert!(!el_package.reorder_sub_elements(&[el_elements.clone(), el_short_name.clone()]));
        assert_eq!(el_package.get_sub_element_at(0).unwrap(), el_short_name);

        // AR-PACKAGES is a bag: any order is allowed there
        let mut reversed: Vec<Element> = el_packages.sub_elements().collect();
        reversed.reverse();
        assert_eq!(reversed.len(), 1);
        assert!(!el_packages.reorder_sub_elements(&reversed));
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
        assert!(
            el_autosar
                .set_attribute(AttributeName::Dest, CharacterData::String(String::from("something")))
                .is_err()
        );

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
        assert!(
            el_ar_packages
                .set_attribute_string(AttributeName::xmlns, "abc")
                .is_err()
        );

        // directly return an attribute as a string
        let xmlns = el_autosar
            .attribute_value(AttributeName::xmlns)
            .map(|cdata| cdata.to_string())
            .unwrap();
        assert_eq!(xmlns, "http://autosar.org/schema/r4.0".to_string());

        // attribute operation fails when a parent element is locked for writing
        let lock = el_autosar.0.write();
        assert!(
            el_ar_packages
                .set_attribute(AttributeName::Uuid, CharacterData::String(String::from("1234")))
                .is_err()
        );
        assert!(
            el_ar_packages
                .set_attribute_string(AttributeName::Uuid, "1234")
                .is_err()
        );
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

        // define a REFERENCE-BASE and then set a relative reference via BASE
        let el_reference_bases = el_ar_package.create_sub_element(ElementName::ReferenceBases).unwrap();
        let el_reference_base = el_reference_bases
            .create_sub_element(ElementName::ReferenceBase)
            .unwrap();
        el_reference_base
            .create_sub_element(ElementName::ShortLabel)
            .and_then(|short_label| short_label.set_character_data("default"))
            .unwrap();
        el_reference_base
            .create_sub_element(ElementName::PackageRef)
            .and_then(|package_ref| package_ref.set_reference_target(&el_ar_package))
            .unwrap();
        assert_eq!(
            el_reference_base.resolve_reference_base("default").as_deref(),
            Some("/Package")
        );

        el_fibex_element_ref
            .set_relative_reference_target(&el_ecu_instance2, "default")
            .unwrap();
        assert_eq!(
            el_fibex_element_ref
                .attribute_value(AttributeName::Base)
                .and_then(|cdata| cdata.string_value())
                .unwrap(),
            "default"
        );
        assert_eq!(
            el_fibex_element_ref
                .character_data()
                .and_then(|cdata| cdata.string_value())
                .unwrap(),
            "EcuInstance2"
        );
        assert_eq!(el_fibex_element_ref.get_reference_target().unwrap(), el_ecu_instance2);
        assert!(
            model
                .get_references_to("/Package/EcuInstance2")
                .contains(&el_fibex_element_ref.downgrade())
        );

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
        // everything up to this point went through the public API, so the caches must be consistent
        assert_eq!(model.verify_reference_caches(), Ok(()));
        // invalid reference: no DEST attribute.
        // DEST is a required attribute, so remove_attribute() refuses to remove it and the test has to
        // reach into the element instead. This also drops the BASE attribute, which desynchronizes the
        // reference caches by design - so verify_reference_caches() must not be called after this.
        el_fibex_element_ref.0.write().attributes.clear(); // remove the DEST attribute
        assert!(el_fibex_element_ref.get_reference_target().is_err());
        el_fibex_element_ref.set_reference_target(&el_ecu_instance2).unwrap();
        // invalid reference: bad reference string
        el_fibex_element_ref
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::EcuInstance))
            .unwrap();
        el_fibex_element_ref.set_character_data("/does/not/exist").unwrap();
        assert!(el_fibex_element_ref.get_reference_target().is_err());
        // invalid reference: refers to the wrong type of element
        el_fibex_element_ref.set_character_data("/Package").unwrap();
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
    fn relative_reference_outside_package() {
        let model = AutosarModel::new();
        model.create_file("test.arxml", AutosarVersion::Autosar_00050).unwrap();
        let el_autosar = model.root_element();
        let el_ar_package = el_autosar
            .create_sub_element(ElementName::ArPackages)
            .and_then(|arpkgs| arpkgs.create_named_sub_element(ElementName::ArPackage, "Package"))
            .unwrap();

        // create a reference element that is not inside any package: AUTOSAR > ADMIN-DATA > SDGS > SDG > SDX-REF
        let el_sdx_ref = el_autosar
            .create_sub_element(ElementName::AdminData)
            .and_then(|admin_data| admin_data.create_sub_element(ElementName::Sdgs))
            .and_then(|sdgs| sdgs.create_sub_element(ElementName::Sdg))
            .and_then(|sdg| sdg.create_sub_element(ElementName::SdxRef))
            .unwrap();
        assert!(el_sdx_ref.is_reference());
        assert!(el_sdx_ref.package().unwrap().is_none());

        // setting a relative reference target must fail (not panic), since there is no
        // containing package against which the base label could be resolved
        let result = el_sdx_ref.set_relative_reference_target(&el_ar_package, "default");
        assert!(matches!(result, Err(AutosarDataError::InvalidReferenceBase)));

        // getting the reference target of a relative reference outside a package must also fail instead of panicking
        el_sdx_ref.set_character_data("Package").unwrap();
        el_sdx_ref
            .set_attribute(AttributeName::Base, CharacterData::String("default".to_string()))
            .unwrap();
        let result = el_sdx_ref.get_reference_target();
        assert!(matches!(result, Err(AutosarDataError::InvalidReference)));
        assert_eq!(model.verify_reference_caches(), Ok(()));
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
        assert!(
            el_pnc_vector_length
                .set_character_data(CharacterData::String("2".to_string()))
                .is_ok()
        ); // "native" type is String, without automatic wrapping
        assert!(el_pnc_vector_length.set_character_data("2".to_string()).is_ok()); // "native" type is String
        assert!(el_pnc_vector_length.set_character_data("2").is_ok()); // automatic conversion: &str -> String
        assert!(el_pnc_vector_length.set_character_data(2).is_ok()); // automatic conversion: u64 -> String

        // set a new SHORT-NAME, this also updates path cache
        assert!(
            el_short_name
                .set_character_data(CharacterData::String("PackageRenamed".to_string()))
                .is_ok()
        );
        assert_eq!(
            el_short_name.character_data().unwrap().string_value().unwrap(),
            "PackageRenamed"
        );
        model.get_element_by_path("/PackageRenamed").unwrap();

        // set a new reference target, which creates an entry in the reference origin cache
        assert!(
            el_fibex_element_ref
                .set_character_data("/PackageRenamed/EcuInstance1")
                .is_ok()
        );
        model
            .0
            .read()
            .reference_origins
            .get("/PackageRenamed/EcuInstance1")
            .unwrap();

        // modify the reference target, which updates the entry in the reference origin cache
        assert!(
            el_fibex_element_ref
                .set_character_data("/PackageRenamed/EcuInstance2")
                .is_ok()
        );
        model
            .0
            .read()
            .reference_origins
            .get("/PackageRenamed/EcuInstance2")
            .unwrap();
        assert!(
            !model
                .0
                .read()
                .reference_origins
                .contains_key("/PackageRenamed/EcuInstance1")
        );

        // can only set character data that are specified with ContentMode::Characters
        assert!(el_autosar.set_character_data("text").is_err());

        // can't set a value that doesn't match the target spec
        assert!(el_short_name.set_character_data(0).is_err());
        assert!(el_short_name.set_character_data("").is_err());

        // remove character data
        assert!(el_pnc_vector_length.remove_character_data().is_ok());

        // remove the character data of a reference
        assert!(el_fibex_element_ref.remove_character_data().is_ok());
        assert!(
            !model
                .0
                .read()
                .reference_origins
                .contains_key("/PackageRenamed/EcuInstance2")
        );

        // remove on an element whose character data has already been removed is not an error
        assert!(el_fibex_element_ref.remove_character_data().is_ok());

        // can't remove SHORT-NAME
        assert!(el_short_name.remove_character_data().is_err());

        // can't remove from elements which do not contain character data
        assert!(el_autosar.remove_character_data().is_err());

        // slightly different behavior for the internal version that is used for locked elements
        assert!(
            el_autosar
                .0
                .write()
                .set_character_data(0, AutosarVersion::Autosar_00050)
                .is_err()
        );
        assert!(
            el_fibex_element_ref
                .0
                .write()
                .set_character_data(0, AutosarVersion::Autosar_00050)
                .is_err()
        );

        // operation fails if the model is needed (e.g. reference or short name update), but the model has been deleted
        el_fibex_element_ref
            .set_character_data("/PackageRenamed/EcuInstance2")
            .unwrap();
        drop(model);
        assert!(
            el_fibex_element_ref
                .set_character_data("/PackageRenamed/EcuInstance1")
                .is_err()
        );
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
<!--comment-->
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
        el_autosar.set_comment(Some("comment".to_string()));

        let mut outstring = String::from(r#"<?xml version="1.0" encoding="utf-8"?>"#);
        el_autosar
            .0
            .read()
            .serialize_internal(&mut outstring, 0, false, &None, None)
            .unwrap();

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
            .load_buffer(FILEBUF.as_bytes(), OsString::from("test"), true)
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
        assert!(
            el_short_name
                .0
                .read()
                .calc_element_insert_range(ElementName::Desc, AutosarVersion::Autosar_00050)
                .is_err()
        );

        // find_element_insert_pos fails to find a place for a sequence element with multiplicity 0-1
        assert!(
            el_autosar
                .0
                .read()
                .calc_element_insert_range(ElementName::ArPackages, AutosarVersion::Autosar_00050)
                .is_err()
        );
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
        el_defref1.set_character_data("/DefRef_999").unwrap();
        // second bsw value
        let el_value2 = el_paramvalues
            .create_sub_element(ElementName::EcucNumericalParamValue)
            .unwrap();
        let el_defref2 = el_value2.create_sub_element(ElementName::DefinitionRef).unwrap();
        el_defref2
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::EcucBooleanParamDef))
            .unwrap();
        el_defref2.set_character_data("/DefRef_111").unwrap();
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
        el_fibex_element_ref1.set_character_data("/ZZZZZ").unwrap();
        let el_fibex_element2 = el_fibex_elements
            .create_sub_element(ElementName::FibexElementRefConditional)
            .unwrap();
        let el_fibex_element_ref2 = el_fibex_element2
            .create_sub_element(ElementName::FibexElementRef)
            .unwrap();
        el_fibex_element_ref2
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::ISignal))
            .unwrap();
        el_fibex_element_ref2.set_character_data("/AAAAA").unwrap();

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

        // validate that the misc elements (FIBEX-ELEMENT-REF-CONDITIONAL) have been sorted
        let mut iter = el_fibex_elements.sub_elements();
        let item1 = iter.next().unwrap();
        let item2 = iter.next().unwrap();
        assert_eq!(item1, el_fibex_element2);
        assert_eq!(item2, el_fibex_element1);
    }

    fn helper_create_bsw_subelem(
        el_subcontainers: &Element,
        short_name: &str,
        defref: &str,
    ) -> Result<Element, AutosarDataError> {
        let e = el_subcontainers.create_named_sub_element(ElementName::EcucContainerValue, short_name)?;
        let defrefelem = e.create_sub_element(ElementName::DefinitionRef)?;
        defrefelem.set_character_data(defref)?;
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
        indexelem.set_character_data(indexstr)?;
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

        let fm: HashSet<WeakArxmlFile> = [file1.downgrade()].iter().cloned().collect();
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
    fn remove_from_last_file() {
        let model = AutosarModel::new();
        let file = model.create_file("test.arxml", AutosarVersion::LATEST).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg")
            .unwrap();

        // removing the root element from the only file is forbidden, since the root element cannot be deleted
        let result = el_autosar.remove_from_file(&file);
        assert!(matches!(result, Err(AutosarDataError::RootElementRemovalForbidden)));
        // the model remains intact and usable
        assert_eq!(model.files().count(), 1);
        assert!(el_autosar.min_version().is_ok());
        assert!(file.serialize().is_ok());

        // CHAPTER is both identifiable and splittable, so remove_from_file can be called for its
        // SHORT-NAME sub element, but deleting a SHORT-NAME is forbidden and the failure must be reported
        let el_chapter = el_ar_package
            .create_sub_element(ElementName::Elements)
            .and_then(|elements| elements.create_named_sub_element(ElementName::Documentation, "Doc"))
            .and_then(|doc| doc.create_sub_element(ElementName::DocumentationContent))
            .and_then(|content| content.create_named_sub_element(ElementName::Chapter, "Chap"))
            .unwrap();
        let el_short_name = el_chapter.get_sub_element(ElementName::ShortName).unwrap();
        let result = el_short_name.remove_from_file(&file);
        assert!(matches!(result, Err(AutosarDataError::ShortNameRemovalForbidden)));
        // the SHORT-NAME element is unchanged and still part of the file
        assert_eq!(el_chapter.item_name().unwrap(), "Chap");
        assert!(!el_short_name.file_membership().unwrap().1.is_empty());

        // an ordinary element is deleted when it is removed from the only file that contains it
        el_ar_package.remove_from_file(&file).unwrap();
        assert!(model.get_element_by_path("/Pkg").is_none());
    }

    #[test]
    fn comment() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_autosar = model.root_element();

        // initially there is no comment
        assert!(el_autosar.comment().is_none());

        // set and get a comment
        el_autosar.set_comment(Some("comment".to_string()));
        assert_eq!(el_autosar.comment().unwrap(), "comment");

        // set a new comment containing "--" which is a forbidden sequence in XML comments
        el_autosar.set_comment(Some("comment--".to_string()));
        assert_eq!(el_autosar.comment().unwrap(), "comment__");

        // remove the comment
        el_autosar.set_comment(None);
        assert!(el_autosar.comment().is_none());
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

        #[allow(clippy::mutable_key_type)]
        let mut hs = HashSet::new();
        hs.insert(el_autosar);
        hs.insert(el_ar_packages);
        // can't insert el_autosar_second_ref, it is already in the set
        assert!(!hs.insert(el_autosar_second_ref));
        assert_eq!(hs.len(), 2);

        let mut hs2 = HashSet::new();
        hs2.insert(weak1);
        assert!(!hs2.insert(weak2));
        assert_eq!(hs2.len(), 1);

        // traits of elementcontent
        let ec_elem = ElementContent::Element(model.root_element());
        assert_eq!(format!("{:?}", model.root_element()), format!("{ec_elem:?}"));
        assert_eq!(ec_elem.unwrap_element(), Some(model.root_element()));
        assert!(ec_elem.unwrap_cdata().is_none());
        let cdata = CharacterData::String("test".to_string());
        let ec_chars = ElementContent::CharacterData(cdata.clone());
        assert_eq!(format!("{cdata:?}"), format!("{ec_chars:?}"));
        assert_eq!(ec_chars.unwrap_cdata(), Some(cdata));
        assert!(ec_chars.unwrap_element().is_none());
    }

    #[test]
    fn element_order() {
        let model = AutosarModel::new();
        let _file = model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_autosar = model.root_element();
        let el_elements = el_autosar
            .create_sub_element(ElementName::ArPackages)
            .unwrap()
            .create_named_sub_element(ElementName::ArPackage, "pkg")
            .unwrap()
            .create_sub_element(ElementName::Elements)
            .unwrap();
        let el_system = el_elements
            .create_named_sub_element(ElementName::System, "sys")
            .unwrap();
        let fibex_elements = el_system.create_sub_element(ElementName::FibexElements).unwrap();

        let item1 = el_elements
            .create_named_sub_element(ElementName::ApplicationPrimitiveDataType, "adt_2")
            .unwrap();
        let item2 = el_elements
            .create_named_sub_element(ElementName::ApplicationPrimitiveDataType, "adt_10")
            .unwrap();
        let item3 = el_elements
            .create_named_sub_element(ElementName::ApplicationArrayDataType, "adt_12")
            .unwrap();
        // items 1 and 2 are sorted after separating the index from the name, so 10 comes after 2
        assert!(item1 < item2);
        // items 2 and 3 are sorted by the element type, so in this case the index does not matter
        assert!(item3 < item1);

        let item4 = fibex_elements
            .create_sub_element(ElementName::FibexElementRefConditional)
            .unwrap();
        let item4_ref = item4.create_sub_element(ElementName::FibexElementRef).unwrap();
        item4_ref
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::ISignal))
            .unwrap();
        item4_ref.set_character_data("/aaa").unwrap();

        let item5 = fibex_elements
            .create_sub_element(ElementName::FibexElementRefConditional)
            .unwrap();
        let item5_ref = item5.create_sub_element(ElementName::FibexElementRef).unwrap();
        item5_ref
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::ISignal))
            .unwrap();
        item5_ref.set_character_data("/bbb").unwrap();

        // items 4 and 5 are sorted by the character data of the reference
        assert!(item4 < item5);

        let item6 = fibex_elements
            .create_sub_element(ElementName::FibexElementRefConditional)
            .unwrap();
        let item6_ref = item6.create_sub_element(ElementName::FibexElementRef).unwrap();
        item6_ref
            .set_attribute(AttributeName::Dest, CharacterData::Enum(EnumItem::EcuInstance))
            .unwrap();

        // items 4 and 6 are sorted by the DEST attribute of the reference
        assert!(item6 < item4);

        let item7 = fibex_elements
            .create_sub_element(ElementName::FibexElementRefConditional)
            .unwrap();
        item7.create_sub_element(ElementName::FibexElementRef).unwrap();

        // item7 is incomplete, lacking the DEST attribute so it is sorted last
        assert!(item7 > item6);
        assert!(item6 < item7);
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
        let (_, _) = model.load_buffer(FILEBUF, "test1", true).unwrap();
        let root_elem = model.root_element();
        let ar_packages_elem = root_elem.get_sub_element(ElementName::ArPackages).unwrap();
        let root_all_count = root_elem.elements_dfs().count();
        let ar_packages_all_count = ar_packages_elem.elements_dfs().count();
        assert_eq!(root_all_count, ar_packages_all_count + 1);

        let root_lvl3_count = root_elem.elements_dfs_with_max_depth(3).count();
        let ar_packages_lvl2_count = ar_packages_elem.elements_dfs_with_max_depth(2).count();
        assert_eq!(root_lvl3_count, ar_packages_lvl2_count + 1);

        root_elem
            .elements_dfs_with_max_depth(3)
            .skip(1)
            .zip(ar_packages_elem.elements_dfs_with_max_depth(2))
            .for_each(|((_, x), (_, y))| assert_eq!(x, y));

        for elem in ar_packages_elem.elements_dfs_with_max_depth(2) {
            assert!(elem.0 <= 2);
        }
    }

    #[test]
    fn parse_reference_bases() {
        // from issue #36
        const FILEBUF: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00048.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>My</SHORT-NAME>
      <AR-PACKAGES>
        <AR-PACKAGE>
          <SHORT-NAME>Lib</SHORT-NAME>
          <AR-PACKAGES>
            <AR-PACKAGE>
              <SHORT-NAME>Hierarchy</SHORT-NAME>
              <AR-PACKAGES>
                <AR-PACKAGE>
                  <SHORT-NAME>Units</SHORT-NAME>
                  <ELEMENTS>
                    <UNIT>
                      <SHORT-NAME>MyUnit</SHORT-NAME>
                      <FACTOR-SI-TO-UNIT>1</FACTOR-SI-TO-UNIT>
                      <OFFSET-SI-TO-UNIT>0</OFFSET-SI-TO-UNIT>
                    </UNIT>
                  </ELEMENTS>
                </AR-PACKAGE>
              </AR-PACKAGES>
            </AR-PACKAGE>
          </AR-PACKAGES>
        </AR-PACKAGE>
        <AR-PACKAGE>
          <SHORT-NAME>SWC</SHORT-NAME>
          <REFERENCE-BASES>
            <REFERENCE-BASE>
              <SHORT-LABEL>Units</SHORT-LABEL>
              <PACKAGE-REF DEST="AR-PACKAGE">/My/Lib/Hierarchy/Units</PACKAGE-REF>
            </REFERENCE-BASE>
          </REFERENCE-BASES>
          <AR-PACKAGES>
            <AR-PACKAGE>
              <SHORT-NAME>ApplicationDataTypes</SHORT-NAME>
              <ELEMENTS>
                <APPLICATION-PRIMITIVE-DATA-TYPE>
                  <SHORT-NAME>MyDataType</SHORT-NAME>
                  <CATEGORY>VALUE</CATEGORY>
                  <SW-DATA-DEF-PROPS>
                    <SW-DATA-DEF-PROPS-VARIANTS>
                      <SW-DATA-DEF-PROPS-CONDITIONAL>
                        <SW-CALIBRATION-ACCESS>NOT-ACCESSIBLE</SW-CALIBRATION-ACCESS>
                        <UNIT-REF DEST="UNIT" BASE="Units">MyUnit</UNIT-REF>
                      </SW-DATA-DEF-PROPS-CONDITIONAL>
                    </SW-DATA-DEF-PROPS-VARIANTS>
                  </SW-DATA-DEF-PROPS>
                </APPLICATION-PRIMITIVE-DATA-TYPE>
              </ELEMENTS>
            </AR-PACKAGE>
          </AR-PACKAGES>
        </AR-PACKAGE>
      </AR-PACKAGES>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#.as_bytes();
        let model = AutosarModel::new();
        let (_, _) = model.load_buffer(FILEBUF, "test1", true).unwrap();

        let unit_elem = model.get_element_by_path("/My/Lib/Hierarchy/Units/MyUnit").unwrap();
        assert_eq!(unit_elem.element_name(), ElementName::Unit);

        // verify that we're correctly tracking incoming references to the unit element from the reference base
        let refs_to_unit = model.get_references_to(&unit_elem.path().unwrap());
        assert_eq!(refs_to_unit.len(), 1);
        let reference_elem = refs_to_unit[0].upgrade().unwrap();
        assert_eq!(reference_elem.element_name(), ElementName::UnitRef);
        assert!(reference_elem.attribute_value(AttributeName::Base).is_some());

        // verify that the reference base has been parsed correctly
        assert_eq!(
            reference_elem.resolve_reference_base("Units").as_deref(),
            Some("/My/Lib/Hierarchy/Units")
        );

        // verify that we can navigate from the reference element to the target unit element
        let target = reference_elem.get_reference_target().unwrap();
        assert_eq!(target, unit_elem);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn modify_relative_ref() {
        const FILEBUF: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00048.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>First</SHORT-NAME>
      <AR-PACKAGES>
        <AR-PACKAGE>
          <SHORT-NAME>Second</SHORT-NAME>
          <ELEMENTS>
            <ECU-INSTANCE>
              <SHORT-NAME>Ecu</SHORT-NAME>
            </ECU-INSTANCE>
          </ELEMENTS>
        </AR-PACKAGE>
      </AR-PACKAGES>
    </AR-PACKAGE>
    <AR-PACKAGE>
      <SHORT-NAME>Ref</SHORT-NAME>
      <REFERENCE-BASES>
        <REFERENCE-BASE>
          <SHORT-LABEL>First</SHORT-LABEL>
          <PACKAGE-REF DEST="AR-PACKAGE">/First</PACKAGE-REF>
        </REFERENCE-BASE>
        <REFERENCE-BASE>
          <SHORT-LABEL>Second</SHORT-LABEL>
          <PACKAGE-REF DEST="AR-PACKAGE">/First/Second</PACKAGE-REF>
        </REFERENCE-BASE>
      </REFERENCE-BASES>
      <ELEMENTS>
        <SYSTEM>
          <SHORT-NAME>System</SHORT-NAME>
          <FIBEX-ELEMENTS>
            <FIBEX-ELEMENT-REF-CONDITIONAL>
              <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE" BASE="First">Second/Ecu</FIBEX-ELEMENT-REF>
            </FIBEX-ELEMENT-REF-CONDITIONAL>
          </FIBEX-ELEMENTS>
        </SYSTEM>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#.as_bytes();
        let model = AutosarModel::new();
        let (_, _) = model.load_buffer(FILEBUF, "test1", true).unwrap();

        let ref_elem = model
            .get_element_by_path("/Ref/System")
            .unwrap()
            .get_sub_element(ElementName::FibexElements)
            .unwrap()
            .get_sub_element(ElementName::FibexElementRefConditional)
            .unwrap()
            .get_sub_element(ElementName::FibexElementRef)
            .unwrap();
        let ecu_elem = ref_elem.get_reference_target().unwrap();
        assert_eq!(ecu_elem.element_name(), ElementName::EcuInstance);
        assert_eq!(model.get_references_to(&ecu_elem.path().unwrap()).len(), 1);

        ref_elem.set_relative_reference_target(&ecu_elem, "Second").unwrap();
        assert_eq!(ref_elem.character_data().unwrap().string_value().unwrap(), "Ecu");
        assert_eq!(model.get_references_to(&ecu_elem.path().unwrap()).len(), 1);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn create_modify_delete_ref_bases() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg")
            .unwrap();
        let el_ar_package2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Pkg2")
            .unwrap();
        let el_reference_bases = el_ar_package.create_sub_element(ElementName::ReferenceBases).unwrap();
        let el_reference_base = el_reference_bases
            .create_sub_element(ElementName::ReferenceBase)
            .unwrap();
        // the reference base is unusable, because label and packageref are missing
        assert_eq!(el_reference_base.resolve_reference_base("Units"), None);
        let el_short_label = el_reference_base.create_sub_element(ElementName::ShortLabel).unwrap();
        el_short_label.set_character_data("Units").unwrap();
        // still unusable, because packageref is missing
        assert_eq!(el_reference_base.resolve_reference_base("Units"), None);
        let el_package_ref = el_reference_base.create_sub_element(ElementName::PackageRef).unwrap();
        el_package_ref.set_reference_target(&el_ar_package2).unwrap();
        // the declaration is complete now, so the base can be resolved
        assert_eq!(
            el_reference_base.resolve_reference_base("Units").as_deref(),
            Some("/Pkg2")
        );

        el_short_label.set_character_data("Modified").unwrap();
        // the base is now declared under the new label
        assert_eq!(el_reference_base.resolve_reference_base("Units"), None);
        assert_eq!(
            el_reference_base.resolve_reference_base("Modified").as_deref(),
            Some("/Pkg2")
        );

        el_package_ref.set_reference_target(&el_ar_package).unwrap();
        // the base points at the new package
        assert_eq!(
            el_reference_base.resolve_reference_base("Modified").as_deref(),
            Some("/Pkg")
        );

        el_reference_base.remove_sub_element(el_package_ref).unwrap();
        // unusable again, because the packageref is gone
        assert_eq!(el_reference_base.resolve_reference_base("Modified"), None);
        // create it again
        let el_package_ref = el_reference_base.create_sub_element(ElementName::PackageRef).unwrap();
        el_package_ref.set_reference_target(&el_ar_package2).unwrap();
        assert_eq!(
            el_reference_base.resolve_reference_base("Modified").as_deref(),
            Some("/Pkg2")
        );
        // remove the SHORT-LABEL
        el_reference_base.remove_sub_element(el_short_label).unwrap();
        // unusable again, because the label is missing
        assert_eq!(el_reference_base.resolve_reference_base("Modified"), None);
        let el_short_label = el_reference_base.create_sub_element(ElementName::ShortLabel).unwrap();
        el_short_label.set_character_data("Units").unwrap();
        // usable again, under the label it was just given
        assert_eq!(
            el_reference_base.resolve_reference_base("Units").as_deref(),
            Some("/Pkg2")
        );

        el_reference_bases
            .remove_sub_element(el_reference_base.clone())
            .unwrap();
        // the declaration is gone with the element
        assert_eq!(el_reference_base.resolve_reference_base("Units"), None);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn rename_reference_base() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();

        let el_ar_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let owner1 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Owner1")
            .unwrap();
        let owner2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Owner2")
            .unwrap();
        let target1 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Target1")
            .unwrap();
        let target2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Target2")
            .unwrap();

        let short_label_1 = owner1
            .create_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.create_sub_element(ElementName::ReferenceBase))
            .and_then(|e| {
                e.create_sub_element(ElementName::ShortLabel)
                    .and_then(|l| l.set_character_data("Shared").map(|_| l))
            })
            .unwrap();
        let owner1_reference_base = owner1
            .get_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.get_sub_element(ElementName::ReferenceBase))
            .unwrap();
        owner1_reference_base
            .create_sub_element(ElementName::PackageRef)
            .and_then(|p| p.set_reference_target(&target1))
            .unwrap();

        owner2
            .create_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.create_sub_element(ElementName::ReferenceBase))
            .and_then(|e| {
                e.create_sub_element(ElementName::ShortLabel)
                    .and_then(|l| l.set_character_data("Shared").map(|_| l))
            })
            .unwrap();
        let owner2_reference_base = owner2
            .get_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.get_sub_element(ElementName::ReferenceBase))
            .unwrap();
        owner2_reference_base
            .create_sub_element(ElementName::PackageRef)
            .and_then(|p| p.set_reference_target(&target2))
            .unwrap();

        // both packages declare "Shared", and each one sees only its own declaration
        assert_eq!(
            short_label_1.resolve_reference_base("Shared").as_deref(),
            Some("/Target1")
        );
        assert_eq!(
            owner2_reference_base.resolve_reference_base("Shared").as_deref(),
            Some("/Target2")
        );

        short_label_1.set_character_data("Renamed").unwrap();

        // only the declaration in /Owner1 was renamed
        assert_eq!(short_label_1.resolve_reference_base("Shared"), None);
        assert_eq!(
            short_label_1.resolve_reference_base("Renamed").as_deref(),
            Some("/Target1")
        );
        assert_eq!(
            owner2_reference_base.resolve_reference_base("Shared").as_deref(),
            Some("/Target2")
        );
        assert_eq!(owner2_reference_base.resolve_reference_base("Renamed"), None);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn changing_base_attribute() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();

        let el_ar_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let base_a = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "BaseA")
            .unwrap();
        let base_b = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "BaseB")
            .unwrap();
        let ref_pkg = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "RefPkg")
            .unwrap();

        let ecu_a = base_a
            .create_sub_element(ElementName::Elements)
            .and_then(|e| e.create_named_sub_element(ElementName::EcuInstance, "Ecu"))
            .unwrap();
        let _ecu_b = base_b
            .create_sub_element(ElementName::Elements)
            .and_then(|e| e.create_named_sub_element(ElementName::EcuInstance, "Ecu"))
            .unwrap();

        let ref_bases = ref_pkg.create_sub_element(ElementName::ReferenceBases).unwrap();
        let rb_a = ref_bases.create_sub_element(ElementName::ReferenceBase).unwrap();
        rb_a.create_sub_element(ElementName::ShortLabel)
            .and_then(|e| e.set_character_data("A"))
            .unwrap();
        rb_a.create_sub_element(ElementName::PackageRef)
            .and_then(|e| e.set_reference_target(&base_a))
            .unwrap();

        let rb_b = ref_bases.create_sub_element(ElementName::ReferenceBase).unwrap();
        rb_b.create_sub_element(ElementName::ShortLabel)
            .and_then(|e| e.set_character_data("B"))
            .unwrap();
        rb_b.create_sub_element(ElementName::PackageRef)
            .and_then(|e| e.set_reference_target(&base_b))
            .unwrap();

        let ref_elem = ref_pkg
            .create_sub_element(ElementName::Elements)
            .and_then(|e| e.create_named_sub_element(ElementName::System, "Sys"))
            .and_then(|e| e.create_sub_element(ElementName::FibexElements))
            .and_then(|e| e.create_sub_element(ElementName::FibexElementRefConditional))
            .and_then(|e| e.create_sub_element(ElementName::FibexElementRef))
            .unwrap();
        ref_elem.set_relative_reference_target(&ecu_a, "A").unwrap();

        ref_elem.set_attribute_string(AttributeName::Base, "B").unwrap();

        // switching the BASE attribute re-resolves the reference: "Ecu" relative to base "B" is
        // /BaseB/Ecu, so that is the target path it is now registered under
        assert_eq!(
            model.relative_reference_target(&ref_elem.downgrade()).as_deref(),
            Some("/BaseB/Ecu")
        );
        assert_eq!(model.get_references_to("/BaseB/Ecu").len(), 1);
        assert!(model.get_references_to("/BaseA/Ecu").is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    // helper for the reference base removal tests: create a REFERENCE-BASE with the given label
    // inside owner_package, pointing at target_package
    fn create_reference_base(owner_package: &Element, label: &str, target_package: &Element) {
        let el_reference_base = owner_package
            .get_or_create_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.create_sub_element(ElementName::ReferenceBase))
            .unwrap();
        el_reference_base
            .create_sub_element(ElementName::ShortLabel)
            .and_then(|e| e.set_character_data(label))
            .unwrap();
        el_reference_base
            .create_sub_element(ElementName::PackageRef)
            .and_then(|e| e.set_reference_target(target_package))
            .unwrap();
    }

    // helper for the reference base removal tests: build a model containing the package
    // "/Outer/Owner", which declares two reference bases pointing at "/Target"
    fn build_reference_base_model() -> (AutosarModel, Element, Element) {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_ar_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let el_target = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Target")
            .unwrap();
        // the owner package is nested inside another package, so that the tests also verify that the
        // full path of the owner package is used, and not just the name of the innermost package
        let el_owner = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Outer")
            .and_then(|e| e.create_sub_element(ElementName::ArPackages))
            .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Owner"))
            .unwrap();
        create_reference_base(&el_owner, "BaseA", &el_target);
        create_reference_base(&el_owner, "BaseB", &el_target);
        // an element inside the owner package, from which the reference bases it declares are in scope
        let el_inside_owner = el_owner
            .create_sub_element(ElementName::Elements)
            .and_then(|e| e.create_named_sub_element(ElementName::System, "System"))
            .unwrap();

        assert_eq!(
            el_inside_owner.resolve_reference_base("BaseA").as_deref(),
            Some("/Target")
        );
        assert_eq!(
            el_inside_owner.resolve_reference_base("BaseB").as_deref(),
            Some("/Target")
        );

        (model, el_owner, el_inside_owner)
    }

    #[test]
    fn remove_reference_bases_element() {
        // removing the REFERENCE-BASES element removes every reference base declared inside it
        let (model, el_owner, el_inside_owner) = build_reference_base_model();
        let el_reference_bases = el_owner.get_sub_element(ElementName::ReferenceBases).unwrap();
        el_owner.remove_sub_element(el_reference_bases).unwrap();
        assert_eq!(el_inside_owner.resolve_reference_base("BaseA"), None);
        assert_eq!(el_inside_owner.resolve_reference_base("BaseB"), None);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn remove_reference_base_owner_package() {
        // Removing the AR-PACKAGE which owns the reference bases takes the declarations with it. A
        // reference base is only in scope inside its declaring package, so everything which could have
        // used it is removed at the same time.
        let (model, el_owner, el_inside_owner) = build_reference_base_model();
        let el_ar_packages = el_owner.parent().unwrap().unwrap();
        el_ar_packages.remove_sub_element(el_owner).unwrap();
        assert_eq!(el_inside_owner.resolve_reference_base("BaseA"), None);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn remove_reference_base_ancestor() {
        // removing any element above the reference bases takes the declarations with it, no matter how
        // far up the hierarchy it is
        let (model, _el_owner, el_inside_owner) = build_reference_base_model();
        let el_ar_packages = model.root_element().get_sub_element(ElementName::ArPackages).unwrap();
        model.root_element().remove_sub_element(el_ar_packages).unwrap();
        assert_eq!(el_inside_owner.resolve_reference_base("BaseA"), None);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn remove_reference_base_short_label_character_data() {
        // removing the character data of the SHORT-LABEL leaves the REFERENCE-BASE without a label,
        // so it can no longer be used and must be removed from the cache
        let (model, el_owner, el_inside_owner) = build_reference_base_model();
        let el_short_label = el_owner
            .get_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.get_sub_element(ElementName::ReferenceBase))
            .and_then(|e| e.get_sub_element(ElementName::ShortLabel))
            .unwrap();
        assert_eq!(
            el_short_label.character_data().unwrap().string_value().unwrap(),
            "BaseA"
        );
        el_short_label.remove_character_data().unwrap();

        assert_eq!(el_inside_owner.resolve_reference_base("BaseA"), None);
        assert_eq!(
            el_inside_owner.resolve_reference_base("BaseB").as_deref(),
            Some("/Target")
        );
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn remove_reference_base_package_ref_character_data() {
        // removing the character data of the PACKAGE-REF leaves the REFERENCE-BASE without a target,
        // so it can no longer be used
        let (model, el_owner, el_inside_owner) = build_reference_base_model();
        let el_package_ref = el_owner
            .get_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.get_sub_element(ElementName::ReferenceBase))
            .and_then(|e| e.get_sub_element(ElementName::PackageRef))
            .unwrap();
        assert_eq!(
            el_package_ref.character_data().unwrap().string_value().unwrap(),
            "/Target"
        );
        el_package_ref.remove_character_data().unwrap();

        assert_eq!(el_inside_owner.resolve_reference_base("BaseA"), None);
        assert_eq!(
            el_inside_owner.resolve_reference_base("BaseB").as_deref(),
            Some("/Target")
        );
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    // helper for the rename/move tests: get the PACKAGE-REF of the first REFERENCE-BASE of a package
    fn get_package_ref(owner_package: &Element) -> Element {
        owner_package
            .get_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.get_sub_element(ElementName::ReferenceBase))
            .and_then(|e| e.get_sub_element(ElementName::PackageRef))
            .unwrap()
    }

    // helper for the rename/move tests: the path which the reference base used by `reference` resolves
    // to, i.e. the path that the relative content of the reference is interpreted against
    fn resolved_reference_base(reference: &Element) -> Option<String> {
        let base_label = reference
            .attribute_value(AttributeName::Base)
            .and_then(|cdata| cdata.string_value())?;
        reference.resolve_reference_base(&base_label)
    }

    // helper for the rename/move tests: create a package containing an ECU-INSTANCE named "Ecu"
    fn create_target_package(ar_packages: &Element, name: &str) -> (Element, Element) {
        let el_package = ar_packages
            .create_named_sub_element(ElementName::ArPackage, name)
            .unwrap();
        let el_ecu = el_package
            .create_sub_element(ElementName::Elements)
            .and_then(|e| e.create_named_sub_element(ElementName::EcuInstance, "Ecu"))
            .unwrap();
        (el_package, el_ecu)
    }

    // helper for the rename/move tests: create a FIBEX-ELEMENT-REF inside a new package
    fn create_relative_reference(
        ar_packages: &Element,
        package_name: &str,
        target: &Element,
        base_label: &str,
    ) -> Element {
        let el_ref = ar_packages
            .create_named_sub_element(ElementName::ArPackage, package_name)
            .and_then(|e| e.create_sub_element(ElementName::Elements))
            .and_then(|e| e.create_named_sub_element(ElementName::System, "System"))
            .and_then(|e| e.create_sub_element(ElementName::FibexElements))
            .and_then(|e| e.create_sub_element(ElementName::FibexElementRefConditional))
            .and_then(|e| e.create_sub_element(ElementName::FibexElementRef))
            .unwrap();
        el_ref.set_relative_reference_target(target, base_label).unwrap();
        el_ref
    }

    // helper for the rename/move tests: build a model in which the package "/Owner" declares the
    // reference base "Base" pointing at "/Target", and the sub-package "/Owner/Sub" contains a
    // relative reference which uses that base to refer to "/Target/Ecu".
    //
    // returns (model, /Owner, /Target, the relative reference element)
    fn build_relative_reference_model() -> (AutosarModel, Element, Element, Element) {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_ar_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let (el_target, el_ecu) = create_target_package(&el_ar_packages, "Target");
        let el_owner = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Owner")
            .unwrap();
        create_reference_base(&el_owner, "Base", &el_target);
        let el_owner_packages = el_owner.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ref = create_relative_reference(&el_owner_packages, "Sub", &el_ecu, "Base");

        // the reference is written as a relative path, and resolves through the reference base
        assert_eq!(el_ref.character_data().unwrap().string_value().unwrap(), "Ecu");
        assert_eq!(el_ref.get_reference_target().unwrap(), el_ecu);
        assert!(model.check_references().is_empty());

        (model, el_owner, el_target, el_ref)
    }

    #[test]
    fn rename_reference_base_owner_package() {
        // renaming the package which declares a reference base changes the scope in which the
        // reference base is visible, so the cached owner package path must be updated
        let (model, el_owner, _el_target, el_ref) = build_relative_reference_model();
        el_owner.set_item_name("Renamed").unwrap();

        assert_eq!(resolved_reference_base(&el_ref).as_deref(), Some("/Target"));
        // the reference has moved to /Renamed/Sub together with the reference base, so it still resolves
        assert_eq!(el_ref.get_reference_target().unwrap().path().unwrap(), "/Target/Ecu");
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn rename_reference_base_target_package() {
        // renaming the package a reference base points at updates the PACKAGE-REF like any other
        // reference; the cached copy of the PACKAGE-REF must be updated to match
        let (model, el_owner, el_target, el_ref) = build_relative_reference_model();
        el_target.set_item_name("Renamed").unwrap();

        let el_package_ref = get_package_ref(&el_owner);
        assert_eq!(
            el_package_ref.character_data().unwrap().string_value().unwrap(),
            "/Renamed"
        );
        assert_eq!(resolved_reference_base(&el_ref).as_deref(), Some("/Renamed"));
        // the relative reference is unchanged, but now resolves to the renamed target
        assert_eq!(el_ref.character_data().unwrap().string_value().unwrap(), "Ecu");
        assert_eq!(el_ref.get_reference_target().unwrap().path().unwrap(), "/Renamed/Ecu");
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn rename_reference_base_path_boundary() {
        // renaming /Pkg1 must not affect anything belonging to /Pkg10
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_ar_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let (el_pkg1, _) = create_target_package(&el_ar_packages, "Pkg1");
        let (el_pkg10, _) = create_target_package(&el_ar_packages, "Pkg10");
        // both packages declare a reference base, each pointing at the other one
        create_reference_base(&el_pkg1, "Base1", &el_pkg10);
        create_reference_base(&el_pkg10, "Base10", &el_pkg1);

        el_pkg1.set_item_name("Renamed").unwrap();

        // each base is still declared by its own package and points at the other one; the PACKAGE-REF
        // of Base10 followed the rename of /Pkg1, and the one in /Pkg10 was not touched by it
        assert_eq!(
            get_package_ref(&el_pkg1).resolve_reference_base("Base1").as_deref(),
            Some("/Pkg10")
        );
        assert_eq!(get_package_ref(&el_pkg1).resolve_reference_base("Base10"), None);
        assert_eq!(
            get_package_ref(&el_pkg10).resolve_reference_base("Base10").as_deref(),
            Some("/Renamed")
        );
        assert_eq!(get_package_ref(&el_pkg10).resolve_reference_base("Base1"), None);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn rename_short_name_of_reference_base_owner() {
        // Element::set_character_data on a SHORT-NAME renames an element without updating any
        // references to it. A reference base is found through the position of the REFERENCE-BASE in the
        // element tree, so renaming the package which declares it this way keeps it in scope.
        let (model, el_owner, el_target, el_ref) = build_relative_reference_model();
        el_owner
            .get_sub_element(ElementName::ShortName)
            .and_then(|e| e.set_character_data("Renamed").ok())
            .unwrap();

        assert_eq!(resolved_reference_base(&el_ref).as_deref(), Some("/Target"));
        assert_eq!(el_ref.get_reference_target().unwrap().path().unwrap(), "/Target/Ecu");

        // The PACKAGE-REF of the reference base *is* a reference, so renaming its target this way
        // deliberately leaves it pointing at the old path. The cache must agree with the document,
        // which means the reference base - and the relative reference using it - now dangle.
        el_target
            .get_sub_element(ElementName::ShortName)
            .and_then(|e| e.set_character_data("MovedAway").ok())
            .unwrap();

        // the PACKAGE-REF still contains "/Target", which no longer exists
        assert_eq!(resolved_reference_base(&el_ref).as_deref(), Some("/Target"));
        assert!(el_ref.get_reference_target().is_err());
        // both the PACKAGE-REF, which still points at "/Target", and the relative reference which
        // resolves through it are now broken
        assert_eq!(model.check_references().len(), 2);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn move_reference_base_owner_package() {
        // moving the package which declares a reference base must update the cached owner package path
        let (model, el_owner, el_target, el_ref) = build_relative_reference_model();
        el_target
            .create_sub_element(ElementName::ArPackages)
            .and_then(|e| e.move_element_here(&el_owner))
            .unwrap();

        assert_eq!(resolved_reference_base(&el_ref).as_deref(), Some("/Target"));
        // the relative reference moved along with the reference base, so it still resolves
        assert!(model.get_element_by_path("/Target/Owner/Sub/System").is_some());
        assert_eq!(el_ref.get_reference_target().unwrap().path().unwrap(), "/Target/Ecu");
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn move_reference_base_target_package() {
        // moving the package a reference base points at updates the PACKAGE-REF like any other
        // reference; the cached copy of the PACKAGE-REF must be updated to match
        let (model, el_owner, el_target, el_ref) = build_relative_reference_model();
        el_owner
            .get_sub_element(ElementName::ArPackages)
            .unwrap()
            .move_element_here(&el_target)
            .unwrap();

        assert_eq!(
            get_package_ref(&el_owner)
                .character_data()
                .unwrap()
                .string_value()
                .unwrap(),
            "/Owner/Target"
        );
        assert_eq!(resolved_reference_base(&el_ref).as_deref(), Some("/Owner/Target"));
        assert_eq!(
            el_ref.get_reference_target().unwrap().path().unwrap(),
            "/Owner/Target/Ecu"
        );
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn move_reference_bases_element_to_other_package() {
        // Moving a REFERENCE-BASES element to a different package changes the owner of every
        // reference base inside it. REFERENCE-BASES is not identifiable, so this cannot be handled
        // by remapping paths - the owning package does not move at all.
        let (model, el_owner, el_target, el_ref) = build_relative_reference_model();
        let el_reference_bases = el_owner.get_sub_element(ElementName::ReferenceBases).unwrap();
        el_target.move_element_here(&el_reference_bases).unwrap();

        // the reference base is no longer in scope for the reference in /Owner/Sub
        assert_eq!(resolved_reference_base(&el_ref), None);
        assert!(el_ref.get_reference_target().is_err());
        assert_eq!(model.check_references().len(), 1);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn move_single_reference_base_to_other_package() {
        // the same applies when a single REFERENCE-BASE is moved out of the REFERENCE-BASES element
        // of one package into that of another package
        let (model, el_owner, el_target, el_ref) = build_relative_reference_model();
        let el_reference_base = el_owner
            .get_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.get_sub_element(ElementName::ReferenceBase))
            .unwrap();
        el_target
            .create_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.move_element_here(&el_reference_base))
            .unwrap();

        assert_eq!(resolved_reference_base(&el_ref), None);
        assert!(el_ref.get_reference_target().is_err());

        // a relative reference inside /Target can use the reference base at its new location
        let el_target_packages = el_target.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ecu = model.get_element_by_path("/Target/Ecu").unwrap();
        let el_new_ref = create_relative_reference(&el_target_packages, "Consumer", &el_ecu, "Base");
        assert_eq!(el_new_ref.get_reference_target().unwrap(), el_ecu);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn move_unnamed_element_containing_packages() {
        // Moving a non-identifiable element which contains identifiable elements: one path remapping
        // entry is created per identifiable element directly inside the moved element. Everything
        // below such an element - further identifiable elements as well as reference bases - is
        // covered by that entry, so the collection loop does not need to descend into it.
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_ar_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let (el_target, el_ecu) = create_target_package(&el_ar_packages, "Target");
        let el_dest = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Dest")
            .unwrap();
        let el_holder_packages = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Holder")
            .and_then(|e| e.create_sub_element(ElementName::ArPackages))
            .unwrap();

        // PkgA declares a reference base and contains a sub-package which uses it
        let el_pkg_a = el_holder_packages
            .create_named_sub_element(ElementName::ArPackage, "PkgA")
            .unwrap();
        create_reference_base(&el_pkg_a, "BaseA", &el_target);
        let el_pkg_a_packages = el_pkg_a.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ref = create_relative_reference(&el_pkg_a_packages, "PkgA_Sub", &el_ecu, "BaseA");
        // PkgB is a sibling of PkgA, and follows it in the element order
        let el_pkg_b = el_holder_packages
            .create_named_sub_element(ElementName::ArPackage, "PkgB")
            .unwrap();
        create_reference_base(&el_pkg_b, "BaseB", &el_target);

        assert_eq!(el_ref.get_reference_target().unwrap(), el_ecu);
        assert!(model.get_element_by_path("/Holder/PkgA/PkgA_Sub").is_some());

        // move the AR-PACKAGES element, which is not identifiable, into /Dest
        el_dest.move_element_here(&el_holder_packages).unwrap();

        // the identifiable elements directly inside the moved element are remapped ...
        assert!(model.get_element_by_path("/Dest/PkgA").is_some());
        assert!(model.get_element_by_path("/Dest/PkgB").is_some());
        // ... and so are the identifiable elements nested below them, which the collection loop
        // never visits because it skips the subtree of PkgA
        assert!(model.get_element_by_path("/Dest/PkgA/PkgA_Sub").is_some());
        assert!(model.get_element_by_path("/Holder/PkgA").is_none());
        assert!(model.get_element_by_path("/Holder/PkgA/PkgA_Sub").is_none());
        assert!(model.get_element_by_path("/Holder/PkgB").is_none());

        // the reference base inside the skipped subtree of PkgA still applies to the reference which
        // uses it, and BaseB proves that the sibling following PkgA is not skipped along with it
        assert_eq!(resolved_reference_base(&el_ref).as_deref(), Some("/Target"));
        assert_eq!(
            get_package_ref(&el_pkg_b).resolve_reference_base("BaseB").as_deref(),
            Some("/Target")
        );

        // the relative reference in the nested sub-package still resolves through BaseA
        assert_eq!(el_ref.get_reference_target().unwrap(), el_ecu);
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn move_relative_reference_out_of_scope() {
        // A subtree containing relative references can be moved to a location where the reference
        // base is no longer in scope. The reference text is not rewritten, so the reference dangles.
        let (model, _el_owner, el_target, el_ref) = build_relative_reference_model();
        let el_sub = model.get_element_by_path("/Owner/Sub").unwrap();
        el_target
            .create_sub_element(ElementName::ArPackages)
            .and_then(|e| e.move_element_here(&el_sub))
            .unwrap();

        assert_eq!(el_ref.character_data().unwrap().string_value().unwrap(), "Ecu");
        assert!(el_ref.get_reference_target().is_err());
        let broken = model.check_references();
        assert_eq!(broken.len(), 1);
        assert_eq!(broken[0].upgrade().unwrap(), el_ref);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn move_relative_reference_into_new_scope() {
        // ... and if a reference base with the same label is in scope at the new location, then the
        // relative reference resolves through that one instead
        let (model, _el_owner, _el_target, el_ref) = build_relative_reference_model();
        let el_ar_packages = model.root_element().get_sub_element(ElementName::ArPackages).unwrap();
        // /Other declares the same base label, but points at a different package
        let (el_other_target, _) = create_target_package(&el_ar_packages, "OtherTarget");
        let el_other = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Other")
            .unwrap();
        create_reference_base(&el_other, "Base", &el_other_target);

        let el_sub = model.get_element_by_path("/Owner/Sub").unwrap();
        el_other
            .create_sub_element(ElementName::ArPackages)
            .and_then(|e| e.move_element_here(&el_sub))
            .unwrap();

        assert_eq!(el_ref.character_data().unwrap().string_value().unwrap(), "Ecu");
        assert_eq!(
            el_ref.get_reference_target().unwrap().path().unwrap(),
            "/OtherTarget/Ecu"
        );
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn move_element_full_migrates_reference_bases() {
        // a cross-model move takes the reference bases along with the moved elements, so a relative
        // reference among them still resolves in the destination model
        let (model_src, el_owner, _el_target, el_ref) = build_relative_reference_model();

        let model_dest = AutosarModel::new();
        model_dest.create_file("dest", AutosarVersion::LATEST).unwrap();
        let el_dest_packages = model_dest
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        // the destination model gets its own copy of the target package, so that the reference base
        // still points at something after the move
        create_target_package(&el_dest_packages, "Target");
        let el_dest_outer = el_dest_packages
            .create_named_sub_element(ElementName::ArPackage, "Outer")
            .and_then(|e| e.create_sub_element(ElementName::ArPackages))
            .unwrap();

        el_dest_outer.move_element_here(&el_owner).unwrap();

        assert!(model_src.0.read().relative_references.is_empty());
        // the moved reference resolves through the moved reference base, in the destination model
        assert_eq!(resolved_reference_base(&el_ref).as_deref(), Some("/Target"));
        // the relative reference is registered in the destination model and resolves there
        assert!(!model_dest.0.read().relative_references.is_empty());
        assert_eq!(el_ref.get_reference_target().unwrap().path().unwrap(), "/Target/Ecu");
        assert_eq!(model_dest.get_references_to("/Target/Ecu").len(), 1);
        assert!(model_dest.check_references().is_empty());
    }

    #[test]
    fn move_element_full_keeps_external_references() {
        // a reference which points outside of the moved subtree keeps its text, but it must still be
        // registered in the destination model
        let model_src = AutosarModel::new();
        model_src.create_file("src", AutosarVersion::LATEST).unwrap();
        let el_src_packages = model_src
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let (_, el_src_ecu) = create_target_package(&el_src_packages, "Target");
        let el_ref = el_src_packages
            .create_named_sub_element(ElementName::ArPackage, "Sub")
            .and_then(|e| e.create_sub_element(ElementName::Elements))
            .and_then(|e| e.create_named_sub_element(ElementName::System, "System"))
            .and_then(|e| e.create_sub_element(ElementName::FibexElements))
            .and_then(|e| e.create_sub_element(ElementName::FibexElementRefConditional))
            .and_then(|e| e.create_sub_element(ElementName::FibexElementRef))
            .unwrap();
        el_ref.set_reference_target(&el_src_ecu).unwrap();

        let model_dest = AutosarModel::new();
        model_dest.create_file("dest", AutosarVersion::LATEST).unwrap();
        let el_dest_packages = model_dest
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        create_target_package(&el_dest_packages, "Target");

        let el_sub = model_src.get_element_by_path("/Sub").unwrap();
        el_dest_packages.move_element_here(&el_sub).unwrap();

        // the reference points outside the moved subtree, so its text is unchanged
        assert_eq!(el_ref.character_data().unwrap().string_value().unwrap(), "/Target/Ecu");
        // it must be findable in the destination model, where it resolves to the destination's own
        // element at that path
        assert_eq!(model_dest.get_references_to("/Target/Ecu").len(), 1);
        assert_eq!(model_src.get_references_to("/Target/Ecu").len(), 0);
        assert!(model_dest.check_references().is_empty());
    }

    #[test]
    fn copy_resolves_relative_references() {
        // the copy of an element which contains both a REFERENCE-BASE and a relative reference using it
        // must resolve within the copy
        let (model, el_owner, _el_target, _el_ref) = build_relative_reference_model();
        let el_ar_packages = model.root_element().get_sub_element(ElementName::ArPackages).unwrap();
        let el_copy = el_ar_packages.create_copied_sub_element(&el_owner).unwrap();

        assert_eq!(el_copy.path().unwrap(), "/Owner_1");

        // the copied relative reference resolves through the copied reference base
        let el_copied_ref = model
            .get_element_by_path("/Owner_1/Sub/System")
            .and_then(|e| e.get_sub_element(ElementName::FibexElements))
            .and_then(|e| e.get_sub_element_at(0))
            .and_then(|e| e.get_sub_element(ElementName::FibexElementRef))
            .unwrap();
        assert_eq!(
            el_copied_ref.get_reference_target().unwrap().path().unwrap(),
            "/Target/Ecu"
        );
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn set_character_data_on_package_ref_retargets_reference_base() {
        // PACKAGE-REF is a reference element, so writing its content takes the reference branch of
        // set_character_data. The reference base declared by it must be updated anyway.
        let (model, el_owner, _el_target, el_ref) = build_relative_reference_model();
        let el_ar_packages = model.root_element().get_sub_element(ElementName::ArPackages).unwrap();
        create_target_package(&el_ar_packages, "OtherTarget");

        assert_eq!(model.get_references_to("/Target/Ecu").len(), 1);

        get_package_ref(&el_owner).set_character_data("/OtherTarget").unwrap();

        assert_eq!(resolved_reference_base(&el_ref).as_deref(), Some("/OtherTarget"));
        assert_eq!(
            el_ref.get_reference_target().unwrap().path().unwrap(),
            "/OtherTarget/Ecu"
        );
        // retargeting the reference base changes what the relative reference points at without
        // changing its character data, so the reference has to be re-registered under the new path
        assert_eq!(model.get_references_to("/OtherTarget/Ecu").len(), 1);
        assert!(model.get_references_to("/Target/Ecu").is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn set_base_attribute_on_package_ref_chains_reference_base() {
        // the BASE attribute of a PACKAGE-REF selects the reference base which the PACKAGE-REF
        // itself is relative to, so it is part of the cached declaration
        let (model, el_owner, el_target, _el_ref) = build_relative_reference_model();
        // add a second reference base in /Owner, whose PACKAGE-REF is relative to "Base"
        let el_inner = el_target
            .get_sub_element(ElementName::ArPackages)
            .unwrap_or_else(|| el_target.create_sub_element(ElementName::ArPackages).unwrap())
            .create_named_sub_element(ElementName::ArPackage, "Inner")
            .unwrap();
        create_reference_base(&el_owner, "Chained", &el_inner);
        let el_owner_package_ref = get_package_ref(&el_owner);
        assert_eq!(
            el_owner_package_ref.resolve_reference_base("Chained").as_deref(),
            Some("/Target/Inner")
        );

        // rewrite the chained base to be relative to "Base"
        let el_chained_package_ref = el_owner
            .get_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.get_sub_element_at(1))
            .and_then(|e| e.get_sub_element(ElementName::PackageRef))
            .unwrap();
        el_chained_package_ref.set_character_data("Inner").unwrap();
        el_chained_package_ref
            .set_attribute_string(AttributeName::Base, "Base")
            .unwrap();

        // the chained form leads to the same package as the absolute form did
        assert_eq!(
            el_owner_package_ref.resolve_reference_base("Chained").as_deref(),
            Some("/Target/Inner")
        );

        // removing the BASE attribute leaves the relative text in place, so the declaration now names
        // the bogus path "Inner"
        assert!(el_chained_package_ref.remove_attribute(AttributeName::Base));
        assert_eq!(
            el_owner_package_ref.resolve_reference_base("Chained").as_deref(),
            Some("Inner")
        );
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn find_named_sub_element_matches_item_name() {
        // The lookup must agree with `item_name` exactly, including the
        // `except` exclusion and document order among same-named siblings.
        let model = AutosarModel::new();
        let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
        let _ = file;
        let base = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap()
            .create_named_sub_element(ElementName::ArPackage, "Pkg")
            .unwrap()
            .create_sub_element(ElementName::Elements)
            .unwrap();
        let alpha = base
            .create_named_sub_element(ElementName::CompuMethod, "Alpha")
            .unwrap();
        let beta = base.create_named_sub_element(ElementName::CompuMethod, "Beta").unwrap();

        assert_eq!(
            base.find_named_sub_element("Beta", None).unwrap(),
            beta,
            "finds an existing child by name"
        );
        assert!(base.find_named_sub_element("Gamma", None).is_none(), "reports absence");
        assert!(
            base.find_named_sub_element("Beta", Some(&beta)).is_none(),
            "excludes the given element, leaving no other candidate"
        );
        assert_eq!(
            base.find_named_sub_element("Alpha", Some(&alpha)),
            None,
            "excluding the only child with that name reports absence"
        );
        assert_eq!(
            base.find_named_sub_element("Alpha", Some(&beta)),
            Some(alpha),
            "excluding an unrelated element does not hide the name"
        );
    }

    #[test]
    fn set_character_data_on_relative_reference() {
        // writing the content of a reference which already has a BASE attribute keeps the reference
        // relative: the new content is resolved against the same reference base, so the reference is
        // re-registered under the target path that results from combining the two
        let (model, _el_owner, el_target, el_ref) = build_relative_reference_model();
        let el_ecu2 = el_target
            .get_sub_element(ElementName::Elements)
            .unwrap()
            .create_named_sub_element(ElementName::EcuInstance, "Ecu2")
            .unwrap();

        el_ref.set_character_data("Ecu2").unwrap();

        assert_eq!(
            el_ref
                .attribute_value(AttributeName::Base)
                .unwrap()
                .string_value()
                .unwrap(),
            "Base"
        );
        assert_eq!(el_ref.get_reference_target().unwrap(), el_ecu2);
        assert_eq!(
            model.relative_reference_target(&el_ref.downgrade()).as_deref(),
            Some("/Target/Ecu2")
        );
        assert_eq!(model.get_references_to("/Target/Ecu2").len(), 1);
        assert!(model.get_references_to("/Target/Ecu").is_empty());
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn set_character_data_incompatible_value() {
        // a value which is not compatible with the character data spec is rejected
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_elements = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))
            .and_then(|e| e.create_sub_element(ElementName::Elements))
            .unwrap();
        let el_ecu_instance = el_elements
            .create_named_sub_element(ElementName::EcuInstance, "Ecu")
            .unwrap();
        let el_sleep_mode_supported = el_ecu_instance
            .create_sub_element(ElementName::SleepModeSupported)
            .unwrap();

        // SLEEP-MODE-SUPPORTED contains a boolean, so an arbitrary string is not valid
        assert!(matches!(
            el_sleep_mode_supported.set_character_data("nonsense"),
            Err(AutosarDataError::InvalidCharacterData {
                element: ElementName::SleepModeSupported,
                ..
            })
        ));
        assert!(el_sleep_mode_supported.character_data().is_none());

        // a valid value is accepted
        el_sleep_mode_supported.set_character_data(true).unwrap();
        assert_eq!(
            el_sleep_mode_supported
                .character_data()
                .unwrap()
                .string_value()
                .unwrap(),
            "true"
        );

        // ABSOLUTE contains a floating point number. Unlike the String and Pattern specs, there is no
        // fallback which converts the value to a plain string, so a string value is rejected.
        let el_absolute = el_elements
            .create_named_sub_element(ElementName::ISignalIPdu, "Pdu")
            .and_then(|e| e.create_sub_element(ElementName::IPduTimingSpecifications))
            .and_then(|e| e.create_sub_element(ElementName::IPduTiming))
            .and_then(|e| e.create_sub_element(ElementName::TransmissionModeDeclaration))
            .and_then(|e| e.create_sub_element(ElementName::TransmissionModeTrueTiming))
            .and_then(|e| e.create_sub_element(ElementName::CyclicTiming))
            .and_then(|e| e.create_sub_element(ElementName::TimePeriod))
            .and_then(|e| e.create_sub_element(ElementName::Tolerance))
            .and_then(|e| e.create_sub_element(ElementName::AbsoluteTolerance))
            .and_then(|e| e.create_sub_element(ElementName::Absolute))
            .unwrap();
        assert!(matches!(
            el_absolute.set_character_data("not a number"),
            Err(AutosarDataError::InvalidCharacterData {
                element: ElementName::Absolute,
                ..
            })
        ));
        el_absolute.set_character_data(1.5).unwrap();
        assert_eq!(el_absolute.character_data(), Some(CharacterData::Float(1.5)));
    }

    #[test]
    fn debug_format_of_elements() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_ar_package = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))
            .unwrap();
        let el_short_name = el_ar_package.get_sub_element(ElementName::ShortName).unwrap();

        // an identifiable element shows its name, and its parent is another element
        let package_text = format!("{el_ar_package:#?}");
        assert!(package_text.contains(r#"name: "Pkg""#));
        assert!(package_text.contains("parent: Element"));

        // the content of the SHORT-NAME is character data
        let short_name_text = format!("{el_short_name:#?}");
        assert!(short_name_text.contains("Pkg"));

        // the parent of the root element is the model
        assert!(format!("{:#?}", model.root_element()).contains("parent: Model"));

        // a deleted element has no parent at all, and its weak reference can't be upgraded any more
        let weak_short_name = el_short_name.downgrade();
        model
            .root_element()
            .get_sub_element(ElementName::ArPackages)
            .unwrap()
            .remove_sub_element(el_ar_package)
            .unwrap();
        assert!(format!("{el_short_name:#?}").contains("parent: None/Invalid"));
        drop(el_short_name);
        assert!(format!("{weak_short_name:#?}").contains("(invalid)"));
    }

    #[test]
    fn reference_base_with_additional_sub_elements() {
        // a REFERENCE-BASE may contain more than just SHORT-LABEL and PACKAGE-REF; the additional
        // sub elements are not part of the declaration and must be skipped when it is read
        let (model, el_owner, _el_target, el_ref) = build_relative_reference_model();
        let el_reference_base = el_owner
            .get_sub_element(ElementName::ReferenceBases)
            .and_then(|e| e.get_sub_element(ElementName::ReferenceBase))
            .unwrap();
        el_reference_base
            .create_sub_element(ElementName::IsDefault)
            .and_then(|e| e.set_character_data(true))
            .unwrap();
        el_reference_base
            .create_sub_element(ElementName::IsGlobal)
            .and_then(|e| e.set_character_data(false))
            .unwrap();

        assert_eq!(resolved_reference_base(&el_ref).as_deref(), Some("/Target"));
        assert_eq!(el_ref.get_reference_target().unwrap().path().unwrap(), "/Target/Ecu");
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn copy_drops_incompatible_optional_attribute() {
        // copying into a file of an older version filters out everything that is not valid there.
        // An optional attribute which does not exist in the target version is simply omitted.
        let model_new = AutosarModel::new();
        model_new.create_file("new", AutosarVersion::LATEST).unwrap();
        let el_package_new = model_new
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .and_then(|e| e.create_named_sub_element(ElementName::ArPackage, "Pkg"))
            .unwrap();
        let el_short_label = el_package_new
            .create_sub_element(ElementName::VariationPoint)
            .and_then(|e| e.create_sub_element(ElementName::ShortLabel))
            .unwrap();
        el_short_label.set_character_data("Label").unwrap();
        el_short_label
            .set_attribute_string(AttributeName::BlueprintValue, "bpv")
            .unwrap();

        let model_old = AutosarModel::new();
        model_old.create_file("old", AutosarVersion::Autosar_4_3_0).unwrap();
        let el_copy = model_old
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .and_then(|e| e.create_copied_sub_element(&el_package_new))
            .unwrap();

        let el_short_label_copy = el_copy
            .get_sub_element(ElementName::VariationPoint)
            .and_then(|e| e.get_sub_element(ElementName::ShortLabel))
            .unwrap();
        assert_eq!(
            el_short_label_copy.character_data().unwrap().string_value().unwrap(),
            "Label"
        );
        // BLUEPRINT-VALUE does not exist in Autosar_4_3_0, and it is optional, so it was dropped
        assert!(
            el_short_label_copy
                .attribute_value(AttributeName::BlueprintValue)
                .is_none()
        );
    }
}
