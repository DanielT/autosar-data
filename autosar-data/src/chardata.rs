use std::borrow::Cow;
use std::fmt::Display;
use std::str::FromStr;

use super::*;

impl CharacterData {
    pub(crate) fn check_value(value: &CharacterData, spec: &CharacterDataSpec, file_version: AutosarVersion) -> bool {
        match spec {
            // the specification must call for an enum
            CharacterDataSpec::Enum { items } => {
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
            CharacterDataSpec::Pattern {
                check_fn, max_length, ..
            } => {
                if let CharacterData::String(stringval) = &value {
                    if (max_length.is_none() || stringval.len() < max_length.unwrap()) && check_fn(stringval.as_bytes())
                    {
                        return true;
                    }
                }
            }
            CharacterDataSpec::String { max_length, .. } => {
                if let CharacterData::String(stringval) = &value {
                    if max_length.is_none() || stringval.len() < max_length.unwrap() {
                        return true;
                    }
                }
            }
            CharacterDataSpec::UnsignedInteger => {
                if let CharacterData::UnsignedInteger(_) = &value {
                    return true;
                }
            }
            CharacterDataSpec::Double => {
                if let CharacterData::Double(_) = &value {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn check_version_compatibility(
        &self,
        data_spec: &CharacterDataSpec,
        target_version: AutosarVersion,
    ) -> (bool, u32) {
        if let CharacterDataSpec::Enum { items } = data_spec {
            if let CharacterData::Enum(attrval) = self {
                if let Some((_, enumitem_version_mask)) = items.iter().find(|(item, _)| *item == *attrval) {
                    if target_version.compatible(*enumitem_version_mask) {
                        return (true, *enumitem_version_mask);
                    } else {
                        return (false, *enumitem_version_mask);
                    }
                } else {
                    // no spec for this item -> not allowed in any version
                    return (false, 0);
                }
            } else {
                // content is not an enum item, but an enum item is expected? shouldn't be possible, maybe panic?
                return (false, u32::MAX);
            }
        }
        (true, u32::MAX)
    }

    pub(crate) fn parse(input: &str, character_data_spec: &CharacterDataSpec, version: AutosarVersion) -> Option<Self> {
        match character_data_spec {
            CharacterDataSpec::Enum { items } => {
                if let Ok(enumitem) = EnumItem::from_str(input) {
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
            CharacterData::Enum(enumval) => outstring.push_str(enumval.to_str()),
            CharacterData::String(strval) => outstring.push_str(&escape_text(strval)),
            CharacterData::UnsignedInteger(intval) => outstring.push_str(&intval.to_string()),
            CharacterData::Double(doubleval) => outstring.push_str(&doubleval.to_string()),
        }
    }

    pub fn enum_value(&self) -> Option<EnumItem> {
        if let CharacterData::Enum(item) = self {
            Some(*item)
        } else {
            None
        }
    }

    pub fn string_value(&self) -> Option<String> {
        if let CharacterData::String(value) = self {
            Some(value.to_owned())
        } else {
            None
        }
    }

    pub fn unsigned_integer_value(&self) -> Option<u64> {
        if let CharacterData::UnsignedInteger(uintval) = self {
            Some(*uintval)
        } else {
            None
        }
    }

    pub fn double_value(&self) -> Option<f64> {
        if let CharacterData::Double(value) = self {
            Some(*value)
        } else {
            None
        }
    }
}

impl Display for CharacterData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterData::Enum(enumitem) => f.write_str(enumitem.to_str()),
            CharacterData::String(stringval) => f.write_str(stringval),
            CharacterData::UnsignedInteger(uintval) => f.write_str(&uintval.to_string()),
            CharacterData::Double(f64val) => f.write_str(&f64val.to_string()),
        }
    }
}

fn escape_text(input: &str) -> Cow<str> {
    if input.contains(&['&', '>', '<', '\'', '"']) {
        let mut escaped = String::with_capacity(input.len() + 6);

        for c in input.chars() {
            match c {
                '<' => escaped.push_str("&lt;"),
                '>' => escaped.push_str("&gt;"),
                '&' => escaped.push_str("&amp;"),
                '"' => escaped.push_str("&quot;"),
                '\'' => escaped.push_str("&apos;"),
                other => escaped.push(other),
            }
        }

        Cow::Owned(escaped)
    } else {
        Cow::Borrowed(input)
    }
}
