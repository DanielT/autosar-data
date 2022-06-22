use std::fmt::{Display, Write};
use std::str::FromStr;

use super::*;

impl CharacterData {
    pub(crate) fn check_value(
        value: &CharacterData,
        spec: &specification::CharacterDataSpec,
        file_version: AutosarVersion,
    ) -> bool {
        match spec {
            // the specification must call for an enum
            specification::CharacterDataSpec::Enum { items } => {
                // the actual value must be an enum item
                if let CharacterData::Enum(enumitem) = &value {
                    // find the given enum item in the specified list of values
                    if let Some((_, version_mask)) = items.iter().find(|(name, _)| *name == *enumitem) {
                        // and finally check that this value is allowed in the active autosar version
                        if *version_mask & (file_version as u32) != 0 {
                            return true;
                        }
                    }
                }
            }
            specification::CharacterDataSpec::Pattern {
                check_fn, max_length, ..
            } => {
                if let CharacterData::String(stringval) = &value {
                    if (max_length.is_none() || stringval.len() < max_length.unwrap()) && check_fn(stringval.as_bytes())
                    {
                        return true;
                    }
                }
            }
            specification::CharacterDataSpec::String { max_length, .. } => {
                if let CharacterData::String(stringval) = &value {
                    if max_length.is_none() || stringval.len() < max_length.unwrap() {
                        return true;
                    }
                }
            }
            specification::CharacterDataSpec::UnsignedInteger => {
                if let CharacterData::UnsignedInteger(_) = &value {
                    return true;
                }
            }
            specification::CharacterDataSpec::Double => {
                if let CharacterData::Double(_) = &value {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn parse(input: &str, character_data_spec: &CharacterDataSpec, version: AutosarVersion) -> Option<Self> {
        match character_data_spec {
            CharacterDataSpec::Enum { items } => {
                if let Ok(enumitem) = specification::EnumItem::from_str(input) {
                    if let Some((_, version_mask)) = items.iter().find(|(item, _)| *item == enumitem) {
                        if version as u32 & version_mask != 0 {
                            return Some(CharacterData::Enum(enumitem));
                        }
                    }
                }
            }
            CharacterDataSpec::Pattern {
                check_fn, max_length, ..
            } => {
                if (max_length.is_none() || input.len() < max_length.unwrap()) && check_fn(input.as_bytes()) {
                    return Some(CharacterData::String(input.to_owned()));
                }
            }
            CharacterDataSpec::String { max_length, .. } => {
                if max_length.is_none() || input.len() < max_length.unwrap() {
                    return Some(CharacterData::String(input.to_owned()));
                }
            }
            CharacterDataSpec::UnsignedInteger => {
                if let Ok(value) = input.parse() {
                    return Some(CharacterData::UnsignedInteger(value));
                }
            }
            CharacterDataSpec::Double => {
                if let Ok(value) = input.parse() {
                    return Some(CharacterData::Double(value));
                }
            }
        }
        None
    }

    pub(crate) fn serialize_internal(&self, outstring: &mut String) {
        match self {
            CharacterData::Enum(enumval) => outstring.write_str(enumval.to_str()).unwrap(),
            CharacterData::String(strval) => outstring.write_str(strval).unwrap(),
            CharacterData::UnsignedInteger(intval) => outstring.write_str(&intval.to_string()).unwrap(),
            CharacterData::Double(doubleval) => outstring.write_str(&doubleval.to_string()).unwrap(),
        }
    }
}

impl Display for CharacterData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterData::Enum(enumitem) => f.write_str(enumitem.to_str()),
            CharacterData::String(stringval) => f.write_str(&**stringval),
            CharacterData::UnsignedInteger(uintval) => f.write_str(&*uintval.to_string()),
            CharacterData::Double(f64val) => f.write_str(&*f64val.to_string()),
        }
    }
}
