use crate::AbstractionElement;

mod can;
mod ethernet;
mod flexray;

pub use can::*;
pub use ethernet::*;
pub use flexray::*;

//##################################################################

/// wraps all different kinds of communication controller
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CommunicationController {
    Can(CanCommunicationController),
    Ethernet(EthernetCommunicationController),
    Flexray(FlexrayCommunicationController),
}

impl AbstractionElement for CommunicationController {
    fn element(&self) -> &autosar_data::Element {
        match self {
            CommunicationController::Can(ccc) => ccc.element(),
            CommunicationController::Ethernet(ecc) => ecc.element(),
            CommunicationController::Flexray(fcc) => fcc.element(),
        }
    }
}

impl From<CanCommunicationController> for CommunicationController {
    fn from(value: CanCommunicationController) -> Self {
        CommunicationController::Can(value)
    }
}

impl From<EthernetCommunicationController> for CommunicationController {
    fn from(value: EthernetCommunicationController) -> Self {
        CommunicationController::Ethernet(value)
    }
}

impl From<FlexrayCommunicationController> for CommunicationController {
    fn from(value: FlexrayCommunicationController) -> Self {
        CommunicationController::Flexray(value)
    }
}
