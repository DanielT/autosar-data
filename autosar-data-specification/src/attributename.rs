use crate::specification::hashfunc;

#[derive(Debug)]
/// The error type ParseAttributeNameError is returned when from_str() / parse() fails for AttributeName
pub struct ParseAttributeNameError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
/// Enum of all attribute names in Autosar
pub enum AttributeName {
    /// ACCESSKEY
    Accesskey               = 0,
    /// ALIGN
    Align                   = 1,
    /// ALLOW-BREAK
    AllowBreak              = 2,
    /// ALT
    Alt                     = 3,
    /// BASE
    Base                    = 4,
    /// BGCOLOR
    Bgcolor                 = 5,
    /// BINDING-TIME
    BindingTime             = 6,
    /// BLUEPRINT-VALUE
    BlueprintValue          = 7,
    /// BREAK
    Break                   = 8,
    /// CLASS
    Class                   = 9,
    /// COLNAME
    Colname                 = 10,
    /// COLNUM
    Colnum                  = 11,
    /// COLOR
    Color                   = 12,
    /// COLS
    Cols                    = 13,
    /// COLSEP
    Colsep                  = 14,
    /// COLWIDTH
    Colwidth                = 15,
    /// COORDS
    Coords                  = 16,
    /// DEST
    Dest                    = 17,
    /// EDIT-HEIGHT
    EditHeight              = 18,
    /// EDIT-WIDTH
    EditWidth               = 19,
    /// EDITFIT
    Editfit                 = 20,
    /// EDITSCALE
    Editscale               = 21,
    /// ENUM-TABLE
    EnumTable               = 22,
    /// FILENAME
    Filename                = 23,
    /// FIT
    Fit                     = 24,
    /// FLOAT
    Float                   = 25,
    /// FONT
    Font                    = 26,
    /// FRAME
    Frame                   = 27,
    /// GENERATOR
    Generator               = 28,
    /// GID
    Gid                     = 29,
    /// HEIGHT
    Height                  = 30,
    /// HELP-ENTRY
    HelpEntry               = 31,
    /// HREF
    Href                    = 32,
    /// HTML-FIT
    HtmlFit                 = 33,
    /// HTML-HEIGHT
    HtmlHeight              = 34,
    /// HTML-SCALE
    HtmlScale               = 35,
    /// HTML-WIDTH
    HtmlWidth               = 36,
    /// INDEX
    Index                   = 37,
    /// INTERVAL-TYPE
    IntervalType            = 38,
    /// ITEM-LABEL-POS
    ItemLabelPos            = 39,
    /// KEEP-WITH-PREVIOUS
    KeepWithPrevious        = 40,
    /// L
    L                       = 41,
    /// LEVEL
    Level                   = 42,
    /// MIME-TYPE
    MimeType                = 43,
    /// MOREROWS
    Morerows                = 44,
    /// NAME
    Name                    = 45,
    /// NAME-PATTERN
    NamePattern             = 46,
    /// NAMEEND
    Nameend                 = 47,
    /// NAMEST
    Namest                  = 48,
    /// NOHREF
    Nohref                  = 49,
    /// NOTATION
    Notation                = 50,
    /// NOTE-TYPE
    NoteType                = 51,
    /// ONBLUR
    Onblur                  = 52,
    /// ONCLICK
    Onclick                 = 53,
    /// ONDBLCLICK
    Ondblclick              = 54,
    /// ONFOCUS
    Onfocus                 = 55,
    /// ONKEYDOWN
    Onkeydown               = 56,
    /// ONKEYPRESS
    Onkeypress              = 57,
    /// ONKEYUP
    Onkeyup                 = 58,
    /// ONMOUSEDOWN
    Onmousedown             = 59,
    /// ONMOUSEMOVE
    Onmousemove             = 60,
    /// ONMOUSEOUT
    Onmouseout              = 61,
    /// ONMOUSEOVER
    Onmouseover             = 62,
    /// ONMOUSEUP
    Onmouseup               = 63,
    /// ORIENT
    Orient                  = 64,
    /// PGWIDE
    Pgwide                  = 65,
    /// RESOLUTION-POLICY
    ResolutionPolicy        = 66,
    /// ROTATE
    Rotate                  = 67,
    /// ROWSEP
    Rowsep                  = 68,
    /// S
    S                       = 69,
    /// SCALE
    Scale                   = 70,
    /// SD
    Sd                      = 71,
    /// SHAPE
    Shape                   = 72,
    /// SHORT-LABEL
    ShortLabel              = 73,
    /// SHOW-CONTENT
    ShowContent             = 74,
    /// SHOW-RESOURCE-ALIAS-NAME
    ShowResourceAliasName   = 75,
    /// SHOW-RESOURCE-CATEGORY
    ShowResourceCategory    = 76,
    /// SHOW-RESOURCE-LONG-NAME
    ShowResourceLongName    = 77,
    /// SHOW-RESOURCE-NUMBER
    ShowResourceNumber      = 78,
    /// SHOW-RESOURCE-PAGE
    ShowResourcePage        = 79,
    /// SHOW-RESOURCE-SHORT-NAME
    ShowResourceShortName   = 80,
    /// SHOW-RESOURCE-TYPE
    ShowResourceType        = 81,
    /// SHOW-SEE
    ShowSee                 = 82,
    /// SI
    Si                      = 83,
    /// SPANNAME
    Spanname                = 84,
    /// STYLE
    Style                   = 85,
    /// T
    T                       = 86,
    /// TABINDEX
    Tabindex                = 87,
    /// TABSTYLE
    Tabstyle                = 88,
    /// TEX-RENDER
    TexRender               = 89,
    /// TITLE
    Title                   = 90,
    /// TYPE
    Type                    = 91,
    /// UUID
    Uuid                    = 92,
    /// VALIDITY
    Validity                = 93,
    /// VALIGN
    Valign                  = 94,
    /// VIEW
    View                    = 95,
    /// WIDTH
    Width                   = 96,
    /// space
    space                   = 97,
    /// xmlns
    xmlns                   = 98,
    /// xmlns:xsi
    xmlnsXsi                = 99,
    /// xsi:schemaLocation
    xsiSchemalocation       = 100,
}

