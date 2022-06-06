use std::{ffi::{OsStr, OsString}, sync::{Arc, Mutex, Weak}, collections::{HashSet,HashMap}};
use std::{fs::File, io::Read};
use thiserror::Error;
use lexer::*;
use parser::*;
use element::*;

mod lexer;
mod parser;
mod specification;
mod iterators;
mod element;

#[derive(Error, Debug)]
pub enum AutosarDataError {
    #[error("Failed to read {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorRead {
        filename: OsString,
        ioerror: std::io::Error
    },
    #[error("Failed to open {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorOpen {
        filename: OsString,
        ioerror: std::io::Error
    },
    #[error("Failed to tokenize {} on line {line}: {source}", .filename.to_string_lossy())]
    LexerError {
        filename: OsString,
        line: usize,
        source: ArxmlLexerError
    },
    #[error("failed to parse {}:{line}: {source}", .filename.to_string_lossy())]
    ParserError {
        filename: OsString,
        line: usize,
        source: ArxmlParserError
    },
    #[error("new data in {} overlaps with the existing loaded data", .filename.to_string_lossy())]
    OverlappingDataError {
        filename: OsString,
    }
}

#[derive(Debug, Clone)]
pub enum CharacterData {
    Enum(specification::EnumItem),
    String(String),
    UnsignedInteger(usize),
    Double(f64)
}

#[derive(Debug)]
pub enum ElementContent {
    Element(Element),
    CharacterData(CharacterData),
}

#[derive(Debug)]
pub struct Attribute {
    pub(crate) attrname: specification::AttributeName,
    pub(crate) content: CharacterData
}

#[derive(Debug)]
pub(crate) struct ElementRaw {
    pub(crate) elemname: specification::ElementName,
    pub(crate) type_id: usize,
    pub(crate) content: Vec<ElementContent>,
    pub(crate) attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct Element(Arc<Mutex<ElementRaw>>);

#[derive(Debug, Clone)]
pub struct WeakElement(Weak<Mutex<ElementRaw>>);

#[derive(Debug)]
pub(crate) struct ArxmlFileRaw {
    pub(crate) version: specification::AutosarVersion,
    pub(crate) filename: OsString,
    pub(crate) root_element: Element,
}

#[derive(Debug, Clone)]
pub struct ArxmlFile(Arc<Mutex<ArxmlFileRaw>>);

#[derive(Debug)]
pub struct AutosarData {
    files: Arc<Mutex<Vec<ArxmlFile>>>,
    identifiables: Arc<Mutex<HashMap<String, WeakElement>>>,
}

impl AutosarData {
    pub fn new() -> AutosarData {
        AutosarData {
            files: Arc::new(Mutex::new(Vec::new())),
            identifiables: Arc::new(Mutex::new(HashMap::new())),
        }
    }


    pub fn load_named_arxml_buffer(&self, buffer: &[u8], filename: &OsStr) -> Result<&Self, AutosarDataError> {
        let mut parser = parser::ArxmlParser::new(filename.to_os_string(), buffer, log_func, false);
        let (root_element, new_idents) = parser.parse_arxml()?;

        let arxml_file = ArxmlFileRaw {
            version: parser.get_fileversion(),
            filename: filename.to_os_string(),
            root_element,
        };

        if let Ok(mut identifiables) = self.identifiables.lock() {
            let keys1: HashSet<&String> = identifiables.keys().collect();
            let keys2: HashSet<&String> = new_idents.keys().collect();
            if !keys1.is_disjoint(&keys2) {
                return Err(AutosarDataError::OverlappingDataError { filename: filename.to_os_string() });
            }
            identifiables.extend(new_idents);
        }
        if let Ok(mut files) = self.files.lock() {
            files.push(ArxmlFile(Arc::new(Mutex::new(arxml_file))));
        }


        Ok(self)
    }


    pub fn load_arxml_file(&self, filename: &OsStr) -> Result<&Self, AutosarDataError> {
        let buffer = load_file_data(filename)?;
        self.load_named_arxml_buffer(&buffer, filename)
    }


    pub fn write_arxml_buffers() {

    }


    pub fn write_arxml_files() {

    }


    pub fn arxml_files(&self) {

    }


    pub fn get_arxml_file_by_name(&self, name: &OsStr) -> Option<ArxmlFile> {
        if let Ok(files) = self.files.lock() {
            for wrapped_file in files.iter() {
                if let Ok(file) = wrapped_file.0.lock() {
                    if file.filename == name {
                        return Some(wrapped_file.clone());
                    }
                }
            }
        }
        None
    }


    pub fn files(&self) -> iterators::ArxmlFileIterator {
        iterators::ArxmlFileIterator {
            data: self,
            index: 0,
        }
    }


    pub fn elements(&self) {

    }
}


impl ArxmlFile {
    pub fn filename(&self) -> OsString {
        let raw = self.0.lock().unwrap();
        raw.filename.clone()
    }
}


fn log_func(err: AutosarDataError) {
    println!("logged error: {err}");
}


fn load_file_data(filename: &OsStr) -> Result<Vec<u8>, AutosarDataError> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => return Err(AutosarDataError::IoErrorOpen { filename: filename.to_os_string(), ioerror: error }),
    };
    
    let filesize = file.metadata().unwrap().len();
    let mut buffer = Vec::with_capacity(filesize as usize);
    match file.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(err) => Err(AutosarDataError::IoErrorRead { filename: filename.to_os_string(), ioerror: err }),
    }
}