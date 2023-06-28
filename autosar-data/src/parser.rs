use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::str::FromStr;
use std::str::Utf8Error;
use thiserror::Error;

use crate::lexer::*;
use crate::*;

#[derive(Debug, Error)]
pub enum ArxmlParserError {
    #[error("Invalid arxml file: bad file header")]
    InvalidArxmlFileHeader,

    #[error("Unknown Autosar xsd file {input_verstring} referenced in the file header")]
    UnknownAutosarVersion { input_verstring: String },

    #[error("Encountered unexpected child element {sub_element} inside element {element}")]
    IncorrectBeginElement {
        element: ElementName,
        sub_element: ElementName,
    },

    #[error("Encountered invalid child element {invalid_element} inside parent element {element}. {invalid_element} is not a known Autosar element.")]
    InvalidBeginElement {
        element: ElementName,
        invalid_element: String,
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
    UnknownAttributeError { element: ElementName, attribute: String },

    #[error("Attribute {attribute} exists in element {element}, but is not allowed in {version}")]
    AttributeVersionError {
        element: ElementName,
        attribute: AttributeName,
        version: AutosarVersion,
    },

    #[error("Attribute {attribute} is required in element {element}, but was not found")]
    RequiredAttributeMissing {
        element: ElementName,
        attribute: AttributeName,
    },

    #[error("Character content found, which is not allowed inside element {element}")]
    CharacterContentForbidden { element: ElementName },

    #[error("enum item {enum_item} is a valid value in element {element}, but is not allowed in {version}")]
    EnumItemVersionError {
        element: ElementName,
        enum_item: EnumItem,
        version: AutosarVersion,
    },

    #[error("string {value} is not a valid enum item")]
    UnknownEnumItem { value: String },

    #[error("string {value} is not a valid enum item in this context")]
    InvalidEnumItem { value: String },

    #[error("string value {value} is too long: max length is {length}")]
    StringValueTooLong { value: String, length: usize },

    #[error("string value {value} is not matched by the validation regex {regex}")]
    RegexMatchError { value: String, regex: String },

    #[error("could not convert value to utf-8: {source}")]
    Utf8Error { source: Utf8Error },

    #[error("Unexpected end of file while parsing element {element}")]
    UnexpectedEndOfFile { element: ElementName },

    #[error("Failed to parse {input} as a number")]
    InvalidNumber { input: String },

    #[error("Additional data found in the input after the final </AUTOSAR> element")]
    AdditionalDataError,

    #[error("Invalid XML entity in {input}")]
    InvalidXmlEntity { input: String },
}

pub(crate) struct ArxmlParser<'a> {
    filename: PathBuf,
    line: usize,
    buffer: &'a [u8],
    fileversion: AutosarVersion,
    current_element: ElementName,
    strict: bool,
    version_compatibility: u32,
    pub(crate) identifiables: Vec<(String, WeakElement)>,
    pub(crate) references: Vec<(String, WeakElement)>,
    pub(crate) warnings: Vec<AutosarDataError>,
}

impl<'a> ArxmlParser<'a> {
    pub(crate) fn new(filename: PathBuf, buffer: &'a [u8], strict: bool) -> Self {
        Self {
            filename,
            line: 1,
            buffer,
            fileversion: AutosarVersion::Autosar_4_0_1, // this is temporary and gets replaced as soon as the xsd declaration in the top-level AUTOSAR element is read
            current_element: ElementName::Autosar,
            strict,
            version_compatibility: u32::MAX,
            identifiables: Vec::new(),
            references: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn next<'b>(&mut self, lexer: &'b mut ArxmlLexer) -> Result<ArxmlEvent<'b>, AutosarDataError> {
        let (line, event) = lexer.next()?;
        self.line = line;
        Ok(event)
    }

    pub(crate) fn error(&self, err: ArxmlParserError) -> AutosarDataError {
        AutosarDataError::ParserError {
            filename: self.filename.clone(),
            line: self.line,
            source: err,
        }
    }

