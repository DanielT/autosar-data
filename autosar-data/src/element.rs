use smallvec::smallvec;
use std::{borrow::Cow, str::FromStr, time::Duration};

use crate::iterators::*;

use super::*;

/// Errors that might occur while working with elements
#[derive(Debug, Error)]
pub enum ElementActionError {
    #[error("Invalid position for an element of this kind")]
    InvalidElementPosition,

    #[error("Version is not compatible")]
    VersionIncompatible,

    #[error("The Element is not identifiable")]
    ElementNotIdentifiable,

    #[error("An item name is required")]
    ItemNameRequired,

    #[error("Incorrect content type")]
    IncorrectContentType,

    #[error("Element insertion conflict")]
    ElementInsertionConflict,

    #[error("Invalid sub element")]
    InvalidSubElement,

    #[error("element not found")]
    ElementNotFound,

    #[error("the SHORT-NAME sub element may not be removed")]
    ShortNameRemovalForbidden,

    #[error("The current element is not a reference")]
    NotReferenceElement,

    #[error("The reference is not valid")]
    InvalidReference,

    #[error("Duplicate item name")]
    DuplicateItemName,

    #[error("Cannot move an element into its own sub element")]
    ForbiddenMoveToSubElement,

    #[error("A parent element is currently locked by a different operation")]
    ParentElementLocked,
}

impl Element {
    /// get the parent element of the current element
    ///
    /// Returns None if the current element is the root, or if it has been deleted from the element hierarchy
    pub fn parent(&self) -> Result<Option<Element>, AutosarDataError> {
        self.0.lock().parent()
    }

    pub(crate) fn set_parent(&self, new_parent: ElementOrFile) {
        self.0.lock().set_parent(new_parent)
    }

    /// get the [ElementName] of the element
    pub fn element_name(&self) -> ElementName {
        self.0.lock().elemname
    }

    /// get the [ElementType] of the element
    ///
    /// The ElementType is needed in order to call methods from the autosar-data-specification crate
    pub fn element_type(&self) -> ElementType {
        self.0.lock().elemtype
    }

    /// get the name of an identifiable element
    ///
    /// An identifiable element has a ```<SHORT-NAME>``` sub element and can be referenced using an autosar path.
    ///
    /// If the element is not identifiable, this function returns None
    pub fn item_name(&self) -> Option<String> {
        self.0.lock().item_name()
    }

    /// Set the item name of this element
    ///
    /// This operation will update all references pointing to the element or its sub-elements so that they remain valid.
    ///
    /// In order to rename an element *without* updating any references, do this instead:
    /// ```
    /// #   use autosar_data::*;
    /// #   let project = AutosarProject::new();
    /// #   let file = project.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// #   let element = file.root_element().create_sub_element(ElementName::ArPackages).and_then(|pkgs| pkgs.create_named_sub_element(ElementName::ArPackage, "name")).unwrap();
    ///     if let Some(short_name) = element.get_sub_element(ElementName::ShortName) {
    ///         short_name.set_character_data(CharacterData::String("the_new_name".to_string()));
    ///     }
    /// ```
    pub fn set_item_name(&self, new_name: &str) -> Result<(), AutosarDataError> {
        // a new name is required
        if new_name.is_empty() {
            return Err(Element::error(ElementActionError::ItemNameRequired));
        }
        let file = self.file()?;
        let project = file.project()?;
        let version = file.version();
        self.0.lock().set_item_name(new_name, &project, version)
    }

    /// returns true if the element is identifiable
    pub fn is_identifiable(&self) -> bool {
        self.0.lock().is_identifiable()
    }

    /// returns true if the element references another element
    ///
    /// The function does not check if the reference is valid
    pub fn is_reference(&self) -> bool {
        self.elemtype().is_ref()
    }

    /// get the Autosar path of an identifiable element
    ///
    /// returns Some(path) if the element is identifiable, None otherwise
    pub fn path(&self) -> Result<Option<String>, AutosarDataError> {
        self.0.lock().path()
    }

