//! Specification of the Autosar arxml file format in the form of rust data structures
//!
//! This crate exists to support the autosar-data crate.
//!
//! The Autosar data model is originally specified as .xsd files - one for each version of the standard.
//! All these separate xsd files were parsed into data structures and combined; this crate contains the
//! combined specification data of all 19 Autosar 4 standard revisions.
//!
//! ## Supported standards:
//!
//! | xsd filename      | description               |
//! |-------------------|---------------------------|
//! | AUTOSAR_4-0-1.xsd | AUTOSAR 4.0.1             |
//! | AUTOSAR_4-0-2.xsd | AUTOSAR 4.0.2             |
//! | AUTOSAR_4-0-3.xsd | AUTOSAR 4.0.3             |
//! | AUTOSAR_4-1-1.xsd | AUTOSAR 4.1.1             |
//! | AUTOSAR_4-1-2.xsd | AUTOSAR 4.1.2             |
//! | AUTOSAR_4-1-3.xsd | AUTOSAR 4.1.3             |
//! | AUTOSAR_4-2-1.xsd | AUTOSAR 4.2.1             |
//! | AUTOSAR_4-2-2.xsd | AUTOSAR 4.2.2             |
//! | AUTOSAR_4-3-0.xsd | AUTOSAR 4.3.0             |
//! | AUTOSAR_00042.xsd | AUTOSAR Adaptive 17-03    |
//! | AUTOSAR_00043.xsd | AUTOSAR Adaptive 17-10    |
//! | AUTOSAR_00044.xsd | AUTOSAR Classic 4.3.1     |
//! | AUTOSAR_00045.xsd | AUTOSAR Adaptive 18-03    |
//! | AUTOSAR_00046.xsd | AUTOSAR Classic 4.4.0 / Adaptive 18-10 |
//! | AUTOSAR_00047.xsd | AUTOSAR Adaptive 19-03    |
//! | AUTOSAR_00048.xsd | AUTOSAR 4.5.0             |
//! | AUTOSAR_00049.xsd | AUTOSAR R20-11            |
//! | AUTOSAR_00050.xsd | AUTOSAR R21-11            |
//! | AUTOSAR_00051.xsd | AUTOSAR R22-11            |
//!
//! ## Using the crate
//!
//! The main datatype is the [ElementType]. The type of the &lt;AUTOSAR&gt; element at the root of every arxml file is
//! available as ElementType::ROOT.
//!
//! ## Note
//!
//! It is not possible to directly convert between [ElementName]s and [ElementType]s, since this is an n:m mapping.
//! If the content of two differently named elements is structurally identical, then they have the same [ElementType];
//! on the other side there are several elements that have different content depending in the context in which they appear.
//!
//! ## Example
//!
//! ```
//! # use autosar_data_specification::*;
//! # use std::str::FromStr;
//! let root_type = ElementType::ROOT;
//!
//! // parsing an element
//! let element_name_text = "AR-PACKAGES";
//! let element_name = ElementName::from_str(element_name_text).unwrap();
//! assert_eq!(element_name, ElementName::ArPackages);
//!
//! let version_mask = AutosarVersion::Autosar_4_3_0 as u32;
//! if let Some((element_type, index_list)) = root_type.find_sub_element(
//!     element_name,
//!     version_mask
//! ) {
//!     // parsing an attribute
//!     let attribute_name = AttributeName::from_str("UUID").unwrap();
//!     if let Some(attribute_spec) = element_type.find_attribute_spec(attribute_name) {
//!         // ...
//!     }
//!
//!     // ...
//! }
//! ```

mod attributename;
mod autosarversion;
mod elementname;
mod enumitem;
mod regex;
mod specification;

use std::ops::BitXor;

pub use attributename::AttributeName;
pub use autosarversion::AutosarVersion;
pub use elementname::ElementName;
pub use enumitem::EnumItem;
use specification::*;

/// ElementMultiplicity specifies how often a single child element may occur within its parent
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ElementMultiplicity {
    ZeroOrOne,
    One,
    Any,
}

/// The ContentMode specifies what content may occur inside an element
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ContentMode {
    /// Sequence: an ordered sequence of elements
    Sequence,
    /// Choice: a single element must be chosen from multiple options.
    /// If the multiplicity of the chosen element is `Any` then it may repeat so there might still be more than one sub element
    Choice,
    /// Bag: From a list of choices, choose a sub element any number of times.
    /// In this ContentMode all allowed sub elements may occur any number of times and in any order
    Bag,
    /// Characters: no sub elements are permitted, there can only be character content
    Characters,
    /// Mixed: both characters content and sub elements are allowed, in any order. It's basically like HTML
    Mixed,
}

