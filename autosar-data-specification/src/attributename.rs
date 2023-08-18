use crate::hashfunc;

#[cfg(feature = "pylib")]
use pyo3::prelude::*;

#[derive(Debug)]
/// The error type ParseAttributeNameError is returned when from_str() / parse() fails for AttributeName
pub struct ParseAttributeNameError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "pylib", pyclass)]
#[repr(u16)]
/// Enum of all attribute names in Autosar
pub enum AttributeName {
    /// ACCESSKEY
    Accesskey               = 79,
    /// ALIGN
    Align                   = 72,
    /// ALLOW-BREAK
    AllowBreak              = 15,
    /// ALT
    Alt                     = 46,
    /// BASE
    Base                    = 40,
    /// BGCOLOR
    Bgcolor                 = 41,
    /// BINDING-TIME
    BindingTime             = 18,
    /// BLUEPRINT-VALUE
    BlueprintValue          = 23,
    /// BREAK
    Break                   = 22,
    /// CLASS
    Class                   = 8,
    /// COLNAME
    Colname                 = 74,
    /// COLNUM
    Colnum                  = 47,
    /// COLOR
    Color                   = 26,
    /// COLS
    Cols                    = 88,
    /// COLSEP
    Colsep                  = 86,
    /// COLWIDTH
    Colwidth                = 65,
    /// COORDS
    Coords                  = 25,
    /// DEST
    Dest                    = 6,
    /// EDIT-HEIGHT
    EditHeight              = 90,
    /// EDIT-WIDTH
    EditWidth               = 20,
    /// EDITFIT
    Editfit                 = 21,
    /// EDITSCALE
    Editscale               = 83,
    /// ENUM-TABLE
    EnumTable               = 35,
    /// FILENAME
    Filename                = 31,
    /// FIT
    Fit                     = 68,
    /// FLOAT
    Float                   = 42,
    /// FONT
    Font                    = 61,
    /// FRAME
    Frame                   = 14,
    /// GENERATOR
    Generator               = 63,
    /// GID
    Gid                     = 51,
    /// HEIGHT
    Height                  = 16,
    /// HELP-ENTRY
    HelpEntry               = 58,
    /// HREF
    Href                    = 67,
    /// HTML-FIT
    HtmlFit                 = 55,
    /// HTML-HEIGHT
    HtmlHeight              = 97,
    /// HTML-SCALE
    HtmlScale               = 85,
    /// HTML-WIDTH
    HtmlWidth               = 69,
    /// INDEX
    Index                   = 32,
    /// INTERVAL-TYPE
    IntervalType            = 60,
    /// ITEM-LABEL-POS
    ItemLabelPos            = 70,
    /// KEEP-WITH-PREVIOUS
    KeepWithPrevious        = 54,
    /// L
    L                       = 43,
    /// LEVEL
    Level                   = 50,
    /// MIME-TYPE
    MimeType                = 96,
    /// MOREROWS
    Morerows                = 3,
    /// NAME
    Name                    = 33,
    /// NAME-PATTERN
    NamePattern             = 89,
    /// NAMEEND
    Nameend                 = 81,
    /// NAMEST
    Namest                  = 19,
    /// NOHREF
    Nohref                  = 0,
    /// NOTATION
    Notation                = 87,
    /// NOTE-TYPE
    NoteType                = 44,
    /// ONBLUR
    Onblur                  = 5,
    /// ONCLICK
    Onclick                 = 98,
    /// ONDBLCLICK
    Ondblclick              = 12,
    /// ONFOCUS
    Onfocus                 = 100,
    /// ONKEYDOWN
    Onkeydown               = 95,
    /// ONKEYPRESS
    Onkeypress              = 73,
    /// ONKEYUP
    Onkeyup                 = 37,
    /// ONMOUSEDOWN
    Onmousedown             = 64,
    /// ONMOUSEMOVE
    Onmousemove             = 71,
    /// ONMOUSEOUT
    Onmouseout              = 1,
    /// ONMOUSEOVER
    Onmouseover             = 56,
    /// ONMOUSEUP
    Onmouseup               = 39,
    /// ORIENT
    Orient                  = 30,
    /// PGWIDE
    Pgwide                  = 29,
    /// RESOLUTION-POLICY
    ResolutionPolicy        = 91,
    /// ROTATE
    Rotate                  = 57,
    /// ROWSEP
    Rowsep                  = 13,
    /// S
    S                       = 52,
    /// SCALE
    Scale                   = 76,
    /// SD
    Sd                      = 92,
    /// SHAPE
    Shape                   = 38,
    /// SHORT-LABEL
    ShortLabel              = 11,
    /// SHOW-CONTENT
    ShowContent             = 36,
    /// SHOW-RESOURCE-ALIAS-NAME
    ShowResourceAliasName   = 75,
    /// SHOW-RESOURCE-CATEGORY
    ShowResourceCategory    = 27,
    /// SHOW-RESOURCE-LONG-NAME
    ShowResourceLongName    = 53,
    /// SHOW-RESOURCE-NUMBER
    ShowResourceNumber      = 10,
    /// SHOW-RESOURCE-PAGE
    ShowResourcePage        = 7,
    /// SHOW-RESOURCE-SHORT-NAME
    ShowResourceShortName   = 9,
    /// SHOW-RESOURCE-TYPE
    ShowResourceType        = 2,
    /// SHOW-SEE
    ShowSee                 = 99,
    /// SI
    Si                      = 77,
    /// SPANNAME
    Spanname                = 4,
    /// STYLE
    Style                   = 45,
    /// T
    T                       = 80,
    /// TABINDEX
    Tabindex                = 93,
    /// TABSTYLE
    Tabstyle                = 48,
    /// TEX-RENDER
    TexRender               = 66,
    /// TITLE
    Title                   = 94,
    /// TYPE
    Type                    = 34,
    /// UUID
    Uuid                    = 59,
    /// VALIDITY
    Validity                = 84,
    /// VALIGN
    Valign                  = 49,
    /// VIEW
    View                    = 24,
    /// WIDTH
    Width                   = 62,
    /// xml:space
    xmlSpace                = 82,
    /// xmlns
    xmlns                   = 28,
    /// xmlns:xsi
    xmlnsXsi                = 17,
    /// xsi:schemaLocation
    xsiSchemalocation       = 78,
}

