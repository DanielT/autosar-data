use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::Write;
use std::str::Utf8Error;
use thiserror::Error;

use crate::specification::*;
use crate::lexer::*;
use crate::*;

#[derive(Debug, Error)]
pub enum ArxmlParserError {
    #[error("Invalid arxml file: bad file header")]
    InvalidArxmlFileHeader,

    #[error("Unknown Autosar xsd file {input_verstring} referenced in the file header")]
    UnknownAutosarVersion {
        input_verstring: String,
    },

    #[error("Encountered unexpected child element {sub_element} inside element {element}")]
    IncorrectBeginElement {
        element: ElementName,
        sub_element: ElementName,        
    },

    #[error("Encountered invalid child element {invalid_element} inside parent element {element}. {invalid_element} is not a known Autosar element.")]
    InvalidBeginElement {
        element: ElementName,
        invalid_element: String
    },

    #[error("Encountered the closing tag for element {other_element}, but element {element} was open.")]
    IncorrectEndElement {
        element: ElementName,
        other_element: ElementName,
    },

    #[error("Encountered invalid end tag for element {invalid_element} inside parent element {parent_element}. {invalid_element} is not a known Autosar element.")]
    InvalidEndElement {
        parent_element: ElementName,
        invalid_element: String,
    },

    #[error("Sub element {sub_element} in {element} occurred out of order")]
    ElementSequenceOutOfOrder {
        element: ElementName,
        sub_element: ElementName,
    },

    #[error("Multiple conflicting sub elements have been added to element {element}. The latest was {sub_element}.")]
    ElementChoiceConflict {
        element: ElementName,
        sub_element: ElementName,
    },

    #[error("Element {sub_element} exists in {element}, but is not allowed in {version}")]
    ElementVersionError {
        element: ElementName,
        sub_element: ElementName,
        version: AutosarVersion,
    },

    #[error("Sub elements are not permitted inside {element}, but sub element {sub_element} was found")]
    SubElementNotPermitted {
        element: ElementName,
        sub_element: ElementName,
    },

    #[error("Only one {sub_element} is allowed inside {element}, but another occurrence was found")]
    TooManySubElements {
        element: ElementName,
        sub_element: ElementName,
    },

    #[error("The required sub element {sub_element} was not found in element {element}")]
    RequiredSubelementMissing {
        element: ElementName,
        sub_element: ElementName,
    },

    #[error("Element {element} contains unknown attribute {attribute}")]
    UnknownAttributeError {
        element: ElementName,
        attribute: String
    },

    #[error("Attribute {attribute} exists in element {element}, but is not allowed in {version}")]
    AttributeVersionError {
        element: ElementName,
        attribute: AttributeName,
        version: AutosarVersion
    },

    #[error("Attribute {attribute} is required in element {element}, but was not found")]
    RequiredAttributeMissing {
        element: ElementName,
        attribute: AttributeName,
    },

    #[error("Character content found, which is not allowed inside element {element}")]
    CharacterContentForbidden {
        element: ElementName
    },

    #[error("enum item {enum_item} is a valid value in element {element}, but is not allowed in {version}")]
    EnumItemVersionError {
        element: ElementName,
        enum_item: EnumItem,
        version: AutosarVersion
    },

    #[error("string {value} is not a valid enum item")]
    UnknownEnumItem {
        value: String
    },

    #[error("string {value} is not a valid enum item in this context")]
    InvalidEnumItem {
        value: String
    },

    #[error("string value {value} is too long: max length is {length}")]
    StringValueTooLong {
        value: String,
        length: usize
    },

    #[error("string value {value} is not matched by the validation regex {regex}")]
    RegexMatchError {
        value: String,
        regex: String
    },

    #[error("could not convert value to utf-8: {source}")]
    Utf8Error {
        source: Utf8Error
    },

    #[error("Unexpected end of file while parsing element {element}")]
    UnexpectedEndOfFile {
        element: ElementName
    }
}


pub(crate) struct ArxmlParser<'a> {
    filename: OsString,
    line: usize,
    buffer: &'a [u8],
    fileversion: AutosarVersion,
    current_element: ElementName,
    log_func: fn (AutosarDataError),
    strict: bool,
    version_compatibility: u32,
    identifiables: HashMap<String, WeakElement>,
}

