use crate::hashfunc;

#[derive(Debug)]
/// The error type ParseAttributeNameError is returned when from_str() / parse() fails for AttributeName
pub struct ParseAttributeNameError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
/// Enum of all attribute names in Autosar
pub enum AttributeName {
    /// NOHREF
    Nohref                  = 0,
    /// ONMOUSEOUT
    Onmouseout              = 1,
    /// xmlns:xsi
    xmlnsXsi                = 2,
    /// MOREROWS
    Morerows                = 3,
    /// SPANNAME
    Spanname                = 4,
    /// TITLE
    Title                   = 5,
    /// DEST
    Dest                    = 6,
    /// COLNUM
    Colnum                  = 7,
    /// CLASS
    Class                   = 8,
    /// SHOW-RESOURCE-SHORT-NAME
    ShowResourceShortName   = 9,
    /// SHOW-RESOURCE-NUMBER
    ShowResourceNumber      = 10,
    /// SHORT-LABEL
    ShortLabel              = 11,
    /// ONDBLCLICK
    Ondblclick              = 12,
    /// ROWSEP
    Rowsep                  = 13,
    /// FRAME
    Frame                   = 14,
    /// ALLOW-BREAK
    AllowBreak              = 15,
    /// HEIGHT
    Height                  = 16,
    /// ONMOUSEDOWN
    Onmousedown             = 17,
    /// BINDING-TIME
    BindingTime             = 18,
    /// NAMEST
    Namest                  = 19,
    /// EDIT-WIDTH
    EditWidth               = 20,
    /// EDITFIT
    Editfit                 = 21,
    /// BREAK
    Break                   = 22,
    /// BLUEPRINT-VALUE
    BlueprintValue          = 23,
    /// VIEW
    View                    = 24,
    /// COORDS
    Coords                  = 25,
    /// COLOR
    Color                   = 26,
    /// SHOW-RESOURCE-CATEGORY
    ShowResourceCategory    = 27,
    /// xmlns
    xmlns                   = 28,
    /// PGWIDE
    Pgwide                  = 29,
    /// ORIENT
    Orient                  = 30,
    /// FILENAME
    Filename                = 31,
    /// INDEX
    Index                   = 32,
    /// NAME
    Name                    = 33,
    /// TYPE
    Type                    = 34,
    /// ENUM-TABLE
    EnumTable               = 35,
    /// ONBLUR
    Onblur                  = 36,
    /// ONKEYUP
    Onkeyup                 = 37,
    /// SHAPE
    Shape                   = 38,
    /// ONMOUSEUP
    Onmouseup               = 39,
    /// SHOW-RESOURCE-TYPE
    ShowResourceType        = 40,
    /// BGCOLOR
    Bgcolor                 = 41,
    /// FLOAT
    Float                   = 42,
    /// L
    L                       = 43,
    /// NOTE-TYPE
    NoteType                = 44,
    /// STYLE
    Style                   = 45,
    /// ALT
    Alt                     = 46,
    /// SHOW-CONTENT
    ShowContent             = 47,
    /// TABSTYLE
    Tabstyle                = 48,
    /// VALIGN
    Valign                  = 49,
    /// LEVEL
    Level                   = 50,
    /// GID
    Gid                     = 51,
    /// S
    S                       = 52,
    /// SHOW-RESOURCE-LONG-NAME
    ShowResourceLongName    = 53,
    /// KEEP-WITH-PREVIOUS
    KeepWithPrevious        = 54,
    /// HTML-FIT
    HtmlFit                 = 55,
    /// ONMOUSEOVER
    Onmouseover             = 56,
    /// ROTATE
    Rotate                  = 57,
    /// HELP-ENTRY
    HelpEntry               = 58,
    /// UUID
    Uuid                    = 59,
    /// INTERVAL-TYPE
    IntervalType            = 60,
    /// FONT
    Font                    = 61,
    /// WIDTH
    Width                   = 62,
    /// GENERATOR
    Generator               = 63,
    /// space
    space                   = 64,
    /// SHOW-RESOURCE-PAGE
    ShowResourcePage        = 65,
    /// TEX-RENDER
    TexRender               = 66,
    /// HREF
    Href                    = 67,
    /// FIT
    Fit                     = 68,
    /// HTML-WIDTH
    HtmlWidth               = 69,
    /// ITEM-LABEL-POS
    ItemLabelPos            = 70,
    /// ONMOUSEMOVE
    Onmousemove             = 71,
    /// ALIGN
    Align                   = 72,
    /// ONKEYPRESS
    Onkeypress              = 73,
    /// COLNAME
    Colname                 = 74,
    /// SHOW-RESOURCE-ALIAS-NAME
    ShowResourceAliasName   = 75,
    /// SCALE
    Scale                   = 76,
    /// SI
    Si                      = 77,
    /// xsi:schemaLocation
    xsiSchemalocation       = 78,
    /// ACCESSKEY
    Accesskey               = 79,
    /// T
    T                       = 80,
    /// NAMEEND
    Nameend                 = 81,
    /// BASE
    Base                    = 82,
    /// EDITSCALE
    Editscale               = 83,
    /// VALIDITY
    Validity                = 84,
    /// HTML-SCALE
    HtmlScale               = 85,
    /// COLSEP
    Colsep                  = 86,
    /// NOTATION
    Notation                = 87,
    /// COLS
    Cols                    = 88,
    /// NAME-PATTERN
    NamePattern             = 89,
    /// EDIT-HEIGHT
    EditHeight              = 90,
    /// RESOLUTION-POLICY
    ResolutionPolicy        = 91,
    /// SD
    Sd                      = 92,
    /// TABINDEX
    Tabindex                = 93,
    /// COLWIDTH
    Colwidth                = 94,
    /// ONKEYDOWN
    Onkeydown               = 95,
    /// MIME-TYPE
    MimeType                = 96,
    /// HTML-HEIGHT
    HtmlHeight              = 97,
    /// ONCLICK
    Onclick                 = 98,
    /// SHOW-SEE
    ShowSee                 = 99,
    /// ONFOCUS
    Onfocus                 = 100,
}