    /// get a reference to the [ArxmlFile] containing the current element
    pub fn file(&self) -> Result<ArxmlFile, AutosarDataError> {
        let mut cur_elem = self.clone();
        loop {
            let parent = {
                let element = cur_elem
                    .0
                    .try_lock_for(std::time::Duration::from_millis(10))
                    .ok_or_else(|| Element::error(ElementActionError::ParentElementLocked))?;
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

    /// get the [ContentType] of the current element
    pub fn content_type(&self) -> ContentType {
        match self.elemtype().content_mode() {
            ContentMode::Sequence => ContentType::Elements,
            ContentMode::Choice => ContentType::Elements,
            ContentMode::Bag => ContentType::Elements,
            ContentMode::Characters => ContentType::CharacterData,
            ContentMode::Mixed => ContentType::Mixed,
        }
    }

    /// create a sub element at a suitable insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// It is not possible to create named sub elements with this function; use create_named_sub_element() for that instead.
    pub fn create_sub_element(&self, element_name: ElementName) -> Result<Element, AutosarDataError> {
        let version = self.file().map(|f| f.version())?;
        self.0
            .lock()
            .create_sub_element(self.downgrade(), element_name, version)
    }

    /// create a sub element at the specified insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// It is not possible to create named sub elements with this function; use create_named_sub_element_at() for that instead.
    /// The specified insertion position will be compared to the range of valid insertion positions; if it falls ooutside that range then the function fails.
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

    /// create a named/identifiable sub element at a suitable insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// This method can only be used to create identifiable sub elements.
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

    /// create a named/identifiable sub element at the specified insertion position
    ///
    /// The given ElementName must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// The specified insertion position will be compared to the range of valid insertion positions; if it falls ooutside that range then the function fails.
    /// This method can only be used to create identifiable sub elements.
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

    /// create a deep copy of the given element and insert it as a sub-element
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
    pub fn create_copied_sub_element(&self, other: &Element) -> Result<Element, AutosarDataError> {
        let file = self.file()?;
        let project = file.project()?;
        let version = file.version();
        self.0
            .lock()
            .create_copied_sub_element(self.downgrade(), other, &project, version)
    }

    /// create a deep copy of the given element and insert it as a sub-element at the given position
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
    pub fn create_copied_sub_element_at(&self, other: &Element, position: usize) -> Result<Element, AutosarDataError> {
        let file = self.file()?;
        let project = file.project()?;
        let version = file.version();
        self.0
            .lock()
            .create_copied_sub_element_at(self.downgrade(), other, position, &project, version)
    }

    /// take an `element` from it's current location and place it in this element as a sub element
    ///
    /// The moved element can be taken from anywhere - even from a different arxml document that is not part of the same AutosarProject
    ///
    /// Restrictions:
    /// 1) The element must have a compatible element type. If it could not have been created here, then it can't be moved either.
    /// 2) The origin document of the element must have exactly the same AutosarVersion as the destination.
    pub fn move_element_here(&self, new_element: &Element) -> Result<Element, AutosarDataError> {
        let new_elem_file = new_element.file()?;
        let project_other = new_elem_file.project()?;
        let version = self.file().map(|f| f.version())?;
        let project = self.file().and_then(|f| f.project())?;
        let version_other = new_elem_file.version();
        if version != version_other {
            return Err(Element::error(ElementActionError::VersionIncompatible));
        }
        self.0
            .lock()
            .move_element_here(self.downgrade(), new_element, &project, &project_other, version)
    }

    /// take an `element` from it's current location and place it at the given position in this element as a sub element
    ///
    /// The moved element can be taken from anywhere - even from a different arxml document that is not part of the same AutosarProject
    ///
    /// Restrictions:
    /// 1) The element must have a compatible element type. If it could not have been created here, then it can't be moved either.
    /// 2) The origin document of the element must have exactly the same AutosarVersion as the destination.
    pub fn move_element_here_at(&self, new_element: &Element, position: usize) -> Result<Element, AutosarDataError> {
        let new_elem_file = new_element.file()?;
        let project_other = new_elem_file.project()?;
        let version = self.file().map(|f| f.version())?;
        let project = self.file().and_then(|f| f.project())?;
        let version_other = new_elem_file.version();
        if version != version_other {
            return Err(Element::error(ElementActionError::VersionIncompatible));
        }
        self.0.lock().move_element_here_at(
            self.downgrade(),
            new_element,
            position,
            &project,
            &project_other,
            version,
        )
    }

    /// remove the sub element sub_element
    ///
    /// The sub_element will be unlinked from the hierarchy of elements. All of the sub-sub-elements nested under the removed element will also be recusively removed.
    /// Since all elements are reference counted, they might not be deallocated immediately, however they do become invalid and unusable immediately.
    pub fn remove_sub_element(&self, sub_element: Element) -> Result<(), AutosarDataError> {
        let project = self.file().and_then(|f| f.project())?;
        self.0.lock().remove_sub_element(sub_element, &project)
    }

    /// Set the reference target for the element to target
    ///
    /// When the reference is updated, the DEST attribute is also updated to match the referenced element.
    /// The current element must be a reference element, otherwise the function fails.
    pub fn set_reference_target(&self, target: &Element) -> Result<(), AutosarDataError> {
        // the current element must be a reference
        if self.is_reference() {
            // the target element must be identifiable, i.e. it has an autosar path
            if let Some(new_ref) = target.path()? {
                // it must be possible to use the name of the referenced element name as an enum item in the dest attribute of the reference
                if let Ok(enum_item) = EnumItem::from_str(target.element_name().to_str()) {
                    let file = self.file()?;
                    let project = file.project()?;
                    let version = file.version();
                    let mut element = self.0.lock();
                    // set the DEST attribute first - this could fail if the target element has the wrong type
                    if element.set_attribute_internal(AttributeName::Dest, CharacterData::Enum(enum_item), version) {
                        // if this reference previously referenced some other element, update
                        if let Some(CharacterData::String(old_ref)) = element.character_data() {
                            project.fix_reference_origins(&old_ref, &new_ref, self.downgrade());
                        } else {
                            // else initialise the new reference
                            project.add_reference_origin(&new_ref, self.downgrade());
                        }
                        element.set_character_data(CharacterData::String(new_ref), version)?;
                    }
                    Ok(())
                } else {
                    Err(Element::error(ElementActionError::InvalidReference))
                }
            } else {
                Err(Element::error(ElementActionError::ElementNotFound))
            }
        } else {
            Err(Element::error(ElementActionError::NotReferenceElement))
        }
    }

    /// Get the referenced element
    pub fn get_reference_target(&self) -> Result<Element, AutosarDataError> {
        if self.is_reference() {
            if let Some(CharacterData::String(reference)) = self.character_data() {
                let project = self.file().and_then(|file| file.project())?;
                let target_elem = project
                    .get_element_by_path(&reference)
                    .ok_or_else(|| Element::error(ElementActionError::InvalidReference))?;

                let dest = self
                    .get_attribute_string(AttributeName::Dest)
                    .ok_or_else(|| Element::error(ElementActionError::InvalidReference))?;
                if dest == target_elem.element_name().to_str() {
                    Ok(target_elem)
                } else {
                    Err(Element::error(ElementActionError::InvalidReference))
                }
            } else {
                Err(Element::error(ElementActionError::InvalidReference))
            }
        } else {
            Err(Element::error(ElementActionError::NotReferenceElement))
        }
    }

    /// set the character data of this element
    ///
    /// This method only applies to elements which contain character data, i.e. element.content_type == CharacterData
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
                                prev_path = parent.path()?;
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
                            if let Some(new_path) = parent.path()? {
                                project.fix_identifiables(&prev_path, &new_path);
                            }
                        }
                    }

                    // reference: update the references hashmap in the top-level AutosarProject
                    if elemtype.is_ref() {
                        if let Some(CharacterData::String(refval)) = self.character_data() {
                            if let Some(old_refval) = old_refval {
                                project.fix_reference_origins(&old_refval, &refval, self.downgrade());
                            }
                        }
                    }

                    return Ok(());
                }
            }
        }
        Err(Element::error(ElementActionError::IncorrectContentType))
    }

    /// insert a character data item into the content of this element
    ///
    /// This method only applies to elements which contain mixed data, i.e. element.content_type == Mixed.
    /// Use create_sub_element_at to add an element instead of a character data item
    ///
    /// return true if the data was added
    pub fn insert_character_content_item(&self, chardata: &str, position: usize) -> bool {
        let mut element = self.0.lock();
        if let ContentMode::Mixed = element.elemtype.content_mode() {
            if position <= element.content.len() {
                element.content.insert(
                    position,
                    ElementContent::CharacterData(CharacterData::String(chardata.to_owned())),
                );
                return true;
            }
        }
        false
    }

    /// remove a character data item from the content of this element
    ///
    /// This method only applies to elements which contain mixed data, i.e. element.content_type == Mixed
    ///
    /// returns true if data was removed
    pub fn remove_content_item(&self, position: usize) -> bool {
        let mut element = self.0.lock();
        if let ContentMode::Mixed = element.elemtype.content_mode() {
            if position < element.content.len() {
                if let ElementContent::CharacterData(_) = element.content[position] {
                    element.content.remove(position);
                    return true;
                }
            }
        }
        false
    }

    /// get the character content of the element
    ///
    /// This method only applies to elements which contain character data, i.e. element.content_type == CharacterData
    pub fn character_data(&self) -> Option<CharacterData> {
        self.0.lock().character_data()
    }

    /// create an iterator over all of the content of this element
    ///
    /// The iterator can return both sub elements and character data, wrapped as ElementContent::Element and ElementContent::CharacterData
    pub fn content(&self) -> ElementContentIterator {
        ElementContentIterator::new(self)
    }

    /// create a wea reference to this element
    pub fn downgrade(&self) -> WeakElement {
        WeakElement(Arc::downgrade(&self.0))
    }

    /// create an iterator over all sub elements of this element
    pub fn sub_elements(&self) -> ElementsIterator {
        ElementsIterator::new(self.clone())
    }

    /// get the sub element with the given element name
    ///
    /// Returns None if no such element exists. if there are multiple sub elements with the requested name, then only the first is returned
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

    /// create a depth first iterator over this element and all of its sub elements
    pub fn elements_dfs(&self) -> ElementsDfsIterator {
        ElementsDfsIterator::new(self)
    }

    /// create an iterator over all the attributes in this element
    pub fn attributes(&self) -> AttributeIterator {
        AttributeIterator {
            element: self.clone(),
            index: 0,
        }
    }

    /// get a single attribute by name
    pub fn get_attribute(&self, attrname: AttributeName) -> Option<CharacterData> {
        self.0.lock().get_attribute(attrname)
    }

    /// get the content of a named attribute as a string
    pub fn get_attribute_string(&self, attrname: AttributeName) -> Option<String> {
        self.0.lock().get_attribute_string(attrname)
    }

    /// set the value of a named attribute
    ///
    /// If no attribute by that name exists, and the attribute is a valid attribute of the element, then the attribute will be created
    pub fn set_attribute(&self, attrname: AttributeName, value: CharacterData) -> bool {
        if let Ok(version) = self.file().map(|f| f.version()) {
            return self.0.lock().set_attribute_internal(attrname, value, version);
        }
        false
    }

    /// set the value of a named attribute from a string
    pub fn set_attribute_string(&self, attrname: AttributeName, stringvalue: &str) -> bool {
        if let Ok(version) = self.file().map(|f| f.version()) {
            let mut locked_elem = self.0.lock();
            if let Some((character_data_spec, _, _)) = locked_elem.elemtype.find_attribute_spec(attrname) {
                if let Some(value) = CharacterData::parse(stringvalue, character_data_spec, version) {
                    if let Some(attr) = locked_elem.attributes.iter_mut().find(|attr| attr.attrname == attrname) {
                        attr.content = value;
                    } else {
                        locked_elem.attributes.push(Attribute {
                            attrname,
                            content: value,
                        });
                    }
                    return true;
                }
            }
        }
        false
    }

    /// remove an attribute from the element
    pub fn remove_attribute(&self, attrname: AttributeName) -> bool {
        self.0.lock().remove_attribute(attrname)
    }

    /// serialize the element and all of its content to a string
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
                if let Some(content) = element.content.get(0) {
                    match content {
                        ElementContent::Element(_) => panic!("Forbidden: Element in CharacterData"),
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
            let (mut sub_element_errors, sub_element_mask) = sub_element.check_version_compatibility(target_version);
            compat_errors.append(&mut sub_element_errors);
            overall_version_mask &= sub_element_mask;
        }

        (compat_errors, overall_version_mask)
    }

    /// list all sub_elements that are valid in the current element
    ///
    /// returns:
    ///     ElementName of the potential sub element
    ///     bool: is the sub element named
    ///     bool: can this sub element be inserted considering the current content of the element
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

    fn error(err: ElementActionError) -> AutosarDataError {
        AutosarDataError::ElementActionError { source: err }
    }
}

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
        // a new name is required
        if new_name.is_empty() {
            return Err(Element::error(ElementActionError::ItemNameRequired));
        }

        if let Some(current_name) = self.item_name() {
            // bail out early if the name is actually the same
            if current_name == new_name {
                return Ok(());
            }

            let old_path = self
                .path()?
                .ok_or_else(|| Element::error(ElementActionError::ElementNotIdentifiable))?;
            let new_path = format!("{}{new_name}", old_path.strip_suffix(&current_name).unwrap());
            if project.get_element_by_path(&new_path).is_some() {
                return Err(Element::error(ElementActionError::DuplicateItemName));
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
            Err(Element::error(ElementActionError::ElementNotIdentifiable))
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
    pub(crate) fn path(&self) -> Result<Option<String>, AutosarDataError> {
        if self.is_identifiable() {
            let path = self.path_unchecked()?;
            // even if this element is identifiable, path might still be "" if the current element has been deleted from the element hierarchy
            if !path.is_empty() {
                Ok(Some(path))
            } else {
                Ok(None)
            }
        } else {
            Err(Element::error(ElementActionError::ElementNotIdentifiable))
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
                .ok_or_else(|| Element::error(ElementActionError::ParentElementLocked))?
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

    /// create a sub element at a suitable insertion position
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

    /// create a sub element at the specified insertion position
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
            Err(Element::error(ElementActionError::InvalidElementPosition))
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
            .ok_or_else(|| Element::error(ElementActionError::InvalidSubElement))?;
        if elemtype.is_named_in_version(version) {
            Err(Element::error(ElementActionError::VersionIncompatible))
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
            Err(Element::error(ElementActionError::InvalidElementPosition))
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
            return Err(Element::error(ElementActionError::ItemNameRequired));
        }
        let item_name_cdata = CharacterData::String(item_name.to_owned());

        let (elemtype, _) = self
            .elemtype
            .find_sub_element(element_name, version as u32)
            .ok_or_else(|| Element::error(ElementActionError::InvalidSubElement))?;

        if elemtype.is_named_in_version(version) {
            // verify that the given item_name is actually a valid name
            if !elemtype
                .find_sub_element(ElementName::ShortName, version as u32)
                .and_then(|(se_type, _)| se_type.chardata_spec())
                .map(|cdata_spec| CharacterData::check_value(&item_name_cdata, cdata_spec, version))
                .unwrap_or(false)
            {
                return Err(Element::error(ElementActionError::IncorrectContentType));
            }

            let parent_path = self.path_unchecked()?;
            let path = format!("{parent_path}/{item_name}");
            // verify that the name is unique: there must be no existing element that already has this autosar path
            if project.get_element_by_path(&path).is_some() {
                return Err(Element::error(ElementActionError::DuplicateItemName));
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
            Err(Element::error(ElementActionError::VersionIncompatible))
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
            Err(Element::error(ElementActionError::InvalidElementPosition))
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
                    .ok_or_else(|| Element::error(ElementActionError::VersionIncompatible))?;
                // check if the attribute is compatible with the target version
                if target_version.compatible(attr_version_mask)
                    && attribute
                        .content
                        .check_version_compatibility(cdataspec, target_version)
                        .0
                {
                    copy.attributes.push(attribute.clone());
                } else if required {
                    return Err(Element::error(ElementActionError::VersionIncompatible));
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
        let orig_name = self
            .item_name()
            .ok_or_else(|| Element::error(ElementActionError::ElementNotIdentifiable))?;
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
                let mut sn_element = short_name_elem.0.lock();
                if sn_element.elemname != ElementName::ShortName {
                    return Err(Element::error(ElementActionError::InvalidSubElement));
                }
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
        new_element: &Element,
        project: &AutosarProject,
        project_other: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let new_elementname = new_element.element_name();
        let (_, end_pos) = self.find_element_insert_pos(new_elementname, version)?;

        self.move_element_here_inner(self_weak, new_element, end_pos, project, project_other)
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
        new_element: &Element,
        position: usize,
        project: &AutosarProject,
        project_other: &AutosarProject,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let new_elementname = new_element.element_name();
        let (start_pos, end_pos) = self.find_element_insert_pos(new_elementname, version)?;
        if start_pos <= position && position <= end_pos {
            self.move_element_here_inner(self_weak, new_element, position, project, project_other)
        } else {
            Err(Element::error(ElementActionError::InvalidElementPosition))
        }
    }

    fn move_element_here_inner(
        &mut self,
        self_weak: WeakElement,
        new_element: &Element,
        position: usize,
        project: &AutosarProject,
        project_other: &AutosarProject,
    ) -> Result<Element, AutosarDataError> {
        // check if self (target of the move) is a sub element of new_element
        // if it is, then the move is not allowed
        let mut wrapped_parent = self.parent.clone();
        while let ElementOrFile::Element(weak_parent) = wrapped_parent {
            let parent = weak_parent.upgrade().ok_or(AutosarDataError::ItemDeleted)?;
            if parent == *new_element {
                return Err(Element::error(ElementActionError::ForbiddenMoveToSubElement));
            }
            wrapped_parent = parent.0.lock().parent.clone();
        }

        let new_elem_parent = new_element
            .parent()?
            .ok_or_else(|| Element::error(ElementActionError::InvalidSubElement))?;

        if new_elem_parent.downgrade() == self_weak {
            return Ok(new_element.clone());
        }

        let new_elem_path = new_element.0.lock().path_unchecked()?;
        let path = self.path_unchecked()?;

        // collect the paths of all identifiable elements under new_element before moving it
        let original_paths: FxHashMap<String, Element> = new_element
            .elements_dfs()
            .filter_map(|(_, e)| e.path().ok().flatten().map(|path| (path, e)))
            .collect();
        // collect all reference targets and referring elements under new_element
        let original_refs: Vec<(String, Element)> = new_element
            .elements_dfs()
            .filter(|(_, e)| e.is_reference())
            .filter_map(|(_, e)| e.character_data().map(|data| (data.to_string(), e)))
            .collect();

        // limit the lifetime of the mutex on new_elem_parent
        {
            // lock the parent of the new element and remove it from the parent's content list
            let mut new_elem_parent_locked = new_elem_parent.0.lock();
            let idx = new_elem_parent_locked
                .content
                .iter()
                .enumerate()
                .find(|(_, item)| {
                    if let ElementContent::Element(elem) = item {
                        *elem == *new_element
                    } else {
                        false
                    }
                })
                .map(|(idx, _)| idx)
                .unwrap();
            new_elem_parent_locked.content.remove(idx);
        }

        // remove all cached references for elements under new_element - they all become invalid as a result of moving it
        for path in original_paths.keys() {
            project_other.remove_identifiable(path);
        }
        if project != project_other {
            // delete all reference origin info for elements under new_element
            for (path, elem) in &original_refs {
                project_other.remove_reference_origin(path, elem.downgrade());
            }
        }

        {
            // set the parent of the new element to the current element
            let mut new_element_locked = new_element.0.lock();
            new_element_locked.parent = ElementOrFile::Element(self_weak);
        }

        let new_name = new_element.0.lock().make_unique_item_name(project, &path)?;

        // cache references to all the identifiable elements in new_element
        for (orig_path, identifiable_element) in &original_paths {
            if let Some(suffix) = orig_path.strip_prefix(&new_elem_path) {
                let path = format!("{path}/{new_name}{suffix}");
                project.add_identifiable(path, identifiable_element.downgrade());
            }
        }
        // cache all newly added (or modified) reference origins in new_element
        for (old_ref, ref_element) in original_refs {
            let mut refstr = old_ref.clone();
            // if the reference points to a known old path, then update it to use the new path instead
            if original_paths.contains_key(&old_ref) {
                if let Some(suffix) = old_ref.strip_prefix(&new_elem_path) {
                    refstr = format!("{path}/{new_name}{suffix}");
                }
                ref_element.set_character_data(CharacterData::String(refstr.clone()))?;
            }
            project.fix_reference_origins(&old_ref, &refstr, ref_element.downgrade());
        }
        // if the new_element was moved within this autosar project, then we might be able to update other references pointing to it
        if project == project_other {
            let mut project_locked = project.0.lock();
            for old_path in original_paths.keys() {
                // the elements under new_element have already been updated above, this will get any other elements all over the autosar project
                if let Some(ref_elements) = project_locked.reference_origins.remove(old_path) {
                    if let Some(suffix) = old_path.strip_prefix(&new_elem_path) {
                        let new_ref_path = format!("{path}/{new_name}{suffix}");
                        for ref_elem_weak in &ref_elements {
                            if let Some(ref_elem) = ref_elem_weak.upgrade() {
                                let mut ref_elem_locked = ref_elem.0.lock();
                                // bypass the extra logic in set_character_data, which also wants to manipulate the currently locked project_locked.reference_origins
                                ref_elem_locked.content.truncate(0);
                                ref_elem_locked
                                    .content
                                    .push(ElementContent::CharacterData(CharacterData::String(
                                        new_ref_path.clone(),
                                    )));
                            }
                        }
                        project_locked.reference_origins.insert(new_ref_path, ref_elements);
                    }
                }
            }
        }

        // insert new_element
        self.content
            .insert(position, ElementContent::Element(new_element.clone()));

        Ok(new_element.clone())
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
            return Err(Element::error(ElementActionError::IncorrectContentType));
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
                                            return Err(Element::error(ElementActionError::ElementInsertionConflict));
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
                                        return Err(Element::error(ElementActionError::ElementInsertionConflict));
                                    }
                                } else {
                                    panic!(); // can't happen
                                }
                                // the existing element and the new element are equal in positioning
                                // start_pos remains at its current value (< current position), while
                                // end_pos is increased to allow inserting before or after this element
                                end_pos = idx + 1;
                            } else {
                                return Err(Element::error(ElementActionError::ElementInsertionConflict));
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
            Err(Element::error(ElementActionError::InvalidSubElement))
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
            .ok_or_else(|| Element::error(ElementActionError::ElementNotFound))?;
        if self.elemtype.is_named() && sub_element_locked.elemname == ElementName::ShortName {
            // may not remove the SHORT-NAME, because that would leave the data in an invalid state
            return Err(Element::error(ElementActionError::ShortNameRemovalForbidden));
        }
        sub_element_locked.remove_internal(sub_element.downgrade(), project, path);
        self.content.remove(pos);
        Ok(())
    }

    // remove all of the content of an element
    fn remove_internal(&mut self, self_weak: WeakElement, project: &AutosarProject, mut path: Cow<str>) {
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
        Err(Element::error(ElementActionError::IncorrectContentType))
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
    pub(crate) fn get_attribute(&self, attrname: AttributeName) -> Option<CharacterData> {
        if let Some(attr) = self.attributes.iter().find(|attr| attr.attrname == attrname) {
            return Some(attr.content.clone());
        }
        None
    }

    /// get the content of a named attribute as a string
    pub(crate) fn get_attribute_string(&self, attrname: AttributeName) -> Option<String> {
        if let Some(chardata) = self.get_attribute(attrname) {
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
    ) -> bool {
        // find the attribute specification in the item type
        if let Some((spec, _, _)) = self.elemtype.find_attribute_spec(attrname) {
            // find the attribute the element's attribute list
            if let Some(attr) = self.attributes.iter_mut().find(|attr| attr.attrname == attrname) {
                // the existing attribute gets updated
                if CharacterData::check_value(&value, spec, file_version) {
                    attr.content = value;
                    return true;
                }
            } else {
                // the attribute didn't exist yet, so it is created here
                if CharacterData::check_value(&value, spec, file_version) {
                    self.attributes.push(Attribute {
                        attrname,
                        content: value,
                    });
                    return true;
                }
            }
        }
        false
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
            .create_named_sub_element(ElementName::CanCluster, "CanCluster")
            .unwrap();
        let el_can_physical_channel = el_can_cluster
            .create_sub_element(ElementName::CanClusterVariants)
            .and_then(|ccv| ccv.create_sub_element(ElementName::CanClusterConditional))
            .and_then(|ccc| ccc.create_sub_element(ElementName::PhysicalChannels))
            .and_then(|pc| pc.create_named_sub_element(ElementName::CanPhysicalChannel, "CanPhysicalChannel"))
            .unwrap();

        let el_can_frame_triggering = el_can_physical_channel
            .create_sub_element(ElementName::FrameTriggerings)
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

        // duplicate name for Package2, renaming should fail
        let result = el_ar_package2.set_item_name("NewPackage");
        assert!(result.is_err());

        // rename package 2 with a valid name
        el_ar_package2.set_item_name("OtherPackage").unwrap();
        let refstr = el_frame_ref.character_data().unwrap().string_value().unwrap();
        assert_eq!(refstr, "/OtherPackage/CanFrame");

        // rename the CanFrame as well
        el_can_frame.set_item_name("CanFrame_renamed").unwrap();
        let refstr = el_frame_ref.character_data().unwrap().string_value().unwrap();
        assert_eq!(refstr, "/OtherPackage/CanFrame_renamed");
    }

    #[test]
    fn element_copy() {
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = project.get_element_by_path("/TestPackage").unwrap();
        let el_elements = el_ar_package.create_sub_element(ElementName::Elements).unwrap();
        let el_compu_method = el_elements
            .create_named_sub_element(ElementName::CompuMethod, "CompuMethod")
            .unwrap();
        el_elements
            .create_named_sub_element(ElementName::DdsServiceInstanceToMachineMapping, "ApItem")
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

        // it should be possible to look up the copied compu method by its path
        let el_compu_method_2 = project2.get_element_by_path("/TestPackage/CompuMethod").unwrap();

        // the copy should not refer to the same memory as the original
        assert_ne!(el_compu_method, el_compu_method_2);
        // the copy should serialize to exactly the same string as the original
        assert_eq!(el_compu_method.serialize(), el_compu_method_2.serialize());

        // verify that the DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING element was not copied
        let result = project2.get_element_by_path("/TestPackage/ApItem");
        assert!(result.is_none());
    }

    #[test]
    fn element_deletion() {
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = project.get_element_by_path("/TestPackage").unwrap();
        let el_short_name = el_ar_package.get_sub_element(ElementName::ShortName).unwrap();
        // removing the SHORT-NAME of an identifiable element is forbidden
        let result = el_ar_package.remove_sub_element(el_short_name);
        if let Err(AutosarDataError::ElementActionError {
            source: ElementActionError::ShortNameRemovalForbidden,
        }) = result
        {
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
    fn element_ops() {
        let project = AutosarProject::new();
        let (file, _) = project
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();

        let el_autosar = file.root_element();
        let el_ar_packages = el_autosar.get_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages.get_sub_element(ElementName::ArPackage).unwrap();

        let el_ar_package2 = project.get_element_by_path("/TestPackage").unwrap();

        assert_eq!(el_ar_package, el_ar_package2);
    }

    #[test]
    fn attributes() {
        let project = AutosarProject::new();
        let (file, _) = project
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_autosar = file.root_element();

        let count = el_autosar.attributes().count();
        assert_eq!(count, 3);

        // set the attribute S on the element AUTOSAR
        let result = el_autosar.set_attribute(AttributeName::S, CharacterData::String(String::from("something")));
        assert_eq!(result, true);

        // AUTOSAR has no DEST attribute, so this should fail
        let result = el_autosar.set_attribute(AttributeName::Dest, CharacterData::String(String::from("something")));
        assert_eq!(result, false);

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
        let result = el_autosar.set_attribute_string(AttributeName::T, "2022-01-31T13:00:59Z");
        assert_eq!(result, true);

        let xmlns = el_autosar.get_attribute_string(AttributeName::xmlns).unwrap();
        assert_eq!(xmlns, "http://autosar.org/schema/r4.0".to_string());
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
        el_l_4.insert_character_content_item("foo", 1);
        el_l_4.create_sub_element(ElementName::Sup).unwrap();
        el_l_4.insert_character_content_item("bar", 0);
        assert_eq!(el_l_4.content().count(), 4);

        // character data item "foo" is now in position 2 and gets removed
        assert_eq!(el_l_4.remove_content_item(2), true);
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
    fn element_move() {
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
        <AR-PACKAGE><SHORT-NAME>Pkg2</SHORT-NAME>
            <ELEMENTS>
                <ECU-INSTANCE><SHORT-NAME>EcuInstance</SHORT-NAME></ECU-INSTANCE>
            </ELEMENTS>
        </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        let project = AutosarProject::new();
        project
            .load_named_arxml_buffer(FILEBUF.as_bytes(), &OsString::from("test"), true)
            .unwrap();
        // get the existing ECU-INSTANCE EcuInstance
        let ecu_instance = project.get_element_by_path("/Pkg/EcuInstance").unwrap();
        // get the existing AR-PACKAGE Pkg2
        let pkg2 = project.get_element_by_path("/Pkg2").unwrap();
        // get the ELEMENTS sub element of Pkg2
        let pkg2_elements = pkg2
            .sub_elements()
            .find(|e| e.element_name() == ElementName::Elements)
            .unwrap();
        // move the EcuInstance into Pkg2
        pkg2_elements.move_element_here(&ecu_instance).unwrap();
        // assert: pkg2 is now the parent of EcuInstance
        assert_eq!(ecu_instance.parent().unwrap().unwrap(), pkg2_elements);
        // assert: due to a name conflict, the moved EcuInstance has been renamed to EcuInstance_1
        assert_eq!(ecu_instance.item_name().unwrap(), "EcuInstance_1");
        // assert: The path of the EcuInstance is now "/Pkg2/EcuInstance" instead of "/Pkg/EcuInstance"
        assert_eq!(ecu_instance.path().unwrap().unwrap(), "/Pkg2/EcuInstance_1");
        let system = project.get_element_by_path("/Pkg/System").unwrap();
        // get the FIBEX-ELEMENT-REF which has DEST=ECU-INSTANCE
        let (_, fibex_elem_ref) = system
            .elements_dfs()
            .find(|(_, e)| {
                e.is_reference()
                    && e.get_attribute(AttributeName::Dest)
                        .map(|val| val == CharacterData::Enum(EnumItem::EcuInstance))
                        .unwrap()
            })
            .unwrap();
        // assert: The reference has been updated to refer to the new path of the ECU-INSTANCE
        assert_eq!(
            fibex_elem_ref.character_data().unwrap().to_string(),
            "/Pkg2/EcuInstance_1"
        );
    }
}