impl<'a> ArxmlParser<'a> {
    pub(crate) fn new(filename: OsString, buffer: &'a [u8], log_func: fn (AutosarDataError), strict: bool) -> Self {
        Self {
            filename: filename.clone(),
            line: 1,
            buffer,
            fileversion : AutosarVersion::Autosar_4_0_1, // this is temporary and gets replaced as soon as the xsd declaration in the top-level AUTOSAR element is read
            current_element: ElementName::Autosar,
            log_func,
            strict,
            version_compatibility: u32::MAX,
            identifiables: HashMap::new(),
        }
    }

    fn next<'b>(&mut self, lexer: &'b mut ArxmlLexer) -> Result<ArxmlEvent<'b>, AutosarDataError> {
        let (line, event) = lexer.next()?;
        self.line = line;
        Ok(event)
    }

    pub(crate) fn error(&self, err: ArxmlParserError) -> AutosarDataError {
        AutosarDataError::ParserError { filename: self.filename.clone(), line: self.line, source: err}
    }

    pub(crate) fn optional_error(&self, err: ArxmlParserError) -> Result<(), AutosarDataError> {
        let wrapped_err = AutosarDataError::ParserError { filename: self.filename.clone(), line: self.line, source: err};
        if self.strict {
            Err(wrapped_err)
        } else {
            (self.log_func)(wrapped_err);
            Ok(())
        }
    }

    fn check_version(&mut self, item_version: u32, error: ArxmlParserError) -> Result<(), AutosarDataError> {
        self.version_compatibility &= item_version;
        if (self.fileversion as u32) & item_version == 0 {
            self.optional_error(error)
        } else {
            Ok(())
        }
    }

    pub(crate) fn parse_arxml(&mut self) -> Result<(Element, HashMap<String, WeakElement>), AutosarDataError> {
        let mut lexer = ArxmlLexer::new(self.buffer, self.filename.clone());

        if let ArxmlEvent::ArxmlHeader = self.next(&mut lexer)? {
            /* OK */
        } else {
            return Err(self.error(ArxmlParserError::InvalidArxmlFileHeader));
        }
    
        if let ArxmlEvent::BeginElement(elemname, attributes_text) = self.next(&mut lexer)? {
            if let Some(ElementName::Autosar) = ElementName::from_bytes(elemname) {
                let type_autosar = &specification::DATATYPES[specification::ROOT_DATATYPE];
    
                let attributes = self.parse_attribute_text(attributes_text, type_autosar.attributes)?;
                let attr_xmlns = attributes.iter().find(|attr| attr.attrname == AttributeName::xmlns);
                let attr_xsi = attributes.iter().find(|attr| attr.attrname == AttributeName::xmlnsXsi);
                let attr_schema = attributes.iter().find(|attr| attr.attrname == AttributeName::xsiSchemalocation);
                if let ( Some(Attribute{content: CharacterData::String(xmlns), ..}), Some(Attribute{content: CharacterData::String(xsi), ..}), Some(Attribute{content: CharacterData::String(schema), ..})) = (attr_xmlns, attr_xsi, attr_schema) {
                    if &*xmlns != "http://autosar.org/schema/r4.0" || &*xsi != "http://www.w3.org/2001/XMLSchema-instance" {
                        return Err(self.error(ArxmlParserError::InvalidArxmlFileHeader));
                    }
                    let mut schema_parts = schema.split(' ');
                    let schema_base = schema_parts.next().unwrap_or("");
                    if schema_base != "http://autosar.org/schema/r4.0" {
                        return Err(self.error(ArxmlParserError::InvalidArxmlFileHeader));
                    }
                    let xsd_file = schema_parts.next().unwrap_or("");
                    if let Some(autosar_version) = AutosarVersion::from_str(xsd_file) {
                        self.fileversion = autosar_version;
                    } else {
                        self.fileversion = AutosarVersion::Autosar_00050;
                        self.optional_error(ArxmlParserError::UnknownAutosarVersion {input_verstring: xsd_file.to_string()})?;
                    }

                    let autosar_root_element = self.parse_element(ElementName::Autosar, attributes, specification::ROOT_DATATYPE, "", &mut lexer)?;

                    let mut identifiables = HashMap::new();
                    std::mem::swap(&mut self.identifiables, &mut identifiables);
                    return Ok((autosar_root_element, identifiables));
                } else {
                    return Err(self.error(ArxmlParserError::InvalidArxmlFileHeader));
                }
            }
        }
        Err(self.error(ArxmlParserError::InvalidArxmlFileHeader))
    }


    fn parse_element(&mut self, element_name: ElementName, attributes: Vec<Attribute>, datatype_id: usize, path_in: &str, lexer: &mut ArxmlLexer) -> Result<Element, AutosarDataError> {
        let mut element = ElementRaw {
            elemname: element_name,
            attributes,
            content: Vec::new(),
            type_id: datatype_id,
        };

        // track the current element name in the parser for error messages
        self.current_element = element_name;
        let datatype = &specification::DATATYPES[datatype_id];
        let mut elem_spec_idx = 0;
        let mut elem_group_idx = 0;
        let mut num_elements: usize = 0;
        let mut short_name_found = false;
        let mut element_count = HashMap::<u16, usize>::new();
        let mut path = path_in.to_owned();

        loop {
            let arxmlevent = self.next(lexer)?;
            match arxmlevent {
                ArxmlEvent::BeginElement(elem_text, attr_text) => {
                    if let Some(name) = ElementName::from_bytes(elem_text) {
                        (elem_spec_idx, elem_group_idx) = match datatype.mode {
                            ContentMode::Sequence => self.find_element_in_sequence_spec(name, datatype_id, elem_spec_idx)?,
                            ContentMode::Choice => self.find_element_in_choice_spec(name, datatype_id, elem_spec_idx, elem_group_idx, num_elements)?,
                            ContentMode::Bag
                            | ContentMode::Mixed => self.find_element_in_bag_spec(name, datatype_id)?,
                            ContentMode::Characters => return Err(self.error(ArxmlParserError::SubElementNotPermitted {element: element.elemname, sub_element: name})),
                        };

                        // the if let is always true, because (elem_spec_idx, elem_group_idx) were only just found by find_element_in_spec()
                        if let Some(SubElement::Element { elemtype, version_mask, .. }) = get_sub_element_type(&datatype.sub_elements, elem_spec_idx, elem_group_idx) {
                            self.check_multiplicity(name, datatype_id, elem_spec_idx, elem_group_idx, &mut element_count)?;

                            self.check_version(*version_mask, ArxmlParserError::ElementVersionError{element: element.elemname, sub_element: name, version: self.fileversion})?;
                            let sub_elem_type = &DATATYPES[*elemtype];
                            let attributes = self.parse_attribute_text(attr_text, sub_elem_type.attributes)?;
                            let sub_element = self.parse_element(name, attributes, *elemtype, &path, lexer)?;
                            if name == ElementName::ShortName {
                                short_name_found = true;
                                if let Some(CharacterData::String(name_string)) = sub_element.get_character_data() {
                                    path.write_char('/').unwrap();
                                    path.write_str(&name_string).unwrap();
                                }
                            }
                            element.content.push(ElementContent::Element(sub_element));
                            num_elements += 1;

                        }
                    } else {
                        return Err(self.error(ArxmlParserError::InvalidBeginElement {element: element.elemname, invalid_element: String::from_utf8_lossy(elem_text).to_string()}));
                    }
                }
                ArxmlEvent::EndElement(elem_text) => {
                    if let Some(name) = ElementName::from_bytes(elem_text) {
                        if name == element.elemname {
                            break;
                        } else {
                            return Err(self.error(ArxmlParserError::IncorrectEndElement {element: element.elemname, other_element: name}));
                        }
                    } else {
                        return Err(self.error(ArxmlParserError::InvalidEndElement {parent_element: element.elemname, invalid_element: String::from_utf8_lossy(elem_text).to_string()}));
                    }
                }
                ArxmlEvent::Characters(text_content) => {
                    if let Some(character_data_spec) = datatype.character_data {
                        match datatype.mode {
                            ContentMode::Sequence
                            | ContentMode::Choice
                            | ContentMode::Bag => self.optional_error(ArxmlParserError::CharacterContentForbidden { element: element.elemname })?,
                            ContentMode::Characters
                            | ContentMode::Mixed => {
                                let value = self.parse_character_data(text_content, character_data_spec)?;
                                element.content.push(ElementContent::CharacterData(value));
                            }
                        }
                    } else {
                        self.optional_error(ArxmlParserError::CharacterContentForbidden { element: element.elemname })?;
                    }
                }
                ArxmlEvent::ArxmlHeader => todo!(),
                ArxmlEvent::EOF => {
                    return Err(self.error(ArxmlParserError::UnexpectedEndOfFile { element: element.elemname }));
                }
            }
        }

        let wrapped_element = Element(Arc::new(Mutex::new(element)));
        if short_name_found {
            self.identifiables.insert(path, wrapped_element.downgrade());
        } else if datatype.is_named & self.version_compatibility != 0 {
            self.optional_error(ArxmlParserError::RequiredSubelementMissing {element: element_name, sub_element: ElementName::ShortName})?;
        }

        Ok(wrapped_element)
    }


    fn find_element_in_sequence_spec(&mut self, name: ElementName, type_id: usize, elem_spec_idx: usize) -> Result<(usize, usize), AutosarDataError> {
        if let Some((spec_idx, group_idx)) = self.find_element_in_spec(name, type_id, elem_spec_idx, 0) {
            Ok((spec_idx, group_idx))
        } else if let Some((spec_idx, group_idx)) = self.find_element_in_spec(name, type_id, 0, 0) {
            self.optional_error(ArxmlParserError::ElementSequenceOutOfOrder {element: self.current_element, sub_element: name})?;
            Ok((spec_idx, group_idx))
        } else {
            Err(self.error(ArxmlParserError::IncorrectBeginElement {element: self.current_element, sub_element: name}))
        }
    }


    fn find_element_in_choice_spec(&mut self, name: ElementName, type_id: usize, elem_spec_idx: usize, elem_group_idx: usize, element_count: usize) -> Result<(usize, usize), AutosarDataError> {
        if let Some((spec_idx, group_idx)) = self.find_element_in_spec(name, type_id, elem_spec_idx, elem_group_idx) {
            if element_count != 0 {
                if elem_spec_idx != spec_idx {
                    return Err(self.error(ArxmlParserError::ElementChoiceConflict {element: self.current_element, sub_element: name}));
                }
                if elem_group_idx < group_idx {
                    self.optional_error(ArxmlParserError::ElementSequenceOutOfOrder {element: self.current_element, sub_element: name})?;
                }
            }
            Ok((spec_idx, group_idx))
        } else if let Some((_, _)) = self.find_element_in_spec(name, type_id, 0, 0) {
            // element was found only after restarting the search from the beginning
            Err(self.error(ArxmlParserError::ElementChoiceConflict {element: self.current_element, sub_element: name}))
        } else {
            // element was not found in the sub element specification
            Err(self.error(ArxmlParserError::IncorrectBeginElement {element: self.current_element, sub_element: name}))
        }
    }


    fn find_element_in_bag_spec(&mut self, name: ElementName, type_id: usize) -> Result<(usize, usize), AutosarDataError> {
        if let Some((spec_idx, group_idx)) = self.find_element_in_spec(name, type_id, 0, 0) {
            Ok((spec_idx, group_idx))
        } else {
            // element was not found in the sub element specification
            Err(self.error(ArxmlParserError::IncorrectBeginElement {element: self.current_element, sub_element: name}))
        }
    }

    fn find_element_in_spec(&mut self, target_name: ElementName, type_id: usize, start_pos: usize, group_pos: usize) -> Option<(usize, usize)> {
        let spec = DATATYPES[type_id].sub_elements;
        for (cur_pos, sub_element) in spec.iter().enumerate().skip(start_pos) {
            match sub_element {
                SubElement::Element { name, .. } => {
                    if *name == target_name {
                        return Some((cur_pos, 0));
                    }
                }
                SubElement::Group { groupid } => {
                    let group_spec = &specification::DATATYPES[*groupid];
                    // the group_pos parameter is only valid when referring to the group at start_pos. All other groups are searched from the beginning
                    let group_pos = if start_pos == cur_pos { group_pos } else { 0 };
                    for (cur_group_pos, group_sub_element) in group_spec.sub_elements.iter().enumerate().skip(group_pos) {
                        if let SubElement::Element { name, .. } = group_sub_element {
                            if *name == target_name {
                                return Some((cur_pos, cur_group_pos));
                            }
                        } else {
                            // at them moment groups can only contain elements, so this case doesn't happen
                            todo!();
                        }
                    }
                }
            }
        }
        None
    }


    fn parse_attribute_text(&mut self, attributes_text: &[u8], attr_definitions: &[(AttributeName, &'static CharacterDataSpec, bool, u32)]) -> Result<Vec<Attribute>, AutosarDataError> {
        let mut attributes = Vec::new();
        // attributes_text is a byte string containig all the attributes of an element
        // for example: xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_4-2-2.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        // when this string is split on ", there will be an odd number of parts, with the last part being empty
        let mut attr_part_iter = attributes_text.split(|c| *c == b'"');
        loop {
            if let (Some(attr_name_part), Some(attr_value_part)) = (attr_part_iter.next(), attr_part_iter.next()) {
                // attr_name_part may have leading whitespace and will always have a trailing '='
                // these need to be stripped
                let name_len = attr_name_part.len() - 1; // exclude the trailing =
                let (name_start, _) = attr_name_part.iter().enumerate().find(|(_, c)| !c.is_ascii_whitespace()).unwrap_or((name_len, &0u8));
                if let Some(attr_name) = AttributeName::from_bytes(&attr_name_part[name_start .. name_len]) {
                    if let Some((_, ctype, _, version_mask)) = attr_definitions.iter().find(|(name,_,_,_)| *name == attr_name) {
                        self.check_version(*version_mask, ArxmlParserError::AttributeVersionError{element: self.current_element, attribute: attr_name, version: self.fileversion})?;
                        let attr_value = self.parse_character_data(attr_value_part, *ctype)?;
                        attributes.push(Attribute {attrname: attr_name, content: attr_value});
                    } else {
                        self.optional_error(ArxmlParserError::UnknownAttributeError {element: self.current_element, attribute: attr_name.to_string()})?;
                    }
                } else {
                    self.optional_error(ArxmlParserError::UnknownAttributeError {element: self.current_element, attribute: String::from_utf8_lossy(&attr_name_part[name_start .. name_len]).to_string()})?;
                }
            } else {
                break;
            }
        }

        for (name, _ctype, required, _ver) in attr_definitions {
            if *required && attributes.iter().find(|attr| attr.attrname == *name).is_none() {
                self.optional_error(ArxmlParserError::RequiredAttributeMissing {element: self.current_element, attribute: *name})?;
            }
        }
    
        Ok(attributes)
    }
    
    
    fn parse_character_data(&mut self, input: &[u8], character_data_spec: &CharacterDataSpec) -> Result<CharacterData, AutosarDataError> {
        let trimmed_input = trim_byte_string(input);
        match character_data_spec {
            CharacterDataSpec::Enum { items } => {
                let value = specification::EnumItem::from_bytes(trimmed_input).ok_or(
                    self.error(ArxmlParserError::UnknownEnumItem { value: String::from_utf8_lossy(&trimmed_input).to_string()})
                )?;
                let (_, version) = items.iter().find(|(item, _)| *item == value).ok_or(
                    self.error(ArxmlParserError::InvalidEnumItem { value: String::from_utf8_lossy(&trimmed_input).to_string()})
                )?;
                self.check_version(*version, ArxmlParserError::EnumItemVersionError {element: self.current_element, enum_item: value, version: self.fileversion})?;
                Ok(CharacterData::Enum(value))
            }
            CharacterDataSpec::Pattern { check_fn, regex, max_length } => {
                if max_length.is_some() && trimmed_input.len() > max_length.unwrap() {
                    self.optional_error(ArxmlParserError::StringValueTooLong { value: String::from_utf8_lossy(&trimmed_input).to_string(), length: max_length.unwrap() })?;
                }
                if check_fn(trimmed_input) == false {
                    self.optional_error(ArxmlParserError::RegexMatchError { value: String::from_utf8_lossy(&trimmed_input).to_string(), regex: regex.to_string() })?;
                }
                match std::str::from_utf8(trimmed_input) {
                    Ok(utf8string) => Ok(CharacterData::String(utf8string.to_owned())),
                    Err(err) => Err(self.error(ArxmlParserError::Utf8Error { source: err }))
                }            
            },
            CharacterDataSpec::String { preserve_whitespace, max_length } => {
                let text = if *preserve_whitespace {
                    input
                } else {
                    trimmed_input
                };
                if max_length.is_some() && text.len() > max_length.unwrap() {
                    self.optional_error(ArxmlParserError::StringValueTooLong { value: String::from_utf8_lossy(&trimmed_input).to_string(), length: max_length.unwrap() })?;
                }
                match std::str::from_utf8(text) {
                    Ok(utf8string) => Ok(CharacterData::String(utf8string.to_owned())),
                    Err(err) => Err(self.error(ArxmlParserError::Utf8Error { source: err }))
                }            
            }
            CharacterDataSpec::UnsignedInteger => {
                let strval = std::str::from_utf8(trimmed_input)
                    .map_err(|err| self.error(ArxmlParserError::Utf8Error { source: err }))?;
                let value = usize::from_str_radix(strval, 10)
                    .map_err(|_| self.error(ArxmlParserError::InvalidArxmlFileHeader))?;
    
                Ok(CharacterData::UnsignedInteger(value))
            }
            CharacterDataSpec::Double => {
                let strval = std::str::from_utf8(trimmed_input)
                    .map_err(|err| self.error(ArxmlParserError::Utf8Error { source: err }))?;
                let value = strval.parse::<f64>()
                    .map_err(|_| self.error(ArxmlParserError::InvalidArxmlFileHeader))?;
    
                Ok(CharacterData::Double(value))
            }
        }
    }


    fn check_multiplicity(&mut self, name: ElementName, type_id: usize, elem_spec_idx: usize, elem_group_idx: usize, element_count: &mut HashMap<u16, usize>) -> Result<(), AutosarDataError> {
        let datatype = &DATATYPES[type_id];
        // check the multiplicity - in practice the only restriction that matters here is having too many elements where the mutliplicity is not Any
        match &datatype.sub_elements[elem_spec_idx] {
            SubElement::Element { multiplicity, .. } => {
                if datatype.mode == ContentMode::Sequence || datatype.mode == ContentMode::Choice {
                    if let Some(val) = element_count.get_mut(&(name as u16)) {
                        *val += 1;
                        if ElementMultiplicity::Any != *multiplicity {
                            self.optional_error(ArxmlParserError::TooManySubElements {element: self.current_element, sub_element: name})?;
                        }
                    } else {
                        element_count.insert(name as u16, 1);
                    }
                }
            }
            SubElement::Group { groupid } => {
                self.check_multiplicity(name, *groupid, elem_group_idx, 0, element_count)?;
            }
        }
        Ok(())
    }

    pub(crate) fn get_fileversion(&self) -> AutosarVersion {
        self.fileversion
    }
}


fn get_sub_element_type(spec: &[SubElement], element_idx: usize, group_idx: usize) -> Option<&SubElement> {
    match spec.get(element_idx) {
        Some(sub_elem_spec @ SubElement::Element { .. }) => {
            Some(sub_elem_spec)
        }
        Some(SubElement::Group { groupid }) => {
            let group_spec = &specification::DATATYPES[*groupid];
            get_sub_element_type(&group_spec.sub_elements, group_idx, usize::MAX)
        }
        None => None
    }
}


fn trim_byte_string(input: &[u8]) -> &[u8] {
    let mut len = input.len();
    while input[len-1].is_ascii_whitespace() {
        len -= 1;
    }
    let (start, _) = input.iter().enumerate().find(|(_, c)| !c.is_ascii_whitespace()).unwrap_or((len, &0u8));
    &input[start .. len]
}