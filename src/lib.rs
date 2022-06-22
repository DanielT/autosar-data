//! Crate autosar-data
//!
//! This crate provides functionality to read, modify and write Autosar arxml files, bith separately and as collections.

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

/// The error type AutosarDataError wraps all errors that can be generated anywhere in the crate
#[derive(Error, Debug)]
pub enum AutosarDataError {
    /// IoErrorRead: An IoError that occurred while reading a file
    #[error("Failed to read {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorRead {
        filename: OsString,
        ioerror: std::io::Error,
    },
    /// IoErrorOpen: an IoError that occurres while opening a file
    #[error("Failed to open {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorOpen {
        filename: OsString,
        ioerror: std::io::Error,
    },
    /// DuplicateFilenameError,
    #[error("Could not {verb} file {}: A file with this name is already loaded", .filename.to_string_lossy())]
    DuplicateFilenameError { verb: &'static str, filename: OsString },
    /// LexerError: An error originating in the lexer, such as unclodes strings, mismatched '<' and '>', etc
    #[error("Failed to tokenize {} on line {line}: {source}", .filename.to_string_lossy())]
    LexerError {
        filename: OsString,
        line: usize,
        source: ArxmlLexerError,
    },
    /// ParserError: A parser error
    #[error("Failed to parse {}:{line}: {source}", .filename.to_string_lossy())]
    ParserError {
        filename: OsString,
        line: usize,
        source: ArxmlParserError,
    },
    #[error("Loading failed: element path {path} of new data in {} overlaps with the existing loaded data", .filename.to_string_lossy())]
    OverlappingDataError { filename: OsString, path: String },

    #[error("Element operation failed: {source}")]
    ElementActionError { source: ElementActionError },

    #[error("Critical error: Mutex poisoned due to a previous fault")]
    MutexPoisoned,

    #[error("Operation failed: the item has been deleted")]
    ItemDeleted,
}

/// The enum CharacterData provides typed access to the content of elements and attributes
///
/// Example:
///
/// In the xml string ```<SHORT-NAME>SomeName</SHORT-NAME>``` the character data
/// "SomeName" will be loaded as CharacterData::String("SomeName"), while the content of the
/// attribute <... DEST="UNIT"> will be loaded as CharacterData::Enum(EnumItem::Unit)
#[derive(Debug, PartialEq, Clone)]
pub enum CharacterData {
    Enum(EnumItem),
    String(String),
    UnsignedInteger(usize),
    Double(f64),
}

/// The content type of an [Element]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ContentType {
    /// The element only contains other elements
    Elements,
    /// The element only contains character data
    CharacterData,
    /// The element contains both character data and sub elements
    Mixed,
}

/// One content item inside an arxml element
///
/// Elements may contain other elements, character data, or a mixture of both, depending on their type.
#[derive(Debug, Clone, PartialEq)]
pub enum ElementContent {
    Element(Element),
    CharacterData(CharacterData),
}

/// A single attribute of an arxml element
#[derive(Debug, Clone)]
pub struct Attribute {
    pub(crate) attrname: AttributeName,
    pub(crate) content: CharacterData,
}

/// Holds a weak reference to either an element or an arxml file
///
/// This enum is used for references to the parent of each element. For all elements other than the
/// root element, the parent is an element. The root element itself has a referenct to the ArxmlFile structure.
#[derive(Debug)]
pub(crate) enum ElementOrFile {
    Element(WeakElement),
    File(WeakArxmlFile),
    None, // needed while constructing the data trees, otherwise there's a chicken vs. egg problem
}

/// The data of an arxml element
#[derive(Debug)]
pub(crate) struct ElementRaw {
    pub(crate) parent: ElementOrFile,
    pub(crate) elemname: ElementName,
    pub(crate) type_id: usize,
    pub(crate) content: SmallVec<[ElementContent; 4]>,
    pub(crate) attributes: SmallVec<[Attribute; 1]>,
}

/// An arxml element
///
/// This is actually a wrapper type which provides all the necessary manupulation functions. The actual element data is
/// held behind Arc<Mutex<>>.
#[derive(Debug, Clone)]
pub struct Element(Arc<Mutex<ElementRaw>>);

/// Weak reference to an Element
///
/// (see the documentation of [std::sync::Arc] for an explanation of weak references)
///
/// This WeakElement can be held indefinitely without forcing the referenced data to remain valid.
/// When access is needed, the method upgrade() will attempt to get a strong reference and return an [Element]
#[derive(Debug, Clone)]
pub struct WeakElement(Weak<Mutex<ElementRaw>>);

/// The data of an arxml file
#[derive(Debug)]
pub(crate) struct ArxmlFileRaw {
    autosar_data: WeakAutosarData,
    pub(crate) version: AutosarVersion,
    pub(crate) filename: OsString,
    pub(crate) root_element: Element,
}