/// Specifies the data type and restrictions of the character data in an element or attribute
pub enum CharacterDataSpec {
    /// The character data is an enum value; valid values are given in items and the character data must match one of these
    Enum {
        items: &'static [(EnumItem, u32)],
    },
    /// The character data is restricted to match a regex, which is given in text form in the filed regex.
    /// The check_fn is a function that validates input according to the regex.
    /// If a max_length is given, then it restricts the length (in bytes).
    Pattern {
        check_fn: fn(&[u8]) -> bool,
        regex: &'static str,
        max_length: Option<usize>,
    },
    /// An arbitrary string; if preserve whitepace is set, then whitespace should be preserved during parsing (see the XML standard)
    String {
        preserve_whitespace: bool,
        max_length: Option<usize>,
    },
    UnsignedInteger,
    Double,
}

/// specification of an attribute
pub struct AttributeSpec {
    /// data type of the attribute content
    pub spec: &'static CharacterDataSpec,
    /// is the attribute required to be present in it's containing element
    pub required: bool,
    /// in which autosar version(s) is this attribute valid. This field is a bitmask.
    pub version: u32,
}

/// ElementType is an abstraction over element types in the specification.
///
/// It provides no public fields, but it has methods to get all the info needed to parse an arxml element.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct ElementType(usize);

enum SubElement {
    Element {
        name: ElementName,
        elemtype: usize,
        multiplicity: ElementMultiplicity,
    },
    Group {
        groupid: usize,
    },
}

struct ElementSpec {
    sub_elements: (u16, u16),
    sub_element_ver: u16,
    attributes: (u16, u16),
    attributes_ver: u16,
    character_data: Option<u16>,
    mode: ContentMode,
    ordered: bool,
    splittable: u32,
}

impl AutosarVersion {
    pub fn compatible(&self, version_mask: u32) -> bool {
        version_mask & *self as u32 != 0
    }
}