impl AttributeName {
    const STRING_TABLE: [&'static str; 101] = ["NOHREF", "ONMOUSEOUT", "xmlns:xsi", "MOREROWS", "SPANNAME", "TITLE", "DEST", "COLNUM", "CLASS", "SHOW-RESOURCE-SHORT-NAME", "SHOW-RESOURCE-NUMBER", "SHORT-LABEL", "ONDBLCLICK", "ROWSEP", "FRAME", "ALLOW-BREAK", "HEIGHT", "ONMOUSEDOWN", "BINDING-TIME", "NAMEST", "EDIT-WIDTH", "EDITFIT", "BREAK", "BLUEPRINT-VALUE", "VIEW", "COORDS", "COLOR", "SHOW-RESOURCE-CATEGORY", "xmlns", "PGWIDE", "ORIENT", "FILENAME", "INDEX", "NAME", "TYPE", "ENUM-TABLE", "ONBLUR", "ONKEYUP", "SHAPE", "ONMOUSEUP", "SHOW-RESOURCE-TYPE", "BGCOLOR", "FLOAT", "L", "NOTE-TYPE", "STYLE", "ALT", "SHOW-CONTENT", "TABSTYLE", "VALIGN", "LEVEL", "GID", "S", "SHOW-RESOURCE-LONG-NAME", "KEEP-WITH-PREVIOUS", "HTML-FIT", "ONMOUSEOVER", "ROTATE", "HELP-ENTRY", "UUID", "INTERVAL-TYPE", "FONT", "WIDTH", "GENERATOR", "space", "SHOW-RESOURCE-PAGE", "TEX-RENDER", "HREF", "FIT", "HTML-WIDTH", "ITEM-LABEL-POS", "ONMOUSEMOVE", "ALIGN", "ONKEYPRESS", "COLNAME", "SHOW-RESOURCE-ALIAS-NAME", "SCALE", "SI", "xsi:schemaLocation", "ACCESSKEY", "T", "NAMEEND", "BASE", "EDITSCALE", "VALIDITY", "HTML-SCALE", "COLSEP", "NOTATION", "COLS", "NAME-PATTERN", "EDIT-HEIGHT", "RESOLUTION-POLICY", "SD", "TABINDEX", "COLWIDTH", "ONKEYDOWN", "MIME-TYPE", "HTML-HEIGHT", "ONCLICK", "SHOW-SEE", "ONFOCUS"];

    /// derive an enum entry from an input string using a perfect hash function
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseAttributeNameError> {
        static DISPLACEMENTS: [(u16, u16); 21] = [(0, 0), (0, 1), (0, 21), (2, 17), (0, 52), (0, 46), (0, 29), (0, 6), (0, 4), (1, 42), (0, 57), (0, 82), (0, 64), (0, 21), (12, 82), (1, 87), (22, 29), (26, 57), (0, 0), (58, 36), (28, 81)];
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

