use element::ElementActionError;
use iterators::*;
use lexer::*;
use parser::*;
use smallvec::SmallVec;
use specification::*;
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    sync::{Arc, Mutex, Weak},
};
use std::{fs::File, io::Read};
use thiserror::Error;

mod arxmlfile;
mod chardata;
mod element;
mod iterators;
mod lexer;
mod parser;
mod spec_support;
pub mod specification;

#[derive(Error, Debug)]
pub enum AutosarDataError {
    #[error("Failed to read {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorRead {
        filename: OsString,
        ioerror: std::io::Error,
    },
    #[error("Failed to open {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorOpen {
        filename: OsString,
        ioerror: std::io::Error,
    },
    #[error("Failed to tokenize {} on line {line}: {source}", .filename.to_string_lossy())]
    LexerError {
        filename: OsString,
        line: usize,
        source: ArxmlLexerError,
    },
    #[error("failed to parse {}:{line}: {source}", .filename.to_string_lossy())]
    ParserError {
        filename: OsString,
        line: usize,
        source: ArxmlParserError,
    },
    #[error("element path {path} of new data in {} overlaps with the existing loaded data", .filename.to_string_lossy())]
    OverlappingDataError { filename: OsString, path: String },

    #[error("element operation failed: {source}")]
    ElementActionError { source: ElementActionError },
}

