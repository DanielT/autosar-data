use super::AutosarDataError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq, Clone, Copy)]
#[non_exhaustive]
/// `ArxmlLexerError` contains all errors that can occur while reading data
pub enum ArxmlLexerError {
    /// Incomplete data, closing '>' was not found
    #[error("Incomplete data, closing '>' was not found")]
    IncompleteData,

    /// Invalid element: '<>'
    #[error("Invalid element: '<>'")]
    InvalidElement,

    /// A processing instruction was started with '<?', but it did not end with '?>'
    #[error("A processing instruction was started with '<?', but it did not end with '?>'")]
    InvalidProcessingInstruction,

    /// Invalid arxml header: The xml header of an arxml file must specify version="1.0" encoding="utf-8"
    #[error("Invalid arxml header: The xml header of an arxml file must specify version=\"1.0\" encoding=\"utf-8\"")]
    InvalidXmlHeader,

    /// Invalid comment: Comments must start with '<!--' and end with '-->'
    #[error("Invalid comment")]
    InvalidComment,
}

#[derive(Debug)]
pub(crate) enum ArxmlEvent<'a> {
    ArxmlHeader(Option<bool>),
    BeginElement(&'a [u8], &'a [u8]),
    EndElement(&'a [u8]),
    Characters(&'a [u8]),
    Comment(&'a [u8]),
    EndOfFile,
}

pub(crate) struct ArxmlLexer<'a> {
    buffer: &'a [u8],
    bufpos: usize,
    line: usize,
    deferred_end: Option<(usize, usize)>,
    sourcefile: PathBuf,
}

impl<'a> ArxmlLexer<'a> {
    pub(crate) fn new(buffer: &'a [u8], name: PathBuf) -> Self {
        // skip the byte-order mark, if it is present
        let bufpos = if buffer.len() > 3 && buffer[0] == 239 && buffer[1] == 187 && buffer[2] == 191 {
            3
        } else {
            0
        };
        Self {
            buffer,
            bufpos,
            line: 1,
            deferred_end: None,
            sourcefile: name,
        }
    }

    fn read_characters(&mut self) -> (ArxmlEvent<'a>, bool) {
        debug_assert!(self.bufpos < self.buffer.len());

        // the start of the next element '<' is the end of this block of characters
        let mut endpos = self.bufpos;
        let mut all_whitespace = true;
        while endpos < self.buffer.len() && self.buffer[endpos] != b'<' {
            // count the lines directly in this loop; it's faster than calling count_lines and this loop is quite hot in the profile...
            if !self.buffer[endpos].is_ascii_whitespace() {
                all_whitespace = false;
            } else if self.buffer[endpos] == b'\n' {
                self.line += 1;
            }
            endpos += 1;
        }
        debug_assert!(endpos > self.bufpos);

        let text = &self.buffer[self.bufpos..endpos];
        self.bufpos = endpos;
        (ArxmlEvent::Characters(text), all_whitespace)
    }

    fn read_element_start(&mut self, endpos: usize) -> ArxmlEvent<'a> {
        debug_assert!(self.bufpos < self.buffer.len());
        debug_assert!(endpos > self.bufpos + 1);
        debug_assert!(self.buffer[self.bufpos] == b'<');

        let (text, is_end) = if self.buffer[endpos - 1] == b'/' {
            (&self.buffer[self.bufpos + 1..endpos - 1], true)
        } else {
            (&self.buffer[self.bufpos + 1..endpos], false)
        };

        let (elemname, attributes) = if let Some(splitpos) = text.iter().position(u8::is_ascii_whitespace) {
            (&text[..splitpos], &text[splitpos + 1..])
        } else {
            (text, &text[0..0])
        };

        // this is a <element/>, so a EndElement event needs to be generated next
        if is_end {
            // calculate the position of the element name inside the text
            self.deferred_end = Some((self.bufpos + 1, self.bufpos + 1 + elemname.len()));
        }

        self.line += count_lines(text);
        self.bufpos = endpos + 1;
        ArxmlEvent::BeginElement(elemname, attributes)
    }

    fn read_element_end(&mut self, endpos: usize) -> ArxmlEvent<'a> {
        debug_assert!(self.bufpos < self.buffer.len());
        debug_assert!(endpos > self.bufpos + 1);
        debug_assert!(self.buffer[self.bufpos] == b'<');

        let text = &self.buffer[self.bufpos + 2..endpos];
        self.bufpos = endpos + 1;

        ArxmlEvent::EndElement(text)
    }

    fn read_xml_header(&mut self, endpos: usize) -> Option<Result<ArxmlEvent<'a>, AutosarDataError>> {
        debug_assert!(self.bufpos < self.buffer.len());
        debug_assert!(endpos > self.bufpos + 1);
        debug_assert!(self.buffer[self.bufpos] == b'<');

        if self.buffer[endpos - 1] != b'?' {
            return Some(Err(self.error(ArxmlLexerError::InvalidProcessingInstruction)));
        }

        let text = &self.buffer[self.bufpos + 2..endpos - 1];
        self.bufpos = endpos + 1;

        let mut splitter = text.split(u8::is_ascii_whitespace);
        let elemname = splitter.next().unwrap();

        let result = if elemname == b"xml" {
            let mut ver = &text[0..0];
            let mut encoding = &text[0..0];
            let mut standalone: Option<bool> = None;
            for attr_text in splitter {
                let (attr_name, attr_val) = if let Some(pos) = attr_text.iter().position(|c| *c == b'=') {
                    (&attr_text[0..pos], &attr_text[pos + 2..attr_text.len() - 1])
                } else {
                    (attr_text, &attr_text[0..0])
                };
                if attr_name == b"version" {
                    ver = attr_val;
                } else if attr_name == b"encoding" {
                    encoding = attr_val;
                } else if attr_name == b"standalone" {
                    standalone = Some(attr_val == b"yes");
                }
            }

            if ver != b"1.0"
                || (encoding != b"utf-8" && encoding != b"UTF-8" && encoding != b"utf8" && encoding != b"UTF8")
            {
                Some(Err(self.error(ArxmlLexerError::InvalidXmlHeader)))
            } else {
                Some(Ok(ArxmlEvent::ArxmlHeader(standalone)))
            }
        } else {
            None
        };

        self.line += count_lines(text);
        result
    }

    fn read_comment(&mut self, endpos: usize) -> Result<ArxmlEvent<'a>, AutosarDataError> {
        debug_assert!(self.bufpos < self.buffer.len());
        debug_assert!(endpos > self.bufpos + 1);

        let startpos = self.bufpos;
        let text = &self.buffer[startpos..endpos];
        self.bufpos = endpos + 1;

        if text.len() < 6 || !text.starts_with(b"<!--") || !text.ends_with(b"--") {
            return Err(AutosarDataError::LexerError {
                filename: self.sourcefile.clone(),
                line: self.line,
                source: ArxmlLexerError::InvalidComment,
            });
        }
        self.line += count_lines(text);
        let comment = &self.buffer[startpos + 4..endpos - 2];
        Ok(ArxmlEvent::Comment(comment))
    }
}

