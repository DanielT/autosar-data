use crate::specification::*;


pub(crate) fn get_sub_element_spec<'a>(spec: &'a[SubElement], element_indices: &[usize]) -> Option<&'a SubElement> {
    if !element_indices.is_empty() {
        let mut current_spec = spec;
        // go through the hierarchy of groups: only the final index in element_indices can refer to a SubElement::Element
        for idx in 0..(element_indices.len()-1) {
            match &current_spec[element_indices[idx]] {
                SubElement::Element { .. } => {
                    // elements are not allowed here
                    return None;
                }
                SubElement::Group { groupid } => {
                    current_spec = DATATYPES[*groupid].sub_elements;
                }
            }
        }
    
        let last_idx = *element_indices.last().unwrap();
        Some(&current_spec[last_idx])
    } else {
        None
    }
}

pub(crate) fn get_parent_type_id(type_id: usize, element_indices: &[usize]) -> Option<usize> {
    if element_indices.len() < 2 {
        // length == 1: this element is a direct sub element, without any groups; 
        return Some(type_id)
    } else {
        let spec = DATATYPES[type_id].sub_elements;
        let len = element_indices.len() - 1;
        if let Some(SubElement::Group {groupid}) = get_sub_element_spec(spec, &element_indices[..len]) {
            return Some(*groupid);
        }
    }
    None
}

/// find an element in the specification
/// 
/// In almost all cases this is simple: there is a flat list of sub elements that either contains the target_name or not.
/// There are a handfull of complicated situations though, for example:
/// 
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
/// 
/// When searching for TOL in PRM-CHAR, the result should be Some(vec![1, 0, 0, 0, 1])!
pub(crate) fn find_sub_element(target_name: ElementName, type_id: usize, version: u32) -> Option<Vec<usize>> {
    let spec = DATATYPES[type_id].sub_elements;
    for (cur_pos, sub_element) in spec.iter().enumerate() {
        match sub_element {
            SubElement::Element { name, version_mask, .. } => {
                if (*name == target_name) && (version & version_mask != 0) {
                    return Some(vec![cur_pos]);
                }
            }
            SubElement::Group { groupid } => {
                // the group_pos parameter is only valid when referring to the group at start_pos. All other groups are searched from the beginning
                if let Some(mut result) = find_sub_element(target_name, *groupid, version) {
                    result.insert(0, cur_pos);
                    return Some(result);
                }
            }
        }
    }
    None
}


pub(crate) fn find_common_group(type_id: usize, element_indices: &[usize], element_indices2: &[usize]) -> &'static ElementSpec {
    let mut result = &DATATYPES[type_id];
    let mut prefix_len = 0;
    while element_indices.len() > prefix_len && element_indices2.len() > prefix_len && element_indices[prefix_len] == element_indices2[prefix_len] {
        let sub_elem = &result.sub_elements[element_indices[prefix_len]];
        match sub_elem {
            SubElement::Element { .. } => return result,
            SubElement::Group { groupid } => {
                result = &DATATYPES[*groupid];
            }
        }
        prefix_len += 1;
    }

    result
}


#[cfg(test)]
mod test {
    use super::find_sub_element;
    use crate::specification::*;

    #[test]
    fn test_find_element_in_spec_complex() {
        let prm_char_type_id = 2952; // note: this may change if the specification is updated
        let result = find_sub_element(ElementName::Tol, prm_char_type_id, 0xffffffff);
        assert!(result.is_some());
        let element_path = result.unwrap();
        assert_eq!(element_path, vec![1, 0, 0, 0, 1]);
    }

    #[test]
    fn test_find_element_in_spec_version_dependent() {
        let sw_base_type_type_id = 3748; // note: this may change if the specification is updated
        let result = find_sub_element(ElementName::BaseTypeSize, sw_base_type_type_id, AutosarVersion::Autosar_4_0_1 as u32);
        assert!(result.is_some());
        let element_path = result.unwrap();
        assert_eq!(element_path, vec![11, 0]);

        let result = find_sub_element(ElementName::BaseTypeSize, sw_base_type_type_id, AutosarVersion::Autosar_4_1_1 as u32);
        assert!(result.is_some());
        let element_path = result.unwrap();
        assert_eq!(element_path, vec![12]);        
    }
}