    pub(crate) fn optional_error(&mut self, err: ArxmlParserError) -> Result<(), AutosarDataError> {
        let wrapped_err = AutosarDataError::ParserError {
            filename: self.filename.clone(),
            line: self.line,
            source: err,
        };
        if self.strict {
            Err(wrapped_err)
        } else {
            self.warnings.push(wrapped_err);
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

    pub(crate) fn parse_arxml(&mut self) -> Result<Element, AutosarDataError> {
        let mut lexer = ArxmlLexer::new(self.buffer, self.filename.clone());

        if let ArxmlEvent::ArxmlHeader = self.next(&mut lexer)? {
            /* OK */
        } else {
            return Err(self.error(ArxmlParserError::InvalidArxmlFileHeader));
        }

        if let ArxmlEvent::BeginElement(elemname, attributes_text) = self.next(&mut lexer)? {
            if let Ok(ElementName::Autosar) = ElementName::from_bytes(elemname) {
                let attributes = self.parse_attribute_text(ElementType::ROOT, attributes_text)?;
                let attr_xmlns = attributes.iter().find(|attr| attr.attrname == AttributeName::xmlns);
                let attr_xsi = attributes.iter().find(|attr| attr.attrname == AttributeName::xmlnsXsi);
                let attr_schema = attributes
                    .iter()
                    .find(|attr| attr.attrname == AttributeName::xsiSchemalocation);
                if let (
                    Some(Attribute {
                        content: CharacterData::String(xmlns),
                        ..
                    }),
                    Some(Attribute {
                        content: CharacterData::String(xsi),
                        ..
                    }),
                    Some(Attribute {
                        content: CharacterData::String(schema),
                        ..
                    }),
                ) = (attr_xmlns, attr_xsi, attr_schema)
                {
                    if xmlns != "http://autosar.org/schema/r4.0" || xsi != "http://www.w3.org/2001/XMLSchema-instance" {
                        return Err(self.error(ArxmlParserError::InvalidArxmlFileHeader));
                    }
                    let mut schema_parts = schema.split(' ');
                    let schema_base = schema_parts.next().unwrap_or("");
                    if schema_base != "http://autosar.org/schema/r4.0" {
                        return Err(self.error(ArxmlParserError::InvalidArxmlFileHeader));
                    }
                    let xsd_file = schema_parts.next().unwrap_or("");
                    if let Ok(autosar_version) = AutosarVersion::from_str(xsd_file) {
                        self.fileversion = autosar_version;
                    } else {
                        self.fileversion = AutosarVersion::Autosar_00050;
                        self.optional_error(ArxmlParserError::UnknownAutosarVersion {
                            input_verstring: xsd_file.to_string(),
                        })?;
                    }

                    let path = Cow::from("");
                    let autosar_root_element = self.parse_element(
                        ElementOrFile::None,
                        ElementName::Autosar,
                        attributes,
                        ElementType::ROOT,
                        path,
                        &mut lexer,
                    )?;
                    self.verify_end_of_input(&mut lexer)?;

                    return Ok(autosar_root_element);
                } else {
                    return Err(self.error(ArxmlParserError::InvalidArxmlFileHeader));
                }
            }
        }
        Err(self.error(ArxmlParserError::InvalidArxmlFileHeader))
    }

    fn parse_element(
        &mut self,
        parent: ElementOrFile,
        element_name: ElementName,
        attributes: SmallVec<[Attribute; 1]>,
        elemtype: ElementType,
        mut path: Cow<str>,
        lexer: &mut ArxmlLexer,
    ) -> Result<Element, AutosarDataError> {
        let wrapped_element = Element(Arc::new(Mutex::new(ElementRaw {
            parent,
            elemname: element_name,
            attributes,
            content: SmallVec::new(),
            elemtype,
        })));
        let mut element = wrapped_element.0.lock();

        let mut elem_idx: Vec<usize> = Vec::new();
        let mut short_name_found = false;
        let mut element_count = FxHashMap::<u16, usize>::default();

        loop {
            // track the current element name in the parser for error messages - set this in every loop iteration, since it gets overwritten during the recursive calls
            self.current_element = element_name;
            let arxmlevent = self.next(lexer)?;
            match arxmlevent {
                ArxmlEvent::BeginElement(elem_text, attr_text) => {
                    if let Ok(name) = ElementName::from_bytes(elem_text) {
                        let (sub_elemtype, idx) = self.find_element_in_spec_checked(name, elemtype)?;
                        self.check_element_conflict(name, elemtype, &elem_idx, &idx)?;
                        elem_idx = idx;

                        // make sure there aren't too many of this element
                        self.check_multiplicity(name, elemtype, &elem_idx, &mut element_count)?;

                        // atributes for the sub-element must be parsed before calling parse_element, or else the borrow checker hates us
                        let attributes = self.parse_attribute_text(sub_elemtype, attr_text)?;
                        // recursively parse the sub element and its sub sub elements
                        let sub_element = self.parse_element(
                            ElementOrFile::Element(wrapped_element.downgrade()),
                            name,
                            attributes,
                            sub_elemtype,
                            Cow::from(path.as_ref()),
                            lexer,
                        )?;
                        // if this sub element was a short name, then Autosar path handling is needed
                        if name == ElementName::ShortName {
                            short_name_found = true;
                            let sub_element_inner = sub_element.0.lock();
                            if let Some(ElementContent::CharacterData(CharacterData::String(name_string))) =
                                sub_element_inner.content.get(0)
                            {
                                let mut new_path = String::with_capacity(path.len() + name_string.len() + 1);
                                new_path.push_str(&path);
                                new_path.push('/');
                                new_path.push_str(name_string);
                                path = Cow::from(new_path.clone());
                                self.identifiables.push((new_path, wrapped_element.downgrade()));
                            }
                        }
                        element.content.push(ElementContent::Element(sub_element));
                    } else {
                        return Err(self.error(ArxmlParserError::InvalidBeginElement {
                            element: element.elemname,
                            invalid_element: String::from_utf8_lossy(elem_text).to_string(),
                        }));
                    }
                }
                ArxmlEvent::EndElement(elem_text) => {
                    if let Ok(name) = ElementName::from_bytes(elem_text) {
                        if name == element.elemname {
                            break;
                        } else {
                            return Err(self.error(ArxmlParserError::IncorrectEndElement {
                                element: element.elemname,
                                other_element: name,
                            }));
                        }
                    } else {
                        return Err(self.error(ArxmlParserError::InvalidEndElement {
                            parent_element: element.elemname,
                            invalid_element: String::from_utf8_lossy(elem_text).to_string(),
                        }));
                    }
                }
                ArxmlEvent::Characters(text_content) => {
                    if let Some(character_data_spec) = elemtype.chardata_spec() {
                        match elemtype.content_mode() {
                            ContentMode::Sequence | ContentMode::Choice | ContentMode::Bag => {
                                self.optional_error(ArxmlParserError::CharacterContentForbidden {
                                    element: element.elemname,
                                })?
                            }
                            ContentMode::Characters | ContentMode::Mixed => {
                                let value = self.parse_character_data(text_content, character_data_spec)?;
                                if elemtype.is_ref() {
                                    if let CharacterData::String(refpath) = &value {
                                        self.references.push((refpath.to_owned(), wrapped_element.downgrade()));
                                    }
                                }
                                element.content.push(ElementContent::CharacterData(value));
                            }
                        }
                    } else {
                        self.optional_error(ArxmlParserError::CharacterContentForbidden {
                            element: element.elemname,
                        })?;
                    }
                }
                ArxmlEvent::ArxmlHeader => todo!(),
                ArxmlEvent::EndOfFile => {
                    return Err(self.error(ArxmlParserError::UnexpectedEndOfFile {
                        element: element.elemname,
                    }));
                }
            }
        }

        if short_name_found {
        } else if elemtype.is_named_in_version(self.fileversion) {
            self.optional_error(ArxmlParserError::RequiredSubelementMissing {
                element: element_name,
                sub_element: ElementName::ShortName,
            })?;
        }

        Ok(wrapped_element.clone())
    }

    fn find_element_in_spec_checked(
        &mut self,
        name: ElementName,
        elemtype: ElementType,
    ) -> Result<(ElementType, Vec<usize>), AutosarDataError> {
        // Some elements have multiple entries, and the correct one must be chosen based on the autosar version
        // First try to find the sub element using the current file version. If that fails then search again
        // allowing elements from all autosar versions. This is useful in order to give better diagnostics.
        let (sub_elem_type, new_elem_indices) = match elemtype.find_sub_element(name, self.fileversion as u32) {
            Some(result) => {
                // normal case: the element was found in the spec, while restricted to only the current version
                result
            }
            None => {
                // fallback: the search is retried, while allowing matching sub-elements from any AutosarVersion
                let (sub_elemtype, elem_idx) = elemtype.find_sub_element(name, u32::MAX).ok_or_else(|| {
                    self.error(ArxmlParserError::IncorrectBeginElement {
                        element: self.current_element,
                        sub_element: name,
                    })
                })?;
                // now we need to get the version mask that tells us in what versions this element was actually allowed in
                // unwrap() is ok here since this can't fail: elem_idx just came from find_sub_element
                let version_mask = elemtype.get_sub_element_version_mask(&elem_idx).unwrap();
                // check_version will return an ElementVersionError is strict parsing is on, otherwise it's a warning
                self.check_version(
                    version_mask,
                    ArxmlParserError::ElementVersionError {
                        element: self.current_element,
                        sub_element: name,
                        version: self.fileversion,
                    },
                )?;
                (sub_elemtype, elem_idx)
            }
        };

        Ok((sub_elem_type, new_elem_indices))
    }

    fn check_element_conflict(
        &mut self,
        name: ElementName,
        elemtype: ElementType,
        elem_indices: &[usize],
        new_elem_indices: &Vec<usize>,
    ) -> Result<(), AutosarDataError> {
        if elem_indices.is_empty() || (elem_indices == new_elem_indices) {
            // when elem_indices is empty, that means that this is the first sub-element or found the exact same element as last time
            // no ordering checks are possible
        } else {
            let group_type = elemtype.find_common_group(elem_indices, new_elem_indices);
            let mode = group_type.content_mode();

            match mode {
                ContentMode::Sequence => {
                    // We could check if the elements are in the specified order.
                    // Unfortunaltely the tool used by the Autosar organisation to derive the xsd files from the meta model seems to be buggy.
                    // For example, VARIATION-POINT should always be last according to the meta model, but some of the xsd files do not place it there.
                    // Since other tools seem to skip this check, lets also ignore ordering.
                }
                ContentMode::Choice => {
                    self.optional_error(ArxmlParserError::ElementChoiceConflict {
                        element: self.current_element,
                        sub_element: name,
                    })?;
                }
                ContentMode::Characters => {
                    // an element with ContentMode::Characters has no sub elements, so the outer "if let Some(new_elem_indices)" is never true
                    panic!("accepted a sub-element inside a character-only element");
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn check_multiplicity(
        &mut self,
        name: ElementName,
        elemtype: ElementType,
        elem_idx: &[usize],
        element_count: &mut FxHashMap<u16, usize>,
    ) -> Result<(), AutosarDataError> {
        // check the multiplicity - in practice the only restriction that matters here is having too many elements where the multiplicity is not Any
        // element_count will contain the current cout for this element if it has been seen before - if not, there cannot be a multiplicity problem
        if let Some(count_value) = element_count.get_mut(&(name as u16)) {
            // get the parent type id, i.e. the type of the containing element or group
            let datatype_mode = elemtype.get_sub_element_container_mode(elem_idx);
            // multiplicity only matters if the mode is Choice or Sequence - modes Mixed and Bag allow arbitrary amounts of all elements
            if datatype_mode == ContentMode::Sequence || datatype_mode == ContentMode::Choice {
                if let Some(multiplicity) = elemtype.get_sub_element_multiplicity(elem_idx) {
                    *count_value += 1;
                    if multiplicity != ElementMultiplicity::Any {
                        self.optional_error(ArxmlParserError::TooManySubElements {
                            element: self.current_element,
                            sub_element: name,
                        })?;
                    }
                }
            }
        } else {
            element_count.insert(name as u16, 1);
        }
        Ok(())
    }

    fn parse_attribute_text(
        &mut self,
        elemtype: ElementType,
        attributes_text: &[u8],
    ) -> Result<SmallVec<[Attribute; 1]>, AutosarDataError> {
        let mut attributes = SmallVec::new();
        // attributes_text is a byte string containig all the attributes of an element
        // for example: xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_4-2-2.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        // when this string is split on ", there will be an odd number of parts, with the last part being empty
        let mut attr_part_iter = attributes_text.split(|c| *c == b'"');
        while let (Some(attr_name_part), Some(attr_value_part)) = (attr_part_iter.next(), attr_part_iter.next()) {
            // attr_name_part may have leading whitespace and will always have a trailing '='
            // these need to be stripped
            let name_len = attr_name_part.len() - 1; // exclude the trailing =
            let (name_start, _) = attr_name_part
                .iter()
                .enumerate()
                .find(|(_, c)| !c.is_ascii_whitespace())
                .unwrap_or((name_len, &0u8));
            if let Ok(attr_name) = AttributeName::from_bytes(&attr_name_part[name_start..name_len]) {
                if let Some(AttributeSpec{spec: ctype, version: version_mask, ..}) = elemtype.find_attribute_spec(attr_name) {
                    self.check_version(
                        version_mask,
                        ArxmlParserError::AttributeVersionError {
                            element: self.current_element,
                            attribute: attr_name,
                            version: self.fileversion,
                        },
                    )?;
                    let attr_value = self.parse_character_data(attr_value_part, ctype)?;
                    attributes.push(Attribute {
                        attrname: attr_name,
                        content: attr_value,
                    });
                } else {
                    self.optional_error(ArxmlParserError::UnknownAttributeError {
                        element: self.current_element,
                        attribute: attr_name.to_string(),
                    })?;
                }
            } else {
                self.optional_error(ArxmlParserError::UnknownAttributeError {
                    element: self.current_element,
                    attribute: String::from_utf8_lossy(&attr_name_part[name_start..name_len]).to_string(),
                })?;
            }
        }

        for (name, _ctype, required) in elemtype.attribute_spec_iter() {
            if required && !attributes.iter().any(|attr: &Attribute| attr.attrname == name) {
                self.optional_error(ArxmlParserError::RequiredAttributeMissing {
                    element: self.current_element,
                    attribute: name,
                })?;
            }
        }

        Ok(attributes)
    }

    fn parse_character_data(
        &mut self,
        input: &[u8],
        character_data_spec: &CharacterDataSpec,
    ) -> Result<CharacterData, AutosarDataError> {
        let trimmed_input = trim_byte_string(input);
        match character_data_spec {
            CharacterDataSpec::Enum { items } => {
                let value = EnumItem::from_bytes(trimmed_input).map_err(|_| {
                    self.error(ArxmlParserError::UnknownEnumItem {
                        value: String::from_utf8_lossy(trimmed_input).to_string(),
                    })
                })?;
                let (_, version) = items.iter().find(|(item, _)| *item == value).ok_or_else(|| {
                    self.error(ArxmlParserError::InvalidEnumItem {
                        value: String::from_utf8_lossy(trimmed_input).to_string(),
                    })
                })?;
                self.check_version(
                    *version,
                    ArxmlParserError::EnumItemVersionError {
                        element: self.current_element,
                        enum_item: value,
                        version: self.fileversion,
                    },
                )?;
                Ok(CharacterData::Enum(value))
            }
            CharacterDataSpec::Pattern {
                check_fn,
                regex,
                max_length,
            } => {
                if max_length.is_some() && trimmed_input.len() > max_length.unwrap() {
                    self.optional_error(ArxmlParserError::StringValueTooLong {
                        value: String::from_utf8_lossy(trimmed_input).to_string(),
                        length: max_length.unwrap(),
                    })?;
                }
                if !check_fn(trimmed_input) {
                    self.optional_error(ArxmlParserError::RegexMatchError {
                        value: String::from_utf8_lossy(trimmed_input).to_string(),
                        regex: regex.to_string(),
                    })?;
                }
                // text with regex pattern validation doesn't need unescaping - none of the regexes will allow any of the the escaped chars
                match std::str::from_utf8(trimmed_input) {
                    Ok(utf8string) => Ok(CharacterData::String(utf8string.to_owned())),
                    Err(err) => {
                        self.optional_error(ArxmlParserError::Utf8Error { source: err })?;
                        Ok(CharacterData::String(
                            String::from_utf8_lossy(trimmed_input).into_owned(),
                        ))
                    }
                }
            }
            CharacterDataSpec::String {
                preserve_whitespace,
                max_length,
            } => {
                let raw_text = if *preserve_whitespace { input } else { trimmed_input };
                if max_length.is_some() && raw_text.len() > max_length.unwrap() {
                    self.optional_error(ArxmlParserError::StringValueTooLong {
                        value: String::from_utf8_lossy(trimmed_input).to_string(),
                        length: max_length.unwrap(),
                    })?;
                }
                let text = match std::str::from_utf8(raw_text) {
                    Ok(utf8string) => Cow::from(utf8string),
                    Err(err) => {
                        self.optional_error(ArxmlParserError::Utf8Error { source: err })?;
                        String::from_utf8_lossy(raw_text)
                    }
                };
                let unescaped_text = self.unescape_string(&text)?.into_owned();
                Ok(CharacterData::String(unescaped_text))
            }
            CharacterDataSpec::UnsignedInteger => {
                let strval = std::str::from_utf8(trimmed_input)
                    .map_err(|err| self.error(ArxmlParserError::Utf8Error { source: err }))?;
                let value = match strval.parse::<u64>() {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        self.optional_error(ArxmlParserError::InvalidNumber {
                            input: strval.to_owned(),
                        })?;
                        0
                    }
                };
                Ok(CharacterData::UnsignedInteger(value))
            }
            CharacterDataSpec::Double => {
                let strval = std::str::from_utf8(trimmed_input)
                    .map_err(|err| self.error(ArxmlParserError::Utf8Error { source: err }))?;
                let value = match strval.parse::<f64>() {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        self.optional_error(ArxmlParserError::InvalidNumber {
                            input: strval.to_owned(),
                        })?;
                        0.0
                    }
                };
                Ok(CharacterData::Double(value))
            }
        }
    }

    fn unescape_string<'b>(&mut self, input: &'b str) -> Result<Cow<'b, str>, AutosarDataError> {
        if input.contains('&') {
            let mut unescaped = String::with_capacity(input.len());
            let mut rem = input;
            while let Some(pos) = rem.find('&') {
                unescaped.push_str(&rem[..pos]);
                rem = &rem[pos..];
                if rem.starts_with("&lt;") {
                    unescaped.push('<');
                    rem = &rem[4..];
                } else if rem.starts_with("&gt;") {
                    unescaped.push('>');
                    rem = &rem[4..];
                } else if rem.starts_with("&amp;") {
                    unescaped.push('&');
                    rem = &rem[5..];
                } else if rem.starts_with("&apos;") {
                    unescaped.push('\'');
                    rem = &rem[6..];
                } else if rem.starts_with("&quot;") {
                    unescaped.push('"');
                    rem = &rem[6..];
                } else if rem.starts_with("&#x") {
                    // hexadecimal character reference
                    let mut valid = false;
                    if let Some(endpos) = rem.find(';') {
                        let hextxt = &rem[3..endpos];
                        if let Ok(hexval) = u32::from_str_radix(hextxt, 16) {
                            if let Some(ch) = char::from_u32(hexval) {
                                unescaped.push(ch);
                                rem = &rem[endpos+1..];
                                valid = true;
                            }
                        }
                    }
                    if !valid {
                        self.optional_error(ArxmlParserError::InvalidXmlEntity {
                            input: input.to_owned(),
                        })?;
                        unescaped.push('&');
                        rem = &rem[1..];
                    }
                } else if rem.starts_with("&#") {
                    // decimal character reference
                    let mut valid = false;
                    if let Some(endpos) = rem.find(';') {
                        let numtxt = &rem[2..endpos];
                        if let Ok(val) = u32::from_str(numtxt) {
                            if let Some(ch) = char::from_u32(val) {
                                unescaped.push(ch);
                                rem = &rem[endpos+1..];
                                valid = true;
                            }
                        }
                    }
                    if !valid {
                        self.optional_error(ArxmlParserError::InvalidXmlEntity {
                            input: input.to_owned(),
                        })?;
                        unescaped.push('&');
                        rem = &rem[1..];
                    }
                } else {
                    self.optional_error(ArxmlParserError::InvalidXmlEntity {
                        input: input.to_owned(),
                    })?;
                    unescaped.push('&');
                    rem = &rem[1..];
                }
            }
            unescaped.push_str(rem);

            Ok(Cow::Owned(unescaped))
        } else {
            Ok(Cow::Borrowed(input))
        }
    }

    pub(crate) fn get_fileversion(&self) -> AutosarVersion {
        self.fileversion
    }

    fn verify_end_of_input(&mut self, lexer: &mut ArxmlLexer) -> Result<(), AutosarDataError> {
        let (_, next_event) = lexer.next()?;
        if let ArxmlEvent::EndOfFile = next_event {
            Ok(())
        } else {
            self.optional_error(ArxmlParserError::AdditionalDataError)?;
            Ok(())
        }
    }
}

fn trim_byte_string(input: &[u8]) -> &[u8] {
    let mut len = input.len();
    if len > 0 {
        while input[len - 1].is_ascii_whitespace() {
            len -= 1;
        }
        let (start, _) = input
            .iter()
            .enumerate()
            .find(|(_, c)| !c.is_ascii_whitespace())
            .unwrap_or((len, &0u8));
        &input[start..len]
    } else {
        input
    }
}

#[cfg(test)]
mod test {
    use crate::parser::*;
    use crate::*;

    fn test_helper(buffer: &[u8], target_error: std::mem::Discriminant<ArxmlParserError>, optional: bool) {
        let mut parser = ArxmlParser::new(PathBuf::from("test_buffer.arxml"), buffer, true);
        let result = parser.parse_arxml();
        println!("Result: {result:?}");
        if let Err(AutosarDataError::ParserError { source, .. }) = result {
            assert_eq!(
                std::mem::discriminant(&source),
                target_error,
                "Did not get the expected parser error"
            );
        } else {
            assert!(false, "Did not get any parser error when one was expected");
        }

        if optional {
            let mut parser = ArxmlParser::new(PathBuf::from("test_buffer.arxml"), buffer, false);
            let result = parser.parse_arxml();
            println!("Result: {result:?}");
            if let Some(AutosarDataError::ParserError { source, .. }) = parser.warnings.get(0) {
                assert_eq!(
                    std::mem::discriminant(source),
                    target_error,
                    "Did not get the expected parser error"
                );
            } else {
                assert!(false, "Did not get a parser warning");
            }
        }
    }

    const INVALID_HEADER_1: &str = "BLA BLA bla";
    const INVALID_HEADER_2: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <something>"#;
    const INVALID_HEADER_3: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="nonsense" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">"#;

    #[test]
    fn test_invalid_header() {
        test_helper(
            INVALID_HEADER_1.as_bytes(),
            std::mem::discriminant(&ArxmlParserError::InvalidArxmlFileHeader),
            false,
        );
        test_helper(
            INVALID_HEADER_2.as_bytes(),
            std::mem::discriminant(&ArxmlParserError::InvalidArxmlFileHeader),
            false,
        );
        test_helper(
            INVALID_HEADER_3.as_bytes(),
            std::mem::discriminant(&ArxmlParserError::InvalidArxmlFileHeader),
            false,
        );
    }

    const UNKNOWN_VERSION: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_something_else.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    </AUTOSAR>"#;

    #[test]
    fn test_unknown_version() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::UnknownAutosarVersion {
            input_verstring: "".to_string(),
        });
        test_helper(UNKNOWN_VERSION.as_bytes(), discriminant, true);
    }