impl ArxmlLexer<'_> {
    pub(crate) fn next(&mut self) -> Result<(usize, ArxmlEvent), AutosarDataError> {
        // if an <element/> was found, then a BeginElement event is returned first, and the EndElement is deferred and must be returned next
        if let Some((startpos, endpos)) = self.deferred_end {
            self.deferred_end = None;
            Ok((self.line, ArxmlEvent::EndElement(&self.buffer[startpos..endpos])))
        } else {
            loop {
                if self.bufpos == self.buffer.len() {
                    break Ok((self.line, ArxmlEvent::EndOfFile));
                } else if self.buffer[self.bufpos] == b'<' {
                    // start of an <element> or </element> or <!--comment-->
                    // find a '>' character
                    let findpos = self.buffer[self.bufpos + 1..]
                        .iter()
                        .position(|c| *c == b'>')
                        .ok_or_else(|| self.error(ArxmlLexerError::IncompleteData))?;
                    let endpos = self.bufpos + findpos + 1;

                    if endpos == self.bufpos + 1 {
                        // string is "<>"
                        return Err(self.error(ArxmlLexerError::InvalidElement));
                    }

                    // got a non-empty sequence of characters that starts with '<' and ends with '>'
                    match self.buffer[self.bufpos + 1] {
                        b'/' => {
                            // second char is '/' -> EndElement
                            return Ok((self.line, self.read_element_end(endpos)));
                        }
                        b'?' => {
                            // second char is '?' -> xml header or processing instruction
                            // processing instructions are ignored, read_xml_header returns None
                            if let Some(result) = self.read_xml_header(endpos) {
                                let value = result?;
                                return Ok((self.line, value));
                            }
                        }
                        b'!' => {
                            // second char is '!' -> parse a comment
                            // we found a '>' character, but comments are allowed to contain unquoted '<' and '>'
                            // this means we need to make sure the end is actually '-->', not just '>'
                            let mut comment_endpos = endpos;
                            while comment_endpos < self.buffer.len()
                                && !self.buffer[comment_endpos - 2..].starts_with(b"-->")
                            {
                                comment_endpos += 1;
                            }
                            if comment_endpos < self.buffer.len() {
                                return self.read_comment(comment_endpos).map(|res| (self.line, res));
                            } else {
                                // hit the end of the input -> unclosed comment
                                return Err(self.error(ArxmlLexerError::InvalidComment));
                            }
                        }
                        _ => {
                            // any other second char -> BeginElement
                            return Ok((self.line, self.read_element_start(endpos)));
                        }
                    }
                } else {
                    // start of character sequence
                    if let (ArxmlEvent::Characters(text), false) = self.read_characters() {
                        // found a character sequence which is not all whitespace
                        return Ok((self.line, ArxmlEvent::Characters(text)));
                    }
                }
                // loop if:
                // - a processing instruction was ignored
                // - empty character data found (whitespace only)
            }
        }
    }

    fn error(&self, err: ArxmlLexerError) -> AutosarDataError {
        AutosarDataError::LexerError {
            filename: self.sourcefile.clone(),
            line: self.line,
            source: err,
        }
    }
}

