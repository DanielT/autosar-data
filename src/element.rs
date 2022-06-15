use smallvec::smallvec;
use std::str::FromStr;

use crate::iterators::*;
use crate::spec_support::*;
use crate::specification::*;

use super::*;

#[derive(Debug, Error)]
pub enum ElementActionError {}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ContentType {
    Elements,
    CharacterData,
    Mixed,
}

impl Element {
    pub fn parent(&self) -> Option<Element> {
        if let Ok(inner) = self.0.lock() {
            if let ElementOrFile::Element(parent) = &inner.parent {
                return parent.upgrade();
            }
        }
        None
    }


    pub(crate) fn set_parent(&self, new_parent: ElementOrFile) {
        if let Ok(mut inner) = self.0.lock() {
            inner.parent = new_parent;
        }
    }


    pub fn element_name(&self) -> ElementName {
        let inner = self.0.lock().unwrap();
        inner.elemname
    }


    pub fn item_name(&self) -> Option<String> {
        let inner = self.0.lock().unwrap();
        // is this element named in any autosar version? - If it's not named here then we'll simply fail in the next step
        if DATATYPES[inner.type_id].is_named != 0 {
            // if an item is named, then the SHORT-NAME sub element that contains the name is always the first sub element
            if let Some(ElementContent::Element(subelem)) = inner.content.get(0) {
                if subelem.element_name() == ElementName::ShortName {
                    if let Some(CharacterData::String(name)) = subelem.character_data() {
                        return Some(name);
                    }
                }
            }
        }
        None
    }


    pub fn is_named(&self) -> bool {
        let inner = self.0.lock().unwrap();
        // is this element named in any autosar version? - If it's not named here then we'll simply fail in the next step
        if DATATYPES[inner.type_id].is_named != 0 {
            // if an item is named, then the SHORT-NAME sub element that contains the name is always the first sub element
            if let Some(ElementContent::Element(subelem)) = inner.content.get(0) {
                if subelem.element_name() == ElementName::ShortName {
                    return true;
                }
            }
        }
        false
    }


    pub fn is_reference(&self) -> bool {
        DATATYPES[self.type_id()].is_ref
    }


    pub fn path(&self) -> Option<String> {
        if self.is_named() {
            let mut cur_elem_opt = Some(self.clone());
            let mut path_components = vec![];
            while let Some(cur_elem) = &cur_elem_opt {
                if let Some(name) = cur_elem.item_name() {
                    path_components.push(name);
                }
                cur_elem_opt = cur_elem.parent();
            }
            path_components.push(String::new());
            path_components.reverse();

            Some(path_components.join("/"))
        } else {
            None
        }
    }


    pub fn containing_file(&self) -> Option<ArxmlFile> {
        let mut cur_elem = self.clone();
        loop {
            let parent = if let Ok(inner) = cur_elem.0.lock() {
                match &inner.parent {
                    ElementOrFile::Element(weak_parent) => weak_parent.upgrade()?,
                    ElementOrFile::File(weak_arxmlfile) => {
                        return weak_arxmlfile.upgrade();
                    }
                    ElementOrFile::None => return None,
                }
            } else {
                return None;
            };
            cur_elem = parent;
        }
    }

    pub fn get_content_type(&self) -> ContentType {
        let datatype = &DATATYPES[self.type_id()];
        match datatype.mode {
            ContentMode::Sequence => ContentType::Elements,
            ContentMode::Choice => ContentType::Elements,
            ContentMode::Bag => ContentType::Elements,
            ContentMode::Characters => ContentType::CharacterData,
            ContentMode::Mixed => ContentType::Mixed,
        }
    }

    pub fn create_sub_element(&self, element_name: ElementName) -> Result<Element, ()> {
        let (_start_pos, end_pos) = self.find_element_insert_pos(element_name)?;
        self.create_sub_element_inner(element_name, end_pos)
    }

    pub fn create_sub_element_at(&self, element_name: ElementName, position: usize) -> Result<Element, ()> {
        let (start_pos, end_pos) = self.find_element_insert_pos(element_name)?;
        if start_pos <= position && position <= end_pos {
            self.create_sub_element_inner(element_name, position)
        } else {
            Err(())
        }
    }

