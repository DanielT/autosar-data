use smallvec::smallvec;
use std::{borrow::Cow, time::Duration};

use crate::element::ElementSortKey;
use autosar_data_specification::{
    AttributeName, AttributeSpec, AutosarVersion, ContentMode, ElementMultiplicity, ElementName,
};
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::collections::HashSet;
use std::sync::Arc;

use crate::{
    Attribute, AutosarDataError, AutosarModel, CharacterData, Element, ElementContent, ElementOrModel, ElementRaw,
    WeakElement,
};

/// `ElementRaw` provides the internal implementation of (almost) all Element operations
///
/// To get an `ElementRaw` the Element operation must lock the element, so all of these operations run with at least one lock held.
///
/// Note regarding deadlock avoidance:
/// Consider the case where two element operations are started in parallel on different threads.
/// One calls file() or path() and traverses the hierarchy of elements upward
/// root <- element <- element <- current element (locked)
///
/// The other calls e.g. `create_copied_sub_element`() or `remove_sub_element`() and wants to lock all of its sub elements
/// root -> current element (locked) -> element -> element
///
/// These two operations could deadlock if they operate on the same tree of elements.
/// To avoid this, parent element locks can only be acquired with `try_lock`(). If the lock is not acquired within a
/// reasonable time (10ms is used here), then the operation aborts with a `ParentElementLocked` error.
impl ElementRaw {
    /// get the parent element of the current element
    pub(crate) fn parent(&self) -> Result<Option<Element>, AutosarDataError> {
        match &self.parent {
            ElementOrModel::Element(parent) => {
                // for items that should have a parent, getting it is not allowed to return None
                let parent = parent.upgrade().ok_or(AutosarDataError::ItemDeleted)?;
                Ok(Some(parent))
            }
            ElementOrModel::Model(_) => Ok(None),
            ElementOrModel::None => Err(AutosarDataError::ItemDeleted),
        }
    }

    pub(crate) fn set_parent(&mut self, new_parent: ElementOrModel) {
        self.parent = new_parent;
    }

    /// get the [`ElementName`] of the element
    pub(crate) fn element_name(&self) -> ElementName {
        self.elemname
    }

