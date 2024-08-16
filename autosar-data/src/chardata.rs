use std::borrow::Cow;
use std::fmt::Display;
use std::str::FromStr;

use super::{AutosarVersion, CharacterData, CharacterDataSpec, EnumItem};

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
                    if stringval.len() <= max_length.unwrap_or(usize::MAX) && check_fn(stringval.as_bytes()) {
                        return true;
                    }
                }
            }
            CharacterDataSpec::String { max_length, .. } => {
                if let CharacterData::String(stringval) = &value {
                    if stringval.len() <= max_length.unwrap_or(usize::MAX) {
                        return true;
                    }
                }
            }
            CharacterDataSpec::UnsignedInteger => {
                if let CharacterData::UnsignedInteger(_) = &value {
                    return true;
                }
            }
            CharacterDataSpec::Float => {
                if let CharacterData::Float(_) = &value {
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
                        (true, *enumitem_version_mask)
                    } else {
                        (false, *enumitem_version_mask)
                    }
                } else {
                    // no spec for this item -> not allowed in any version
                    (false, 0)
                }
            } else {
                // content is not an enum item, but an enum item is expected
                (false, u32::MAX)
            }
        } else {
            (true, u32::MAX)
        }
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
                if input.len() <= max_length.unwrap_or(usize::MAX) && check_fn(input.as_bytes()) {
                    return Some(CharacterData::String(input.to_owned()));
                }
            }
            CharacterDataSpec::String { max_length, .. } => {
                if input.len() <= max_length.unwrap_or(usize::MAX) {
                    return Some(CharacterData::String(input.to_owned()));
                }
            }
            CharacterDataSpec::UnsignedInteger => {
                if let Ok(value) = input.parse() {
                    return Some(CharacterData::UnsignedInteger(value));
                }
            }
            CharacterDataSpec::Float => {
                if let Ok(value) = input.parse() {
                    return Some(CharacterData::Float(value));
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
            CharacterData::Float(floatval) => outstring.push_str(&floatval.to_string()),
        }
    }

    /// Get the contained enum value
    ///
    /// Returns the enum value if the content is an enum, or None otherwise
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn unsigned_integer_value(&self) -> Option<u64> {
        if let CharacterData::UnsignedInteger(uintval) = self {
            Some(*uintval)
        } else {
            None
        }
    }

    /// Get the contained floating point value
    ///
    /// Returns the value if the content is a float, or None otherwise
    #[must_use]
    pub fn float_value(&self) -> Option<f64> {
        if let CharacterData::Float(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// parse the stored charcter data value as an integer
    ///
    /// Many numbers are stored as strings in order to allow hexadecimal, octal, and binary encoding.
    /// This function handles the conversion from text to integer.
    /// If the stored value is already an integer, it will be converted to the output type.
    ///
    /// Returns the value if the conversion succeeds, or None otherwise
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::CharacterData;
    /// let data = CharacterData::String("0x1234".to_string());
    /// let value = data.parse_integer::<u32>().unwrap();
    /// assert_eq!(value, 0x1234);
    /// ```
    #[must_use]
    pub fn parse_integer<T: num_traits::Num + TryFrom<u64>>(&self) -> Option<T> {
        if let CharacterData::String(text) = self {
            if text == "0" {
                // handle this first to avoid hitting the octal case
                T::try_from(0u64).ok()
            } else if let Some(hexstr) = text.strip_prefix("0x") {
                T::from_str_radix(hexstr, 16).ok()
            } else if let Some(hexstr) = text.strip_prefix("0X") {
                T::from_str_radix(hexstr, 16).ok()
            } else if let Some(binstr) = text.strip_prefix("0b") {
                T::from_str_radix(binstr, 2).ok()
            } else if let Some(binstr) = text.strip_prefix("0B") {
                T::from_str_radix(binstr, 2).ok()
            } else if let Some(octstr) = text.strip_prefix('0') {
                T::from_str_radix(octstr, 8).ok()
            } else {
                T::from_str_radix(text, 10).ok()
            }
        } else if let CharacterData::UnsignedInteger(value) = self {
            T::try_from(*value).ok()
        } else {
            None
        }
    }

    /// parse the stored character data value as a floating point number
    ///
    /// When the meta model declares that a value is of class "Numerical", this means it is stored as a string.
    /// The regex associated with "Numerical" allows signed integers, floating point values (including scientific
    /// notation, as well as INF and NaN), hexadecimal, octal, and binary numbers.
    /// This function handles the conversion from text to floating point.
    ///
    /// Returns the value if the conversion succeeds, or None otherwise
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::CharacterData;
    /// let data = CharacterData::String("0x1234".to_string());
    /// let value = data.parse_float().unwrap();
    /// assert_eq!(value, 0x1234 as f64);
    ///
    /// let data = CharacterData::String("1.234e5".to_string());
    /// let value = data.parse_float().unwrap();
    /// assert_eq!(value, 1.234e5);
    /// ```
    #[must_use]
    pub fn parse_float(&self) -> Option<f64> {
        if let CharacterData::String(text) = self {
            if text == "0" {
                // handle this first to avoid hitting the octal case
                Some(0f64)
            } else if let Some(hexval) = text
                .strip_prefix("0x")
                .and_then(|hextxt| u64::from_str_radix(hextxt, 16).ok())
            {
                Some(hexval as f64)
            } else if let Some(hexval) = text
                .strip_prefix("0X")
                .and_then(|hextxt| u64::from_str_radix(hextxt, 16).ok())
            {
                Some(hexval as f64)
            } else if let Some(binval) = text
                .strip_prefix("0b")
                .and_then(|bintxt| u64::from_str_radix(bintxt, 2).ok())
            {
                Some(binval as f64)
            } else if let Some(binval) = text
                .strip_prefix("0B")
                .and_then(|bintxt| u64::from_str_radix(bintxt, 2).ok())
            {
                Some(binval as f64)
            } else if let Some(octval) = text
                .strip_prefix('0')
                .and_then(|octtxt| u64::from_str_radix(octtxt, 8).ok())
            {
                Some(octval as f64)
            } else {
                // normal float conversion
                text.parse().ok()
            }
        } else if let CharacterData::Float(value) = self {
            Some(*value)
        } else if let CharacterData::UnsignedInteger(value) = self {
            Some(*value as f64)
        } else {
            None
        }
    }
}

impl From<String> for CharacterData {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for CharacterData {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<EnumItem> for CharacterData {
    fn from(value: EnumItem) -> Self {
        Self::Enum(value)
    }
}

impl From<u64> for CharacterData {
    fn from(value: u64) -> Self {
        Self::UnsignedInteger(value)
    }
}

impl From<f64> for CharacterData {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl Display for CharacterData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterData::Enum(enumitem) => f.write_str(enumitem.to_str()),
            CharacterData::String(stringval) => f.write_str(stringval),
            CharacterData::UnsignedInteger(uintval) => f.write_str(&uintval.to_string()),
            CharacterData::Float(f64val) => f.write_str(&f64val.to_string()),
        }
    }
}

impl Ord for CharacterData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // compare two CharacterData values
        // if the types are different, the order is determined by the type so that there is a consistent order
        // sort order: enum < string < unsigned integer < float - this is arbitrary
        match (self, other) {
            (CharacterData::Enum(a), CharacterData::Enum(b)) => a.to_str().cmp(b.to_str()),
            (CharacterData::String(a), CharacterData::String(b)) => a.cmp(b),
            (CharacterData::UnsignedInteger(a), CharacterData::UnsignedInteger(b)) => a.cmp(b),
            (CharacterData::Float(a), CharacterData::Float(b)) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
            (CharacterData::Enum(_), _) => std::cmp::Ordering::Less,
            (CharacterData::String(_), CharacterData::Enum(_)) => std::cmp::Ordering::Greater,
            (CharacterData::String(_), _) => std::cmp::Ordering::Less,
            (CharacterData::UnsignedInteger(_), CharacterData::Enum(_)) => std::cmp::Ordering::Greater,
            (CharacterData::UnsignedInteger(_), CharacterData::String(_)) => std::cmp::Ordering::Greater,
            (CharacterData::UnsignedInteger(_), _) => std::cmp::Ordering::Less,
            (CharacterData::Float(_), _) => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for CharacterData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for CharacterData {}

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
        assert!(CharacterData::check_value(
            &data,
            &spec_enum,
            AutosarVersion::Autosar_00050
        ));
        let data = CharacterData::Enum(EnumItem::preserve);
        assert!(!CharacterData::check_value(
            &data,
            &spec_enum,
            AutosarVersion::Autosar_00050
        ));
        let data = CharacterData::Enum(EnumItem::Abstract);
        assert!(!CharacterData::check_value(
            &data,
            &spec_enum,
            AutosarVersion::Autosar_00050
        ));
        let data = CharacterData::Float(1.23);
        assert!(!CharacterData::check_value(
            &data,
            &spec_enum,
            AutosarVersion::Autosar_00050
        ));

        let spec_double = CharacterDataSpec::Float;
        let data = CharacterData::Float(1.23);
        assert!(CharacterData::check_value(
            &data,
            &spec_double,
            AutosarVersion::Autosar_00050
        ));
        let data = CharacterData::String("1.23".to_string());
        assert!(!CharacterData::check_value(
            &data,
            &spec_double,
            AutosarVersion::Autosar_00050
        ));

        let spec_uint = CharacterDataSpec::UnsignedInteger;
        let data = CharacterData::UnsignedInteger(123);
        assert!(CharacterData::check_value(
            &data,
            &spec_uint,
            AutosarVersion::Autosar_00050
        ));
        let data = CharacterData::String("123".to_string());
        assert!(!CharacterData::check_value(
            &data,
            &spec_uint,
            AutosarVersion::Autosar_00050
        ));

        let spec_string = CharacterDataSpec::String {
            preserve_whitespace: false,
            max_length: Some(10),
        };
        let data = CharacterData::String("123".to_string());
        assert!(CharacterData::check_value(
            &data,
            &spec_string,
            AutosarVersion::Autosar_00050
        ));
        let data = CharacterData::String("12345678901".to_string());
        assert!(!CharacterData::check_value(
            &data,
            &spec_string,
            AutosarVersion::Autosar_00050
        ));
        let data = CharacterData::UnsignedInteger(1);
        assert!(!CharacterData::check_value(
            &data,
            &spec_string,
            AutosarVersion::Autosar_00050
        ));

        let spec_pattern = CharacterDataSpec::Pattern {
            check_fn: dummy_validate,
            regex: r"0|1",
            max_length: Some(1),
        };
        let data = CharacterData::String("0".to_string());
        assert!(CharacterData::check_value(
            &data,
            &spec_pattern,
            AutosarVersion::Autosar_00050
        ));
        let data = CharacterData::String("2".to_string());
        assert!(!CharacterData::check_value(
            &data,
            &spec_pattern,
            AutosarVersion::Autosar_00050
        ));
    }

    #[test]
    fn check_version_compatibility() {
        let spec_enum = CharacterDataSpec::Enum {
            items: &[(EnumItem::default, 0x0001), (EnumItem::preserve, 0x0002)],
        };
        let data = CharacterData::Enum(EnumItem::default);
        let (result, _) = data.check_version_compatibility(&spec_enum, AutosarVersion::Autosar_4_0_1);
        assert!(result);
        let (result, _) = data.check_version_compatibility(&spec_enum, AutosarVersion::Autosar_00050);
        assert!(!result);
        let data = CharacterData::Enum(EnumItem::Abstract);
        let (result, _) = data.check_version_compatibility(&spec_enum, AutosarVersion::Autosar_00050);
        assert!(!result);
        let data = CharacterData::UnsignedInteger(0);
        let (result, _) = data.check_version_compatibility(&spec_enum, AutosarVersion::Autosar_00050);
        assert!(!result);
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

        let spec_double = CharacterDataSpec::Float;
        let data = CharacterData::parse("2.0", &spec_double, AutosarVersion::Autosar_00050).unwrap();
        assert_eq!(data.float_value().unwrap(), 2.0);
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
        let data = CharacterData::Float(1.23);
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

        assert!(CharacterData::Float(1.23).float_value().is_some());
        assert!(CharacterData::Float(1.23).unsigned_integer_value().is_none());

        assert!(CharacterData::String("x".to_string()).string_value().is_some());
        assert!(CharacterData::String("x".to_string()).float_value().is_none());

        assert!(CharacterData::UnsignedInteger(1).unsigned_integer_value().is_some());
        assert!(CharacterData::UnsignedInteger(1).enum_value().is_none());
    }

    #[test]
    fn parse_integer() {
        let data = CharacterData::String("text".to_string());
        let result = data.parse_integer::<u32>();
        assert!(result.is_none());

        let data = CharacterData::String("0x123412341234".to_string());
        let result = data.parse_integer::<u32>();
        assert!(result.is_none());

        let data = CharacterData::String("0x1234".to_string());
        let result = data.parse_integer::<u32>().unwrap();
        assert_eq!(result, 0x1234);

        let data = CharacterData::String("0X123456".to_string());
        let result = data.parse_integer::<u32>().unwrap();
        assert_eq!(result, 0x123456);

        let data = CharacterData::String("0b1010".to_string());
        let result = data.parse_integer::<u32>().unwrap();
        assert_eq!(result, 10);

        let data = CharacterData::String("0B101010".to_string());
        let result = data.parse_integer::<u32>().unwrap();
        assert_eq!(result, 42);

        let data = CharacterData::String("0733".to_string());
        let result = data.parse_integer::<u32>().unwrap();
        assert_eq!(result, 475);

        let data = CharacterData::String("0".to_string());
        let result = data.parse_integer::<u32>().unwrap();
        assert_eq!(result, 0);

        let data = CharacterData::String("-55".to_string());
        let result = data.parse_integer::<i32>().unwrap();
        assert_eq!(result, -55);

        let data = CharacterData::UnsignedInteger(0);
        let result = data.parse_integer::<u32>().unwrap();
        assert_eq!(result, 0);

        let data = CharacterData::Float(0.0);
        let result = data.parse_integer::<u32>();
        assert!(result.is_none());
    }

    #[test]
    fn parse_float() {
        // not a number
        let data = CharacterData::String("text".to_string());
        let result = data.parse_float();
        assert!(result.is_none());

        // zero
        let data = CharacterData::String("0".to_string());
        let result = data.parse_float().unwrap();
        assert_eq!(result, 0.0);

        // hex (> 32 bits)
        let data = CharacterData::String("0xFFFFFFFFF".to_string());
        let result = data.parse_float().unwrap();
        assert_eq!(result, 68719476735.0);

        // invalid format, hex and float can't be mixed like this
        let data = CharacterData::String("0x1.234".to_string());
        let result = data.parse_float();
        assert!(result.is_none());

        // float: normal number
        let data = CharacterData::String("0.0001".to_string());
        let result = data.parse_float().unwrap();
        assert_eq!(result, 0.0001);

        // float: normal number
        let data = CharacterData::String("1.234e5".to_string());
        let result = data.parse_float().unwrap();
        assert_eq!(result, 1.234e5);

        // float: 0.12 - not octal, note the leading zero!
        let data = CharacterData::String("00.12".to_string());
        let result = data.parse_float().unwrap();
        assert_eq!(result, 0.12);

        // float: NaN
        let data = CharacterData::String("NaN".to_string());
        let result = data.parse_float().unwrap();
        assert!(result.is_nan());

        // float: infinity
        let data = CharacterData::String("INF".to_string());
        let result = data.parse_float().unwrap();
        assert!(result.is_infinite());

        // hex
        let data = CharacterData::String("0x1234".to_string());
        let result = data.parse_float().unwrap();
        assert_eq!(result, 4660.0);

        // hex
        let data = CharacterData::String("0X1234".to_string());
        let result = data.parse_float().unwrap();
        assert_eq!(result, 4660.0);

        // binary
        let data = CharacterData::String("0b1101".to_string());
        let result = data.parse_float().unwrap();
        assert_eq!(result, 13.0);

        // binary
        let data = CharacterData::String("0B1101".to_string());
        let result = data.parse_float().unwrap();
        assert_eq!(result, 13.0);

        // octal
        let data = CharacterData::String("0777".to_string());
        let result = data.parse_float().unwrap();
        assert_eq!(result, 511.0);

        // integer value
        let data = CharacterData::UnsignedInteger(0);
        let result = data.parse_float().unwrap();
        assert_eq!(result, 0.0);

        // float value
        let data = CharacterData::Float(5.0);
        let result = data.parse_float().unwrap();
        assert_eq!(result, 5.0);

        // enum value
        let data = CharacterData::Enum(EnumItem::Abstract);
        let result = data.parse_float();
        assert!(result.is_none());
    }

    #[test]
    fn ordering() {
        let enum1 = CharacterData::Enum(EnumItem::Abstract);
        let enum2 = CharacterData::Enum(EnumItem::default);
        assert!(enum1 < enum2);

        let string1 = CharacterData::String("abcdef".to_string());
        let string2 = CharacterData::String("text".to_string());
        assert!(string1 < string2);

        let integer1 = CharacterData::UnsignedInteger(123);
        let integer2 = CharacterData::UnsignedInteger(456);
        assert!(integer1 < integer2);

        let float1 = CharacterData::Float(1.23);
        let float2 = CharacterData::Float(4.56);
        assert!(float1 < float2);

        // for unequal data types, the order is (arbitrarily) defined as enum < string < unsigned integer < float
        assert!(enum1 < string1);
        assert!(enum1 < integer1);
        assert!(enum1 < float1);
        assert!(string1 > enum1);
        assert!(string1 < integer1);
        assert!(string1 < float1);
        assert!(integer1 > enum1);
        assert!(integer1 > string1);
        assert!(integer1 < float1);
        assert!(float1 > enum1);
        assert!(float1 > string1);
        assert!(float1 > integer1);

        // PartialOrd
        assert!(enum1.partial_cmp(&enum2).unwrap() == std::cmp::Ordering::Less);
    }

    #[test]
    fn cdata_conversion() {
        let cdata: CharacterData = 0.into();
        assert_eq!(cdata, CharacterData::UnsignedInteger(0));

        let cdata: CharacterData = (1.0).into();
        assert_eq!(cdata, CharacterData::Float(1.0));

        let cdata: CharacterData = "text".into();
        assert_eq!(cdata, CharacterData::String("text".to_string()));

        let cdata: CharacterData = String::from("text").into();
        assert_eq!(cdata, CharacterData::String("text".to_string()));

        let cdata: CharacterData = EnumItem::Abstract.into();
        assert_eq!(cdata, CharacterData::Enum(EnumItem::Abstract));
    }
}
