//! Crate autosar-data
//!
//! This crate provides functionality to read, modify and write Autosar arxml files,
//! both separately and in projects consisting of multiple files.
//!
//! Features:
//!
//!  - read and write arxml files
//!  - fully validate all data when it is loaded
//!  - non-strict mode so that invalid but structurally sound data can be loaded
//!  - various element operations to modify and create sub-elements, data and attributes
//!  - support for Autosar paths and cross references
//!  - all operations are thread safe, e.g. it is possible to load mutliple files on separate threads
//!
//! # Examples
//!
//! ```no_run
//! use autosar_data::*;
//! # fn main() -> Result<(), AutosarDataError> {
//! /* load a multi-file project */
//! let project = AutosarProject::new();
//! let (file_1, warnings_1) = project.load_arxml_file("some_file.arxml", false)?;
//! let (file_2, warnings_2) = project.load_arxml_file("other_file.arxml", false)?;
//! /* load a buffer */
//! # let buffer = b"";
//! let (file_3, _) = project.load_named_arxml_buffer(buffer, "filename.arxml", true)?;
//!
//! /* write all files of the project */
//! project.write()?;
//!
//! /* alternatively: */
//! for file in project.files() {
//!     let file_data = file.serialize();
//!     // do something with file_data
//! }
//!
//! /* iterate over all elements in all files */
//! for (depth, element) in project.elements_dfs() {
//!     if element.is_identifiable() {
//!         /* the element is identifiable using an Autosar path */
//!         println!("{depth}: {}, {}", element.element_name(), element.path()?);
//!     } else {
//!         println!("{depth}: {}", element.element_name());
//!     }
//! }
//!
//! /* get an element by its Autosar path */
//! let pdu_element = project.get_element_by_path("/Package/Mid/PduName").unwrap();
//!
//! /* work with the content of elements */
//! if let Some(length) = pdu_element
//!     .get_sub_element(ElementName::Length)
//!     .and_then(|elem| elem.character_data())
//!     .and_then(|cdata| cdata.string_value())
//! {
//!     println!("Pdu Length: {length}");
//! }
//!
//! /* modify the attributes of an element */
//! pdu_element.set_attribute_string(AttributeName::Uuid, "12ab34cd-1234-1234-1234-12ab34cd56ef");
//! pdu_element.remove_attribute(AttributeName::Uuid);
//!
//! # Ok(())
//! # }
//! ```
//!
//! # Example Programs
//!
//! Two complete example programs can be found in the examples directory of the source repostitory. They are:
//!
//!  - businfo, which extracts information about bus settings, frames, pdus and signals from an autosar ECU extract
//!  - generate_files, which for each Autosar version generates an arxml file containing a least one instance of every specified element
//!

use autosar_data_specification::*;
use iterators::*;
use lexer::*;
use parking_lot::Mutex;
use parser::*;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use std::path::PathBuf;
use std::sync::{Arc, Weak};
use std::{fs::File, io::Read};
use thiserror::Error;

mod arxmlfile;
mod autosarproject;
mod chardata;
mod element;
mod iterators;
mod lexer;
mod parser;

// reexport some of the info from the specification
pub use autosar_data_specification::AttributeName;
pub use autosar_data_specification::AutosarVersion;
pub use autosar_data_specification::ElementName;
pub use autosar_data_specification::EnumItem;

/// AutosarProject is the top level data type in the autosar-data crate.
///
/// All manipulations of arxml files are performed through an instance of AutosarProject
#[derive(Debug, Clone)]
pub struct AutosarProject(Arc<Mutex<AutosarProjectRaw>>);

// Weak reference to an instance of AutosarProject
#[derive(Debug, Clone)]
pub(crate) struct WeakAutosarProject(Weak<Mutex<AutosarProjectRaw>>);

/// An autosar project
///
/// The project consists of a number auf arxml files, each of which contains a heirarchy of elements.
/// In addition, this top-level structure provides chaching of Autosar paths, to allow quick resolution of cross-references.
#[derive(Debug)]
pub(crate) struct AutosarProjectRaw {
    files: Vec<ArxmlFile>,
    /// identifiables is a HashMap of all named elements, needed to resolve references without doing a full search.
    identifiables: FxHashMap<String, WeakElement>,
    /// reference_origins is a HashMap of all referencing alements. This is needed to efficiently fix up the references when a referenced element is renamed.
    reference_origins: FxHashMap<String, Vec<WeakElement>>,
}

/// The error type AutosarDataError wraps all errors that can be generated anywhere in the crate
#[derive(Error, Debug)]
pub enum AutosarDataError {
    /// IoErrorRead: An IoError that occurred while reading a file
    #[error("Failed to read {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorRead { filename: PathBuf, ioerror: std::io::Error },

    /// IoErrorOpen: an IoError that occurres while opening a file
    #[error("Failed to open {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorOpen { filename: PathBuf, ioerror: std::io::Error },

