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
//! /* load a multi-file data model */
//! let model = AutosarModel::new();
//! let (file_1, warnings_1) = model.load_file("some_file.arxml", false)?;
//! let (file_2, warnings_2) = model.load_file("other_file.arxml", false)?;
//! /* load a buffer */
//! # let buffer = b"";
//! let (file_3, _) = model.load_buffer(buffer, "filename.arxml", true)?;
//!
//! /* write all files of the model */
//! model.write()?;
//!
//! /* alternatively: */
//! for file in model.files() {
//!     let file_data = file.serialize();
//!     // do something with file_data
//! }
//!
//! /* iterate over all elements in all files */
//! for (depth, element) in model.elements_dfs() {
//!     if element.is_identifiable() {
//!         /* the element is identifiable using an Autosar path */
//!         println!("{depth}: {}, {}", element.element_name(), element.path()?);
//!     } else {
//!         println!("{depth}: {}", element.element_name());
//!     }
//! }
//!
//! /* get an element by its Autosar path */
//! let pdu_element = model.get_element_by_path("/Package/Mid/PduName").unwrap();
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
//!  - `generate_files`, which for each Autosar version generates an arxml file containing a least one instance of every specified element
//!

#![warn(missing_docs)]

use autosar_data_specification::{AttributeSpec, CharacterDataSpec, ContentMode, ElementType};
use fxhash::{FxBuildHasher, FxHashMap};
use indexmap::IndexMap;
pub use iterators::*;
use parking_lot::RwLock;
use parser::ArxmlParser;
use smallvec::SmallVec;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};
use std::{fs::File, io::Read};
use thiserror::Error;

mod arxmlfile;
mod autosarmodel;
mod chardata;
mod element;
mod elementraw;
mod iterators;
mod lexer;
mod parser;

// allow public access to the error sub-types
pub use lexer::ArxmlLexerError;
pub use parser::ArxmlParserError;

// reexport some of the info from the specification
pub use autosar_data_specification::AttributeName;
pub use autosar_data_specification::AutosarVersion;
pub use autosar_data_specification::ElementName;
pub use autosar_data_specification::EnumItem;

type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;

/// `AutosarModel` is the top level data type in the autosar-data crate.
///
/// An instance of `AutosarModel` is required for all other operations.
///
/// The model contains the hierarchy of Autosar elements. It can be created manually or loaded from one or more arxml files.
/// It stores the association between elements and files.
/// In addition, this top-level structure provides caching of Autosar paths, to allow quick resolution of cross-references.
#[derive(Clone)]
pub struct AutosarModel(Arc<RwLock<AutosarModelRaw>>);

// Weak reference to an instance of AutosarModel
#[derive(Clone)]
pub(crate) struct WeakAutosarModel(Weak<RwLock<AutosarModelRaw>>);

/// The inner autosar data model (unlocked)
///
/// The model contains the hierarchy of Autosar elements. It can be created manually or loaded from one or more arxml files.
/// It stores the association between elements and files.
/// In addition, this top-level structure provides caching of Autosar paths, to allow quick resolution of cross-references.
pub(crate) struct AutosarModelRaw {
    root_element: Element,
    files: Vec<ArxmlFile>,
    /// `identifiables` is a `HashMap` of all named elements, needed to resolve references without doing a full search.
    identifiables: FxIndexMap<String, WeakElement>,
    /// `reference_origins` is a `HashMap` of all referencing alements. This is needed to efficiently fix up the references when a referenced element is renamed.
    reference_origins: FxHashMap<String, Vec<WeakElement>>,
}

