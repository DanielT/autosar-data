use std::ffi::OsString;
use thiserror::Error;
use super::AutosarDataError;

#[derive(Debug, Error)]
pub enum ArxmlLexerError {
    #[error("Incomplete data, closing '>' was not found")]
    IncompleteData,

    #[error("Invalid element: '<>'")]
    InvalidElement,

    #[error("A processing instruction was started with '<?', but it did not end with '?>'")]
    InvalidProcessingInstruction,

    #[error("Invalid arxml header: The xml header of an arxml file must specify version=\"1.0\" encoding=\"utf-8\"")]
    InvalidXmlHeader,

    #[error("Invalid comment")]
    InvalidComment,
}

#[derive(Debug)]
pub(crate) enum ArxmlEvent<'a> {
    ArxmlHeader,
    BeginElement(&'a [u8], &'a [u8]),
    EndElement(&'a [u8]),
    Characters(&'a [u8]),
    EndOfFile,
}


pub(crate) struct ArxmlLexer<'a> {
    buffer: &'a [u8],
    bufpos: usize,
    line: usize,
    deferred_end: Option<(usize, usize)>,
    sourcefile: OsString
}


impl<'a> ArxmlLexer<'a> {
    pub(crate) fn new(buffer: &'a [u8], name: OsString) -> Self {
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
            sourcefile: name
        }
    }


    fn read_characters(&mut self) -> ArxmlEvent<'a> {
        debug_assert!(self.bufpos < self.buffer.len());

        // the start of the next element '<' is the end of this block of characters
        let mut endpos = self.bufpos;
        while endpos < self.buffer.len() && self.buffer[endpos] != b'<' {
            endpos += 1;
        }
        debug_assert!(endpos > self.bufpos);

        let text = &self.buffer[self.bufpos .. endpos];
        self.line += count_lines(text);
        self.bufpos = endpos;
        ArxmlEvent::Characters(text)
    }


    fn read_element_start(&mut self, endpos: usize) -> ArxmlEvent<'a> {
        debug_assert!(self.bufpos < self.buffer.len());
        debug_assert!(endpos > self.bufpos + 1);
        debug_assert!(self.buffer[self.bufpos] == b'<');

        let (text, is_end) = if self.buffer[endpos-1] == b'/' {
            (&self.buffer[self.bufpos + 1 .. endpos - 1], true)
        } else {
            (&self.buffer[self.bufpos + 1 .. endpos], false)
        };

        let (elemname, attributes) = if let Some((splitpos, _)) = text.iter().enumerate().find(|(_, c)| c.is_ascii_whitespace()) {
            (&text[..splitpos], &text[splitpos+1 ..])
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

        if elemname.is_empty() {
            println!("empty element name in {}", String::from_utf8_lossy(text))
        }
        ArxmlEvent::BeginElement(elemname, attributes)
    }


    fn read_element_end(&mut self, endpos: usize) -> ArxmlEvent<'a> {
        debug_assert!(self.bufpos < self.buffer.len());
        debug_assert!(endpos > self.bufpos + 1);
        debug_assert!(self.buffer[self.bufpos] == b'<');

        let text = &self.buffer[self.bufpos + 2 .. endpos];
        self.line += count_lines(text);
        self.bufpos = endpos + 1;

        ArxmlEvent::EndElement(text)
    }


    fn read_xml_header(&mut self, endpos: usize) -> Option<Result<ArxmlEvent<'a>, AutosarDataError>> {
        debug_assert!(self.bufpos < self.buffer.len());
        debug_assert!(endpos > self.bufpos + 1);
        debug_assert!(self.buffer[self.bufpos] == b'<');

        if self.buffer[endpos - 1] != b'?' {
            return Some(Err(AutosarDataError::LexerError { filename: self.sourcefile.clone() , line: self.line, source: ArxmlLexerError::InvalidProcessingInstruction }))
        }

        let text = &self.buffer[self.bufpos + 2 .. endpos - 1];
        self.bufpos = endpos + 1;

        let mut splitter = text.split(|c| c.is_ascii_whitespace());
        let elemname = splitter.next().unwrap();

        let result = if elemname == b"xml" {
            let mut ver = &text[0..0];
            let mut encoding = &text[0..0];
            for attr_text in splitter {
                let (attr_name, attr_val) = if let Some((pos, _)) = attr_text.iter().enumerate().find(|(_, c)| **c == b'=') {
                    (&attr_text[0..pos], &attr_text[pos+2 .. attr_text.len() -1])
                } else {
                    (attr_text, &attr_text[0..0])
                };
                if attr_name == b"version" {
                    ver = attr_val;
                } else if attr_name == b"encoding" {
                    encoding = attr_val;
                }
            }

            if ver != b"1.0" || (encoding != b"utf-8" && encoding != b"UTF-8" && encoding != b"utf8" && encoding != b"UTF8") {
                Some(Err(AutosarDataError::LexerError { filename: self.sourcefile.clone() , line: self.line, source: ArxmlLexerError::InvalidXmlHeader }))
            } else {
                Some(Ok(ArxmlEvent::ArxmlHeader))
            }
        } else {
            None
        };

        self.line += count_lines(text);
        result
    }


    fn read_comment(&mut self, endpos: usize) -> Result<(), AutosarDataError> {
        debug_assert!(self.bufpos < self.buffer.len());
        debug_assert!(endpos > self.bufpos + 1);

        let text = &self.buffer[self.bufpos .. endpos];
        self.bufpos = endpos + 1;

        if !text.starts_with(b"<!--") || !text.ends_with(b"--") {
            return Err(AutosarDataError::LexerError { filename: self.sourcefile.clone() , line: self.line, source: ArxmlLexerError::InvalidComment });
        }
        self.line += count_lines(text);
        Ok(())
    }
}