/// An Autosar arxml file
#[derive(Debug, Clone)]
pub struct ArxmlFile(Arc<Mutex<ArxmlFileRaw>>);

/// Weak reference to an arxml file
///
/// (see the documentation of [std::sync::Arc] for an explanation of weak references)
#[derive(Debug, Clone)]
pub struct WeakArxmlFile(Weak<Mutex<ArxmlFileRaw>>);

/// Data of an autosar project
///
/// This data consists of a number auf arxml files, each of which contains a heirarchy of elements.
/// In addition, this top-level strucutre provides chaching of Autosar paths, to allow quick resolution of cross-references.
#[derive(Debug)]
pub(crate) struct AutosarDataRaw {
    files: Vec<ArxmlFile>,
    /// identifiables is a HashMap of all named elements, needed to resolve references without doing a full search.
    identifiables: HashMap<String, WeakElement>,
    /// reference_origins is a HashMap of all referencing alements. This is needed to efficiently fix up the references when a referenced element is renamed.
    reference_origins: HashMap<String, Vec<WeakElement>>,
}

/// AutosarData is the top level data type in the autosar-data crate.
///
/// All manipulations of arxml files are performed through an instance of AutosarDatas
#[derive(Debug, Clone)]
pub struct AutosarData(Arc<Mutex<AutosarDataRaw>>);

// Weak reference to an instance of AutosarData
#[derive(Debug, Clone)]
pub(crate) struct WeakAutosarData(Weak<Mutex<AutosarDataRaw>>);

impl AutosarData {
    /// Create an instance of AutosarData
    ///
    /// Initially it contains no arxml files
    pub fn new() -> AutosarData {
        AutosarData(Arc::new(Mutex::new(AutosarDataRaw {
            files: Vec::new(),
            identifiables: HashMap::new(),
            reference_origins: HashMap::new(),
        })))
    }

    /// Create a new [ArxmlFile] inside this AutosarData structure
    ///
    /// You must provide a filename for the [ArxmlFile], even if you do not plan to write the data to disk.
    /// You must also specify an [AutosarVersion]. All methods manipulation the data insdie the file will ensure conformity with the version specified here.
    /// The newly created ArxmlFile will be created with a root AUTOSAR element.
    pub fn create_file(&self, filename: OsString, version: AutosarVersion) -> Result<ArxmlFile, AutosarDataError> {
        let mut inner = self.0.lock().map_err(|_| AutosarDataError::MutexPoisoned)?;

        if inner.files.iter().any(|af| af.filename() == filename) {
            return Err(AutosarDataError::DuplicateFilenameError {
                verb: "create",
                filename,
            });
        }

        let new_file = ArxmlFile::new(filename, version, self);
        inner.files.push(new_file.clone());
        Ok(new_file)
    }

    /// Load a named buffer containig arxml data
    ///
    /// If you have e.g. received arxml data over a network, or decompressed it from an archive, etc, then you may load it with this method.
    ///
    /// Parameters:
    ///  - buffer: The data inside the buffer must be valid utf-8. Optionally it may begin with a UTF-8-BOM, which will be silently ignored.
    ///  - filename: the original filename of the data, or a newly generated name that is unique within the AutosarData instance.
    ///  - strict: toggle strict parsing. Some parsing errors are recoverable and can be issued as warnings.
    ///
    /// This method may be called concurrently on multiple threads to load different buffers
    pub fn load_named_arxml_buffer(
        &self,
        buffer: &[u8],
        filename: &OsStr,
        strict: bool,
    ) -> Result<(ArxmlFile, Vec<AutosarDataError>), AutosarDataError> {
        if self.files().any(|file| file.filename() == filename) {
            return Err(AutosarDataError::DuplicateFilenameError {
                verb: "load",
                filename: filename.to_owned(),
            });
        }

        let mut parser = ArxmlParser::new(filename.to_os_string(), buffer, strict);
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

        let mut inner = self.0.lock().map_err(|_| AutosarDataError::MutexPoisoned)?;
        inner.identifiables.reserve(parser.identifiables.len());
        for (key, value) in parser.identifiables {
            if let Some(existing) = inner.identifiables.insert(key, value) {
                if let Some(element) = existing.upgrade() {
                    return Err(AutosarDataError::OverlappingDataError {
                        filename: filename.to_os_string(),
                        path: element.path().unwrap().unwrap_or_else(|| "".to_owned()),
                    });
                }
            }
        }
        for (refpath, referring_element) in parser.references {
            if let Some(xref) = inner.reference_origins.get_mut(&refpath) {
                xref.push(referring_element);
            } else {
                inner.reference_origins.insert(refpath, vec![referring_element]);
            }
        }
        inner.files.push(arxml_file.clone());

        Ok((arxml_file, parser.warnings))
    }