impl ElementType {
    fn get_sub_elements(&self) -> &'static [SubElement] {
        let (idx_start, idx_end) = self.get_sub_element_idx();
        &SUBELEMENTS[idx_start..idx_end]
    }

    fn get_sub_element_idx(&self) -> (usize, usize) {
        let (start, end) = DATATYPES[self.0].sub_elements;
        (start as usize, end as usize)
    }

    fn get_sub_element_ver(&self) -> usize {
        DATATYPES[self.0].sub_element_ver as usize
    }

    fn get_attributes_idx(&self) -> (usize, usize) {
        let (start, end) = DATATYPES[self.0].attributes;
        (start as usize, end as usize)
    }

    fn get_attributes_ver(&self) -> usize {
        DATATYPES[self.0].attributes_ver as usize
    }

    /// get the spec of a sub element from the index list
    fn get_sub_element_spec<'a>(&self, element_indices: &[usize]) -> Option<(&'a SubElement, u32)> {
        let spec = self.get_sub_elements();
        let ver_list_start = self.get_sub_element_ver();
        if !element_indices.is_empty() {
            let mut current_spec = spec;
            let mut current_ver_list_start = ver_list_start;
            // go through the hierarchy of groups: only the final index in element_indices can refer to a SubElement::Element
            for idx in 0..(element_indices.len() - 1) {
                match &current_spec[element_indices[idx]] {
                    SubElement::Element { .. } => {
                        // elements are not allowed here
                        return None;
                    }
                    SubElement::Group { groupid } => {
                        current_spec = ElementType(*groupid).get_sub_elements();
                        current_ver_list_start = ElementType(*groupid).get_sub_element_ver();
                    }
                }
            }

            let last_idx = *element_indices.last().unwrap();
            Some((&current_spec[last_idx], VERSION_INFO[current_ver_list_start + last_idx]))
        } else {
            None
        }
    }

    /// get the version mask of a sub element
    pub fn get_sub_element_version_mask(&self, element_indices: &[usize]) -> Option<u32> {
        match self.get_sub_element_spec(element_indices) {
            Some((_, version_mask)) => Some(version_mask),
            _ => None,
        }
    }

    /// get the multiplicity of a sub element within the current ElementType
    ///
    /// The sub element is identified by an indx list, as returned by `find_sub_element()`
    pub fn get_sub_element_multiplicity(&self, element_indices: &[usize]) -> Option<ElementMultiplicity> {
        match self.get_sub_element_spec(element_indices) {
            Some((SubElement::Element { multiplicity, .. }, _)) => Some(*multiplicity),
            _ => None,
        }
    }

    /// get the ContentMode of the container of a sub element of the current ElementType
    ///
    /// The sub element is identified by an index list, as returned by `find_sub_element()`
    pub fn get_sub_element_container_mode(&self, element_indices: &[usize]) -> ContentMode {
        if element_indices.len() < 2 {
            // length == 1: this element is a direct sub element, without any groups;
            DATATYPES[self.0].mode
        } else {
            let len = element_indices.len() - 1;
            if let Some((SubElement::Group { groupid }, _)) = self.get_sub_element_spec(&element_indices[..len]) {
                DATATYPES[*groupid].mode
            } else {
                panic!("impossible: element container is not a group");
            }
        }
    }

    /// find a sub element in the specification of the current ElementType
    ///
    /// Note: Version here is NOT an AutosarVersion, it is a u32. it is a bitmask which can contain multiple AutosarVersions, or any version by using u32::MAX
    ///
    /// In almost all cases this is simple: there is a flat list of sub elements that either contains the target_name or not.
    /// The result in those simple cases is a vec with one entry which is the index of the element in the list.
    /// There are a handfull of complicated situations though, where the list of sub elements contains groups of
    /// elements that have a different ContentMode than the other elements.
    ///
    /// For example:
    /// ```text
    ///     PRM-CHAR (Sequence)
    ///      -> Element: COND
    ///      -> Group (Choice)
    ///         -> Group (Sequence)
    ///             -> Group (Choice)
    ///                 -> Group (Sequence)
    ///                     -> Element: ABS
    ///                     -> Element: TOL
    ///                 -> Group (Sequence)
    ///                     -> Element: MIN
    ///                     -> Element: TYP
    ///                     -> Element: MAX
    ///             -> Element: PRM-UNIT
    ///         -> Element: TEXT
    ///      -> Element: REMARK
    /// ```
    /// When searching for TOL in PRM-CHAR, the result should be Some(vec![1, 0, 0, 0, 1])!
    pub fn find_sub_element(&self, target_name: ElementName, version: u32) -> Option<(ElementType, Vec<usize>)> {
        let spec = self.get_sub_elements();
        for (cur_pos, sub_element) in spec.iter().enumerate() {
            match sub_element {
                SubElement::Element { name, elemtype, .. } => {
                    let ver_info_start = self.get_sub_element_ver();
                    let version_mask = VERSION_INFO[ver_info_start + cur_pos];
                    if (*name == target_name) && (version & version_mask != 0) {
                        return Some((ElementType(*elemtype), vec![cur_pos]));
                    }
                }
                SubElement::Group { groupid } => {
                    let group = ElementType(*groupid);
                    if let Some((elemtype, mut indices)) = group.find_sub_element(target_name, version) {
                        indices.insert(0, cur_pos);
                        return Some((elemtype, indices));
                    }
                }
            }
        }
        None
    }

    /// find the commmon group of two subelements of the current ElementType
    ///
    /// The subelements are identified by their index lists, returned by find_sub_element().
    ///
    /// In simple cases without sub-groups of elements, the "common group" is simply the current ElementType.
    pub fn find_common_group(&self, element_indices: &[usize], element_indices2: &[usize]) -> ElementType {
        let mut result = self.0;
        let mut prefix_len = 0;
        while element_indices.len() > prefix_len
            && element_indices2.len() > prefix_len
            && element_indices[prefix_len] == element_indices2[prefix_len]
        {
            let sub_elem = &ElementType(result).get_sub_elements()[element_indices[prefix_len]];
            match sub_elem {
                SubElement::Element { .. } => return ElementType(result),
                SubElement::Group { groupid } => {
                    result = *groupid;
                }
            }
            prefix_len += 1;
        }

        ElementType(result)
    }

    /// are elements of this ElementType named in any Autosar version
    pub fn is_named(&self) -> bool {
        self.short_name_version_mask().is_some()
    }

    pub(crate) fn short_name_version_mask(&self) -> Option<u32> {
        let (start_idx, end_idx) = DATATYPES[self.0].sub_elements;
        if start_idx != end_idx {
            if let SubElement::Element {
                name: ElementName::ShortName,
                ..
            } = SUBELEMENTS[start_idx as usize]
            {
                Some(VERSION_INFO[DATATYPES[self.0].sub_element_ver as usize])
            } else {
                None
            }
        } else {
            None
        }
    }

    /// are elements of this elementType named in the given Autosar version
    ///
    /// Named elements must have a SHORT-NAME sub element. For some elements this
    /// depends on the Autosar version.
    ///
    /// One example of this is END-2-END-METHOD-PROTECTION-PROPS, which was first
    /// defined in Autosar_00048, but only has a name in Autosar_00050.
    pub fn is_named_in_version(&self, version: AutosarVersion) -> bool {
        self.short_name_version_mask()
            .map_or(false, |ver_mask| version.compatible(ver_mask))
    }

    /// is the ElementType a reference
    pub fn is_ref(&self) -> bool {
        if let Some(idx) = DATATYPES[self.0].character_data {
            idx == REFERENCE_TYPE_IDX
        } else {
            false
        }
    }

    /// get the content mode for this ElementType
    pub fn content_mode(&self) -> ContentMode {
        DATATYPES[self.0].mode
    }

    /// get the character data spec for this ElementType
    pub fn chardata_spec(&self) -> Option<&'static CharacterDataSpec> {
        if let Some(chardata_id) = DATATYPES[self.0].character_data {
            Some(&CHARACTER_DATA[chardata_id as usize])
        } else {
            None
        }
    }

    /// find the spec for a single attribute by name
    pub fn find_attribute_spec(&self, attrname: AttributeName) -> Option<AttributeSpec> {
        let (idx_start, idx_end) = self.get_attributes_idx();
        let attributes = &ATTRIBUTES[idx_start..idx_end];
        if let Some((find_pos, (_, chardata_id, required))) =
            attributes.iter().enumerate().find(|(_, (name, ..))| *name == attrname)
        {
            let idx_ver_start = self.get_attributes_ver();
            let version = VERSION_INFO[idx_ver_start + find_pos];
            Some(AttributeSpec {
                spec: &CHARACTER_DATA[*chardata_id as usize],
                required: *required,
                version,
            })
        } else {
            None
        }
    }

    /// create an iterator over all attribute definitions in the current ElementType
    pub fn attribute_spec_iter(&self) -> AttrDefinitionsIter {
        AttrDefinitionsIter {
            type_id: self.0,
            pos: 0,
        }
    }

    /// create an iterator over all sub elements of the current ElementType
    pub fn sub_element_spec_iter(&self) -> SubelemDefinitionsIter {
        SubelemDefinitionsIter {
            type_id_stack: vec![self.0],
            indices: vec![0],
        }
    }

    /// Is the current ElementType ordered
    ///
    /// It his is true, then the position of the sub elements of this element is semantically meaningful
    /// and they may not be sorted / re-ordered without changing the meaning of the file.
    ///
    /// An example of this is ARGUMENTS in BSW-MODULE-ENTRY. ARGUMENTS is ordered, because each of its
    /// SW-SERVICE-ARG sub elements represents a function argument
    pub fn ordered(&self) -> bool {
        DATATYPES[self.0].ordered
    }

    /// Is the current ElementType splittable
    ///
    /// This function returns a bitfield that indicates in which versions (if any) the ElementType is marked as splittable.
    /// A splittable element may be split across multiple arxml files
    pub fn splittable(&self) -> u32 {
        DATATYPES[self.0].splittable
    }

    /// Is the current ElementType splittable in the given version
    ///
    /// A splittable element may be split across multiple arxml files
    pub fn splittable_in(&self, version: AutosarVersion) -> bool {
        (DATATYPES[self.0].splittable & (version as u32)) != 0
    }

    /// ElementType::ROOT is the root ElementType of the Autosar arxml document, i.e. this is the ElementType of the AUTOSAR element
    pub const ROOT: Self = ElementType(ROOT_DATATYPE);
}

