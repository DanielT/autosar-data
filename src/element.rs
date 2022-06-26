use smallvec::smallvec;
use std::str::FromStr;

use crate::iterators::*;
use crate::spec_support::*;
use crate::specification::*;

use super::*;

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
}

impl Element {
    /// get the parent element of the current element
    ///
    /// Returns None if the current element is the root, or if it has been deleted from the element hierarchy
    pub fn parent(&self) -> Result<Option<Element>, AutosarDataError> {
        let element = self.0.lock();
        match &element.parent {
            ElementOrFile::Element(parent) => {
                // for items that should have a parent, getting it is not allowed to return None
                let parent = parent.upgrade().ok_or(AutosarDataError::ItemDeleted)?;
                Ok(Some(parent))
            }
            ElementOrFile::File(_) => Ok(None),
            ElementOrFile::None => Err(AutosarDataError::ItemDeleted),
        }
    }

    pub(crate) fn set_parent(&self, new_parent: ElementOrFile) {
        let mut element = self.0.lock();
        element.parent = new_parent;
    }

    /// get the [ElementName] of the element
    pub fn element_name(&self) -> ElementName {
        self.0.lock().elemname
    }

    /// get the name of an identifiable element
    ///
    /// An identifiable element has a ```<SHORT-NAME>``` sub element and can be referenced using an autosar path.
    ///
    /// If the element is not identifiable, this function returns None
    pub fn item_name(&self) -> Option<String> {
        let element = self.0.lock();
        // is this element named in any autosar version? - If it's not named here then we'll simply fail in the next step
        if DATATYPES[element.type_id].is_named != 0 {
            // if an item is named, then the SHORT-NAME sub element that contains the name is always the first sub element
            if let Some(ElementContent::Element(subelem)) = element.content.get(0) {
                if subelem.element_name() == ElementName::ShortName {
                    if let Some(CharacterData::String(name)) = subelem.character_data() {
                        return Some(name);
                    }
                }
            }
        }
        None
    }

    /// returns true if the element is identifiable
    pub fn is_identifiable(&self) -> bool {
        let element = self.0.lock();
        // is this element named in any autosar version? - If it's not named here then we'll simply fail in the next step
        if DATATYPES[element.type_id].is_named != 0 {
            // if an item is named, then the SHORT-NAME sub element that contains the name is always the first sub element
            if let Some(ElementContent::Element(subelem)) = element.content.get(0) {
                if subelem.element_name() == ElementName::ShortName {
                    return true;
                }
            }
        }
        false
    }

    /// returns true if the element references another element
    ///
    /// The function does not check if the reference is valid
    pub fn is_reference(&self) -> bool {
        DATATYPES[self.type_id()].is_ref
    }