    /// Load an arxml file
    ///
    /// This function is a wrapper around load_named_arxml_buffer to make the common case of loading a file from disk more convenient
    ///
    /// Parameters:
    ///  - filename: the original filename of the data, or a newly generated name that is unique within the AutosarData instance.
    ///  - strict: toggle strict parsing. Some parsing errors are recoverable and can be issued as warnings.
    pub fn load_arxml_file(
        &self,
        filename: &OsStr,
        strict: bool,
    ) -> Result<(ArxmlFile, Vec<AutosarDataError>), AutosarDataError> {
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
        file.read_to_end(&mut buffer)
            .map_err(|err| AutosarDataError::IoErrorRead {
                filename: filename.to_os_string(),
                ioerror: err,
            })?;

        self.load_named_arxml_buffer(&buffer, filename, strict)
    }

    /// serialize each of the files in the data set
    ///
    /// returns the result in a HashMap of <file_name, file_content>
    pub fn serialize_files(&self) -> HashMap<OsString, String> {
        let mut result = HashMap::new();
        for file in self.files() {
            let data = file.serialize();
            result.insert(file.filename(), data);
        }
        result
    }

    /// create an iterator over all [ArxmlFile]s in this AutosarData object
    pub fn files(&self) -> ArxmlFileIterator {
        ArxmlFileIterator::new(self.clone())
    }

    /// get a named element by its Autosar path
    ///
    /// This is a lookup in a hash table and runs in O(1) time
    pub fn get_element_by_path(&self, path: &str) -> Result<Option<Element>, AutosarDataError> {
        let inner = self.0.lock().map_err(|_| AutosarDataError::MutexPoisoned)?;
        Ok(inner.identifiables.get(path).and_then(|element| element.upgrade()))
    }

    /// create a depth-first iterator over all [Element]s in all [ArxmlFile]s
    pub fn elements_dfs(&self) -> AutosarDataElementsDfsIterator {
        AutosarDataElementsDfsIterator::new(self.files())
    }

    /// create a weak reference to this data
    pub(crate) fn downgrade(&self) -> WeakAutosarData {
        WeakAutosarData(Arc::downgrade(&self.0))
    }

    pub(crate) fn fix_identifiables(&self, old_path: Option<String>, element: Element) {
        if let Ok(mut data) = self.0.lock() {
            let new_path = match element.path() {
                Ok(Some(path)) => path,
                _ => return,
            };
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

    pub(crate) fn add_reference_origin(&self, new_ref: &str, origin: WeakElement) {
        if let Ok(mut data) = self.0.lock() {
            // add the new entry
            if let Some(referrer_list) = data.reference_origins.get_mut(new_ref) {
                referrer_list.push(origin);
            } else {
                data.reference_origins.insert(new_ref.to_owned(), vec![origin]);
            }
        }
    }

    pub(crate) fn fix_reference_origins(&self, old_ref: &str, new_ref: &str, origin: WeakElement) {
        if old_ref != new_ref {
            if let Ok(mut data) = self.0.lock() {
                // remove the old entry
                if let Some(referrer_list) = data.reference_origins.get_mut(old_ref) {
                    if let Some((index, _)) = referrer_list.iter().enumerate().find(|(_, x)| **x == origin) {
                        referrer_list.swap_remove(index);
                    }
                }
                // add the new entry
                if let Some(referrer_list) = data.reference_origins.get_mut(new_ref) {
                    referrer_list.push(origin);
                } else {
                    data.reference_origins.insert(new_ref.to_owned(), vec![origin]);
                }
            }
        }
    }

    pub(crate) fn remove_reference_origin(&self, reference: &str, element: WeakElement) {
        if let Ok(mut data) = self.0.lock() {
            if let Some(referrer_list) = data.reference_origins.get_mut(reference) {
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

impl PartialEq for AutosarData {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

impl WeakAutosarData {
    pub(crate) fn upgrade(&self) -> Option<AutosarData> {
        Weak::upgrade(&self.0).map(AutosarData)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_file() {
        let data = AutosarData::new();
        let file = data.create_file(OsString::from("test"), AutosarVersion::Autosar_00050);
        assert!(file.is_ok());
    }

    #[test]
    fn load_buffer() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES></AR-PACKAGES></AUTOSAR>"#;
        let data = AutosarData::new();
        let result = data.load_named_arxml_buffer(FILEBUF.as_bytes(), &OsString::from("test"), true);
        assert!(result.is_ok());
    }

    #[test]
    fn references() {
        let data = AutosarData::new();
        let weak = data.downgrade();
        let data2 = weak.upgrade();
        assert_eq!(Arc::strong_count(&data.0), 2);
        assert_eq!(data, data2.unwrap());
    }
}
