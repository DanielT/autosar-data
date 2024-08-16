use num_derive::FromPrimitive;
use num_traits::cast::FromPrimitive;

#[derive(Debug)]
/// Error type returned when `from_str()` / `parse()` for `AutosarVersion` fails
pub struct ParseAutosarVersionError;

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, FromPrimitive)]
#[repr(u32)]
#[non_exhaustive]
/// Enum of all Autosar versions
pub enum AutosarVersion {
    /// AUTOSAR 4.0.1 - xsd file name: `AUTOSAR_4-0-1.xsd`
    Autosar_4_0_1 = 0x1,
    /// AUTOSAR 4.0.2 - xsd file name: `AUTOSAR_4-0-2.xsd`
    Autosar_4_0_2 = 0x2,
    /// AUTOSAR 4.0.3 - xsd file name: `AUTOSAR_4-0-3.xsd`
    Autosar_4_0_3 = 0x4,
    /// AUTOSAR 4.1.1 - xsd file name: `AUTOSAR_4-1-1.xsd`
    Autosar_4_1_1 = 0x8,
    /// AUTOSAR 4.1.2 - xsd file name: `AUTOSAR_4-1-2.xsd`
    Autosar_4_1_2 = 0x10,
    /// AUTOSAR 4.1.3 - xsd file name: `AUTOSAR_4-1-3.xsd`
    Autosar_4_1_3 = 0x20,
    /// AUTOSAR 4.2.1 - xsd file name: `AUTOSAR_4-2-1.xsd`
    Autosar_4_2_1 = 0x40,
    /// AUTOSAR 4.2.2 - xsd file name: `AUTOSAR_4-2-2.xsd`
    Autosar_4_2_2 = 0x80,
    /// AUTOSAR 4.3.0 - xsd file name: `AUTOSAR_4-3-0.xsd`
    Autosar_4_3_0 = 0x100,
    /// AUTOSAR Adaptive 17-03 - xsd file name: `AUTOSAR_00042.xsd`
    Autosar_00042 = 0x200,
    /// AUTOSAR Adaptive 17-10 - xsd file name: `AUTOSAR_00043.xsd`
    Autosar_00043 = 0x400,
    /// AUTOSAR Classic 4.3.1 - xsd file name: `AUTOSAR_00044.xsd`
    Autosar_00044 = 0x800,
    /// AUTOSAR Adaptive 18-03 - xsd file name: `AUTOSAR_00045.xsd`
    Autosar_00045 = 0x1000,
    /// AUTOSAR Classic 4.4.0 / Adaptive 18-10 - xsd file name: `AUTOSAR_00046.xsd`
    Autosar_00046 = 0x2000,
    /// AUTOSAR Adaptive 19-03 - xsd file name: `AUTOSAR_00047.xsd`
    Autosar_00047 = 0x4000,
    /// AUTOSAR 4.5.0 - xsd file name: `AUTOSAR_00048.xsd`
    Autosar_00048 = 0x8000,
    /// AUTOSAR R20-11 - xsd file name: `AUTOSAR_00049.xsd`
    Autosar_00049 = 0x10000,
    /// AUTOSAR R21-11 - xsd file name: `AUTOSAR_00050.xsd`
    Autosar_00050 = 0x20000,
    /// AUTOSAR R22-11 - xsd file name: `AUTOSAR_00051.xsd`
    Autosar_00051 = 0x40000,
    /// AUTOSAR R23-11 - xsd file name: `AUTOSAR_00052.xsd`
    Autosar_00052 = 0x80000,
}