impl<'a> ArxmlLexer<'a> {
    pub(crate) fn next(&mut self) -> Result<(usize, ArxmlEvent), AutosarDataError> {
        // if an <element/> was found, then a BeginElement event is returned first, and the EndElement is deferred and must be returned next
        if let Some((startpos, endpos)) = self.deferred_end {
            self.deferred_end = None;
            return Ok((self.line, ArxmlEvent::EndElement(&self.buffer[startpos .. endpos])));
        } else {
            loop {
                if self.bufpos == self.buffer.len() {
                    break Ok((self.line, ArxmlEvent::EndOfFile))
                } else if self.buffer[self.bufpos] == b'<' {
                    // start of an <element> or </element> or <!--comment-->
                    let mut endpos = self.bufpos;
                    while endpos < self.buffer.len() && self.buffer[endpos] != b'>' {
                        endpos += 1;
                    }
                    if endpos == self.buffer.len() {
                        // not found
                        break Err(AutosarDataError::LexerError { filename: self.sourcefile.clone() , line: self.line, source: ArxmlLexerError::IncompleteData })
                    } else if endpos <= self.bufpos + 1 {
                        break Err(AutosarDataError::LexerError { filename: self.sourcefile.clone() , line: self.line, source: ArxmlLexerError::InvalidElement })
                    } else if self.buffer[self.bufpos + 1] == b'/' {
                        break Ok((self.line, self.read_element_end(endpos)))
                    } else if self.buffer[self.bufpos + 1] == b'?' {
                        if let Some(result) = self.read_xml_header(endpos) {
                            break Ok((self.line, result?));
                        }
                    } else if self.buffer[self.bufpos + 1] == b'!' {
                        self.read_comment(endpos)?;
                    } else {
                        break Ok((self.line, self.read_element_start(endpos)))
                    }
                } else {
                    // start of characters or whitespace
                    if let ArxmlEvent::Characters(text) = self.read_characters() {
                        let is_whitespace = text.iter().all(|c| c.is_ascii_whitespace());
                        // only break and return a value if the text is not empty
                        if !is_whitespace {
                            break Ok((self.line, ArxmlEvent::Characters(text)));
                        }
                    }
                }
            }
        }
    }
}


fn count_lines(text: &[u8]) -> usize {
    text.iter().filter(|c| **c == b'\n').count()
}


#[test]
fn test_buffer_parser() {
    let data = b"<?xml version=\"1.0\" encoding=\"utf-8\"?><element attr=\"gggg\" attr3>contained characters</element>";
    let mut parser = ArxmlLexer::new(data, OsString::from("(buffer)"));
    match parser.next() {
        Ok((_, ArxmlEvent::ArxmlHeader)) => {}
        _ => assert!(false)
    }
    match parser.next() {
        Ok((_, ArxmlEvent::BeginElement(elem, attrs))) => {
            assert_eq!(elem, b"element");
            assert_eq!(attrs.len(), 17)
        }
        _ => assert!(false)
    }
    match parser.next() {
        Ok((_, ArxmlEvent::Characters(text))) => {
            assert_eq!(text, b"contained characters")
        }
        _ => assert!(false)
    }
    match parser.next() {
        Ok((_, ArxmlEvent::EndElement(elem))) => {
            assert_eq!(elem, b"element");
        }
        _ => assert!(false)
    }
}