    /// get the Autosar path of an identifiable element
    ///
    /// returns Some(path) if the element is identifiable, None otherwise
    pub fn path(&self) -> Result<Option<String>, AutosarDataError> {
        if self.is_identifiable() {
            let mut cur_elem_opt = Some(self.clone());
            let mut path_components = vec![];
            while let Some(cur_elem) = &cur_elem_opt {
                if let Some(name) = cur_elem.item_name() {
                    path_components.push(name);
                }
                cur_elem_opt = cur_elem.parent()?;
            }
            path_components.push(String::new());
            path_components.reverse();

            let path = path_components.join("/");
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

    /// get a reference to the [ArxmlFile] containing the current element
    pub fn containing_file(&self) -> Result<ArxmlFile, AutosarDataError> {
        let mut cur_elem = self.clone();
        loop {
            let parent = {
                let element = cur_elem.0.lock();
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
        let datatype = &DATATYPES[self.type_id()];
        match datatype.mode {
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
        let (_start_pos, end_pos) = self.find_element_insert_pos(element_name)?;
        self.create_sub_element_inner(element_name, end_pos)
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
        let (start_pos, end_pos) = self.find_element_insert_pos(element_name)?;
        if start_pos <= position && position <= end_pos {
            self.create_sub_element_inner(element_name, position)
        } else {
            Err(Element::error(ElementActionError::InvalidElementPosition))
        }
    }

    /// helper function for create_sub_element and create_sub_element_at
    fn create_sub_element_inner(
        &self,
        element_name: ElementName,
        position: usize,
    ) -> Result<Element, AutosarDataError> {
        let version = self.containing_file().map(|f| f.version())?;
        let type_id = self.type_id();
        let element_indices = find_sub_element(element_name, type_id, version as u32).unwrap();
        let sub_element_spec = get_sub_element_spec(DATATYPES[type_id].sub_elements, &element_indices).unwrap();
        if let SubElement::Element { elemtype, .. } = sub_element_spec {
            if DATATYPES[*elemtype].is_named & version as u32 != 0 {
                Err(Element::error(ElementActionError::VersionIncompatible))
            } else {
                let sub_element = Element(Arc::new(Mutex::new(ElementRaw {
                    parent: ElementOrFile::Element(self.downgrade()),
                    elemname: element_name,
                    type_id: *elemtype,
                    content: smallvec![],
                    attributes: smallvec![],
                })));
                let mut element = self.0.lock();
                element
                    .content
                    .insert(position, ElementContent::Element(sub_element.clone()));
                Ok(sub_element)
            }
        } else {
            panic!(); // impossible
        }
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
        let (_start_pos, end_pos) = self.find_element_insert_pos(element_name)?;
        self.create_named_sub_element_inner(element_name, item_name, end_pos)
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
        let (start_pos, end_pos) = self.find_element_insert_pos(element_name)?;
        if start_pos <= position && position <= end_pos {
            self.create_named_sub_element_inner(element_name, item_name, position)
        } else {
            Err(Element::error(ElementActionError::InvalidElementPosition))
        }
    }

    /// helper function for create_named_sub_element and create_named_sub_element_at
    fn create_named_sub_element_inner(
        &self,
        element_name: ElementName,
        item_name: &str,
        position: usize,
    ) -> Result<Element, AutosarDataError> {
        if item_name.is_empty() {
            return Err(Element::error(ElementActionError::ItemNameRequired));
        }

        let file = self.containing_file()?;
        let type_id = self.type_id();
        let element_indices = find_sub_element(element_name, type_id, file.version() as u32).unwrap();
        let sub_element_spec = get_sub_element_spec(DATATYPES[type_id].sub_elements, &element_indices).unwrap();
        if let SubElement::Element { elemtype, .. } = sub_element_spec {
            if DATATYPES[*elemtype].is_named & file.version() as u32 != 0 {
                let sub_element = Element(Arc::new(Mutex::new(ElementRaw {
                    parent: ElementOrFile::Element(self.downgrade()),
                    elemname: element_name,
                    type_id: *elemtype,
                    content: smallvec![],
                    attributes: smallvec![],
                })));
                // insert the new sub element
                {
                    // separate scope to limit the lifetime of the mutex
                    let mut element = self.0.lock();
                    element
                        .content
                        .insert(position, ElementContent::Element(sub_element.clone()));
                }
                // create a SHORT-NAME for the sub element
                let shortname_element = sub_element.create_sub_element(ElementName::ShortName)?;
                shortname_element
                    .set_character_data(CharacterData::String(item_name.to_owned()))
                    .unwrap();
                Ok(sub_element)
            } else {
                Err(Element::error(ElementActionError::VersionIncompatible))
            }
        } else {
            panic!(); // impossible
        }
    }

    /// create a deep copy of the given element and insert it as a sub-element
    ///
    /// The other element must be a permissible sub-element in this element and not conflict with any existing sub element.
    /// The other element can originate from any loaded [AutosarData], it does not have to originate from the same project or file as the current element.
    ///
    /// The [AutosarVersion] of the other element might differ from the version of the current file;
    /// in this case a partial copy will be performed that omits all incompatible elements.
    ///
    /// If the copied element is identifiable, then the item name might be extended with a numerical suffix, if one is required in order to make the name unique.
    /// For example: An identifiable element "Foo" already exists at the same path; the copied identifiable element will be renamed to "Foo_1".
    ///
    /// If the copied element or the hierarchy of elements under it contain any references, then these will need to be adjusted manually after copying.
    pub fn create_copied_sub_element(&self, other: &Element) -> Result<Element, AutosarDataError> {
        let other_elemname = {
            // need to do this inside its own scope to limit the lifetime of the mutex
            let other_element = other.0.lock();
            other_element.elemname
        };
        let (_, end) = self.find_element_insert_pos(other_elemname)?;
        self.create_copied_sub_element_inner(other, end)
    }

    /// create a deep copy of the given element and insert it as a sub-element at the given position
    ///
    /// The other element must be a permissible sub-element in this element and not conflict with any existing sub element.
    /// The other element can originate from any loaded [AutosarData], it does not have to originate from the same project or file as the current element.
    ///
    /// The [AutosarVersion] of the other element might differ from the version of the current file;
    /// in this case a partial copy will be performed that omits all incompatible elements.
    ///
    /// If the copied element is identifiable, then the item name might be extended with a numerical suffix, if one is required in order to make the name unique.
    /// For example: An identifiable element "Foo" already exists at the same path; the copied identifiable element will be renamed to "Foo_1".
    ///
    /// If the copied element or the hierarchy of elements under it contain any references, then these will need to be adjusted manually after copying.
    pub fn create_copied_sub_element_at(&self, other: &Element, position: usize) -> Result<Element, AutosarDataError> {
        let other_elemname = {
            // need to do this inside its own scope to limit the lifetime of the mutex
            let other_element = other.0.lock();
            other_element.elemname
        };
        let (start_pos, end_pos) = self.find_element_insert_pos(other_elemname)?;
        if start_pos <= position && position <= end_pos {
            self.create_copied_sub_element_inner(other, position)
        } else {
            Err(Element::error(ElementActionError::InvalidElementPosition))
        }
    }

    fn create_copied_sub_element_inner(&self, other: &Element, position: usize) -> Result<Element, AutosarDataError> {
        let version = self.containing_file().map(|f| f.version())?;
        let autosar_data = self.containing_file().and_then(|f| f.autosar_data())?;

        // Arc overrides clone() so that it only manipulates the reference count, so a separate deep_copy operation is needed here.
        // Additionally, implementing this manually provides the opportunity to filter out
        // elements that ae not compatible with the version of the current file.
        let newelem = other.deep_copy(version)?;

        // set the parent of the newelem - the methods path(), containing_file(), etc become available on newelem
        newelem.set_parent(ElementOrFile::Element(self.downgrade()));
        if newelem.is_identifiable() {
            newelem.make_unique_item_name()?;
        }

        for (_, sub_elem) in newelem.elements_dfs() {
            // add all identifiable sub elements to the identifiables hashmap
            if sub_elem.is_identifiable() {
                autosar_data.fix_identifiables(None, &sub_elem);
            }
            // add all references to the reference_origins hashmap
            if sub_elem.is_reference() {
                if let Some(CharacterData::String(reference)) = sub_elem.character_data() {
                    autosar_data.add_reference_origin(&reference, sub_elem.downgrade())
                }
            }
        }

        self.0
            .lock()
            .content
            .insert(position, ElementContent::Element(newelem.clone()));

        Ok(newelem)
    }

    /// make_unique_item_name ensures that a copied element has a unique name
    fn make_unique_item_name(&self) -> Result<(), AutosarDataError> {
        let autosar_data = self.containing_file().and_then(|f| f.autosar_data())?;
        let orig_name = self.item_name().unwrap();
        let mut path = self.path()?.unwrap();
        let mut counter = 1;

        while autosar_data.get_element_by_path(&path)?.is_some() {
            let name = format!("{orig_name}_{counter}");
            counter += 1;
            {
                let element = self.0.lock();
                // set the name directly by modifying the character content of the short name element
                // note: the method set_character_data is not suitable here, because it updates the identifiables hashmap
                if let Some(ElementContent::Element(short_name_elem)) = element.content.get(0) {
                    let mut sn_element = short_name_elem.0.lock();
                    if sn_element.elemname != ElementName::ShortName {
                        return Err(Element::error(ElementActionError::InvalidSubElement));
                    }
                    sn_element.content.truncate(0);
                    sn_element
                        .content
                        .push(ElementContent::CharacterData(CharacterData::String(name)));
                }
            }
            path = self.path()?.unwrap();
        }

        Ok(())
    }

    /// perform a deep copy of an element, but keep only those sub elements etc, which are compatible with target_version
    fn deep_copy(&self, target_version: AutosarVersion) -> Result<Element, AutosarDataError> {
        let element = self.0.lock();

        let copy_wrapped = Element(Arc::new(Mutex::new(ElementRaw {
            elemname: element.elemname,
            type_id: element.type_id,
            content: SmallVec::with_capacity(element.content.len()),
            attributes: SmallVec::with_capacity(element.attributes.len()),
            parent: ElementOrFile::None,
        })));

        {
            let mut copy = copy_wrapped.0.lock();
            let datatype = &DATATYPES[element.type_id];
            for attribute in &element.attributes {
                // get the specification of the attribute
                let (_, cdataspec, required, attr_version_mask) = datatype
                    .attributes
                    .iter()
                    .find(|(name, ..)| *name == attribute.attrname)
                    .ok_or_else(|| Element::error(ElementActionError::VersionIncompatible))?;
                // check if the attribute is compatible with the target version
                if target_version.compatible(*attr_version_mask)
                    && attribute
                        .content
                        .check_version_compatibility(cdataspec, target_version)
                        .0
                {
                    copy.attributes.push(attribute.clone());
                } else if *required {
                    return Err(Element::error(ElementActionError::VersionIncompatible));
                } else {
                    // no action, the attribute is not compatible, but it's not required either
                }
            }

            for content_item in &element.content {
                match content_item {
                    ElementContent::Element(sub_elem) => {
                        let sub_elem_name = sub_elem.element_name();
                        // since find_sub_element already considers the version, finding the element also means it's valid in the target_version
                        if find_sub_element(sub_elem_name, element.type_id, target_version as u32).is_some() {
                            if let Ok(copied_sub_elem) = sub_elem.deep_copy(target_version) {
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

    /// find the upper and lower bound on the insert position for a sub element
    fn find_element_insert_pos(&self, element_name: ElementName) -> Result<(usize, usize), AutosarDataError> {
        let version = self.containing_file().map(|f| f.version())?;
        let type_id = self.type_id();
        let datatype = &DATATYPES[type_id];
        if datatype.mode == ContentMode::Characters {
            // cant't insert at all, only character data is permitted
            return Err(Element::error(ElementActionError::IncorrectContentType));
        }

        if let Some(new_element_indices) = find_sub_element(element_name, type_id, version as u32) {
            let element = self.0.lock();
            let mut start_pos = 0;
            let mut end_pos = 0;
            for (idx, content_item) in element.content.iter().enumerate() {
                if let ElementContent::Element(subelement) = content_item {
                    let existing_element_indices =
                        find_sub_element(subelement.element_name(), type_id, version as u32).unwrap();
                    let group = find_common_group(type_id, &new_element_indices, &existing_element_indices);
                    match group.mode {
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
                                    let sub_elem_spec =
                                        get_sub_element_spec(DATATYPES[type_id].sub_elements, &new_element_indices)
                                            .unwrap();
                                    if let SubElement::Element { multiplicity, .. } = sub_elem_spec {
                                        if *multiplicity != ElementMultiplicity::Any {
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
                                let sub_elem_spec =
                                    get_sub_element_spec(DATATYPES[type_id].sub_elements, &new_element_indices)
                                        .unwrap();
                                if let SubElement::Element { multiplicity, .. } = sub_elem_spec {
                                    if *multiplicity != ElementMultiplicity::Any {
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
    pub fn remove_sub_element(&self, sub_element: Element) -> Result<(), AutosarDataError> {
        // find the position of sub_element in the parent element first to verify that sub_element actuall *is* a sub element
        let pos = {
            // lock the mutext inside this scope, so that the lock gets dropped again after the position has been found
            let element = self.0.lock();
            element
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
                .ok_or_else(|| Element::error(ElementActionError::ElementNotFound))?
        };
        let autosar_data = self.containing_file().and_then(|f| f.autosar_data())?;
        if self.is_identifiable() && sub_element.element_name() == ElementName::ShortName {
            // may not remove the SHORT-NAME, because that would leave the data in an invalid state
            return Err(Element::error(ElementActionError::ShortNameRemovalForbidden));
        }
        // remove all the sub-sub-elements. This needs to be done explicitly so that any related cached paths and references can be found and removed
        for sub_sub_element in sub_element.sub_elements() {
            // don't care if this succeeds or not
            let _ = sub_element.remove_sub_element(sub_sub_element);
        }

        // if sub_element is a named element, then a reference to it exists in the autosar_data.identifiables HashMap
        if sub_element.is_identifiable() {
            if let Some(path) = sub_element.path()? {
                // remove the identifiables-reference (terminology???)
                autosar_data.remove_identifiable(&path);
            }
        }
        // if the sub_element is a reference, then autosar_data.references caches this relation
        if sub_element.is_reference() {
            if let Some(CharacterData::String(reference)) = sub_element.character_data() {
                // remove the references-reference (ugh. terminology???)
                autosar_data.remove_reference_origin(&reference, sub_element.downgrade());
            }
        }

        // remove the back-reference to the parent inside the sub_element.
        // This matters if other references to the sub_element exist, causing it to remain alive
        let mut sub_element_inner = sub_element.0.lock();
        sub_element_inner.parent = ElementOrFile::None;
        sub_element_inner.content.truncate(0);

        self.0.lock().content.remove(pos);
        Ok(())
    }

    /// Set the reference target for the element to target
    ///
    /// When the reference is updated, the DEST attribute is also updated to match the referenced element.
    /// The current element must be a reference element, otherwise the function fails.
    pub fn set_reference_target(&self, target: Element) {
        // the current element must be a reference
        if self.is_reference() {
            // the target element must be identifiable, i.e. it has an autosar path
            if let Ok(Some(new_ref)) = target.path() {
                // it must be possible to use the name of the referenced element name as an enum item in the dest attribute of the reference
                if let Ok(enum_item) = EnumItem::from_str(target.element_name().to_str()) {
                    // need a reference to the AutosarData struct all this is part of - shouldn't ever fail
                    if let Ok(data) = self.containing_file().and_then(|f| f.autosar_data()) {
                        // if this reference previously referenced some other element, update
                        if let Some(CharacterData::String(old_ref)) = self.character_data() {
                            data.fix_reference_origins(&old_ref, &new_ref, self.downgrade());
                        } else {
                            // else initialise the new reference
                            data.add_reference_origin(&new_ref, self.downgrade());
                        }
                    }
                    let _ = self.set_character_data(CharacterData::String(new_ref));
                    self.set_attribute(AttributeName::Dest, CharacterData::Enum(enum_item));
                }
            }
        }
    }

    /// set the character data of this element
    ///
    /// This method only applies to elements which contain character data, i.e. element.content_type == CharacterData
    pub fn set_character_data(&self, chardata: CharacterData) -> Result<(), AutosarDataError> {
        let type_id = self.type_id();
        let datatype = &DATATYPES[type_id];
        if datatype.mode == ContentMode::Characters {
            if let Some(cdata_spec) = datatype.character_data {
                let autosar_data = self.containing_file().and_then(|file| file.autosar_data())?;
                let version = self.containing_file().map(|f| f.version())?;
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
                    let old_refval = if datatype.is_ref {
                        if let Some(CharacterData::String(refval)) = self.character_data() {
                            Some(refval)
                        } else {
                            None
                        }
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

                    // short-name: make sure the hashmap in the top-level AutosarData struct is updated so that this element can still be found
                    if self.element_name() == ElementName::ShortName {
                        if let Some(parent) = self.parent()? {
                            autosar_data.fix_identifiables(prev_path, &parent);
                        }
                    }

                    // reference: update the references hashmap in the top-level AutosarData struct
                    if datatype.is_ref {
                        if let Some(CharacterData::String(refval)) = self.character_data() {
                            if let Some(old_refval) = old_refval {
                                autosar_data.fix_reference_origins(&old_refval, &refval, self.downgrade());
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
        if let ContentMode::Mixed = &DATATYPES[element.type_id].mode {
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
        if let ContentMode::Mixed = &DATATYPES[element.type_id].mode {
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
        let element = self.0.lock();
        if let ContentMode::Characters = &DATATYPES[element.type_id].mode {
            if let Some(ElementContent::CharacterData(cdata)) = element.content.get(0) {
                return Some(cdata.clone());
            }
        }
        None
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
        let element = self.0.lock();
        if let Some(attr) = element.attributes.iter().find(|attr| attr.attrname == attrname) {
            return Some(attr.content.clone());
        }
        None
    }

    /// get the content of a named attribute as a string
    pub fn get_attribute_string(&self, attrname: AttributeName) -> Option<String> {
        if let Some(chardata) = self.get_attribute(attrname) {
            match chardata {
                CharacterData::String(stringval) => return Some(stringval),
                other => return Some(other.to_string()),
            }
        }
        None
    }

    /// set the value of a named attribute
    ///
    /// If no attribute by that name exists, and the attribute is a valid attribute of the element, then the attribute will be created
    pub fn set_attribute(&self, attrname: AttributeName, value: CharacterData) -> bool {
        if let Ok(version) = self.containing_file().map(|f| f.version()) {
            return self.set_attribute_internal(attrname, value, version);
        }
        false
    }

    /// set the value of a named attribute from a string
    pub fn set_attribute_string(&self, attrname: AttributeName, stringvalue: &str) -> bool {
        if let Ok(version) = self.containing_file().map(|f| f.version()) {
            let mut locked_elem = self.0.lock();
            let attr_types = DATATYPES[locked_elem.type_id].attributes;
            if let Some((_, character_data_spec, _, _)) = attr_types.iter().find(|(name, ..)| *name == attrname) {
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

    pub(crate) fn set_attribute_internal(
        &self,
        attrname: AttributeName,
        value: CharacterData,
        file_version: AutosarVersion,
    ) -> bool {
        let attr_types = DATATYPES[self.type_id()].attributes;
        // find the attribute specification in the item type
        if let Some((_, spec, _, _)) = attr_types.iter().find(|(name, ..)| *name == attrname) {
            // find the attribute the element's attribute list
            let mut element = self.0.lock();
            if let Some(attr) = element.attributes.iter_mut().find(|attr| attr.attrname == attrname) {
                // the existing attribute gets updated
                if CharacterData::check_value(&value, spec, file_version) {
                    attr.content = value;
                    return true;
                }
            } else {
                // the attribute didn't exist yet, so it is created here
                if CharacterData::check_value(&value, spec, file_version) {
                    element.attributes.push(Attribute {
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
    pub fn remove_attribute(&self, attrname: AttributeName) -> bool {
        let mut element = self.0.lock();
        let attrspec = DATATYPES[element.type_id].attributes;
        // find the index of the attribute in the attribute list of the element
        for idx in 0..element.attributes.len() {
            if element.attributes[idx].attrname == attrname {
                // find the definition of this attribute in the specification
                if let Some((_, _, required, _)) = attrspec.iter().find(|(name, ..)| *name == attrname) {
                    // the attribute can only be removed if it is optional
                    if !required {
                        element.attributes.remove(idx);
                        return true;
                    }
                }
            }
        }
        false
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

    pub(crate) fn type_id(&self) -> usize {
        self.0.lock().type_id
    }

    /// check if the sub elements and attributes of this element are compatible with some target_version
    pub fn check_version_compatibility(&self, target_version: AutosarVersion) -> (Vec<CompatibilityError>, u32) {
        let mut compat_errors = Vec::new();
        let mut overall_version_mask = u32::MAX;

        // check the compatibility of all the attributes in this element
        {
            let element = self.0.lock();
            let datatype = &DATATYPES[element.type_id];
            for attribute in &element.attributes {
                // find the specification for the current attribute
                if let Some((_, value_spec, _, version_mask)) = datatype
                    .attributes
                    .iter()
                    .find(|(name, ..)| *name == attribute.attrname)
                {
                    overall_version_mask &= *version_mask;
                    // check if the attribute is allowed at all
                    if !target_version.compatible(*version_mask) {
                        compat_errors.push(CompatibilityError::IncompatibleAttribute {
                            element: self.clone(),
                            attribute: attribute.attrname,
                            version_mask: *version_mask,
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

    fn error(err: ElementActionError) -> AutosarDataError {
        AutosarDataError::ElementActionError { source: err }
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
        let autosar_data = AutosarData::new();
        autosar_data
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = autosar_data.get_element_by_path("/TestPackage").unwrap().unwrap();

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
        let (start_pos, end_pos) = el_elements.find_element_insert_pos(ElementName::CompuMethod).unwrap();
        assert_eq!(start_pos, 0);
        assert_eq!(end_pos, 3); // upper limit is 3 since there are currently 3 elements

        // check if create_named_sub_element correctly registered the element in the hashmap so that it can be found
        let el_compu_method_test = autosar_data
            .get_element_by_path("/TestPackage/TestCompuMethod")
            .unwrap()
            .unwrap();
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
        let (start_pos, end_pos) = el_compu_scale.find_element_insert_pos(ElementName::ShortLabel).unwrap();
        assert_eq!(start_pos, 0);
        assert_eq!(end_pos, 0);

        // COMPU-CONST should only be allowed after DESC inside COMPU-SCALE
        let (start_pos, end_pos) = el_compu_scale.find_element_insert_pos(ElementName::CompuConst).unwrap();
        assert_eq!(start_pos, 1);
        assert_eq!(end_pos, 1);

        // create COMPU-RATIONAL-COEFFS in COMPU-SCALE. It's presence excludes COMPU-CONST from being inserted
        el_compu_scale
            .create_sub_element(ElementName::CompuRationalCoeffs)
            .unwrap();
        // try to insert COMPU-CONST anyway
        let result = el_compu_scale.find_element_insert_pos(ElementName::CompuConst);
        assert!(result.is_err());
    }

    #[test]
    fn element_copy() {
        let autosar_data = AutosarData::new();
        autosar_data
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = autosar_data.get_element_by_path("/TestPackage").unwrap().unwrap();
        let el_elements = el_ar_package.create_sub_element(ElementName::Elements).unwrap();
        let el_compu_method = el_elements
            .create_named_sub_element(ElementName::CompuMethod, "CompuMethod")
            .unwrap();
        el_elements
            .create_named_sub_element(ElementName::DdsServiceInstanceToMachineMapping, "ApItem")
            .unwrap();

        let autosar_data2 = AutosarData::new();
        let file = autosar_data2
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
        let el_compu_method_2 = autosar_data2
            .get_element_by_path("/TestPackage/CompuMethod")
            .unwrap()
            .unwrap();

        // the copy should not refer to the same memory as the original
        assert_ne!(el_compu_method, el_compu_method_2);
        // the copy should serialize to exactly the same string as the original
        assert_eq!(el_compu_method.serialize(), el_compu_method_2.serialize());

        // verify that the DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING element was not copied
        let result = autosar_data2.get_element_by_path("/TestPackage/ApItem").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn element_deletion() {
        let autosar_data = AutosarData::new();
        autosar_data
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = autosar_data.get_element_by_path("/TestPackage").unwrap().unwrap();
        let el_short_name = el_ar_package.sub_elements().next().unwrap();
        // removing the SHORT-NAME of an identifiable element is formbidden
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
        assert!(result.is_ok());
    }

    #[test]
    fn attributes() {
        let autosar_data = AutosarData::new();
        let (file, _) = autosar_data
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
        let autosar_data = AutosarData::new();
        autosar_data
            .load_named_arxml_buffer(BASIC_AUTOSAR_FILE.as_bytes(), &OsString::from("test.arxml"), true)
            .unwrap();
        let el_ar_package = autosar_data.get_element_by_path("/TestPackage").unwrap().unwrap();
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
}