impl AutosarVersion {
    /// get the name of the xsd file matching the Autosar version
    #[must_use]
    pub fn filename(&self) -> &'static str {
        match self {
            Self::Autosar_4_0_1 => "AUTOSAR_4-0-1.xsd",
            Self::Autosar_4_0_2 => "AUTOSAR_4-0-2.xsd",
            Self::Autosar_4_0_3 => "AUTOSAR_4-0-3.xsd",
            Self::Autosar_4_1_1 => "AUTOSAR_4-1-1.xsd",
            Self::Autosar_4_1_2 => "AUTOSAR_4-1-2.xsd",
            Self::Autosar_4_1_3 => "AUTOSAR_4-1-3.xsd",
            Self::Autosar_4_2_1 => "AUTOSAR_4-2-1.xsd",
            Self::Autosar_4_2_2 => "AUTOSAR_4-2-2.xsd",
            Self::Autosar_4_3_0 => "AUTOSAR_4-3-0.xsd",
            Self::Autosar_00042 => "AUTOSAR_00042.xsd",
            Self::Autosar_00043 => "AUTOSAR_00043.xsd",
            Self::Autosar_00044 => "AUTOSAR_00044.xsd",
            Self::Autosar_00045 => "AUTOSAR_00045.xsd",
            Self::Autosar_00046 => "AUTOSAR_00046.xsd",
            Self::Autosar_00047 => "AUTOSAR_00047.xsd",
            Self::Autosar_00048 => "AUTOSAR_00048.xsd",
            Self::Autosar_00049 => "AUTOSAR_00049.xsd",
            Self::Autosar_00050 => "AUTOSAR_00050.xsd",
            Self::Autosar_00051 => "AUTOSAR_00051.xsd",
            Self::Autosar_00052 => "AUTOSAR_00052.xsd",
        }
    }

    /// Human readable description of the Autosar version
    ///
    /// This is particularly useful for the later versions, where the xsd files are just sequentially numbered.
    /// For example `Autosar_00050` -> "AUTOSAR R21-11"
    #[must_use]
    pub fn describe(&self) -> &'static str {
        match self {
            Self::Autosar_4_0_1 => "AUTOSAR 4.0.1",
            Self::Autosar_4_0_2 => "AUTOSAR 4.0.2",
            Self::Autosar_4_0_3 => "AUTOSAR 4.0.3",
            Self::Autosar_4_1_1 => "AUTOSAR 4.1.1",
            Self::Autosar_4_1_2 => "AUTOSAR 4.1.2",
            Self::Autosar_4_1_3 => "AUTOSAR 4.1.3",
            Self::Autosar_4_2_1 => "AUTOSAR 4.2.1",
            Self::Autosar_4_2_2 => "AUTOSAR 4.2.2",
            Self::Autosar_4_3_0 => "AUTOSAR 4.3.0",
            Self::Autosar_00042 => "AUTOSAR Adaptive 17-03",
            Self::Autosar_00043 => "AUTOSAR Adaptive 17-10",
            Self::Autosar_00044 => "AUTOSAR Classic 4.3.1",
            Self::Autosar_00045 => "AUTOSAR Adaptive 18-03",
            Self::Autosar_00046 => "AUTOSAR Classic 4.4.0 / Adaptive 18-10",
            Self::Autosar_00047 => "AUTOSAR Adaptive 19-03",
            Self::Autosar_00048 => "AUTOSAR 4.5.0",
            Self::Autosar_00049 => "AUTOSAR R20-11",
            Self::Autosar_00050 => "AUTOSAR R21-11",
            Self::Autosar_00051 => "AUTOSAR R22-11",
            Self::Autosar_00052 => "AUTOSAR R23-11",
        }
    }

    /// make an `AutosarVersion` from a u32 value
    ///
    /// All `AutosarVersion`s are associated with a power of two u32 value, for example `Autosar_4_3_0` == 0x100
    /// If the given value is a valid constant of `AutosarVersion`, the enum value will be returnd
    ///
    /// This is useful in order to decode version masks
    #[must_use]
    pub fn from_val(n: u32) -> Option<Self> {
        Self::from_u32(n)
    }

    /// `AutosarVersion::LATEST` is an alias of which ever is the latest version
    pub const LATEST: AutosarVersion = AutosarVersion::Autosar_00052;
}

impl std::str::FromStr for AutosarVersion {
    type Err = ParseAutosarVersionError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "AUTOSAR_4-0-1.xsd" => Ok(Self::Autosar_4_0_1),
            "AUTOSAR_4-0-2.xsd" => Ok(Self::Autosar_4_0_2),
            "AUTOSAR_4-0-3.xsd" => Ok(Self::Autosar_4_0_3),
            "AUTOSAR_4-1-1.xsd" => Ok(Self::Autosar_4_1_1),
            "AUTOSAR_4-1-2.xsd" => Ok(Self::Autosar_4_1_2),
            "AUTOSAR_4-1-3.xsd" => Ok(Self::Autosar_4_1_3),
            "AUTOSAR_4-2-1.xsd" => Ok(Self::Autosar_4_2_1),
            "AUTOSAR_4-2-2.xsd" => Ok(Self::Autosar_4_2_2),
            "AUTOSAR_4-3-0.xsd" => Ok(Self::Autosar_4_3_0),
            "AUTOSAR_00042.xsd" => Ok(Self::Autosar_00042),
            "AUTOSAR_00043.xsd" => Ok(Self::Autosar_00043),
            "AUTOSAR_00044.xsd" => Ok(Self::Autosar_00044),
            "AUTOSAR_00045.xsd" => Ok(Self::Autosar_00045),
            "AUTOSAR_00046.xsd" => Ok(Self::Autosar_00046),
            "AUTOSAR_00047.xsd" => Ok(Self::Autosar_00047),
            "AUTOSAR_00048.xsd" => Ok(Self::Autosar_00048),
            "AUTOSAR_00049.xsd" => Ok(Self::Autosar_00049),
            "AUTOSAR_00050.xsd" => Ok(Self::Autosar_00050),
            "AUTOSAR_00051.xsd" => Ok(Self::Autosar_00051),
            "AUTOSAR_00052.xsd" => Ok(Self::Autosar_00052),

            _ => Err(ParseAutosarVersionError),
        }
    }
}

impl std::fmt::Display for AutosarVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.describe())
    }
}
