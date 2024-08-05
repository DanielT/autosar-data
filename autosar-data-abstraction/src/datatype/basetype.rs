use crate::*;
use autosar_data::ElementName;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwBaseType(Element);
abstraction_element!(SwBaseType, SwBaseType);

impl SwBaseType {
    pub fn new(
        name: &str,
        package: &ArPackage,
        bit_length: u32,
        base_type_encoding: BaseTypeEncoding,
        byte_order: Option<ByteOrder>,
        mem_alignment: Option<u32>,
        native_declaration: Option<&str>,
    ) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let sw_base_type = Self(elements.create_named_sub_element(ElementName::SwBaseType, name)?);
        sw_base_type
            .element()
            .create_sub_element(ElementName::Category)?
            .set_character_data("FIXED_LENGTH")?;
        sw_base_type.set_base_type_encoding(base_type_encoding)?;
        sw_base_type.set_bit_length(bit_length)?;
        if let Some(byte_order) = byte_order {
            sw_base_type.set_byte_order(byte_order)?;
        }
        if let Some(mem_alignment) = mem_alignment {
            sw_base_type.set_mem_alignment(mem_alignment)?;
        }
        if let Some(native_declaration) = native_declaration {
            sw_base_type.set_native_declaration(native_declaration)?;
        }

        Ok(sw_base_type)
    }

    /// set the base type size (in bits) of the SwBaseType
    pub fn set_bit_length(&self, bit_length: u32) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::BaseTypeSize)?
            .set_character_data(bit_length.to_string())?;
        Ok(())
    }

    /// get the bit length of the SwBaseType
    pub fn bit_length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::BaseTypeSize)?
            .character_data()?
            .parse_integer()
    }

    /// set the base type encoding of the SwBaseType
    pub fn set_base_type_encoding(&self, base_type_encoding: BaseTypeEncoding) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::BaseTypeEncoding)?
            .set_character_data(base_type_encoding.to_string())?;
        Ok(())
    }

    /// get the base type encoding of the SwBaseType
    pub fn base_type_encoding(&self) -> Option<BaseTypeEncoding> {
        let encoding_text = self
            .element()
            .get_sub_element(ElementName::BaseTypeEncoding)?
            .character_data()?
            .string_value()?;
        BaseTypeEncoding::try_from(&*encoding_text).ok()
    }

    /// set the byte order of the SwBaseType
    ///
    /// The byte order is platform specific and should only be set when it is really needed.
    pub fn set_byte_order(&self, byte_order: ByteOrder) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::ByteOrder)?
            .set_character_data::<EnumItem>(byte_order.into())?;
        Ok(())
    }

    /// get the byte order of the SwBaseType
    pub fn byte_order(&self) -> Option<ByteOrder> {
        self.element()
            .get_sub_element(ElementName::ByteOrder)?
            .character_data()?
            .enum_value()?
            .try_into()
            .ok()
    }

    /// set the memory alignment of the SwBaseType
    ///
    /// The memory alignment describes the slignement in bits. Example: 8 means that the type is aligned to a byte.
    /// Since the memory alignment is platform specific, it should only be set when it is really needed.
    pub fn set_mem_alignment(&self, mem_alignment: u32) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::MemAlignment)?
            .set_character_data(mem_alignment.to_string())?;
        Ok(())
    }

    /// get the memory alignment of the SwBaseType
    pub fn mem_alignment(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::MemAlignment)?
            .character_data()?
            .parse_integer()
    }

    /// set the native declaration of the SwBaseType
    ///
    /// The native declaration is a string that represents the type in the native programming language.
    pub fn set_native_declaration(&self, native_declaration: &str) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::NativeDeclaration)?
            .set_character_data(native_declaration)?;
        Ok(())
    }

    /// get the native declaration of the SwBaseType
    pub fn native_declaration(&self) -> Option<String> {
        self.element()
            .get_sub_element(ElementName::NativeDeclaration)?
            .character_data()?
            .string_value()
    }
}

//#########################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseTypeEncoding {
    OnesComplement,
    /// TwosComplement is used for signed integers
    TwosComplement,
    /// SignMagnitude is used for signed integers
    SignMagnitude,
    /// BcdPacked is used for packed binary coded decimals
    BcdPacked,
    /// BcdUnpacked is used for unpacked binary coded decimals
    BcdUnpacked,
    /// DspFractional is used for values in a digital signal processor
    DspFractional,
    /// Ieee754 is used for floating point numbers
    Ieee754,
    /// encoding: Iso8859_1 is used for strings
    Iso8859_1,
    /// encoding: Iso8859_2 is used for strings
    Iso8859_2,
    /// encoding: Windows1252 is used for strings
    Windows1252,
    /// encoding: Utf8 is used for strings
    Utf8,
    /// encoding: Utf16 is used for strings - byte order must be specified
    Utf16,
    /// encoding: Ucs2 is used for strings
    Ucs2,
    /// encoding: Boolean is used for boolean values
    Boolean,
    /// encoding: Void is used for C void types
    Void,
    /// encoding: None is used for unsigned integers
    None,
}