impl AttributeName {
    const STRING_TABLE: [&'static str; 101] = ["ACCESSKEY", "ALIGN", "ALLOW-BREAK", "ALT", "BASE", "BGCOLOR", "BINDING-TIME", "BLUEPRINT-VALUE", "BREAK", "CLASS", "COLNAME", "COLNUM", "COLOR", "COLS", "COLSEP", "COLWIDTH", "COORDS", "DEST", "EDIT-HEIGHT", "EDIT-WIDTH", "EDITFIT", "EDITSCALE", "ENUM-TABLE", "FILENAME", "FIT", "FLOAT", "FONT", "FRAME", "GENERATOR", "GID", "HEIGHT", "HELP-ENTRY", "HREF", "HTML-FIT", "HTML-HEIGHT", "HTML-SCALE", "HTML-WIDTH", "INDEX", "INTERVAL-TYPE", "ITEM-LABEL-POS", "KEEP-WITH-PREVIOUS", "L", "LEVEL", "MIME-TYPE", "MOREROWS", "NAME", "NAME-PATTERN", "NAMEEND", "NAMEST", "NOHREF", "NOTATION", "NOTE-TYPE", "ONBLUR", "ONCLICK", "ONDBLCLICK", "ONFOCUS", "ONKEYDOWN", "ONKEYPRESS", "ONKEYUP", "ONMOUSEDOWN", "ONMOUSEMOVE", "ONMOUSEOUT", "ONMOUSEOVER", "ONMOUSEUP", "ORIENT", "PGWIDE", "RESOLUTION-POLICY", "ROTATE", "ROWSEP", "S", "SCALE", "SD", "SHAPE", "SHORT-LABEL", "SHOW-CONTENT", "SHOW-RESOURCE-ALIAS-NAME", "SHOW-RESOURCE-CATEGORY", "SHOW-RESOURCE-LONG-NAME", "SHOW-RESOURCE-NUMBER", "SHOW-RESOURCE-PAGE", "SHOW-RESOURCE-SHORT-NAME", "SHOW-RESOURCE-TYPE", "SHOW-SEE", "SI", "SPANNAME", "STYLE", "T", "TABINDEX", "TABSTYLE", "TEX-RENDER", "TITLE", "TYPE", "UUID", "VALIDITY", "VALIGN", "VIEW", "WIDTH", "space", "xmlns", "xmlns:xsi", "xsi:schemaLocation"];
    const HASH_TABLE_1: [u16; 54] = [35, 26, 58, 46, 84, 83, 65535, 66, 47, 53, 26, 91, 19, 65535, 48, 97, 38, 6, 65535, 96, 25, 26, 40, 27, 75, 67, 77, 30, 23, 44, 88, 15, 78, 88, 76, 94, 78, 3, 35, 89, 62, 41, 41, 98, 21, 64, 99, 92, 90, 87, 50, 38, 25, 25];
    const HASH_TABLE_2: [u16; 54] = [67, 23, 65, 89, 81, 0, 67, 43, 38, 9, 56, 13, 72, 45, 65535, 92, 95, 57, 36, 57, 48, 37, 82, 61, 32, 58, 85, 69, 75, 0, 72, 96, 20, 87, 48, 15, 0, 65, 6, 77, 10, 82, 0, 50, 43, 59, 59, 8, 96, 97, 8, 44, 51, 75];

    // derive an enum entry from an input string using a perfect hash function
    // here, hashfunc(input, <param>) is an ordinary hash function which may produce collisions
    // it is possible to create two tables so that
    //     table1[hashfunc(input, param1)] + table2[hashfunc(input, param2)] == desired enumeration value
    // these tables are pre-built and included here as constants, since the values to be hashed don't change
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseAttributeNameError> {
        let hashval1: usize = hashfunc(input, 13929);
        let hashval2: usize = hashfunc(input, 17554);
        let val1 = AttributeName::HASH_TABLE_1[hashval1 % 54];
        let val2 = AttributeName::HASH_TABLE_2[hashval2 % 54];
        if val1 == u16::MAX || val2 == u16::MAX {
            return Err(ParseAttributeNameError);
        }
        let item_idx = (val1 + val2) as usize % 101;
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