    const INCORRECT_BEGIN_ELEMENT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <ELEMENT>"#;

    #[test]
    fn test_incorrect_begin_element() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::IncorrectBeginElement {
            element: ElementName::Autosar,
            sub_element: ElementName::Autosar,
        });
        test_helper(INCORRECT_BEGIN_ELEMENT.as_bytes(), discriminant, false);
    }

    const INVALID_BEGIN_ELEMENT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <NOT_AN_AUTOSAR_ELEMENT>"#;

    #[test]
    fn test_invalid_begin_element() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::InvalidBeginElement {
            element: ElementName::Autosar,
            invalid_element: "".to_string(),
        });
        test_helper(INVALID_BEGIN_ELEMENT.as_bytes(), discriminant, false);
    }

    const INCORRECT_END_ELEMENT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <AR-PACKAGES></AUTOSAR>"#;

    #[test]
    fn test_incorrect_end_element() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::IncorrectEndElement {
            element: ElementName::Autosar,
            other_element: ElementName::Autosar,
        });
        test_helper(INCORRECT_END_ELEMENT.as_bytes(), discriminant, false);
    }

    const INVALID_END_ELEMENT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <AR-PACKAGES></NOT_AN_AUTOSAR_ELEMENT>"#;

    #[test]
    fn test_invalid_end_element() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::InvalidEndElement {
            parent_element: ElementName::Autosar,
            invalid_element: "".to_string(),
        });
        test_helper(INVALID_END_ELEMENT.as_bytes(), discriminant, false);
    }

    const ELEMENT_VERSION_ERROR: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_4-0-1.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>TestPackage</SHORT-NAME><ELEMENTS><DIAGNOSTIC-ACCESS-PERMISSION>"#;

    #[test]
    fn test_element_version_error() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::ElementVersionError {
            element: ElementName::Autosar,
            sub_element: ElementName::Autosar,
            version: AutosarVersion::Autosar_00050,
        });
        test_helper(ELEMENT_VERSION_ERROR.as_bytes(), discriminant, false);
    }

    const CHOICE_CONFLICT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
            <AR-PACKAGE>
                <SHORT-NAME>base</SHORT-NAME>
                <ELEMENTS>
                    <DIAGNOSTIC-CONTRIBUTION-SET>
                        <SHORT-NAME>dcs</SHORT-NAME>
                        <COMMON-PROPERTIES>
                            <DIAGNOSTIC-COMMON-PROPS-VARIANTS>
                                <DIAGNOSTIC-COMMON-PROPS-CONDITIONAL>
                                    <DEBOUNCE-ALGORITHM-PROPSS>
                                        <DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS>
                                            <SHORT-NAME>props</SHORT-NAME>
                                            <DEBOUNCE-ALGORITHM>
                                                <DIAG-EVENT-DEBOUNCE-COUNTER-BASED>
                                                    <SHORT-NAME>abc</SHORT-NAME>
                                                </DIAG-EVENT-DEBOUNCE-COUNTER-BASED>
                                                <DIAG-EVENT-DEBOUNCE-TIME-BASED>
                                                    <SHORT-NAME>def</SHORT-NAME>
                                                </DIAG-EVENT-DEBOUNCE-TIME-BASED>"#;

    #[test]
    fn test_choice_conflict() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::ElementChoiceConflict {
            element: ElementName::Autosar,
            sub_element: ElementName::Autosar,
        });
        test_helper(CHOICE_CONFLICT.as_bytes(), discriminant, true);
    }

    const TOO_MANY_SUBELEMENTS: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
            <AR-PACKAGE>
                <SHORT-NAME>base</SHORT-NAME>
                <SHORT-NAME>base</SHORT-NAME>"#;

    #[test]
    fn test_too_many_sub_elements() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::TooManySubElements {
            element: ElementName::Autosar,
            sub_element: ElementName::Autosar,
        });
        test_helper(TOO_MANY_SUBELEMENTS.as_bytes(), discriminant, true);
    }

    const REQUIRED_SUB_ELEMENT_MISSING: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
            <AR-PACKAGE></AR-PACKAGE>"#;

    #[test]
    fn test_required_sub_element_missing() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::RequiredSubelementMissing {
            element: ElementName::Autosar,
            sub_element: ElementName::Autosar,
        });
        test_helper(REQUIRED_SUB_ELEMENT_MISSING.as_bytes(), discriminant, false);
    }

    const UNKNOWN_ATTRIBUTE: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <AR-PACKAGES UnknownAttribute="value">
    </AR-PACKAGES></AUTOSAR>"#;

    #[test]
    fn test_unknown_attribute() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::UnknownAttributeError {
            element: ElementName::Autosar,
            attribute: "".to_string(),
        });
        test_helper(UNKNOWN_ATTRIBUTE.as_bytes(), discriminant, true);
    }

    const REQUIRED_ATTRIBUTE_MISSING: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0">
    </AUTOSAR>"#;

    #[test]
    fn test_required_attribute_missing() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::RequiredAttributeMissing {
            element: ElementName::Autosar,
            attribute: AttributeName::Accesskey,
        });
        test_helper(REQUIRED_ATTRIBUTE_MISSING.as_bytes(), discriminant, true);
    }

    const CHARACTER_CONTENT_FORBIDDEN: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    abcdef"#;

    #[test]
    fn test_character_content_forbidden() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::CharacterContentForbidden {
            element: ElementName::Autosar,
        });
        test_helper(CHARACTER_CONTENT_FORBIDDEN.as_bytes(), discriminant, false);
    }

    const WRONG_ENUM_ITEM_VERSION: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00044.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
            <AR-PACKAGE>
                <SHORT-NAME>base</SHORT-NAME>
                <ELEMENTS>
                    <SYSTEM>
                        <SHORT-NAME>System</SHORT-NAME>
                        <FIBEX-ELEMENTS>
                            <FIBEX-ELEMENT-REF-CONDITIONAL>
                                <FIBEX-ELEMENT-REF DEST="SERVICE-INSTANCE-COLLECTION-SET">"#;

    #[test]
    fn test_enum_item_version() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::EnumItemVersionError {
            element: ElementName::Autosar,
            enum_item: EnumItem::Aa,
            version: AutosarVersion::Autosar_00050,
        });
        test_helper(WRONG_ENUM_ITEM_VERSION.as_bytes(), discriminant, false);
    }

    const UNKNOWN_ENUM_ITEM: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
            <AR-PACKAGE>
                <SHORT-NAME>base</SHORT-NAME>
                <ELEMENTS>
                    <SYSTEM>
                        <SHORT-NAME>System</SHORT-NAME>
                        <FIBEX-ELEMENTS>
                            <FIBEX-ELEMENT-REF-CONDITIONAL>
                                <FIBEX-ELEMENT-REF DEST="invalid_value_for_the_test">"#;

    #[test]
    fn test_unknown_enum_item() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::UnknownEnumItem { value: "".to_string() });
        test_helper(UNKNOWN_ENUM_ITEM.as_bytes(), discriminant, false);
    }

    const INVALID_ENUM_ITEM: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
            <AR-PACKAGE>
                <SHORT-NAME>base</SHORT-NAME>
                <ELEMENTS>
                    <SYSTEM>
                        <SHORT-NAME>System</SHORT-NAME>
                        <FIBEX-ELEMENTS>
                            <FIBEX-ELEMENT-REF-CONDITIONAL>
                                <FIBEX-ELEMENT-REF DEST="default">"#;

    #[test]
    fn test_invalid_enum_item() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::InvalidEnumItem { value: "".to_string() });
        test_helper(INVALID_ENUM_ITEM.as_bytes(), discriminant, false);
    }

    const STRING_VALUE_TOO_LONG: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE>
            <SHORT-NAME>xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx</SHORT-NAME>
        </AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#;

    #[test]
    fn test_string_value_too_long() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::StringValueTooLong {
            value: "".to_string(),
            length: 1,
        });
        test_helper(STRING_VALUE_TOO_LONG.as_bytes(), discriminant, true);
    }

    const REGEX_MATCH_ERROR: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE>
            <SHORT-NAME>0a</SHORT-NAME>
        </AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#;

    #[test]
    fn test_regex_match_error() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::RegexMatchError {
            value: "".to_string(),
            regex: "".to_string(),
        });
        test_helper(REGEX_MATCH_ERROR.as_bytes(), discriminant, true);
    }

    const UTF8_ERROR: &[u8] = b"<?xml version=\"1.0\" encoding=\"utf-8\"?>
    <AUTOSAR xsi:schemaLocation=\"http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd\" xmlns=\"http://autosar.org/schema/r4.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\">
        <AR-PACKAGES><AR-PACKAGE S=\"\xff\xff\">";

    #[test]
    fn test_utf8_error() {
        let mut parser = ArxmlParser::new(PathBuf::from("test_buffer.arxml"), UTF8_ERROR, true);
        let result = parser.parse_arxml();
        assert!(
            matches!(
                result,
                Err(AutosarDataError::ParserError {
                    source: ArxmlParserError::Utf8Error { .. },
                    ..
                })
            ),
            "Did not get the expected parser error"
        );

        let mut parser = ArxmlParser::new(PathBuf::from("test_buffer.arxml"), UTF8_ERROR, false);
        let _ = parser.parse_arxml();
        let warning = parser.warnings.get(0);
        assert!(
            matches!(
                warning,
                Some(AutosarDataError::ParserError {
                    source: ArxmlParserError::Utf8Error { .. },
                    ..
                })
            ),
            "Did not get the expected parser warning"
        );
    }

    const UNEXPECTED_END_OF_FILE: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">"#;

    #[test]
    fn test_unexpected_end_of_file() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::UnexpectedEndOfFile {
            element: ElementName::Autosar,
        });
        test_helper(UNEXPECTED_END_OF_FILE.as_bytes(), discriminant, false);
    }

    const INVALID_NUMBER: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <AR-PACKAGES><AR-PACKAGE>
        <SHORT-NAME>base</SHORT-NAME>
        <ELEMENTS><I-SIGNAL-I-PDU>
            <SHORT-NAME>Pdu</SHORT-NAME>
            <I-PDU-TIMING-SPECIFICATIONS><I-PDU-TIMING><TRANSMISSION-MODE-DECLARATION><TRANSMISSION-MODE-TRUE-TIMING><CYCLIC-TIMING>
            <TIME-PERIOD><TOLERANCE><ABSOLUTE-TOLERANCE><ABSOLUTE>not a number</ABSOLUTE>"#;

    #[test]
    fn test_invalid_number() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::InvalidNumber { input: "".to_string() });
        test_helper(INVALID_NUMBER.as_bytes(), discriminant, true);
    }

    const ADDITIONAL_DATA: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    </AUTOSAR>
    <extra>"#;

    #[test]
    fn test_additional_data_error() {
        let discriminant = std::mem::discriminant(&ArxmlParserError::AdditionalDataError);
        test_helper(ADDITIONAL_DATA.as_bytes(), discriminant, true);
    }

    #[test]
    fn unescape_entities() {
        let mut parser = ArxmlParser::new(PathBuf::from("test_buffer.arxml"), &[], true);
        let result = parser
            .unescape_string("&amp;&amp;&lt;FOO&gt;&quot;&quot;&apos;&#32;&#x20;end")
            .unwrap();
        assert_eq!(&result, r#"&&<FOO>""'  end"#);
        let result = parser.unescape_string("&amp;&amp;&gt;FOO&lt;&quot&quot;&apos;end");
        assert!(result.is_err());
        // numeric character entity does not accept hex values
        let result = parser.unescape_string("&#abcde;");
        assert!(result.is_err());
        // values from 0x110000 to 0x1FFFFF are not valid unicode code points -> 0x110000 = 1114112
        let result = parser.unescape_string("&#1114112;");
        assert!(result.is_err());
    }

    const PARSER_TEST_DATA: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
            <AR-PACKAGE>
                <SHORT-NAME>base</SHORT-NAME>
                <ELEMENTS>
                    <SYSTEM UUID="12345678" S="some string" T="2022-01-31T13:00:59Z">
                        <SHORT-NAME>System</SHORT-NAME>
                        <FIBEX-ELEMENTS>
                            <FIBEX-ELEMENT-REF-CONDITIONAL>
                                <FIBEX-ELEMENT-REF DEST="I-SIGNAL-I-PDU">/base/Pdu</FIBEX-ELEMENT-REF>
                            </FIBEX-ELEMENT-REF-CONDITIONAL>
                        </FIBEX-ELEMENTS>
                    </SYSTEM>
                    <I-SIGNAL-I-PDU>
                        <SHORT-NAME>Pdu</SHORT-NAME>
                        <I-PDU-TIMING-SPECIFICATIONS>
                            <I-PDU-TIMING>
                                <TRANSMISSION-MODE-DECLARATION>
                                    <TRANSMISSION-MODE-TRUE-TIMING>
                                        <CYCLIC-TIMING>
                                            <TIME-PERIOD>
                                                <TOLERANCE>
                                                    <ABSOLUTE-TOLERANCE>
                                                        <ABSOLUTE>1.0</ABSOLUTE>
                                                    </ABSOLUTE-TOLERANCE>
                                                </TOLERANCE>
                                            </TIME-PERIOD>
                                        </CYCLIC-TIMING>
                                    </TRANSMISSION-MODE-TRUE-TIMING>
                                </TRANSMISSION-MODE-DECLARATION>
                            </I-PDU-TIMING>
                        </I-PDU-TIMING-SPECIFICATIONS>
                    </I-SIGNAL-I-PDU>
                </ELEMENTS>
            </AR-PACKAGE>
        </AR-PACKAGES>
    </AUTOSAR>
    "#;

    #[test]
    fn test_basic_functionality() {
        let mut parser = ArxmlParser::new(PathBuf::from("test_buffer.arxml"), PARSER_TEST_DATA.as_bytes(), true);
        let result = parser.parse_arxml();
        assert!(result.is_ok());
    }

    const EMPTY_CHARACTER_DATA: &str = r#"<?xml version="1.0" encoding="utf-8"?>
    <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
            <AR-PACKAGE UUID="">
                <SHORT-NAME>x</SHORT-NAME>
            </AR-PACKAGE>
        </AR-PACKAGES>
    </AUTOSAR>
    "#;

    #[test]
    fn test_empty_character_data() {
        let mut parser = ArxmlParser::new(
            PathBuf::from("test_buffer.arxml"),
            EMPTY_CHARACTER_DATA.as_bytes(),
            true,
        );
        let result = parser.parse_arxml();
        assert!(result.is_ok());
    }
}