impl Display for BaseTypeEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseTypeEncoding::OnesComplement => f.write_str("1C"),
            BaseTypeEncoding::TwosComplement => f.write_str("2C"),
            BaseTypeEncoding::SignMagnitude => f.write_str("SM"),
            BaseTypeEncoding::BcdPacked => f.write_str("BCD-P"),
            BaseTypeEncoding::BcdUnpacked => f.write_str("BCD-UP"),
            BaseTypeEncoding::DspFractional => f.write_str("DSP-FRACTIONAL"),
            BaseTypeEncoding::Ieee754 => f.write_str("IEEE754"),
            BaseTypeEncoding::Iso8859_1 => f.write_str("ISO-8859-1"),
            BaseTypeEncoding::Iso8859_2 => f.write_str("ISO-8859-2"),
            BaseTypeEncoding::Windows1252 => f.write_str("WINDOWS-1252"),
            BaseTypeEncoding::Utf8 => f.write_str("UTF-8"),
            BaseTypeEncoding::Utf16 => f.write_str("UTF-16"),
            BaseTypeEncoding::Ucs2 => f.write_str("UCS-2"),
            BaseTypeEncoding::Boolean => f.write_str("BOOLEAN"),
            BaseTypeEncoding::Void => f.write_str("VOID"),
            BaseTypeEncoding::None => f.write_str("NONE"),
        }
    }
}

impl TryFrom<&str> for BaseTypeEncoding {
    type Error = AutosarAbstractionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1C" => Ok(BaseTypeEncoding::OnesComplement),
            "2C" => Ok(BaseTypeEncoding::TwosComplement),
            "SM" => Ok(BaseTypeEncoding::SignMagnitude),
            "BCD-P" => Ok(BaseTypeEncoding::BcdPacked),
            "BCD-UP" => Ok(BaseTypeEncoding::BcdUnpacked),
            "DSP-FRACTIONAL" => Ok(BaseTypeEncoding::DspFractional),
            "IEEE754" => Ok(BaseTypeEncoding::Ieee754),
            "ISO-8859-1" => Ok(BaseTypeEncoding::Iso8859_1),
            "ISO-8859-2" => Ok(BaseTypeEncoding::Iso8859_2),
            "WINDOWS-1252" => Ok(BaseTypeEncoding::Windows1252),
            "UTF-8" => Ok(BaseTypeEncoding::Utf8),
            "UTF-16" => Ok(BaseTypeEncoding::Utf16),
            "UCS-2" => Ok(BaseTypeEncoding::Ucs2),
            "BOOLEAN" => Ok(BaseTypeEncoding::Boolean),
            "VOID" => Ok(BaseTypeEncoding::Void),
            "NONE" => Ok(BaseTypeEncoding::None),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "BaseTypeEncoding".to_string(),
            }),
        }
    }
}

//#########################################################

#[cfg(test)]
mod tests {
    use super::*;
    use autosar_data::AutosarVersion;

    #[test]
    fn test_base_type_encoding() {
        let encoding = BaseTypeEncoding::OnesComplement;
        assert_eq!(encoding.to_string(), "1C");
        assert_eq!(BaseTypeEncoding::try_from("1C").unwrap(), encoding);
    }

    #[test]
    fn test_sw_base_type() {
        let model = AutosarModel::new();
        let _file = model.create_file("test.arxml", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/BaseTypes").unwrap();

        let sw_base_type = SwBaseType::new(
            "TestType",
            &package,
            32,
            BaseTypeEncoding::None,
            Some(ByteOrder::MostSignificantByteFirst),
            Some(8),
            Some("uint32"),
        )
        .unwrap();
        assert_eq!(sw_base_type.bit_length(), Some(32));
        assert_eq!(sw_base_type.base_type_encoding(), Some(BaseTypeEncoding::None));
        assert_eq!(sw_base_type.byte_order(), Some(ByteOrder::MostSignificantByteFirst));
        assert_eq!(sw_base_type.mem_alignment(), Some(8));
        assert_eq!(sw_base_type.native_declaration(), Some("uint32".to_string()));
    }
}