/// The error type `AutosarDataError` wraps all errors that can be generated anywhere in the crate
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AutosarDataError {
    /// `IoErrorRead`: An `IoError` that occurred while reading a file
    #[error("Failed to read {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorRead {
        /// The filename that caused the error
        filename: PathBuf,
        /// The underlying `std::io::Error`
        ioerror: std::io::Error,
    },

    /// `IoErrorOpen`: an `IoError` that occurres while opening a file
    #[error("Failed to open {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorOpen {
        /// The filename that caused the error
        filename: PathBuf,
        /// The underlying `std::io::Error`
        ioerror: std::io::Error,
    },

    /// `IoErrorWrite`: An `IoError` that occurred while writing a file
    #[error("Failed to write {}: {ioerror}", .filename.to_string_lossy())]
    IoErrorWrite {
        /// The filename that caused the error
        filename: PathBuf,
        /// The underlying `std::io::Error`
        ioerror: std::io::Error,
    },

    /// `DuplicateFilenameError`: The model can'#'t contain two files with identical names
    #[error("Could not {verb} file {}: A file with this name is already loaded", .filename.to_string_lossy())]
    DuplicateFilenameError {
        /// description of the operation that failed
        verb: &'static str,
        /// The filename that caused the error
        filename: PathBuf,
    },

    /// `LexerError`: An error originating in the lexer, such as unclodes strings, mismatched '<' and '>', etc
    #[error("Failed to tokenize {} on line {line}: {source}", .filename.to_string_lossy())]
    LexerError {
        /// The filename that caused the error
        filename: PathBuf,
        /// The line number where the error occurred
        line: usize,
        /// The underlying `ArxmlLexerError`
        source: ArxmlLexerError,
    },

    /// `ParserError`: A parser error
    #[error("Failed to parse {}:{line}: {source}", .filename.to_string_lossy())]
    ParserError {
        /// The filename that caused the error
        filename: PathBuf,
        /// The line number where the error occurred
        line: usize,
        /// The underlying `ArxmlParserError`
        source: ArxmlParserError,
    },

    /// A file could not be loaded into the model, because the Autosar paths of the new data overlapped with the Autosar paths of the existing data
    #[error("Loading failed: element path {path} of new data in {} overlaps with the existing loaded data", .filename.to_string_lossy())]
    OverlappingDataError {
        /// The filename that caused the error
        filename: PathBuf,
        /// Autosar path of the element that caused the error
        path: String,
    },

    /// An operation failed because one of the elements involved is in the deleted state and will be freed once its reference count reaches zero
    #[error("Operation failed: the item has been deleted")]
    ItemDeleted,

    /// A sub element could not be created at or moved to the given position
    #[error("Invalid position for an element of this kind")]
    InvalidPosition,

    /// The Autosar version of the new file or element did not match the version already in use
    #[error("Version mismatch between existing {} and new {}", .version_cur, .version_new)]
    VersionMismatch {
        /// The current version of the model
        version_cur: AutosarVersion,
        /// The version of the new file or element
        version_new: AutosarVersion,
    },

    /// The Autosar version is not compatible with the data
    #[error("Version {} is not compatible with the element data", .version)]
    VersionIncompatibleData {
        /// The incompatible version
        version: AutosarVersion,
    },

    /// A function that only applies to identifiable elements was called on an element which is not identifiable
    #[error("The element at {} is not identifiable", .xmlpath)]
    ElementNotIdentifiable {
        /// The "xml path" (a string representation of the path to the element) where the error occurred
        xmlpath: String,
    },

    /// An item name is required to perform this action
    #[error("An item name is required for element {}", .element)]
    ItemNameRequired {
        /// The element where the item name is required
        element: ElementName,
    },

    /// The element has the wrong content type for the requested operation, e.g. inserting elements when the content type only allows character data
    #[error("Incorrect content type for element {}", .element)]
    IncorrectContentType {
        /// The element where the content type is incorrect
        element: ElementName,
    },

    /// Could not insert a sub element, because it conflicts with an existing sub element
    #[error("Element insertion conflict: {} could not be inserted in {}", .element, .parent)]
    ElementInsertionConflict {
        /// The name of the parent element
        parent: ElementName,
        /// The name of the element that could not be inserted
        element: ElementName,
    },

    /// The `ElementName` is not a valid sub element according to the specification.
    #[error("Element {} is not a valid sub element of {}", .element, .parent)]
    InvalidSubElement {
        /// The name of the parent element
        parent: ElementName,
        /// The name of the element that is not a valid sub element
        element: ElementName,
    },

    /// Remove operation failed: the given element is not a sub element of the element from which it was supposed to be removed
    #[error("element {} not found in parent {}", .target, .parent)]
    ElementNotFound {
        /// The name of the element that was not found
        target: ElementName,
        /// The name of the parent element
        parent: ElementName,
    },

    /// [`Element::remove_sub_element`] cannot remove the SHORT-NAME of identifiable elements, as this would render the data invalid
    #[error("the SHORT-NAME sub element may not be removed")]
    ShortNameRemovalForbidden,

    /// get/set reference target was called for an element that is not a reference
    #[error("The current element is not a reference")]
    NotReferenceElement,

    /// The reference is invalid
    #[error("The reference is not valid")]
    InvalidReference,

    /// An element could not be renamed, since this item name is already used by a different element
    #[error("Duplicate item name {} in {}", .item_name, .element)]
    DuplicateItemName {
        /// The name of the element that could not be renamed
        element: ElementName,
        /// The target name that caused the error
        item_name: String,
    },

    /// Cannot move an element into its own sub element
    #[error("Cannot move an element into its own sub element")]
    ForbiddenMoveToSubElement,

    /// Cannot copy an element (or a hierarchy including the element) into itself
    #[error("Cannot create a copy that includes the destination")]
    ForbiddenCopyOfParent,

    /// A parent element is currently locked by a different operation. The operation wa aborted to avoid a deadlock.
    #[error("A parent element is currently locked by a different operation")]
    ParentElementLocked,

    /// The attribute is invalid here
    #[error("The attribute is not valid for this element")]
    InvalidAttribute,

    /// The attribute value is invalid
    #[error("The given value is not valid for this attribute")]
    InvalidAttributeValue,

    /// The file is from a different model and may not be used in this operation
    #[error("The file is from a different model and may not be used in this operation")]
    InvalidFile,

    /// The file is empty and cannot be serialized
    #[error("The file is empty and cannot be serialized")]
    EmptyFile,

    /// The newly loaded file diverges from the combined model on an element which is not splittable according to the metamodel
    #[error("The new file could not be merged, because it diverges from the model on non-splittable element {}", .path)]
    InvalidFileMerge {
        /// The path of the element where the merge failed
        path: String,
    },

    /// The operation cannot be completed because the model does not contain any files
    #[error("The operation cannot be completed because the model does not contain any files")]
    NoFilesInModel,

    /// Modifying the fileset of this element is not allowed
    #[error(
        "Modifying the fileset of this element is not allowed, because the parent of the element is not marked as splittable"
    )]
    FilesetModificationForbidden,
}

