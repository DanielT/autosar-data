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
                    if (max_length.is_none() || stringval.len() <= max_length.unwrap())
                        && check_fn(stringval.as_bytes())
                    {
                        return true;
                    }
                }
            }
            CharacterDataSpec::String { max_length, .. } => {
                if let CharacterData::String(stringval) = &value {
                    if max_length.is_none() || stringval.len() <= max_length.unwrap() {
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
                // content is not an enum item, but an enum item is expected
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
                if (max_length.is_none() || input.len() <= max_length.unwrap()) && check_fn(input.as_bytes()) {
                    return Some(CharacterData::String(input.to_owned()));
                }
            }
            CharacterDataSpec::String { max_length, .. } => {
                if max_length.is_none() || input.len() <= max_length.unwrap() {
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

    /// Get the contained enum value
    ///
    /// Returns the enum value if the content is an enum, or None otherwise
    pub fn enum_value(&self) -> Option<EnumItem> {
        if let CharacterData::Enum(item) = self {
            Some(*item)
        } else {
            None
        }
    }

    /// Get the contained string
    ///
    /// Returns the string if the content is a string, or None otherwise
    pub fn string_value(&self) -> Option<String> {
        if let CharacterData::String(value) = self {
            Some(value.to_owned())
        } else {
            None
        }
    }

    /// Get the contained unsigned integer
    ///
    /// Returns the string if the content is a string, or None otherwise
    pub fn unsigned_integer_value(&self) -> Option<u64> {
        if let CharacterData::UnsignedInteger(uintval) = self {
            Some(*uintval)
        } else {
            None
        }
    }

    /// Get the contained double
    ///
    /// Returns the value content is a double, or None otherwise
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
    if input.contains(['&', '>', '<', '\'', '"']) {
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

#[cfg(test)]
mod test {
    use super::*;

    fn dummy_validate(s: &[u8]) -> bool {
        s.len() == 1 && (s[0] == b'0' || s[0] == b'1')
    }

    #[test]
    fn check_value() {
        let spec_enum = CharacterDataSpec::Enum {
            items: &[(EnumItem::default, 0x3ffff), (EnumItem::preserve, 0x0ffff)],
        };
        let data = CharacterData::Enum(EnumItem::default);
        assert_eq!(
            CharacterData::check_value(&data, &spec_enum, AutosarVersion::Autosar_00050),
            true
        );
        let data = CharacterData::Enum(EnumItem::preserve);
        assert_eq!(
            CharacterData::check_value(&data, &spec_enum, AutosarVersion::Autosar_00050),
            false
        );
        let data = CharacterData::Enum(EnumItem::Abstract);
        assert_eq!(
            CharacterData::check_value(&data, &spec_enum, AutosarVersion::Autosar_00050),
            false
        );
        let data = CharacterData::Double(1.23);
        assert_eq!(
            CharacterData::check_value(&data, &spec_enum, AutosarVersion::Autosar_00050),
            false
        );

        let spec_double = CharacterDataSpec::Double;
        let data = CharacterData::Double(1.23);
        assert_eq!(
            CharacterData::check_value(&data, &spec_double, AutosarVersion::Autosar_00050),
            true
        );
        let data = CharacterData::String("1.23".to_string());
        assert_eq!(
            CharacterData::check_value(&data, &spec_double, AutosarVersion::Autosar_00050),
            false
        );

        let spec_uint = CharacterDataSpec::UnsignedInteger;
        let data = CharacterData::UnsignedInteger(123);
        assert_eq!(
            CharacterData::check_value(&data, &spec_uint, AutosarVersion::Autosar_00050),
            true
        );
        let data = CharacterData::String("123".to_string());
        assert_eq!(
            CharacterData::check_value(&data, &spec_uint, AutosarVersion::Autosar_00050),
            false
        );

        let spec_string = CharacterDataSpec::String {
            preserve_whitespace: false,
            max_length: Some(10),
        };
        let data = CharacterData::String("123".to_string());
        assert_eq!(
            CharacterData::check_value(&data, &spec_string, AutosarVersion::Autosar_00050),
            true
        );
        let data = CharacterData::String("12345678901".to_string());
        assert_eq!(
            CharacterData::check_value(&data, &spec_string, AutosarVersion::Autosar_00050),
            false
        );
        let data = CharacterData::UnsignedInteger(1);
        assert_eq!(
            CharacterData::check_value(&data, &spec_string, AutosarVersion::Autosar_00050),
            false
        );

        let spec_pattern = CharacterDataSpec::Pattern {
            check_fn: dummy_validate,
            regex: r"0|1",
            max_length: Some(1),
        };
        let data = CharacterData::String("0".to_string());
        assert_eq!(
            CharacterData::check_value(&data, &spec_pattern, AutosarVersion::Autosar_00050),
            true
        );
        let data = CharacterData::String("2".to_string());
        assert_eq!(
            CharacterData::check_value(&data, &spec_pattern, AutosarVersion::Autosar_00050),
            false
        );
    }

    #[test]
    fn check_version_compatibility() {
        let spec_enum = CharacterDataSpec::Enum {
            items: &[(EnumItem::default, 0x0001), (EnumItem::preserve, 0x0002)],
        };
        let data = CharacterData::Enum(EnumItem::default);
        let (result, _) = data.check_version_compatibility(&spec_enum, AutosarVersion::Autosar_4_0_1);
        assert_eq!(result, true);
        let (result, _) = data.check_version_compatibility(&spec_enum, AutosarVersion::Autosar_00050);
        assert_eq!(result, false);
        let data = CharacterData::Enum(EnumItem::Abstract);
        let (result, _) = data.check_version_compatibility(&spec_enum, AutosarVersion::Autosar_00050);
        assert_eq!(result, false);
        let data = CharacterData::UnsignedInteger(0);
        let (result, _) = data.check_version_compatibility(&spec_enum, AutosarVersion::Autosar_00050);
        assert_eq!(result, false);
    }

    #[test]
    fn parse() {
        let spec_enum = CharacterDataSpec::Enum {
            items: &[(EnumItem::default, 0x3ffff), (EnumItem::preserve, 0x0ffff)],
        };
        let data = CharacterData::parse("default", &spec_enum, AutosarVersion::Autosar_00050).unwrap();
        assert_eq!(data.enum_value().unwrap(), EnumItem::default);
        let data = CharacterData::parse("preserve", &spec_enum, AutosarVersion::Autosar_00050);
        assert!(data.is_none());
        let data = CharacterData::parse("ABSTRACT", &spec_enum, AutosarVersion::Autosar_00050);
        assert!(data.is_none());
        let data = CharacterData::parse("", &spec_enum, AutosarVersion::Autosar_00050);
        assert!(data.is_none());

        let spec_double = CharacterDataSpec::Double;
        let data = CharacterData::parse("2.0", &spec_double, AutosarVersion::Autosar_00050).unwrap();
        assert_eq!(data.double_value().unwrap(), 2.0);
        let data = CharacterData::parse("text", &spec_double, AutosarVersion::Autosar_00050);
        assert!(data.is_none());

        let spec_uint = CharacterDataSpec::UnsignedInteger;
        let data = CharacterData::parse("2", &spec_uint, AutosarVersion::Autosar_00050).unwrap();
        assert_eq!(data.unsigned_integer_value().unwrap(), 2);
        let data = CharacterData::parse("text", &spec_uint, AutosarVersion::Autosar_00050);
        assert!(data.is_none());

        let spec_string = CharacterDataSpec::String {
            preserve_whitespace: false,
            max_length: Some(10),
        };
        let data = CharacterData::parse("text", &spec_string, AutosarVersion::Autosar_00050).unwrap();
        assert_eq!(data.string_value().unwrap(), "text");
        let data = CharacterData::parse("text text text", &spec_string, AutosarVersion::Autosar_00050);
        assert!(data.is_none());

        let spec_pattern = CharacterDataSpec::Pattern {
            check_fn: dummy_validate,
            regex: r"0|1",
            max_length: Some(1),
        };
        let data = CharacterData::parse("0", &spec_pattern, AutosarVersion::Autosar_00050).unwrap();
        assert_eq!(data.string_value().unwrap(), "0");
        let data = CharacterData::parse("2", &spec_pattern, AutosarVersion::Autosar_00050);
        assert!(data.is_none());
    }

    #[test]
    fn serialize() {
        let mut out = "".to_string();
        let data = CharacterData::Enum(EnumItem::Abstract);
        data.serialize_internal(&mut out);
        assert_eq!(out, "ABSTRACT");
        assert_eq!(format!("{data}"), "ABSTRACT");

        let mut out = "".to_string();
        let data = CharacterData::Double(1.23);
        data.serialize_internal(&mut out);
        assert_eq!(out, "1.23");
        assert_eq!(format!("{data}"), "1.23");

        let mut out = "".to_string();
        let data = CharacterData::UnsignedInteger(0);
        data.serialize_internal(&mut out);
        assert_eq!(out, "0");
        assert_eq!(format!("{data}"), "0");

        let mut out = "".to_string();
        let data = CharacterData::String("text".to_string());
        data.serialize_internal(&mut out);
        assert_eq!(out, "text");
        assert_eq!(format!("{data}"), "text");

        let mut out = "".to_string();
        let data = CharacterData::String("special chars: <, >, &, \', \"".to_string());
        data.serialize_internal(&mut out);
        assert_eq!(out, "special chars: &lt;, &gt;, &amp;, &apos;, &quot;");
        assert_eq!(format!("{data}"), "special chars: <, >, &, \', \"");
    }

    #[test]
    fn get_value() {
        assert!(CharacterData::Enum(EnumItem::Abstract).enum_value().is_some());
        assert!(CharacterData::Enum(EnumItem::Abstract).string_value().is_none());

        assert!(CharacterData::Double(1.23).double_value().is_some());
        assert!(CharacterData::Double(1.23).unsigned_integer_value().is_none());

        assert!(CharacterData::String("x".to_string()).string_value().is_some());
        assert!(CharacterData::String("x".to_string()).double_value().is_none());

        assert!(CharacterData::UnsignedInteger(1).unsigned_integer_value().is_some());
        assert!(CharacterData::UnsignedInteger(1).enum_value().is_none());
    }
}
