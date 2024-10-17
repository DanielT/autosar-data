use crate::AutosarAbstractionError;
use autosar_data::EnumItem;

mod cluster;
mod controller;
mod datatransformation;
mod frame;
mod pdu;
mod physical_channel;
mod signal;

pub use cluster::*;
pub use controller::*;
pub use datatransformation::*;
pub use frame::*;
pub use pdu::*;
pub use physical_channel::*;
pub use signal::*;

//#########################################################

/// The [`CommunicationDirection`] is used by the communication ports for frames, PDUs and signals
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommunicationDirection {
    In,
    Out,
}

impl TryFrom<EnumItem> for CommunicationDirection {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<Self, Self::Error> {
        match value {
            EnumItem::In => Ok(CommunicationDirection::In),
            EnumItem::Out => Ok(CommunicationDirection::Out),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "CommunicationDirection".to_string(),
            }),
        }
    }
}

impl From<CommunicationDirection> for EnumItem {
    fn from(value: CommunicationDirection) -> Self {
        match value {
            CommunicationDirection::In => EnumItem::In,
            CommunicationDirection::Out => EnumItem::Out,
        }
    }
}