/// An Autosar arxml file
#[derive(Clone)]
pub struct ArxmlFile(Arc<RwLock<ArxmlFileRaw>>);

/// Weak reference to an arxml file
///
/// (see the documentation of [`std::sync::Arc`] for an explanation of weak references)
#[derive(Clone)]
pub struct WeakArxmlFile(Weak<RwLock<ArxmlFileRaw>>);

/// The data of an arxml file
pub(crate) struct ArxmlFileRaw {
    pub(crate) version: AutosarVersion,
    model: WeakAutosarModel,
    pub(crate) filename: PathBuf,
    pub(crate) xml_standalone: Option<bool>, // preserve the xml standalone attribute
}

/// An arxml element
///
/// This is a wrapper type which provides all the necessary manipulation functions.
#[derive(Clone)]
pub struct Element(Arc<RwLock<ElementRaw>>);

/// Weak reference to an Element
///
/// (see the documentation of [`std::sync::Arc`] for an explanation of weak references)
///
/// This `WeakElement` can be held indefinitely without forcing the referenced data to remain valid.
/// When access is needed, the method `upgrade()` will attempt to get a strong reference and return an [Element]
#[derive(Clone)]
pub struct WeakElement(Weak<RwLock<ElementRaw>>);

/// The data of an arxml element
pub(crate) struct ElementRaw {
    pub(crate) parent: ElementOrModel,
    pub(crate) elemname: ElementName,
    pub(crate) elemtype: ElementType,
    pub(crate) content: SmallVec<[ElementContent; 4]>,
    pub(crate) attributes: SmallVec<[Attribute; 1]>,
    pub(crate) file_membership: HashSet<WeakArxmlFile>,
    pub(crate) comment: Option<String>,
}