    fn create_sub_element_inner(&self, element_name: ElementName, position: usize) -> Result<Element, ()> {
        let version = self.containing_file().map(|f| f.version()).ok_or(())?;
        let type_id = self.type_id();
        let element_indices = find_sub_element(element_name, type_id, version as u32).unwrap();
        let sub_element_spec = get_sub_element_spec(DATATYPES[type_id].sub_elements, &element_indices).unwrap();
        if let SubElement::Element { elemtype, .. } = sub_element_spec {
            if DATATYPES[*elemtype].is_named & version as u32 != 0 {
                Err(())
            } else {
                let sub_element = Element(Arc::new(Mutex::new(ElementRaw {
                    parent: ElementOrFile::Element(self.downgrade()),
                    elemname: element_name,
                    type_id: *elemtype,
                    content: smallvec![],
                    attributes: smallvec![],
                })));
                let mut inner = self.0.lock().unwrap();
                inner
                    .content
                    .insert(position, ElementContent::Element(sub_element.clone()));
                Ok(sub_element)
            }
        } else {
            panic!(); // impossible
        }
    }

    pub fn create_named_sub_element(&self, element_name: ElementName, item_name: &str) -> Result<Element, ()> {
        let (_start_pos, end_pos) = self.find_element_insert_pos(element_name)?;
        self.create_named_sub_element_inner(element_name, item_name, end_pos)
    }

    pub fn create_named_sub_element_at(
        &self,
        element_name: ElementName,
        item_name: &str,
        position: usize,
    ) -> Result<Element, ()> {
        let (start_pos, end_pos) = self.find_element_insert_pos(element_name)?;
        if start_pos <= position && position <= end_pos {
            self.create_named_sub_element_inner(element_name, item_name, position)
        } else {
            Err(())
        }
    }