/// Iterator for attribute definitions
pub struct AttrDefinitionsIter {
    type_id: usize,
    pos: usize,
}

impl Iterator for AttrDefinitionsIter {
    type Item = (AttributeName, &'static CharacterDataSpec, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let (idx_start, idx_end) = ElementType(self.type_id).get_attributes_idx();
        let cur_pos = self.pos;
        self.pos += 1;
        if idx_start + cur_pos < idx_end {
            let (name, chardata_id, required) = ATTRIBUTES[idx_start + cur_pos];
            Some((name, &CHARACTER_DATA[chardata_id as usize], required))
        } else {
            None
        }
    }
}

/// Iterator over sub element definitions
///
/// returns the tuple (name: ElementName, etype: ElementType, version_mask: u32, name_version_mask: u32)
pub struct SubelemDefinitionsIter {
    type_id_stack: Vec<usize>,
    indices: Vec<usize>,
}

impl Iterator for SubelemDefinitionsIter {
    // ElementName, elementType, version_mask, is_named
    type Item = (ElementName, ElementType, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.type_id_stack.is_empty() {
            debug_assert_eq!(self.type_id_stack.len(), self.indices.len());

            let depth = self.indices.len() - 1;
            let current_type = self.type_id_stack[depth];
            let cur_pos = self.indices[depth];
            let (start_idx, end_idx) = ElementType(current_type).get_sub_element_idx();

            if start_idx + cur_pos < end_idx {
                match &SUBELEMENTS[start_idx + cur_pos] {
                    SubElement::Element { name, elemtype, .. } => {
                        // found an element, return it and advance
                        self.indices[depth] += 1;
                        let ver_idx = ElementType(current_type).get_sub_element_ver();
                        let version_mask = VERSION_INFO[ver_idx + cur_pos];
                        let is_named = ElementType(*elemtype).short_name_version_mask().unwrap_or(0);
                        Some((*name, ElementType(*elemtype), version_mask, is_named))
                    }
                    SubElement::Group { groupid } => {
                        // found a group, descend into it
                        self.type_id_stack.push(*groupid);
                        self.indices.push(0);
                        self.next()
                    }
                }
            } else {
                // finished processing this element / group; remove it from the stack
                self.indices.pop();
                self.type_id_stack.pop();
                if !self.indices.is_empty() {
                    self.indices[depth - 1] += 1;
                }
                self.next()
            }
        } else {
            None
        }
    }
}