    /// IoErrorWrite: An IoError that occurred while writing a file
    #[error("Failed to write {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorWrite { filename: PathBuf, ioerror: std::io::Error },

    /// DuplicateFilenameError,
    #[error("Could not {verb} file {}: A file with this name is already loaded", .filename.to_string_lossy())]
    DuplicateFilenameError { verb: &'static str, filename: PathBuf },

    /// LexerError: An error originating in the lexer, such as unclodes strings, mismatched '<' and '>', etc
    #[error("Failed to tokenize {} on line {line}: {source}", .filename.to_string_lossy())]
    LexerError {
        filename: PathBuf,
        line: usize,
        source: ArxmlLexerError,
    },

    /// ParserError: A parser error
    #[error("Failed to parse {}:{line}: {source}", .filename.to_string_lossy())]
    ParserError {
        filename: PathBuf,
        line: usize,
        source: ArxmlParserError,
    },

    /// A file could not be loaded into the project, because the Autosar paths of the new data overlapped with the Autosar paths of the existing data
    #[error("Loading failed: element path {path} of new data in {} overlaps with the existing loaded data", .filename.to_string_lossy())]
    OverlappingDataError { filename: PathBuf, path: String },

    /// An operation failed because one of the elements involved is in the deleted state and will be freed once its reference count reaches zero
    #[error("Operation failed: the item has been deleted")]
    ItemDeleted,

    /// A sub element could not be created at or moved to the given position
    #[error("Invalid position for an element of this kind")]
    InvalidPosition,

    /// The Autosar version of the containing file was not compatible
    #[error("Version is not compatible")]
    VersionIncompatible,

    /// A function that only applies to identifiable elements was called on an element which is not identifiable
    #[error("The Element is not identifiable")]
    ElementNotIdentifiable,

    /// An item name is required to perform this action
    #[error("An item name is required")]
    ItemNameRequired,

    /// The element has the wrong content type for the requested operation, e.g. inserting elements when the content type only allows character data
    #[error("Incorrect content type")]
    IncorrectContentType,

    /// Could not insert a sub element, because it conflicts with an existing sub element
    #[error("Element insertion conflict")]
    ElementInsertionConflict,

    /// The ElementName is not a valid sub element according to the specification.
    #[error("Invalid sub element")]
    InvalidSubElement,

    /// Remove operation failed: the given element is not a sub element of the element from which it was supposed to be removed
    #[error("element not found")]
    ElementNotFound,

    /// Remove_sub_element cannot remove the SHORT-NAME of identifiable elements, as this would render the data invalid
    #[error("the SHORT-NAME sub element may not be removed")]
    ShortNameRemovalForbidden,

    /// get/set reference target was called for an element that is not a reference
    #[error("The current element is not a reference")]
    NotReferenceElement,

    /// The reference is invalid
    #[error("The reference is not valid")]
    InvalidReference,

    /// An element could not be renamed, since this item name is already used by a different element
    #[error("Duplicate item name")]
    DuplicateItemName,

    /// Cannot move an element into its own sub element
    #[error("Cannot move an element into its own sub element")]
    ForbiddenMoveToSubElement,

    /// A parent element is currently locked by a different operation. The operation wa aborted to avoid a deadlock.
    #[error("A parent element is currently locked by a different operation")]
    ParentElementLocked,

    /// The attribute is invalid here
    #[error("The attribute is not valid for this element")]
    InvalidAttribute,

    /// The attribute value is invalid
    #[error("The given value is not valid for this attribute")]
    InvalidAttributeValue,
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
    project: WeakAutosarProject,
    pub(crate) version: AutosarVersion,
    pub(crate) filename: PathBuf,
    pub(crate) root_element: Element,
    pub(crate) xml_standalone: Option<bool>, // preserve the xml standalone attribute
}

/// An arxml element
///
/// This is actually a wrapper type which provides all the necessary manipulation functions. The actual element data is
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
    pub attrname: AttributeName,
    pub content: CharacterData,
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
    UnsignedInteger(u64),
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
#[derive(Debug, Clone)]
pub(crate) enum ElementOrFile {
    Element(WeakElement),
    File(WeakArxmlFile),
    None, // needed while constructing the data trees, otherwise there's a chicken vs. egg problem
}

/// Possible kinds of compatibility errors that can be found by `ArxmlFile::check_version_compatibility()`
pub enum CompatibilityError {
    /// The element is not allowed in the target version
    IncompatibleElement { element: Element, version_mask: u32 },
    /// The attribute is not allowed in the target version
    IncompatibleAttribute {
        element: Element,
        attribute: AttributeName,
        version_mask: u32,
    },
    /// The attribute value is not allowed in the target version
    IncompatibleAttributeValue {
        element: Element,
        attribute: AttributeName,
        version_mask: u32,
    },
}