/// A single attribute of an arxml element
#[derive(Clone, PartialEq, Eq)]
pub struct Attribute {
    /// The name of the attribute
    pub attrname: AttributeName,
    /// The content of the attribute
    pub content: CharacterData,
}

/// One content item inside an arxml element
///
/// Elements may contain other elements, character data, or a mixture of both, depending on their type.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ElementContent {
    /// A sub element
    Element(Element),
    /// Character data
    CharacterData(CharacterData),
}

/// The enum `CharacterData` provides typed access to the content of elements and attributes
///
/// Example:
///
/// In the xml string ```<SHORT-NAME>SomeName</SHORT-NAME>``` the character data
/// "`SomeName`" will be loaded as `CharacterData::String("SomeName`"), while the content of the
/// attribute <... DEST="UNIT"> will be loaded as `CharacterData::Enum(EnumItem::Unit`)
#[derive(Debug, PartialEq, Clone)]
pub enum CharacterData {
    /// Character data is an enum value
    Enum(EnumItem),
    /// Character data is a string
    String(String),
    /// Character data is an unsigned integer
    UnsignedInteger(u64),
    /// Character data is a floating point number
    Float(f64),
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
/// root element, the parent is an element. The root element itself has a reference to the `ArxmlFile` structure.
#[derive(Clone)]
pub(crate) enum ElementOrModel {
    Element(WeakElement),
    Model(WeakAutosarModel),
    None, // needed while constructing the data trees, otherwise there's a chicken vs. egg problem
}

/// Possible kinds of compatibility errors that can be found by `ArxmlFile::check_version_compatibility()`
pub enum CompatibilityError {
    /// The element is not allowed in the target version
    IncompatibleElement {
        /// The element that is not allowed
        element: Element,
        /// The version mask of the element which indicates all allowed versions
        version_mask: u32,
    },
    /// The attribute is not allowed in the target version
    IncompatibleAttribute {
        /// The element that contains the incompatible attribute
        element: Element,
        /// The incompatible attribute
        attribute: AttributeName,
        /// The version mask of the element which indicates all versions where the attribute is allowed
        version_mask: u32,
    },
    /// The attribute value is not allowed in the target version
    IncompatibleAttributeValue {
        /// The element that contains the incompatible attribute
        element: Element,
        /// The incompatible attribute
        attribute: AttributeName,
        /// The incompatible attribute value
        attribute_value: String,
        /// The version mask of the element which indicates all versions where the attribute value is allowed
        version_mask: u32,
    },
}

/// information about a sub element
///
/// This structure is returned by [`Element::list_valid_sub_elements()`]
pub struct ValidSubElementInfo {
    /// name of the potential sub element
    pub element_name: ElementName,
    /// is the sub element named, i.e. does it need to be created with [`Element::create_named_sub_element()`]
    pub is_named: bool,
    /// is the sub element currently allowed, given the existing content of the element. Note that some sub elements are mutually exclusive.
    pub is_allowed: bool,
}

const CHECK_FILE_SIZE: usize = 4096; // 4kb

/// Check a file to see if it looks like an arxml file
///
/// Reads the beginning of the given file and checks if the data starts with a valid arxml header.
/// If a header is found it immediately returns true and does not check any further data
///
/// The function returns false if the file cannot be read or if the data does not start with an arxml header
///
/// # Parameters
/// - filename: name of the file to check
///
/// # Example
///
/// ```
/// # let filename = "";
/// if autosar_data::check_file(filename) {
///     // it looks like an arxml file
/// }
/// ```
pub fn check_file<P: AsRef<Path>>(filename: P) -> bool {
    let mut buffer: [u8; CHECK_FILE_SIZE] = [0; CHECK_FILE_SIZE];

    if File::open(filename).and_then(|mut file| file.read(&mut buffer)).is_ok() {
        check_buffer(&buffer)
    } else {
        false
    }
}

/// Check a buffer to see if the content looks like arxml data
///
/// The function returns true if the buffer starts with a valid arxml header (after skipping whitespace and comments).
/// This function does not check anything after the header.
///
/// # Parameters
/// - buffer: u8 slice containing the data to check
///
/// # Example
/// ```
/// # let buffer = Vec::new();
/// if autosar_data::check_buffer(&buffer) {
///     // it looks like arxml data
/// }
/// ```
#[must_use]
pub fn check_buffer(buffer: &[u8]) -> bool {
    let mut parser = ArxmlParser::new(PathBuf::from("none"), buffer, false);
    parser.check_arxml_header()
}

/// Custom Debug implementation for `Attribute`, in order to provide better formatting
impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Attribute: {:?} = {:?}", self.attrname, self.content)
    }
}