    fn create_named_sub_element_inner(
        &self,
        element_name: ElementName,
        item_name: &str,
        position: usize,
    ) -> Result<Element, ()> {
        if !item_name.is_empty() {
            let file = self.containing_file().ok_or(())?;
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
                    {
                        // separate scope to limit the lifetime of the mutex
                        let mut inner = self.0.lock().map_err(|_| ())?;
                        inner
                            .content
                            .insert(position, ElementContent::Element(sub_element.clone()));
                    }
                    let shortname_element = sub_element.create_sub_element(ElementName::ShortName)?;
                    shortname_element
                        .set_character_data(CharacterData::String(item_name.to_owned()))
                        .unwrap();
                    Ok(sub_element)
                } else {
                    Err(())
                }
            } else {
                panic!(); // impossible
            }
        } else {
            Err(())
        }
    }


    fn find_element_insert_pos(&self, element_name: ElementName) -> Result<(usize, usize), ()> {
        let version = self.containing_file().map(|f| f.version()).ok_or(())?;
        let type_id = self.type_id();
        let datatype = &DATATYPES[type_id];
        if datatype.mode == ContentMode::Characters {
            // cant't insert at all, only character data is permitted
            return Err(());
        }

        if let Some(new_element_indices) = find_sub_element(element_name, type_id, version as u32) {
            let inner = self.0.lock().unwrap();
            let mut start_pos = 0;
            let mut end_pos = 0;
            for (idx, content_item) in inner.content.iter().enumerate() {
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
                                            return Err(());
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
                                        return Err(());
                                    }
                                } else {
                                    panic!(); // can't happen
                                }
                                // the existing element and the new element are equal in positioning
                                // start_pos remains at its current value (< current position), while
                                // end_pos is increased to allow inserting before or after this element
                                end_pos = idx + 1;
                            } else {
                                return Err(());
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
            Err(())
        }
    }


    pub fn remove_sub_element(&self, sub_element: Element) -> Result<(), ()> {
        if self.is_named() && sub_element.element_name() == ElementName::ShortName {
            // may not remove the SHORT-NAME, because that would leave the data in an invalid state
            return Err(());
        }
        // remove all the sub-sub-elements. This needs to be done explicitly so that any related cached paths and references can be found and removed
        for sub_sub_element in sub_element.sub_elements() {
            // don't care if this succeeds or not
            let _ = sub_element.remove_sub_element(sub_sub_element);
        }

        // if sub_element is a named element, then a reference to it exists in the autosar_data.identifiables HashMap
        if sub_element.is_named() {
            if let Some(data) = self.containing_file().and_then(|f| f.autosar_data()) {
                if let Some(path) = sub_element.path() {
                    // remove the identifiables-reference (terminology???)
                    data.remove_identifiable(&path);
                }
            }
        }
        // if the sub_element is a reference, then autosar_data.references caches this relation
        if sub_element.is_reference() {
            if let Some(CharacterData::String(reference)) = sub_element.character_data() {
                if let Some(data) = self.containing_file().and_then(|f| f.autosar_data()) {
                    // remove the references-reference (ugh. terminology???)
                    data.remove_reference(&reference, sub_element.downgrade());
                }
            }
        }

        // remove the back-reference to the parent inside the sub_element.
        // This matters if other references to the sub_element exist, causing it to remain alive
        if let Ok(mut sub_element_inner) = sub_element.0.lock() {
            sub_element_inner.parent = ElementOrFile::None;
            sub_element_inner.content.truncate(0);
        }
        let mut inner = self.0.lock().map_err(|_| ())?;
        let (pos, _) = inner
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
            .ok_or(())?;
        inner.content.remove(pos);
        Ok(())
    }


    pub fn set_reference_target(&self, target: Element) {
        // the current element must be a reference
        if self.is_reference() {
            // the target element must be identifiable, i.e. it has an autosar path
            if let Some(new_ref) = target.path() {
                // it must be possible to use the name of the referenced element name as an enum item in the dest attribute of the reference
                if let Ok(enum_item) = EnumItem::from_str(target.element_name().to_str()) {
                    // need a reference to the AutosarData struct all this is part of - shouldn't ever fail
                    if let Some(data) = self.containing_file().and_then(|f| f.autosar_data()) {
                        // if this reference previously referenced some other element, update
                        if let Some(CharacterData::String(old_ref)) = self.character_data() {
                            data.fix_references(&old_ref, &new_ref, self.downgrade());
                        } else {
                            // else initialise the new reference
                            data.add_reference(&new_ref, self.downgrade());
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
    /// This method only applies to elements which contain character data, i.e. element.get_content_type == CharacterData
    pub fn set_character_data(&self, chardata: CharacterData) -> Result<(), ()> {
        let type_id = self.type_id();
        let datatype = &DATATYPES[type_id];
        if datatype.mode == ContentMode::Characters {
            if let Some(cdata_spec) = datatype.character_data {
                let version = self.containing_file().map(|f| f.version()).ok_or(())?;
                if CharacterData::check_value(&chardata, cdata_spec, version) {
                    // if this is a SHORT-NAME element a whole lot of handling is needed in order to unbreak all the cross references
                    let prev_path = if self.element_name() == ElementName::ShortName {
                        self.path()
                    } else {
                        None
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
                    if let Ok(mut inner) = self.0.lock() {
                        if inner.content.is_empty() {
                            inner.content.push(ElementContent::CharacterData(chardata));
                        } else {
                            inner.content[0] = ElementContent::CharacterData(chardata);
                        }
                    }

                    // short-name: make sure the hashmap in the top-level AutosarData struct is updated so that this element can still be found
                    if self.element_name() == ElementName::ShortName {
                        if let Some(data) = self.containing_file().and_then(|file| file.autosar_data()) {
                            if let Some(parent) = self.parent() {
                                data.fix_identifiables(prev_path, parent);
                            }
                        }
                    }

                    // reference: update the references hashmap in the top-level AutosarData struct
                    if datatype.is_ref {
                        if let Some(CharacterData::String(refval)) = self.character_data() {
                            if let Some(old_refval) = old_refval {
                                if let Some(data) = self.containing_file().and_then(|file| file.autosar_data()) {
                                    data.fix_references(&old_refval, &refval, self.downgrade());
                                }
                            }
                        }
                    }

                    return Ok(());
                }
            }
        }
        Err(())
    }


    /// insert a character data item into the content of this element
    ///
    /// This method only applies to elements which contain mixed data, i.e. element.get_content_type == Mixed.
    /// Use create_sub_element_at to add an element instead of a character data item
    ///
    /// return true if the data was added
    pub fn insert_character_content_item(&self, chardata: &str, position: usize) -> bool {
        if let Ok(mut inner) = self.0.lock() {
            if let ContentMode::Mixed = &DATATYPES[inner.type_id].mode {
                if position <= inner.content.len() {
                    inner.content.insert(
                        position,
                        ElementContent::CharacterData(CharacterData::String(chardata.to_owned())),
                    );
                    return true;
                }
            }
        }
        false
    }

    /// remove a character data item from the content of this element
    ///
    /// This method only applies to elements which contain mixed data, i.e. element.get_content_type == Mixed
    ///
    /// returns true if data was removed
    pub fn remove_content_item(&self, position: usize) -> bool {
        if let Ok(mut inner) = self.0.lock() {
            if let ContentMode::Mixed = &DATATYPES[inner.type_id].mode {
                if position < inner.content.len() {
                    if let ElementContent::CharacterData(_) = inner.content[position] {
                        inner.content.remove(position);
                        return true;
                    }
                }
            }
        }
        false
    }


    /// get the character content of the element
    ///
    /// This method only applies to elements which contain character data, i.e. element.get_content_type == CharacterData
    pub fn character_data(&self) -> Option<CharacterData> {
        if let Ok(inner) = self.0.lock() {
            if let ContentMode::Characters = &DATATYPES[inner.type_id].mode {
                if let Some(ElementContent::CharacterData(cdata)) = inner.content.get(0) {
                    return Some(cdata.clone());
                }
            }
        }
        None
    }


    pub fn content(&self) -> ElementContentIterator {
        ElementContentIterator::new(self)
    }


    pub fn downgrade(&self) -> WeakElement {
        WeakElement(Arc::downgrade(&self.0))
    }


    pub fn sub_elements(&self) -> ElementsIterator {
        ElementsIterator::new(self.clone())
    }


    pub fn elements_dfs(&self) -> ElementsDfsIterator {
        ElementsDfsIterator::new(self)
    }


    pub fn attributes(&self) -> AttributeIterator {
        AttributeIterator {
            element: self.clone(),
            index: 0,
        }
    }

    pub fn get_attribute(&self, attrname: AttributeName) -> Option<CharacterData> {
        if let Ok(inner) = self.0.lock() {
            if let Some(attr) = inner.attributes.iter().find(|attr| attr.attrname == attrname) {
                return Some(attr.content.clone());
            }
        }
        None
    }

    pub fn get_attribute_string(&self, attrname: AttributeName) -> Option<String> {
        if let Some(chardata) = self.get_attribute(attrname) {
            match chardata {
                CharacterData::String(stringval) => return Some(stringval),
                other => return Some(other.to_string()),
            }
        }
        None
    }

    pub fn set_attribute(&self, attrname: AttributeName, value: CharacterData) -> bool {
        if let Some(version) = self.containing_file().map(|f| f.version()) {
            return self.set_attribute_internal(attrname, value, version);
        }
        false
    }

    pub fn set_attribute_string(&self, attrname: AttributeName, stringvalue: &str) -> bool {
        if let Some(version) = self.containing_file().map(|f| f.version()) {
            if let Ok(mut locked_elem) = self.0.lock() {
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
                    }
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
            if let Ok(mut inner) = self.0.lock() {
                if let Some(attr) = inner.attributes.iter_mut().find(|attr| attr.attrname == attrname) {
                    // the existing attribute gets updated
                    if CharacterData::check_value(&value, spec, file_version) {
                        attr.content = value;
                        return true;
                    }
                } else {
                    // the attribute didn't exist yet, so it is created here
                    if CharacterData::check_value(&value, spec, file_version) {
                        inner.attributes.push(Attribute {
                            attrname,
                            content: value,
                        });
                        return true;
                    }
                }
            }
        }
        false
    }


    pub fn remove_attribute(&self, attrname: AttributeName) -> bool {
        if let Ok(mut inner) = self.0.lock() {
            for idx in 0..inner.attributes.len() {
                if inner.attributes[idx].attrname == attrname {
                    inner.attributes.remove(idx);
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn type_id(&self) -> usize {
        self.0.lock().unwrap().type_id
    }
}


impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}


impl WeakElement {
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

    #[test]
    fn test_element_creation() {
        let autosar_data = AutosarData::new();
        let file = autosar_data
            .create_file(OsString::from("test.arxml"), AutosarVersion::LATEST)
            .unwrap();
        let el_autosar = file.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "TestPackage")
            .unwrap();
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

        // inserting another COMPU-METHOD into ELEMENTS hould be allowed at any position
        let (start_pos, end_pos) = el_elements.find_element_insert_pos(ElementName::CompuMethod).unwrap();
        assert_eq!(start_pos, 0);
        assert_eq!(end_pos, 3); // upper limit is 3 since there are currently 3 elements

        // check if create_named_sub_element correctly registered the element in the hashmap so that it can be found
        let el_compu_method_test = autosar_data.get_named_element("/TestPackage/TestCompuMethod").unwrap();
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
}