    /// get the name of an identifiable element
    pub(crate) fn item_name(&self) -> Option<String> {
        // is this element named in any autosar version? - If it's not named here then we'll simply fail in the next step
        if self.elemtype.is_named() {
            // if an item is named, then the SHORT-NAME sub element that contains the name is always the first sub element
            if let Some(ElementContent::Element(subelem)) = self.content.first() {
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
        model: &AutosarModel,
        version: AutosarVersion,
    ) -> Result<(), AutosarDataError> {
        if let Some(current_name) = self.item_name() {
            // bail out early if the name is actually the same
            if current_name == new_name {
                return Ok(());
            }

            let old_path = self.path()?;
            let new_path = format!("{}{new_name}", old_path.strip_suffix(&current_name).unwrap());
            if model.get_element_by_path(&new_path).is_some() {
                return Err(AutosarDataError::DuplicateItemName);
            }

            // if an item is named, then the SHORT-NAME sub element that contains the name is always the first sub element
            if let Some(ElementContent::Element(subelem_wrapped)) = self.content.first() {
                let mut subelem = subelem_wrapped.0.lock();
                if subelem.element_name() == ElementName::ShortName {
                    subelem.set_character_data(CharacterData::String(new_name.to_owned()), version)?;
                    model.fix_identifiables(&old_path, &new_path);
                    let new_prefix = new_path;
                    let mut model_locked = model.0.lock();

                    // check all references and update those that point to this element or its sub elements
                    let refpaths = model_locked.reference_origins.keys().cloned().collect::<Vec<String>>();
                    for refpath in refpaths {
                        // if the existing reference has the old path as a prefix, then it needs to be updated
                        if let Some(partial_path) = refpath.strip_prefix(&old_path) {
                            // prevent ref updates from being applied to e.g. /package10 while renaming /package1
                            if partial_path.is_empty() || partial_path.starts_with('/') {
                                if let Some(reflist) = model_locked.reference_origins.remove(&refpath) {
                                    let refpath_new = format!("{new_prefix}{partial_path}");

                                    for weak_ref_elem in &reflist {
                                        if let Some(ref_elem) = weak_ref_elem.upgrade() {
                                            let mut ref_elem_locked = ref_elem.0.lock();
                                            // can't use .set_character_data() here, because the model is locked
                                            ref_elem_locked.content[0] = ElementContent::CharacterData(
                                                CharacterData::String(refpath_new.clone()),
                                            );
                                        }
                                    }
                                    model_locked.reference_origins.insert(refpath_new, reflist);
                                }
                            }
                        }
                    }
                }
            }

            Ok(())
        } else {
            Err(AutosarDataError::ElementNotIdentifiable {
                xmlpath: self.xml_path(),
            })
        }
    }

    /// returns true if the element is identifiable
    pub(crate) fn is_identifiable(&self) -> bool {
        // is this element named in any autosar version? - If it's not named here then we'll simply fail in the next step
        if self.elemtype.is_named() {
            // if an item is named, then the SHORT-NAME sub element that contains the name is always the first sub element
            if let Some(ElementContent::Element(subelem)) = self.content.first() {
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
            Err(AutosarDataError::ElementNotIdentifiable {
                xmlpath: self.xml_path(),
            })
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

    pub(crate) fn xml_path(&self) -> String {
        let mut path_components = vec![];

        if let Some(name) = self.item_name() {
            path_components.push(name.to_string());
        } else {
            path_components.push(format!("<{}>", self.element_name()));
        }

        if let Ok(mut cur_elem_opt) = self.parent() {
            while let Some(cur_elem) = cur_elem_opt {
                if let Some(locked_cur_elem) = cur_elem.0.try_lock_for(std::time::Duration::from_millis(10)) {
                    // build a string for this path component
                    if locked_cur_elem.elemtype.is_named() {
                        if let Some(item_name) = locked_cur_elem.item_name() {
                            path_components.push(item_name.to_string());
                        } else {
                            path_components.push(format!("<{}>(name missing)", locked_cur_elem.element_name()));
                        }
                    } else {
                        path_components.push(format!("<{}>", locked_cur_elem.element_name()));
                    }
                    // try to get the parent of the current element
                    match &locked_cur_elem.parent {
                        ElementOrModel::Element(weak_parent) => {
                            if let Some(parent) = weak_parent.upgrade() {
                                cur_elem_opt = Some(parent);
                            } else {
                                // failed to upgrade the weak ref to the parent - this can't happen in single threaded execution,
                                // but it could be possible when racing remove_element on another thread
                                path_components.push("(DELETED)".to_string());
                                cur_elem_opt = None;
                            }
                        }
                        ElementOrModel::Model(_) => {
                            // done
                            cur_elem_opt = None;
                        }
                        ElementOrModel::None => {
                            // this happens if xml_path is called on an element which has been deleted
                            // remove_element sets the parent to None
                            path_components.push("(DELETED)".to_string());
                            cur_elem_opt = None;
                        }
                    }
                } else {
                    path_components.push("(LOCKED)".to_string());
                    cur_elem_opt = None;
                }
            }
        } else {
            path_components.push("(DELETED)".to_string());
        }

        path_components.push(String::new());
        path_components.reverse();
        path_components.join("/")
    }

    /// Create a sub element at a suitable insertion position
    ///
    /// The given `ElementName` must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// It is not possible to create named sub elements with this function; use `create_named_sub_element`() for that instead.
    pub(crate) fn create_sub_element(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let (_start_pos, end_pos) = self.calc_element_insert_range(element_name, version)?;
        self.create_sub_element_inner(self_weak, element_name, end_pos, version)
    }

    /// Create a sub element at the specified insertion position
    ///
    /// The given `ElementName` must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// It is not possible to create named sub elements with this function; use `create_named_sub_element_at`() for that instead.
    /// The specified insertion position will be compared to the range of valid insertion positions; if it falls ooutside that range then the function fails.
    pub(crate) fn create_sub_element_at(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        position: usize,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let (start_pos, end_pos) = self.calc_element_insert_range(element_name, version)?;
        if start_pos <= position && position <= end_pos {
            self.create_sub_element_inner(self_weak, element_name, position, version)
        } else {
            Err(AutosarDataError::InvalidPosition)
        }
    }

    /// helper function for `create_sub_element` and `create_sub_element_at`
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
            let sub_element = ElementRaw {
                parent: ElementOrModel::Element(self_weak),
                elemname: element_name,
                elemtype,
                content: smallvec![],
                attributes: smallvec![],
                file_membership: HashSet::with_capacity(0),
                comment: None,
            }
            .wrap();
            self.content
                .insert(position, ElementContent::Element(sub_element.clone()));
            Ok(sub_element)
        }
    }

    /// create a named/identifiable sub element at a suitable insertion position
    ///
    /// The given `ElementName` must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// This method can only be used to create identifiable sub elements.
    pub(crate) fn create_named_sub_element(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        item_name: &str,
        model: &AutosarModel,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let (_start_pos, end_pos) = self.calc_element_insert_range(element_name, version)?;
        self.create_named_sub_element_inner(self_weak, element_name, item_name, end_pos, model, version)
    }

    /// create a named/identifiable sub element at the specified insertion position
    ///
    /// The given `ElementName` must be allowed on a sub element in this element, taking into account any sub elements that may already exist.
    /// The specified insertion position will be compared to the range of valid insertion positions; if it falls ooutside that range then the function fails.
    /// This method can only be used to create identifiable sub elements.
    pub(crate) fn create_named_sub_element_at(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        item_name: &str,
        position: usize,
        model: &AutosarModel,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let (start_pos, end_pos) = self.calc_element_insert_range(element_name, version)?;
        if start_pos <= position && position <= end_pos {
            self.create_named_sub_element_inner(self_weak, element_name, item_name, position, model, version)
        } else {
            Err(AutosarDataError::InvalidPosition)
        }
    }

    /// helper function for `create_named_sub_element` and `create_named_sub_element_at`
    fn create_named_sub_element_inner(
        &mut self,
        self_weak: WeakElement,
        element_name: ElementName,
        item_name: &str,
        position: usize,
        model: &AutosarModel,
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
                .is_some_and(|cdata_spec| CharacterData::check_value(&item_name_cdata, cdata_spec, version))
            {
                return Err(AutosarDataError::IncorrectContentType);
            }

            let parent_path = self.path_unchecked()?;
            let path = format!("{parent_path}/{item_name}");
            // verify that the name is unique: there must be no existing element that already has this autosar path
            if model.get_element_by_path(&path).is_some() {
                return Err(AutosarDataError::DuplicateItemName);
            }

            // create the new element
            let sub_element = ElementRaw {
                parent: ElementOrModel::Element(self_weak),
                elemname: element_name,
                elemtype,
                content: smallvec![],
                attributes: smallvec![],
                file_membership: HashSet::with_capacity(0),
                comment: None,
            }
            .wrap();
            self.content
                .insert(position, ElementContent::Element(sub_element.clone()));
            // create a SHORT-NAME for the sub element
            let shortname_element =
                sub_element
                    .0
                    .lock()
                    .create_sub_element(sub_element.downgrade(), ElementName::ShortName, version)?;
            let _ = shortname_element.0.lock().set_character_data(item_name_cdata, version);
            model.add_identifiable(path, sub_element.downgrade());
            Ok(sub_element)
        } else {
            Err(AutosarDataError::ElementNotIdentifiable {
                xmlpath: self.xml_path(),
            })
        }
    }

    /// create a deep copy of the given element and insert it as a sub-element
    pub(crate) fn create_copied_sub_element(
        &mut self,
        self_weak: WeakElement,
        other: &Element,
        model: &AutosarModel,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let other_elemname = {
            // need to do this inside its own scope to limit the lifetime of the mutex
            let other_element = other.0.lock();
            other_element.elemname
        };
        let (_, end) = self.calc_element_insert_range(other_elemname, version)?;
        self.create_copied_sub_element_inner(self_weak, other, end, model, version)
    }

    /// create a deep copy of the given element and insert it as a sub-element at the given position
    pub(crate) fn create_copied_sub_element_at(
        &mut self,
        self_weak: WeakElement,
        other: &Element,
        position: usize,
        model: &AutosarModel,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let other_elemname = {
            // need to do this inside its own scope to limit the lifetime of the mutex
            let other_element = other.0.lock();
            other_element.elemname
        };
        let (start_pos, end_pos) = self.calc_element_insert_range(other_elemname, version)?;
        if start_pos <= position && position <= end_pos {
            self.create_copied_sub_element_inner(self_weak, other, position, model, version)
        } else {
            Err(AutosarDataError::InvalidPosition)
        }
    }

    fn create_copied_sub_element_inner(
        &mut self,
        self_weak: WeakElement,
        other: &Element,
        position: usize,
        model: &AutosarModel,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        // check if self (target of the move) is a sub element of new_element
        // if it is, then the move is not allowed
        let mut wrapped_parent = self.parent.clone();
        while let ElementOrModel::Element(weak_parent) = wrapped_parent {
            let parent = weak_parent.upgrade().ok_or(AutosarDataError::ItemDeleted)?;
            if parent == *other {
                return Err(AutosarDataError::ForbiddenCopyOfParent);
            }
            wrapped_parent = parent.0.lock().parent.clone();
        }

        // Arc overrides clone() so that it only manipulates the reference count, so a separate deep_copy operation is needed here.
        // Additionally, implementing this manually provides the opportunity to filter out
        // elements that ae not compatible with the version of the current file.
        let newelem = other.0.lock().deep_copy(version)?;
        let path = self.path_unchecked()?;

        // set the parent of the newelem - the methods path(), containing_file(), etc become available on newelem
        newelem.set_parent(ElementOrModel::Element(self_weak));
        if newelem.is_identifiable() {
            newelem.0.lock().make_unique_item_name(model, &path)?;
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
                    .filter_map(std::clone::Clone::clone)
                    .collect::<Vec<String>>()
                    .join("/");
                model.add_identifiable(sub_elem_path, sub_elem.downgrade());
            } else {
                path_parts.push(None);
            }
            // add all references to the reference_origins hashmap
            if sub_elem.is_reference() {
                if let Some(CharacterData::String(reference)) = sub_elem.character_data() {
                    model.add_reference_origin(&reference, sub_elem.downgrade());
                }
            }
        }

        self.content.insert(position, ElementContent::Element(newelem.clone()));

        Ok(newelem)
    }

    /// perform a deep copy of an element, but keep only those sub elements etc, which are compatible with `target_version`
    fn deep_copy(&self, target_version: AutosarVersion) -> Result<Element, AutosarDataError> {
        let copy_wrapped = ElementRaw {
            elemname: self.elemname,
            elemtype: self.elemtype,
            content: SmallVec::with_capacity(self.content.len()),
            attributes: SmallVec::with_capacity(self.attributes.len()),
            parent: ElementOrModel::None,
            file_membership: HashSet::with_capacity(0),
            comment: self.comment.clone(),
        }
        .wrap();

        {
            let mut copy = copy_wrapped.0.lock();
            // copy all the attributes
            for attribute in &self.attributes {
                // get the specification of the attribute
                let AttributeSpec {
                    spec: cdataspec,
                    required,
                    version: attr_version_mask,
                } = self.elemtype.find_attribute_spec(attribute.attrname).ok_or(
                    AutosarDataError::VersionIncompatibleData {
                        version: target_version,
                    },
                )?;
                // check if the attribute is compatible with the target version
                if target_version.compatible(attr_version_mask)
                    && attribute
                        .content
                        .check_version_compatibility(cdataspec, target_version)
                        .0
                {
                    copy.attributes.push(attribute.clone());
                } else if required {
                    return Err(AutosarDataError::VersionIncompatibleData {
                        version: target_version,
                    });
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
                                copied_sub_elem.0.lock().parent = ElementOrModel::Element(copy_wrapped.downgrade());
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

    /// `make_unique_item_name` ensures that a copied element has a unique name
    fn make_unique_item_name(&self, model: &AutosarModel, parent_path: &str) -> Result<String, AutosarDataError> {
        let orig_name = self.item_name().ok_or(AutosarDataError::ElementNotIdentifiable {
            xmlpath: self.xml_path(),
        })?;
        let mut name = orig_name.clone();
        let mut counter = 1;

        let mut path = format!("{parent_path}/{orig_name}");
        while model.get_element_by_path(&path).is_some() {
            name = format!("{orig_name}_{counter}");
            counter += 1;
            path = format!("{parent_path}/{name}");
        }

        if counter > 1 {
            // set the name directly by modifying the character content of the short name element
            // note: the method set_character_data is not suitable here, because it updates the identifiables hashmap
            if let Some(ElementContent::Element(short_name_elem)) = self.content.first() {
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
    /// The moved element can be taken from anywhere - even from a different arxml document that is not part of the same `AutosarModel`
    ///
    /// Restrictions:
    /// 1) The element must have a compatible element type. If it could not have been created here, then it can't be moved either.
    /// 2) The origin document of the element must have exactly the same `AutosarVersion` as the destination.
    pub(crate) fn move_element_here(
        &mut self,
        self_weak: WeakElement,
        move_element: &Element,
        model: &AutosarModel,
        model_src: &AutosarModel,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let move_element_name = move_element.element_name();
        let (_, end_pos) = self.calc_element_insert_range(move_element_name, version)?;

        if model == model_src {
            let src_parent = move_element.parent()?.ok_or(AutosarDataError::InvalidSubElement)?;
            if src_parent.downgrade() == self_weak {
                Ok(move_element.clone())
            } else {
                // move the element within the same model
                self.move_element_local(self_weak, move_element, end_pos, model, version)
            }
        } else {
            // move the element between different models
            self.move_element_full(self_weak, move_element, end_pos, model, model_src, version)
        }
    }

    /// take an `element` from it's current location and place it at the given position in this element as a sub element
    ///
    /// The moved element can be taken from anywhere - even from a different arxml document that is not part of the same `AutosarModel`
    ///
    /// Restrictions:
    /// 1) The element must have a compatible element type. If it could not have been created here, then it can't be moved either.
    /// 2) The origin document of the element must have exactly the same `AutosarVersion` as the destination.
    pub(crate) fn move_element_here_at(
        &mut self,
        self_weak: WeakElement,
        move_element: &Element,
        position: usize,
        model: &AutosarModel,
        model_src: &AutosarModel,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let move_element_name = move_element.element_name();
        let (start_pos, end_pos) = self.calc_element_insert_range(move_element_name, version)?;

        if start_pos <= position && position <= end_pos {
            if model == model_src {
                let src_parent = move_element.parent()?.ok_or(AutosarDataError::InvalidSubElement)?;
                if src_parent.downgrade() == self_weak {
                    // move new_element to a different position within the current element
                    self.move_element_position(move_element, position)
                } else {
                    // move the element within the same model
                    self.move_element_local(self_weak, move_element, position, model, version)
                }
            } else {
                // move the element between different models
                self.move_element_full(self_weak, move_element, position, model, model_src, version)
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
                .position(|item| {
                    if let ElementContent::Element(elem) = item {
                        *elem == *move_element
                    } else {
                        false
                    }
                })
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
    // the current location must be within the same model
    // All references to the moved sub element will be updated to refer to the new path
    fn move_element_local(
        &mut self,
        self_weak: WeakElement,
        move_element: &Element,
        position: usize,
        model: &AutosarModel,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        // check if self (target of the move) is a sub element of new_element
        // if it is, then the move is not allowed
        let mut wrapped_parent = self.parent.clone();
        while let ElementOrModel::Element(weak_parent) = wrapped_parent {
            let parent = weak_parent.upgrade().ok_or(AutosarDataError::ItemDeleted)?;
            if parent == *move_element {
                return Err(AutosarDataError::ForbiddenMoveToSubElement);
            }
            wrapped_parent = parent.0.lock().parent.clone();
        }

        let src_parent = move_element.parent()?.ok_or(AutosarDataError::InvalidSubElement)?;

        // collect the paths of all identifiable elements under new_element before moving it
        let original_paths: Vec<String> = move_element
            .elements_dfs()
            .filter_map(|(_, e)| {
                if e.element_type().is_named() {
                    e.path().ok()
                } else {
                    None
                }
            })
            .collect();

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
                .position(|item| {
                    if let ElementContent::Element(elem) = item {
                        *elem == *move_element
                    } else {
                        false
                    }
                })
                .unwrap();
            src_parent_locked.content.remove(idx);
        }

        // set the parent of the new element to the current element
        move_element.0.lock().parent = ElementOrModel::Element(self_weak);
        let dest_path = if move_element.is_identifiable() {
            let new_name = move_element.0.lock().make_unique_item_name(model, &dest_path_prefix)?;
            format!("{dest_path_prefix}/{new_name}")
        } else {
            dest_path_prefix
        };

        // fix the identifiables cache
        if move_element.is_identifiable() {
            // simple case: the moved element is identifiable; fix_identifiables automatically handles the sub-elements
            model.fix_identifiables(&src_path_prefix, &dest_path);
        } else {
            // the moved element is not identifiable, so its identifiable sub-elements must be fixed individually
            for orig_path in &original_paths {
                if let Some(suffix) = orig_path.strip_prefix(&src_path_prefix) {
                    let updated_path = format!("{dest_path}{suffix}");
                    model.fix_identifiables(orig_path, &updated_path);
                }
            }
        }

        // the move_element was moved within this autosar model, so we can update all other references pointing to it
        let mut model_locked = model.0.lock();
        for orig_ref in &original_paths {
            if let Some(suffix) = orig_ref.strip_prefix(&src_path_prefix) {
                // e.g. orig_ref = "/Pkg/Foo/Sub/Element" and src_path_prefix = "/Pkg/Foo" then suffix = "/Sub/Element"
                // strip prefix can't fail, because all original_paths have the src_path_prefix
                if let Some(ref_elements) = model_locked.reference_origins.remove(orig_ref) {
                    let refstr = format!("{dest_path}{suffix}");
                    for ref_element_weak in &ref_elements {
                        if let Some(ref_element) = ref_element_weak.upgrade() {
                            ref_element
                                .0
                                .lock()
                                .set_character_data(CharacterData::String(refstr.clone()), version)?;
                        }
                    }
                    model_locked.reference_origins.insert(refstr, ref_elements);
                }
            }
        }

        // insert move_element
        self.content
            .insert(position, ElementContent::Element(move_element.clone()));

        Ok(move_element.clone())
    }

    // remove an element from its current parent and make it a sub element of this element
    // This is a move between two different models
    // If the moved sub element contains its own tree of sub elements, then references within that tree will be updated
    fn move_element_full(
        &mut self,
        self_weak: WeakElement,
        move_element: &Element,
        position: usize,
        model: &AutosarModel,
        model_src: &AutosarModel,
        version: AutosarVersion,
    ) -> Result<Element, AutosarDataError> {
        let src_path_prefix = move_element.0.lock().path_unchecked()?;
        let dest_path_prefix = self.path_unchecked()?;
        let src_parent = move_element.parent()?.ok_or(AutosarDataError::InvalidSubElement)?;

        // collect the paths of all identifiable elements under move_element before moving it
        let original_paths: FxHashMap<String, Element> = move_element
            .elements_dfs()
            .filter_map(|(_, e)| {
                if e.element_type().is_named() {
                    e.path().ok().map(|path| (path, e))
                } else {
                    None
                }
            })
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
                .position(|item| {
                    if let ElementContent::Element(elem) = item {
                        *elem == *move_element
                    } else {
                        false
                    }
                })
                .unwrap();
            src_parent_locked.content.remove(idx);
        }

        // remove all cached references for elements under move_element - they all become invalid as a result of moving it
        for path in original_paths.keys() {
            model_src.remove_identifiable(path);
        }
        // delete all reference origin info for elements under move_element
        for (path, elem) in &original_refs {
            model_src.remove_reference_origin(path, elem.downgrade());
        }

        // set the parent of the new element to the current element
        move_element.0.lock().parent = ElementOrModel::Element(self_weak);
        let dest_path = if move_element.is_identifiable() {
            let new_name = move_element.0.lock().make_unique_item_name(model, &dest_path_prefix)?;
            format!("{dest_path_prefix}/{new_name}")
        } else {
            dest_path_prefix
        };

        // cache references to all the identifiable elements in move_element
        for (orig_path, identifiable_element) in &original_paths {
            if let Some(suffix) = orig_path.strip_prefix(&src_path_prefix) {
                let path = format!("{dest_path}{suffix}");
                model.add_identifiable(path, identifiable_element.downgrade());
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
                model.add_reference_origin(&refstr, ref_element.downgrade());
            }
        }

        // insert move_element
        self.content
            .insert(position, ElementContent::Element(move_element.clone()));

        Ok(move_element.clone())
    }

    /// find the upper and lower bound on the insert position for a new sub element
    pub(crate) fn calc_element_insert_range(
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
            // element_name is a valid sub-element according to the specification; new_element_inidices describes the element grouping and ordering
            let mut start_pos = 0;
            let mut end_pos = 0;
            // compare the new element to the existing elements
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

    /// remove the sub element `sub_element`
    ///
    /// The `sub_element` will be unlinked from the hierarchy of elements. All of the sub-sub-elements nested under the removed element will also be recusively removed.
    /// Since all elements are reference counted, they might not be deallocated immediately, however they do become invalid and unusable immediately.
    pub(crate) fn remove_sub_element(
        &mut self,
        sub_element: Element,
        model: &AutosarModel,
    ) -> Result<(), AutosarDataError> {
        let path = Cow::from(self.path_unchecked()?);
        let mut sub_element_locked = sub_element.0.lock();
        // find the position of sub_element in the parent element first to verify that sub_element actuall *is* a sub element
        let pos = self
            .content
            .iter()
            .position(|item| {
                if let ElementContent::Element(elem) = item {
                    *elem == sub_element
                } else {
                    false
                }
            })
            .ok_or(AutosarDataError::ElementNotFound)?;
        if self.elemtype.is_named() && sub_element_locked.elemname == ElementName::ShortName {
            // may not remove the SHORT-NAME, because that would leave the data in an invalid state
            return Err(AutosarDataError::ShortNameRemovalForbidden);
        }
        sub_element_locked.remove_internal(sub_element.downgrade(), model, path);
        self.content.remove(pos);
        Ok(())
    }

    // remove all of the content of an element
    pub(crate) fn remove_internal(&mut self, self_weak: WeakElement, model: &AutosarModel, mut path: Cow<str>) {
        if self.is_identifiable() {
            if let Some(name) = self.item_name() {
                let mut new_path = String::with_capacity(path.len() + name.len() + 1);
                new_path.push_str(&path);
                new_path.push('/');
                new_path.push_str(&name);
                path = Cow::from(new_path.clone());

                model.remove_identifiable(&path);
            }
        }
        if self.elemtype.is_ref() {
            if let Some(CharacterData::String(reference)) = self.character_data() {
                // remove the references-reference (ugh. terminology???)
                model.remove_reference_origin(&reference, self_weak);
            }
        }
        for item in &self.content {
            if let ElementContent::Element(sub_element) = item {
                sub_element
                    .0
                    .lock()
                    .remove_internal(sub_element.downgrade(), model, Cow::from(path.as_ref()));
            }
        }
        self.content.clear();
        self.parent = ElementOrModel::None;
    }

    /// set the character data of this element
    ///
    /// This method only applies to elements which contain character data, i.e. `element.content_type` == `CharacterData`,
    /// or elements with `element.content_type` == Mixed, but which only contain a single `CharacterData` item
    pub(crate) fn set_character_data(
        &mut self,
        chardata: CharacterData,
        version: AutosarVersion,
    ) -> Result<(), AutosarDataError> {
        if self.elemtype.content_mode() == ContentMode::Characters
            || (self.elemtype.content_mode() == ContentMode::Mixed && self.content.len() <= 1)
        {
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
    /// This method only applies to elements which contain character data, i.e. `element.content_type` == `CharacterData`,
    /// or elements with `element.content_type` == Mixed, but which only contain a single `CharacterData` item
    pub(crate) fn character_data(&self) -> Option<CharacterData> {
        if self.content.len() == 1
            && (self.elemtype.content_mode() == ContentMode::Characters
                || self.elemtype.content_mode() == ContentMode::Mixed)
        {
            if let Some(ElementContent::CharacterData(cdata)) = self.content.first() {
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

    pub(crate) fn set_attribute_internal(
        &mut self,
        attrname: AttributeName,
        value: CharacterData,
        file_version: AutosarVersion,
    ) -> Result<(), AutosarDataError> {
        // find the attribute specification in the item type
        if let Some(AttributeSpec { spec, .. }) = self.elemtype.find_attribute_spec(attrname) {
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
        if let Some(AttributeSpec {
            spec: character_data_spec,
            ..
        }) = self.elemtype.find_attribute_spec(attrname)
        {
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
                if let Some(AttributeSpec { required, .. }) = self.elemtype.find_attribute_spec(attrname) {
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

    /// sort all sub-elements of this element
    pub(crate) fn sort(&mut self) {
        match self.elemtype.content_mode() {
            ContentMode::Sequence | ContentMode::Choice | ContentMode::Bag => {
                // sort the content if sorting is allowed (!ordered) and there is more than one child element
                let len = self.content.len();
                if !self.elemtype.is_ordered() && len > 1 {
                    // remove all child elements from this element and sort them
                    let mut sorting_vec: Vec<(Vec<usize>, ElementSortKey, Element)> = Vec::with_capacity(len);
                    for ec_elem in &self.content {
                        if let ElementContent::Element(elem) = ec_elem {
                            // descend into the element and sort it before doing anything else with it
                            elem.sort();
                            let (_, elem_indices) =
                                self.elemtype.find_sub_element(elem.element_name(), u32::MAX).unwrap();
                            sorting_vec.push((elem_indices, ElementSortKey::default(), elem.clone()));
                        }
                        // Sequence, Choice and Bag do not have character content, so else {} is not needed
                    }

                    // basic sort, typically a no-op
                    sorting_vec
                        .sort_by(|(elem_indices_a, _, _), (elem_indices_b, _, _)| elem_indices_a.cmp(elem_indices_b));
                    // try to find out if there are any conflicts during basic sorting, i.e. items that need more info to sort
                    let mut sort_key_needed: Vec<bool> = vec![false; len];
                    for idx in 1..len {
                        let (prev_elem_indices, _, _) = &sorting_vec[idx - 1];
                        let (elem_indices, _, _) = &sorting_vec[idx];
                        if elem_indices == prev_elem_indices {
                            sort_key_needed[idx - 1] = true;
                            sort_key_needed[idx] = true;
                        }
                    }
                    // if any sub element needs sort-keys (i.e. there is a conflict) then these sort keys are generated and the list is sorted again
                    if sort_key_needed.iter().any(|&val| val) {
                        for idx in 0..len {
                            if sort_key_needed[idx] {
                                sorting_vec[idx].1 = sorting_vec[idx].2.get_sort_key();
                            }
                        }
                        // sort again, taking into account the sorting keys
                        sorting_vec.sort_by(|(elem_indices_a, sort_key_a, _), (elem_indices_b, sort_key_b, _)| {
                            match elem_indices_a.cmp(elem_indices_b) {
                                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                std::cmp::Ordering::Equal => sort_key_a.cmp(sort_key_b),
                                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                            }
                        });
                    }

                    self.content.clear();
                    // put the sorted elements back
                    for (_, _, elem) in sorting_vec {
                        self.content.push(ElementContent::Element(elem));
                    }
                } else {
                    // 0 or 1 content items -or- the element is ordered and sorting it is forbidden.
                    // in either case we need to descend into the child element(s)
                    for ec in &self.content {
                        if let ElementContent::Element(elem) = ec {
                            elem.sort();
                        }
                    }
                }
            }
            ContentMode::Characters | ContentMode::Mixed => {}
        }
    }

    pub(crate) fn wrap(self) -> Element {
        Element(Arc::new(Mutex::new(self)))
    }
}