impl AttributeName {
    const STRING_TABLE: [&'static str; 101] = ["NOHREF", "ONMOUSEOUT", "SHOW-RESOURCE-TYPE", "MOREROWS", "SPANNAME", "ONBLUR", "DEST", "SHOW-RESOURCE-PAGE", "CLASS", "SHOW-RESOURCE-SHORT-NAME", "SHOW-RESOURCE-NUMBER", "SHORT-LABEL", "ONDBLCLICK", "ROWSEP", "FRAME", "ALLOW-BREAK", "HEIGHT", "xmlns:xsi", "BINDING-TIME", "NAMEST", "EDIT-WIDTH", "EDITFIT", "BREAK", "BLUEPRINT-VALUE", "VIEW", "COORDS", "COLOR", "SHOW-RESOURCE-CATEGORY", "xmlns", "PGWIDE", "ORIENT", "FILENAME", "INDEX", "NAME", "TYPE", "ENUM-TABLE", "SHOW-CONTENT", "ONKEYUP", "SHAPE", "ONMOUSEUP", "BASE", "BGCOLOR", "FLOAT", "L", "NOTE-TYPE", "STYLE", "ALT", "COLNUM", "TABSTYLE", "VALIGN", "LEVEL", "GID", "S", "SHOW-RESOURCE-LONG-NAME", "KEEP-WITH-PREVIOUS", "HTML-FIT", "ONMOUSEOVER", "ROTATE", "HELP-ENTRY", "UUID", "INTERVAL-TYPE", "FONT", "WIDTH", "GENERATOR", "ONMOUSEDOWN", "COLWIDTH", "TEX-RENDER", "HREF", "FIT", "HTML-WIDTH", "ITEM-LABEL-POS", "ONMOUSEMOVE", "ALIGN", "ONKEYPRESS", "COLNAME", "SHOW-RESOURCE-ALIAS-NAME", "SCALE", "SI", "xsi:schemaLocation", "ACCESSKEY", "T", "NAMEEND", "xml:space", "EDITSCALE", "VALIDITY", "HTML-SCALE", "COLSEP", "NOTATION", "COLS", "NAME-PATTERN", "EDIT-HEIGHT", "RESOLUTION-POLICY", "SD", "TABINDEX", "TITLE", "ONKEYDOWN", "MIME-TYPE", "HTML-HEIGHT", "ONCLICK", "SHOW-SEE", "ONFOCUS"];

    /// derive an enum entry from an input string using a perfect hash function
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseAttributeNameError> {
        static DISPLACEMENTS: [(u16, u16); 21] = [(0, 0), (0, 1), (0, 21), (0, 55), (0, 52), (0, 46), (0, 29), (0, 6), (0, 4), (1, 42), (0, 57), (0, 82), (0, 64), (0, 21), (12, 82), (1, 87), (22, 29), (7, 30), (0, 0), (44, 96), (64, 36)];
        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 21) as usize];
        let item_idx = (d2 as u32).wrapping_add(f1.wrapping_mul(d1 as u32)).wrapping_add(f2) as usize % 101;
        if AttributeName::STRING_TABLE[item_idx].as_bytes() != input {
            return Err(ParseAttributeNameError);
        }
        Ok(unsafe {
            std::mem::transmute::<u16, Self>(item_idx as u16)
        })
    }

    /// get the str corresponding to an item
    ///
    /// The returned &str has static lifetime, becasue it is a reference to an entry in a list of constants
    pub fn to_str(&self) -> &'static str {
        AttributeName::STRING_TABLE[*self as usize]
    }
}

impl std::str::FromStr for AttributeName {
    type Err = ParseAttributeNameError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(input.as_bytes())
    }
}

impl std::fmt::Debug for AttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(AttributeName::STRING_TABLE[*self as usize])
    }
}

impl std::fmt::Display for AttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(AttributeName::STRING_TABLE[*self as usize])
    }
}