fn count_lines(text: &[u8]) -> usize {
    text.iter().filter(|c| **c == b'\n').count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let data =
            b"<?xml version=\"1.0\" encoding=\"utf-8\"?><element attr=\"gggg\" attr3>contained characters</element>";
        let mut lexer = ArxmlLexer::new(data, PathBuf::from("(buffer)"));
        assert!(matches!(lexer.next(), Ok((_, ArxmlEvent::ArxmlHeader(None)))));
        assert!(
            matches!(lexer.next(), Ok((_, ArxmlEvent::BeginElement(elem, attrs))) if elem == b"element" && attrs.len() == 17)
        );
        assert!(matches!(lexer.next(), Ok((_, ArxmlEvent::Characters(text))) if text == b"contained characters"));
        assert!(matches!(lexer.next(), Ok((_, ArxmlEvent::EndElement(elem))) if elem == b"element"));
        assert!(matches!(lexer.next(), Ok((_, ArxmlEvent::EndOfFile))));
    }

    #[test]
    fn skip_byte_order_mark() {
        let data =
            b"\xEF\xBB\xBF<?xml version=\"1.0\" encoding=\"utf-8\"?><element attr=\"gggg\" attr3>contained characters</element>";
        let mut lexer = ArxmlLexer::new(data, PathBuf::from("(buffer)"));
        assert!(matches!(lexer.next(), Ok((_, ArxmlEvent::ArxmlHeader(None)))));
    }

    #[test]
    fn test_incomplete_data() {
        let data = b"<element";
        let mut lexer = ArxmlLexer::new(data, PathBuf::from("(buffer)"));
        assert!(
            matches!(lexer.next(), Err(AutosarDataError::LexerError {source, ..}) if source == ArxmlLexerError::IncompleteData)
        );
    }

    #[test]
    fn test_invalid_element() {
        let data = b"<element><>";
        let mut lexer = ArxmlLexer::new(data, PathBuf::from("(buffer)"));
        assert!(lexer.next().is_ok());
        assert!(
            matches!(lexer.next(), Err(AutosarDataError::LexerError{source, ..}) if source == ArxmlLexerError::InvalidElement)
        );
    }

    #[test]
    fn test_invalid_processing_instruction() {
        let data = b"<element><?what>";
        let mut lexer = ArxmlLexer::new(data, PathBuf::from("(buffer)"));
        assert!(lexer.next().is_ok());
        assert!(
            matches!(lexer.next(), Err(AutosarDataError::LexerError{source, ..}) if source == ArxmlLexerError::InvalidProcessingInstruction)
        );
    }

    #[test]
    fn test_comment() {
        let data = b"<!-- foo--><element>";
        let mut lexer = ArxmlLexer::new(data, PathBuf::from("(buffer)"));
        assert!(matches!(lexer.next(), Ok((_, ArxmlEvent::Comment(_)))));
        assert!(matches!(lexer.next(), Ok((_, ArxmlEvent::BeginElement(_elem, _attrs)))));
    }

    #[test]
    fn test_invalid_comment() {
        let data = b"<element><!-- foo>";
        let mut lexer = ArxmlLexer::new(data, PathBuf::from("(buffer)"));
        assert!(lexer.next().is_ok());
        assert!(
            matches!(lexer.next(), Err(AutosarDataError::LexerError{source, ..}) if source == ArxmlLexerError::InvalidComment)
        );
    }

    #[test]
    fn test_invalid_xml_header() {
        let data = br#"<?xml version="1.0" encoding="cp1252"?>"#;
        let mut lexer = ArxmlLexer::new(data, PathBuf::from("(buffer)"));
        assert!(
            matches!(lexer.next(), Err(AutosarDataError::LexerError{source, ..}) if source == ArxmlLexerError::InvalidXmlHeader)
        );

        let data = br#"<?xml ?>"#;
        let mut lexer = ArxmlLexer::new(data, PathBuf::from("(buffer)"));
        assert!(
            matches!(lexer.next(), Err(AutosarDataError::LexerError{source, ..}) if source == ArxmlLexerError::InvalidXmlHeader)
        );
    }

    #[test]
    fn traits() {
        // ArxmlLexerError: Debug, Error, Eq, PartialEq, Clone
        let err = ArxmlLexerError::IncompleteData;
        let err2 = err;
        assert_eq!(err, err2);
        assert_eq!(format!("{err:#?}"), format!("{err2:#?}"));
        assert_eq!(format!("{err}"), format!("{err2}"));

        // ArxmlEvent: Debug
        let event = ArxmlEvent::ArxmlHeader(None);
        let _ = format!("{event:#?}");
    }

    /// github issue #15 - comments can contain '<' and '>'
    #[test]
    fn test_w3c_comment_example() {
        let data = b"<!-- declarations for <head> & <body> -->";
        let mut lexer = ArxmlLexer::new(data, PathBuf::from("(buffer)"));
        assert!(matches!(lexer.next(), Ok((_, ArxmlEvent::Comment(_)))));
    }
}