// manually implement Debug for CharacterDataSpec; deriving it is not possible, because that fails on the check_fn field in ::Pattern.
// The check_fn field is simply omitted here.
impl std::fmt::Debug for CharacterDataSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enum { items } => f.debug_struct("Enum").field("items", items).finish(),
            Self::Pattern { regex, max_length, .. } => f
                .debug_struct("Pattern")
                .field("regex", regex)
                .field("max_length", max_length)
                .finish(),
            Self::String {
                preserve_whitespace,
                max_length,
            } => f
                .debug_struct("String")
                .field("preserve_whitespace", preserve_whitespace)
                .field("max_length", max_length)
                .finish(),
            Self::UnsignedInteger => write!(f, "UnsignedInteger"),
            Self::Double => write!(f, "Double"),
        }
    }
}

pub(crate) fn hashfunc(mut data: &[u8]) -> (u32, u32, u32) {
    const HASHCONST1: u32 = 0x541C69B2; // these 4 constant values are not special, just random values
    const HASHCONST2: u32 = 0x3B17161B;

    let mut f1 = 0x33143C63u32;
    let mut f2 = 0x88B0B21Eu32;
    while data.len() >= 4 {
        let val = u32::from_ne_bytes(data[..4].try_into().unwrap());
        f1 = f1.rotate_left(5).bitxor(val).wrapping_mul(HASHCONST1);
        f2 = f2.rotate_left(6).bitxor(val).wrapping_mul(HASHCONST2);
        data = &data[4..];
    }
    if data.len() >= 2 {
        let val = u16::from_ne_bytes(data[..2].try_into().unwrap()) as u32;
        f1 = f1.rotate_left(5).bitxor(val).wrapping_mul(HASHCONST1);
        f2 = f2.rotate_left(6).bitxor(val).wrapping_mul(HASHCONST2);
        data = &data[2..];
    }
    if !data.is_empty() {
        f1 = f1.rotate_left(5).bitxor(data[0] as u32).wrapping_mul(HASHCONST1);
        f2 = f2.rotate_left(6).bitxor(data[0] as u32).wrapping_mul(HASHCONST2);
    }
    let g = f1.bitxor(f2);
    (g, f1, f2)
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, str::FromStr};

    use super::*;

    fn get_prm_char_element_type() -> ElementType {
        let (ar_packages_type, _) = ElementType::ROOT
            .find_sub_element(ElementName::ArPackages, u32::MAX)
            .unwrap();
        let (ar_package_type, _) = ar_packages_type
            .find_sub_element(ElementName::ArPackage, u32::MAX)
            .unwrap();
        let (elements_type, _) = ar_package_type
            .find_sub_element(ElementName::Elements, u32::MAX)
            .unwrap();
        let (documentation_type, _) = elements_type
            .find_sub_element(ElementName::Documentation, u32::MAX)
            .unwrap();
        let (documentation_content_type, _) = documentation_type
            .find_sub_element(ElementName::DocumentationContent, u32::MAX)
            .unwrap();
        let (prms_type, _) = documentation_content_type
            .find_sub_element(ElementName::Prms, u32::MAX)
            .unwrap();
        let (prm_type, _) = prms_type.find_sub_element(ElementName::Prm, u32::MAX).unwrap();
        let (prm_char_type, _) = prm_type.find_sub_element(ElementName::PrmChar, u32::MAX).unwrap();

        prm_char_type
    }

    #[test]
    fn find_sub_element() {
        let prm_char_type = get_prm_char_element_type();
        let (_, indices) = prm_char_type.find_sub_element(ElementName::Tol, 0xffffffff).unwrap();
        assert_eq!(indices, vec![1, 0, 0, 0, 1]);
    }

    #[test]
    fn find_sub_element_version_dependent() {
        let (ar_packages_type, _) = ElementType::ROOT
            .find_sub_element(ElementName::ArPackages, u32::MAX)
            .unwrap();
        let (ar_package_type, _) = ar_packages_type
            .find_sub_element(ElementName::ArPackage, u32::MAX)
            .unwrap();
        let (elements_type, _) = ar_package_type
            .find_sub_element(ElementName::Elements, u32::MAX)
            .unwrap();
        let (sw_base_type_type, _) = elements_type
            .find_sub_element(ElementName::SwBaseType, u32::MAX)
            .unwrap();
        let (_, indices) = sw_base_type_type
            .find_sub_element(ElementName::BaseTypeSize, AutosarVersion::Autosar_4_0_1 as u32)
            .unwrap();
        assert_eq!(indices, vec![11, 0]);

        let (_, indices) = sw_base_type_type
            .find_sub_element(ElementName::BaseTypeSize, AutosarVersion::Autosar_4_1_1 as u32)
            .unwrap();
        assert_eq!(indices, vec![13]);
    }

    #[test]
    fn get_sub_element_spec() {
        let prm_char_type = get_prm_char_element_type();
        let (abs_type, indices) = prm_char_type.find_sub_element(ElementName::Abs, u32::MAX).unwrap();
        let sub_elem_spec = prm_char_type.get_sub_element_spec(&indices);
        let (sub_element, _) = sub_elem_spec.unwrap();
        if let SubElement::Element { name, elemtype, .. } = sub_element {
            assert_eq!(*name, ElementName::Abs);
            assert_eq!(ElementType(*elemtype), abs_type);
        }

        // the element_indices passed to get_sub_element_spec may not be empty
        let sub_elem_spec2 = prm_char_type.get_sub_element_spec(&[]);
        assert!(sub_elem_spec2.is_none());
        // element_indices is nonsense
        let sub_elem_spec2 = prm_char_type.get_sub_element_spec(&[0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(sub_elem_spec2.is_none());
    }

    #[test]
    fn get_sub_element_version_mask() {
        let prm_char_type = get_prm_char_element_type();
        let (_, indices) = prm_char_type.find_sub_element(ElementName::Abs, u32::MAX).unwrap();
        let sub_elem_spec = prm_char_type.get_sub_element_spec(&indices).unwrap();
        let version_mask2 = prm_char_type.get_sub_element_version_mask(&indices).unwrap();
        let (_, version_mask) = sub_elem_spec;
        assert_eq!(version_mask, version_mask2);

        let no_result = prm_char_type.get_sub_element_version_mask(&[]);
        assert!(no_result.is_none());
    }

    #[test]
    fn get_sub_element_multiplicity() {
        let prm_char_type = get_prm_char_element_type();
        let (_, indices) = prm_char_type.find_sub_element(ElementName::Abs, u32::MAX).unwrap();
        let sub_elem_spec = prm_char_type.get_sub_element_spec(&indices);
        let multiplicity2 = prm_char_type.get_sub_element_multiplicity(&indices).unwrap();
        if let Some((SubElement::Element { multiplicity, .. }, ..)) = sub_elem_spec {
            assert_eq!(*multiplicity, multiplicity2);
        }

        let no_result = prm_char_type.get_sub_element_multiplicity(&[]);
        assert!(no_result.is_none());
    }

    #[test]
    fn get_sub_element_container_mode() {
        let prm_char_type = get_prm_char_element_type();
        let (_, indices) = prm_char_type.find_sub_element(ElementName::Abs, u32::MAX).unwrap();
        let mode = prm_char_type.get_sub_element_container_mode(&indices);
        assert_eq!(mode, ContentMode::Sequence);
    }

    #[test]
    fn find_common_group() {
        let prm_char_type = get_prm_char_element_type();
        let (_, indices_abs) = prm_char_type.find_sub_element(ElementName::Abs, u32::MAX).unwrap();
        let (_, indices_tol) = prm_char_type.find_sub_element(ElementName::Tol, u32::MAX).unwrap();
        let (_, indices_min) = prm_char_type.find_sub_element(ElementName::Min, u32::MAX).unwrap();
        // see the documentation on find_sub_element for the complex structure under PRM-CHAR
        // ABS and TOL share a sequence group (top level)
        let group1 = prm_char_type.find_common_group(&indices_abs, &indices_tol);
        assert_eq!(group1.content_mode(), ContentMode::Sequence);
        // ABS and MIN have the second level choice group in common
        let group2 = prm_char_type.find_common_group(&indices_abs, &indices_min);
        assert_eq!(group2.content_mode(), ContentMode::Choice);
    }

    #[test]
    fn find_attribute_spec() {
        let AttributeSpec {
            spec,
            required,
            version,
        } = ElementType::ROOT.find_attribute_spec(AttributeName::xmlns).unwrap();
        let spec_dbgstr = format!("{:#?}", spec);
        assert!(!spec_dbgstr.is_empty());
        // xmlns in AUTOSAR is required
        assert!(required);
        // must be specified both in the first and latest versions (and every one in between - not tested)
        assert_ne!(version & AutosarVersion::Autosar_00050 as u32, 0);
        assert_ne!(version & AutosarVersion::Autosar_4_0_1 as u32, 0);
    }

    #[test]
    fn subelement_definition_iterator() {
        let (ar_packages_type, _) = ElementType::ROOT
            .find_sub_element(ElementName::ArPackages, u32::MAX)
            .unwrap();
        let (ar_package_type, _) = ar_packages_type
            .find_sub_element(ElementName::ArPackage, u32::MAX)
            .unwrap();
        let (elements_type, _) = ar_package_type
            .find_sub_element(ElementName::Elements, u32::MAX)
            .unwrap();

        let se_iter = elements_type.sub_element_spec_iter();
        assert_eq!(se_iter.count(), 586); // this test breaks when support for new versions is added

        let prm_char_type = get_prm_char_element_type();
        let pc_iter = prm_char_type.sub_element_spec_iter();
        // not all items in the sub element spec are compatible with the latest Autosar version, count only the ones that are compatible
        let compatible_count = pc_iter
            .filter(|(_, _, version_mask, _)| AutosarVersion::Autosar_00050.compatible(*version_mask))
            .count();
        assert_eq!(compatible_count, 9);
    }

    #[test]
    fn autosar_version() {
        // does from_str work correctly?
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_4-0-1.xsd").unwrap(),
            AutosarVersion::Autosar_4_0_1
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_4-0-2.xsd").unwrap(),
            AutosarVersion::Autosar_4_0_2
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_4-0-3.xsd").unwrap(),
            AutosarVersion::Autosar_4_0_3
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_4-1-1.xsd").unwrap(),
            AutosarVersion::Autosar_4_1_1
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_4-1-2.xsd").unwrap(),
            AutosarVersion::Autosar_4_1_2
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_4-1-3.xsd").unwrap(),
            AutosarVersion::Autosar_4_1_3
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_4-2-1.xsd").unwrap(),
            AutosarVersion::Autosar_4_2_1
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_4-2-2.xsd").unwrap(),
            AutosarVersion::Autosar_4_2_2
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_4-3-0.xsd").unwrap(),
            AutosarVersion::Autosar_4_3_0
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_00042.xsd").unwrap(),
            AutosarVersion::Autosar_00042
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_00043.xsd").unwrap(),
            AutosarVersion::Autosar_00043
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_00044.xsd").unwrap(),
            AutosarVersion::Autosar_00044
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_00045.xsd").unwrap(),
            AutosarVersion::Autosar_00045
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_00046.xsd").unwrap(),
            AutosarVersion::Autosar_00046
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_00047.xsd").unwrap(),
            AutosarVersion::Autosar_00047
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_00048.xsd").unwrap(),
            AutosarVersion::Autosar_00048
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_00049.xsd").unwrap(),
            AutosarVersion::Autosar_00049
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_00050.xsd").unwrap(),
            AutosarVersion::Autosar_00050
        );
        assert_eq!(
            AutosarVersion::from_str("AUTOSAR_00051.xsd").unwrap(),
            AutosarVersion::Autosar_00051
        );

        // do all the version descriptions exist & make sense?
        assert!(AutosarVersion::Autosar_4_0_1.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_4_0_2.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_4_0_3.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_4_1_1.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_4_1_2.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_4_1_3.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_4_2_1.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_4_2_2.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_4_3_0.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_00042.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_00043.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_00044.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_00045.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_00046.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_00047.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_00048.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_00049.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_00050.describe().starts_with("AUTOSAR"));
        assert!(AutosarVersion::Autosar_00051.describe().starts_with("AUTOSAR"));

        // do all the xsd file names exist?
        assert!(AutosarVersion::Autosar_4_0_1.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_4_0_2.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_4_0_3.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_4_1_1.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_4_1_2.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_4_1_3.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_4_2_1.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_4_2_2.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_4_3_0.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_00042.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_00043.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_00044.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_00045.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_00046.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_00047.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_00048.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_00049.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_00050.filename().ends_with(".xsd"));
        assert!(AutosarVersion::Autosar_00051.filename().ends_with(".xsd"));

        // to_string() should give the same result as describe()
        assert_eq!(
            AutosarVersion::Autosar_4_0_1.to_string(),
            AutosarVersion::Autosar_4_0_1.describe()
        );

        // clone impl exists
        let cloned = AutosarVersion::Autosar_00050;
        assert_eq!(cloned, AutosarVersion::Autosar_00050);

        // version parse error
        let error = AutosarVersion::from_str("something else");
        assert_eq!(format!("{:#?}", error.unwrap_err()), "ParseAutosarVersionError");

        //Autosar version implements Hash and can be inserted into HashSet / HashMap
        let mut hashset = HashSet::<AutosarVersion>::new();
        hashset.insert(AutosarVersion::Autosar_00050);
    }

    #[test]
    fn attribute_name_basics() {
        // attribute name round trip: enum -> str -> enum
        assert_eq!(
            AttributeName::Uuid,
            AttributeName::from_str(AttributeName::Uuid.to_str()).unwrap()
        );

        // to_string()
        assert_eq!(AttributeName::Uuid.to_string(), "UUID");

        // clone impl exists
        let cloned = AttributeName::Uuid;
        assert_eq!(cloned, AttributeName::Uuid);

        // attribute parse error
        let error = AttributeName::from_str("unknown attribute name");
        assert_eq!(format!("{:#?}", error.unwrap_err()), "ParseAttributeNameError");

        // attribute names implement Hash and can be inserted into HashSet / HashMap
        let mut hashset = HashSet::<AttributeName>::new();
        hashset.insert(AttributeName::Dest);
    }

    #[test]
    fn element_name_basics() {
        // element name round trip: enum -> str -> enum
        assert_eq!(
            ElementName::Autosar,
            ElementName::from_str(ElementName::Autosar.to_str()).unwrap()
        );

        // to_string()
        assert_eq!(ElementName::Autosar.to_string(), "AUTOSAR");

        // clone impl exists
        let cloned = ElementName::Autosar;
        assert_eq!(cloned, ElementName::Autosar);

        // element name parse error
        let error = ElementName::from_str("unknown element name");
        assert_eq!(format!("{:#?}", error.unwrap_err()), "ParseElementNameError");

        // element names implement Hash and can be inserted into HashSet / HashMap
        let mut hashset = HashSet::<ElementName>::new();
        hashset.insert(ElementName::Autosar);
    }

    #[test]
    fn enum_item_basics() {
        // enum item round trip: enum -> str -> enum
        assert_eq!(
            EnumItem::Default,
            EnumItem::from_str(EnumItem::Default.to_str()).unwrap()
        );

        // to_string()
        assert_eq!(EnumItem::Default.to_string(), "DEFAULT");

        // clone impl exists
        let cloned = EnumItem::Abstract;
        assert_eq!(cloned, EnumItem::Abstract);

        // enum item parse error
        let error = EnumItem::from_str("unknown enum item");
        assert_eq!(format!("{:#?}", error.unwrap_err()), "ParseEnumItemError");

        // enum items implement Hash and can be inserted into HashSet / HashMap
        let mut hashset = HashSet::<EnumItem>::new();
        hashset.insert(EnumItem::Abstract);
    }

    #[test]
    fn ordered() {
        let (ar_packages_type, _) = ElementType::ROOT
            .find_sub_element(ElementName::ArPackages, u32::MAX)
            .unwrap();
        let (ar_package_type, _) = ar_packages_type
            .find_sub_element(ElementName::ArPackage, u32::MAX)
            .unwrap();
        let (elements_type, _) = ar_package_type
            .find_sub_element(ElementName::Elements, u32::MAX)
            .unwrap();
        // BSW-MODULE-ENTRY: This class represents a single API entry (C-function prototype) into the BSW module or cluster.
        let (bsw_module_entry, _) = elements_type
            .find_sub_element(ElementName::BswModuleEntry, u32::MAX)
            .unwrap();
        // ARGUMENTS: Arguments belonging of the BswModuleEntry.
        let (arguments, _) = bsw_module_entry
            .find_sub_element(ElementName::Arguments, u32::MAX)
            .unwrap();

        assert!(!bsw_module_entry.ordered());
        assert!(arguments.ordered());
    }

    #[test]
    fn splittable() {
        let (ar_packages_type, _) = ElementType::ROOT
            .find_sub_element(ElementName::ArPackages, u32::MAX)
            .unwrap();
        let (ar_package_type, _) = ar_packages_type
            .find_sub_element(ElementName::ArPackage, u32::MAX)
            .unwrap();
        let (elements_type, _) = ar_package_type
            .find_sub_element(ElementName::Elements, u32::MAX)
            .unwrap();

        assert!(!ar_package_type.splittable_in(AutosarVersion::Autosar_00051));
        assert_ne!(ar_packages_type.splittable() & AutosarVersion::Autosar_00051 as u32, 0);
        assert!(ar_packages_type.splittable_in(AutosarVersion::Autosar_00051));
        assert_ne!(elements_type.splittable() & AutosarVersion::Autosar_00051 as u32, 0);
    }

    #[test]
    fn traits() {
        // this test is basically nonsense - derived traits should all be ok
        // but there is no way to exclude them from coverage
        // ElementMultiplicity: Debug & Clone
        let mult = ElementMultiplicity::Any;
        let m2 = mult.clone(); // must be .clone(), otherwise the copy impl is tested instead
        assert_eq!(format!("{:#?}", mult), format!("{:#?}", m2));

        // ContentMode: Debug, Clone
        let cm = ContentMode::Sequence;
        let cm2 = cm.clone(); // must be .clone(), otherwise the copy impl is tested instead
        assert_eq!(format!("{:#?}", cm), format!("{:#?}", cm2));

        // ElementType: Debug, Clone, Eq & Hash
        let et = ElementType::ROOT;
        let et2 = et.clone(); // must be .clone(), otherwise the copy impl is tested instead
        assert_eq!(format!("{:#?}", et), format!("{:#?}", et2));
        let mut hashset = HashSet::<ElementType>::new();
        hashset.insert(et);
        let inserted = hashset.insert(et2);
        assert!(!inserted);

        // AutosarVersion: Debug, Clone, Hash
        let ver = AutosarVersion::LATEST;
        let ver2 = ver.clone(); // must be .clone(), otherwise the copy impl is tested instead
        assert_eq!(format!("{ver:#?}"), format!("{ver2:#?}"));
        let mut hashset = HashSet::<AutosarVersion>::new();
        hashset.insert(ver);
        let inserted = hashset.insert(ver2);
        assert!(!inserted);

        // ElementName: Debug, Clone, Hash
        let en = ElementName::Autosar;
        let en2 = en.clone(); // must be .clone(), otherwise the copy impl is tested instead
        assert_eq!(format!("{en:#?}"), format!("{en2:#?}"));
        let mut hashset = HashSet::<ElementName>::new();
        hashset.insert(en);
        let inserted = hashset.insert(en2);
        assert!(!inserted);
    }
}
