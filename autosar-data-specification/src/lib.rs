//! Specification of the Autosar arxml file format in the form of rust data structures

mod attributename;
mod autosarversion;
mod elementname;
mod enumitem;
mod regex;
mod specification;

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
    /// If the multiplicity of the chosen element is `Any` then it may repeat so there might sill be more than one sub element
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

/// ElementType is an abstraction over element types in the specification.
///
/// It provides no public fields, but it has methods to get all the info needed to parse an arxml element.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
    sub_elements: (usize, usize),
    sub_element_ver: usize,
    attributes: (usize, usize),
    attributes_ver: usize,
    character_data: Option<usize>,
    mode: ContentMode,
    is_named: u32,
    is_ref: bool,
}

impl AutosarVersion {
    pub fn compatible(&self, version_mask: u32) -> bool {
        version_mask & *self as u32 != 0
    }
}

impl ElementType {
    fn get_sub_elements(&self) -> &'static [SubElement] {
        let (idx_start, idx_end) = DATATYPES[self.0].sub_elements;
        &SUBELEMENTS[idx_start..idx_end]
    }

    /// get the spec of a sub element from the index list
    fn get_sub_element_spec<'a>(&self, element_indices: &[usize]) -> Option<(&'a SubElement, u32)> {
        let spec = self.get_sub_elements();
        let ver_list_start = DATATYPES[self.0].sub_element_ver;
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
                        current_ver_list_start = DATATYPES[*groupid].sub_element_ver;
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
    /// The sub element is identified by an indx list, as returned by `find_sub_element()`
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
                    let ver_info_start = DATATYPES[self.0].sub_element_ver;
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
        DATATYPES[self.0].is_named != 0
    }

    /// are elements of this elementType named in the given Autosar version
    ///
    /// Named elements must have a SHORT-NAME sub element. For some elements this
    /// depends on the Autosar version.
    ///
    /// One example of this is END-2-END-METHOD-PROTECTION-PROPS, which was first
    /// defined in Autosar_00048, but only has a name in Autosar_00050.
    pub fn is_named_in_version(&self, version: AutosarVersion) -> bool {
        version.compatible(DATATYPES[self.0].is_named)
    }

    /// is the ElementType a reference
    pub fn is_ref(&self) -> bool {
        DATATYPES[self.0].is_ref
    }

    /// get the content mode for this ElementType
    pub fn content_mode(&self) -> ContentMode {
        DATATYPES[self.0].mode
    }

    /// get the character data spec for this ElementType
    pub fn chardata_spec(&self) -> Option<&'static CharacterDataSpec> {
        if let Some(chardata_id) = DATATYPES[self.0].character_data {
            Some(&CHARACTER_DATA[chardata_id])
        } else {
            None
        }
    }

    /// find the spec for a single attribute by name
    pub fn find_attribute_spec(&self, attrname: AttributeName) -> Option<(&'static CharacterDataSpec, bool, u32)> {
        let (idx_start, idx_end) = DATATYPES[self.0].attributes;
        let attributes = &ATTRIBUTES[idx_start..idx_end];
        if let Some((find_pos, (_, chardata_id, required))) =
            attributes.iter().enumerate().find(|(_, (name, ..))| *name == attrname)
        {
            let idx_ver_start = DATATYPES[self.0].attributes_ver;
            let version = VERSION_INFO[idx_ver_start + find_pos];
            Some((&CHARACTER_DATA[*chardata_id], *required, version))
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

    /// ElementType::ROOT is the root ElementType of the Autosar arxml document, i.e. this is the ElementType of the AUTOSAR element
    pub const ROOT: Self = ElementType(ROOT_DATATYPE);
}

pub struct AttrDefinitionsIter {
    type_id: usize,
    pos: usize,
}

impl Iterator for AttrDefinitionsIter {
    type Item = (AttributeName, &'static CharacterDataSpec, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let (idx_start, idx_end) = DATATYPES[self.type_id].attributes;
        let cur_pos = self.pos;
        self.pos += 1;
        if idx_start + cur_pos < idx_end {
            let (name, chardata_id, required) = ATTRIBUTES[idx_start + cur_pos];
            Some((name, &CHARACTER_DATA[chardata_id], required))
        } else {
            None
        }
    }
}

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
            let datatype = &DATATYPES[current_type];
            let (start_idx, end_idx) = datatype.sub_elements;

            if start_idx + cur_pos < end_idx {
                match &SUBELEMENTS[start_idx + cur_pos] {
                    SubElement::Element { name, elemtype, .. } => {
                        // found an element, return it and advance
                        self.indices[depth] += 1;
                        let ver_idx = datatype.sub_element_ver;
                        let version_mask = VERSION_INFO[ver_idx + cur_pos];
                        //let elem_datatype = &DATATYPES[*elemtype];
                        let is_named = DATATYPES[*elemtype].is_named;
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

// manually implement Debug for CharacterDataSpec; deriving it is not possible, because that fails on the chack_fn field in ::Pattern.
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

pub(crate) fn hashfunc(data: &[u8], param: usize) -> usize {
    data.iter().fold(100usize, |acc, val| {
        usize::wrapping_add(usize::wrapping_mul(acc, param), *val as usize)
    })
}

#[cfg(test)]
mod test {
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
        if let Some((SubElement::Element { name, elemtype, .. }, _)) = sub_elem_spec {
            assert_eq!(*name, ElementName::Abs);
            assert_eq!(ElementType(*elemtype), abs_type);
        } else {
            panic!("incorrect return value from get_sub_element_spec()");
        }
    }

    #[test]
    fn get_sub_element_version_mask() {
        let prm_char_type = get_prm_char_element_type();
        let (_, indices) = prm_char_type.find_sub_element(ElementName::Abs, u32::MAX).unwrap();
        let sub_elem_spec = prm_char_type.get_sub_element_spec(&indices);
        let version_mask2 = prm_char_type.get_sub_element_version_mask(&indices).unwrap();
        if let Some((_, version_mask)) = sub_elem_spec {
            assert_eq!(version_mask, version_mask2);
        } else {
            panic!("incorrect return value from get_sub_element_version_mask()");
        }
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
        let (_, req, ver) = ElementType::ROOT.find_attribute_spec(AttributeName::xmlns).unwrap();
        // xmlns in AUTOSAR is required
        assert_eq!(req, true);
        // must be specified both in the first and latest versions (and every one in between - not tested)
        assert_ne!(ver & AutosarVersion::Autosar_00050 as u32, 0);
        assert_ne!(ver & AutosarVersion::Autosar_4_0_1 as u32, 0);
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
        assert_eq!(se_iter.count(), 560);

        let prm_char_type = get_prm_char_element_type();
        let pc_iter = prm_char_type.sub_element_spec_iter();
        // not all items in the sub element spec are compatible with the latest Autosar version, count only the ones that are compatible
        let compatible_count = pc_iter
            .filter(|(_, _, version_mask, _)| AutosarVersion::Autosar_00050.compatible(*version_mask))
            .count();
        assert_eq!(compatible_count, 9);
    }
}
