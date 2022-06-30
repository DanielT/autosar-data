//! Crate autosar-data
//!
//! This crate provides functionality to read, modify and write Autosar arxml files, both separately and as collections.

use autosar_data_specification::*;
use element::ElementActionError;
use iterators::*;
use lexer::*;
use parking_lot::Mutex;
use parser::*;
use smallvec::SmallVec;
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    sync::{Arc, Weak},
};
use std::{fs::File, io::Read};
use thiserror::Error;

mod arxmlfile;
mod autosardata;
mod chardata;
mod element;
mod iterators;
mod lexer;
mod parser;

/// AutosarData is the top level data type in the autosar-data crate.
///
/// All manipulations of arxml files are performed through an instance of AutosarDatas
#[derive(Debug, Clone)]
pub struct AutosarData(Arc<Mutex<AutosarDataRaw>>);

// Weak reference to an instance of AutosarData
#[derive(Debug, Clone)]
pub(crate) struct WeakAutosarData(Weak<Mutex<AutosarDataRaw>>);

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

    #[error("Operation failed: the item has been deleted")]
    ItemDeleted,
}

/// An Autosar arxml file
#[derive(Debug, Clone)]
pub struct ArxmlFile(Arc<Mutex<ArxmlFileRaw>>);

/// Weak reference to an arxml file
///
/// (see the documentation of [std::sync::Arc] for an explanation of weak references)
#[derive(Debug, Clone)]
pub struct WeakArxmlFile(Weak<Mutex<ArxmlFileRaw>>);

/// The data of an arxml file
#[derive(Debug)]
pub(crate) struct ArxmlFileRaw {
    autosar_data: WeakAutosarData,
    pub(crate) version: AutosarVersion,
    pub(crate) filename: OsString,
    pub(crate) root_element: Element,
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

/// The data of an arxml element
#[derive(Debug)]
pub(crate) struct ElementRaw {
    pub(crate) parent: ElementOrFile,
    pub(crate) elemname: ElementName,
    pub(crate) elemtype: ElementType,
    pub(crate) content: SmallVec<[ElementContent; 4]>,
    pub(crate) attributes: SmallVec<[Attribute; 1]>,
}

/// A single attribute of an arxml element
#[derive(Debug, Clone)]
pub struct Attribute {
    pub(crate) attrname: AttributeName,
    pub(crate) content: CharacterData,
}

/// One content item inside an arxml element
///
/// Elements may contain other elements, character data, or a mixture of both, depending on their type.
#[derive(Debug, Clone, PartialEq)]
pub enum ElementContent {
    Element(Element),
    CharacterData(CharacterData),
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

/// Possible kinds of compatibility errors that can be found by `ArxmlFile::check_version_compatibility()`
pub enum CompatibilityError {
    IncompatibleElement {
        element: Element,
        version_mask: u32,
    },
    IncompatibleAttribute {
        element: Element,
        attribute: AttributeName,
        version_mask: u32,
    },
    IncompatibleAttributeValue {
        element: Element,
        attribute: AttributeName,
        version_mask: u32,
    },
}