/// provide PartialOrd for attributes; this is used while sorting elements
impl PartialOrd for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// provide Ord for attributes; this is used while sorting elements
impl Ord for Attribute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.attrname
            .to_str()
            .cmp(other.attrname.to_str())
            .then(self.content.cmp(&other.content))
    }
}

#[cfg(test)]
mod test {
    use std::{error::Error, io::Write, path::PathBuf};
    use tempfile::tempdir;

    use crate::*;

    #[test]
    fn error_traits() {
        let err = AutosarDataError::ParserError {
            filename: PathBuf::from("filename.arxml"),
            line: 123,
            source: crate::parser::ArxmlParserError::InvalidArxmlFileHeader,
        };
        assert!(err.source().is_some());
        let errstr = format!("{err}");
        let errdbg = format!("{err:#?}");
        assert!(errstr != errdbg);
    }

    #[test]
    fn test_check_file() {
        let dir = tempdir().unwrap();

        // called on a directory rather than a file -> false
        assert!(!check_file(dir.path()));

        // nonexistent file -> false
        let nonexistent = dir.path().with_file_name("nonexistent.arxml");
        assert!(!check_file(nonexistent));

        // arbitrary non-arxml data -> false
        let not_arxml_file = dir.path().with_file_name("not_arxml.bin");
        File::create(&not_arxml_file)
            .and_then(|mut file| write!(file, "text"))
            .unwrap();
        assert!(!check_file(not_arxml_file));

        // file containing a valid arxml header -> true
        let header = r#"<?xml version="1.0" encoding="utf-8"?>
        <!-- comment --><!-- comment 2 -->
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">"#;
        let arxml_file = dir.path().with_file_name("file.arxml");
        File::create(&arxml_file)
            .and_then(|mut file| file.write(header.as_bytes()))
            .unwrap();
        assert!(check_file(arxml_file));

        assert!(check_buffer(header.as_bytes()));
    }

    #[test]
    fn attribute_order() {
        // attribute ordering: first by name, then by value
        let a1 = Attribute {
            attrname: AttributeName::Uuid,
            content: CharacterData::String("Value1".to_string()),
        };

        let a2 = Attribute {
            attrname: AttributeName::Uuid,
            content: CharacterData::String("Value2".to_string()),
        };
        assert!(a1 < a2);

        let a3 = Attribute {
            attrname: AttributeName::T,
            content: CharacterData::String("xyz".to_string()),
        };
        assert!(a3 < a1);

        // PartialOrd
        assert!(a1.partial_cmp(&a2) == Some(std::cmp::Ordering::Less));
    }
}
