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

//##################################################################

#[cfg(test)]
mod tests {
    use crate::{ArPackage, EcuInstance};
    use super::*;
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn test_communication_controller() {
        let model = AutosarModel::new();
        let _file = model.create_file("test.arxml", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/test").unwrap();
        let ecu = EcuInstance::new("ecu", &package).unwrap();
        let can = CanCommunicationController::new("can", &ecu).unwrap();
        let ethernet = EthernetCommunicationController::new("ethernet", &ecu, None).unwrap();
        let flexray = FlexrayCommunicationController::new("flexray", &ecu).unwrap();

        let can_cc: CommunicationController = can.into();
        let ethernet_cc: CommunicationController = ethernet.into();
        let flexray_cc: CommunicationController = flexray.into();

        assert_eq!(can_cc.element().item_name().unwrap(), "can");
        assert_eq!(ethernet_cc.element().item_name().unwrap(), "ethernet");
        assert_eq!(flexray_cc.element().item_name().unwrap(), "flexray");
    }
}