#[derive(Debug, PartialEq, Clone)]
pub enum CharacterData {
    Enum(EnumItem),
    String(String),
    UnsignedInteger(usize),
    Double(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementContent {
    Element(Element),
    CharacterData(CharacterData),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub(crate) attrname: AttributeName,
    pub(crate) content: CharacterData,
}

#[derive(Debug)]
pub enum ElementOrFile {
    Element(WeakElement),
    File(WeakArxmlFile),
    None, // needed while constructing the data trees, otherwise there's a chicken vs. egg problem
}

#[derive(Debug)]
pub(crate) struct ElementRaw {
    pub(crate) parent: ElementOrFile,
    pub(crate) elemname: ElementName,
    pub(crate) type_id: usize,
    pub(crate) content: SmallVec<[ElementContent; 4]>,
    pub(crate) attributes: SmallVec<[Attribute; 1]>,
}

#[derive(Debug, Clone)]
pub struct Element(Arc<Mutex<ElementRaw>>);

#[derive(Debug, Clone)]
pub struct WeakElement(Weak<Mutex<ElementRaw>>);

#[derive(Debug)]
pub(crate) struct ArxmlFileRaw {
    autosar_data: WeakAutosarData,
    pub(crate) version: AutosarVersion,
    pub(crate) filename: OsString,
    pub(crate) root_element: Element,
}

#[derive(Debug, Clone)]
pub struct ArxmlFile(Arc<Mutex<ArxmlFileRaw>>);

#[derive(Debug, Clone)]
pub struct WeakArxmlFile(Weak<Mutex<ArxmlFileRaw>>);

#[derive(Debug)]
pub(crate) struct AutosarDataRaw {
    files: Vec<ArxmlFile>,
    identifiables: HashMap<String, WeakElement>,
    references: HashMap<String, Vec<WeakElement>>,
}

#[derive(Debug, Clone)]
pub struct AutosarData(Arc<Mutex<AutosarDataRaw>>);

#[derive(Debug, Clone)]
pub struct WeakAutosarData(Weak<Mutex<AutosarDataRaw>>);


impl AutosarData {
    pub fn new() -> AutosarData {
        AutosarData(Arc::new(Mutex::new(AutosarDataRaw {
            files: Vec::new(),
            identifiables: HashMap::new(),
            references: HashMap::new(),
        })))
    }

    pub fn create_file(&self, filename: OsString, version: AutosarVersion) -> Result<ArxmlFile, ()> {
        if let Ok(mut inner) = self.0.lock() {
            if !inner.files.iter().any(|af| af.filename() == filename) {
                let new_file = ArxmlFile::new(filename, version, self);
                inner.files.push(new_file.clone());
                return Ok(new_file);
            }
        }
        Err(())
    }

    pub fn load_named_arxml_buffer(&self, buffer: &[u8], filename: &OsStr) -> Result<ArxmlFile, AutosarDataError> {
        let mut parser = parser::ArxmlParser::new(filename.to_os_string(), buffer, log_func, false);
        let root_element = parser.parse_arxml()?;

        let arxml_file = ArxmlFile(Arc::new(Mutex::new(ArxmlFileRaw {
            autosar_data: self.downgrade(),
            version: parser.get_fileversion(),
            filename: filename.to_os_string(),
            root_element,
        })));
        // graft on the back-link from the root element to the file
        let new_parent = ElementOrFile::File(arxml_file.downgrade());
        arxml_file.root_element().set_parent(new_parent);

        if let Ok(mut inner) = self.0.lock() {
            inner.identifiables.reserve(parser.identifiables.len());
            for (key, value) in parser.identifiables {
                if let Some(existing) = inner.identifiables.insert(key, value) {
                    if let Some(element) = existing.upgrade() {
                        return Err(AutosarDataError::OverlappingDataError {
                            filename: filename.to_os_string(),
                            path: element.path().unwrap_or_else(|| "".to_owned()),
                        });
                    }
                }
            }
            for (refpath, referring_element) in parser.references {
                if let Some(xref) = inner.references.get_mut(&refpath) {
                    xref.push(referring_element);
                } else {
                    inner.references.insert(refpath, vec![referring_element]);
                }
            }
            inner.files.push(arxml_file.clone());
        }

        Ok(arxml_file)
    }


    pub fn load_arxml_file(&self, filename: &OsStr) -> Result<ArxmlFile, AutosarDataError> {
        let buffer = load_file_data(filename)?;
        self.load_named_arxml_buffer(&buffer, filename)
    }


    pub fn write_arxml_buffers(&self) {}


    pub fn write_arxml_files(&self) {}

    pub fn files(&self) -> ArxmlFileIterator {
        ArxmlFileIterator::new(self.clone())
    }


    pub fn get_named_element(&self, path: &str) -> Result<Element, ()> {
        if let Ok(data) = self.0.lock() {
            if let Some(weak_element) = data.identifiables.get(path) {
                let element = weak_element.upgrade().ok_or(())?;
                Ok(element)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn elements_dfs(&self) -> AutosarDataElementsDfsIterator {
        AutosarDataElementsDfsIterator::new(self.files())
    }

    pub fn downgrade(&self) -> WeakAutosarData {
        WeakAutosarData(Arc::downgrade(&self.0))
    }

    pub(crate) fn fix_identifiables(&self, old_path: Option<String>, element: Element) {
        if let Ok(mut data) = self.0.lock() {
            let new_path = element.path().unwrap();
            if let Some(old_path) = old_path {
                data.identifiables.remove(&old_path);

                if element.element_name() == ElementName::ArPackage {
                    // a package has been renamed, so it might contain other identifiable elements that are affected by the renaming
                    let keys: Vec<String> = data.identifiables.keys().cloned().collect();
                    for key in keys {
                        // find keys referring to entries inside the renamed package
                        if key.starts_with(&old_path) {
                            // construct the new element path
                            let (_, suffix) = key.split_at(old_path.len());
                            let new_key = format!("{new_path}{suffix}");
                            // fix the identifiables hashmap
                            let entry = data.identifiables.remove(&key).unwrap();
                            data.identifiables.insert(new_key, entry);
                        }
                    }
                }
            }
            // insert under the new name regardless of whether an old name existed or not
            data.identifiables.insert(new_path, element.downgrade());
        }
    }

    pub(crate) fn remove_identifiable(&self, path: &str) {
        if let Ok(mut data) = self.0.lock() {
            data.identifiables.remove(path);
        }
    }

    pub(crate) fn add_reference(&self, new_ref: &str, origin: WeakElement) {
        if let Ok(mut data) = self.0.lock() {
            // add the new entry
            if let Some(referrer_list) = data.references.get_mut(new_ref) {
                referrer_list.push(origin);
            } else {
                data.references.insert(new_ref.to_owned(), vec![origin]);
            }
        }
    }

    pub(crate) fn fix_references(&self, old_ref: &str, new_ref: &str, origin: WeakElement) {
        if old_ref != new_ref {
            if let Ok(mut data) = self.0.lock() {
                // remove the old entry
                if let Some(referrer_list) = data.references.get_mut(old_ref) {
                    if let Some((index, _)) = referrer_list.iter().enumerate().find(|(_, x)| **x == origin) {
                        referrer_list.swap_remove(index);
                    }
                }
                // add the new entry
                if let Some(referrer_list) = data.references.get_mut(new_ref) {
                    referrer_list.push(origin);
                } else {
                    data.references.insert(new_ref.to_owned(), vec![origin]);
                }
            }
        }
    }

    pub(crate) fn remove_reference(&self, reference: &str, element: WeakElement) {
        if let Ok(mut data) = self.0.lock() {
            if let Some(referrer_list) = data.references.get_mut(reference) {
                if let Some((index, _)) = referrer_list.iter().enumerate().find(|(_, x)| **x == element) {
                    referrer_list.swap_remove(index);
                }
            }
        }
    }
}


impl Default for AutosarData {
    fn default() -> Self {
        Self::new()
    }
}


impl WeakAutosarData {
    pub fn upgrade(&self) -> Option<AutosarData> {
        Weak::upgrade(&self.0).map(AutosarData)
    }
}


fn log_func(err: AutosarDataError) {
    println!("logged error: {err}");
}


fn load_file_data(filename: &OsStr) -> Result<Vec<u8>, AutosarDataError> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            return Err(AutosarDataError::IoErrorOpen {
                filename: filename.to_os_string(),
                ioerror: error,
            })
        }
    };

    let filesize = file.metadata().unwrap().len();
    let mut buffer = Vec::with_capacity(filesize as usize);
    match file.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(err) => Err(AutosarDataError::IoErrorRead {
            filename: filename.to_os_string(),
            ioerror: err,
        }),
    }
}
