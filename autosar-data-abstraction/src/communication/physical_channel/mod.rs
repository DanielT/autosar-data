use crate::{AbstractionElement, AutosarAbstractionError, EcuInstance};
use autosar_data::{Element, ElementName};

mod can;
mod ethernet;
mod flexray;

pub use can::*;
pub use ethernet::*;
pub use flexray::*;

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalChannel {
    Can(CanPhysicalChannel),
    Ethernet(EthernetPhysicalChannel),
    FlexRay(FlexrayPhysicalChannel),
}

impl PhysicalChannel {
    // get the connector element between this channel and an ecu
    pub(crate) fn get_ecu_connector(&self, ecu_instance: &EcuInstance) -> Option<Element> {
        // get a connector referenced by this physical channel which is contained in the ecu_instance
        // iterate over the CommunicationConnectorRefConditionals
        for ccrc in self
            .element()
            .get_sub_element(ElementName::CommConnectors)?
            .sub_elements()
        {
            // check the ecu instance of each referenced communication connector
            if let Some(comm_connector) = ccrc
                .get_sub_element(ElementName::CommunicationConnectorRef)
                .and_then(|ccr| ccr.get_reference_target().ok())
            {
                if let Some(ecu_of_connector) = comm_connector.named_parent().ok().flatten() {
                    if &ecu_of_connector == ecu_instance.element() {
                        return Some(comm_connector);
                    }
                }
            }
        }
        None
    }
}

impl AbstractionElement for PhysicalChannel {
    fn element(&self) -> &autosar_data::Element {
        match self {
            PhysicalChannel::Can(cpc) => cpc.element(),
            PhysicalChannel::Ethernet(epc) => epc.element(),
            PhysicalChannel::FlexRay(fpc) => fpc.element(),
        }
    }
}

impl From<CanPhysicalChannel> for PhysicalChannel {
    fn from(value: CanPhysicalChannel) -> Self {
        PhysicalChannel::Can(value)
    }
}

impl From<EthernetPhysicalChannel> for PhysicalChannel {
    fn from(value: EthernetPhysicalChannel) -> Self {
        PhysicalChannel::Ethernet(value)
    }
}

impl From<FlexrayPhysicalChannel> for PhysicalChannel {
    fn from(value: FlexrayPhysicalChannel) -> Self {
        PhysicalChannel::FlexRay(value)
    }
}

impl TryFrom<Element> for PhysicalChannel {
    type Error = AutosarAbstractionError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        match element.element_name() {
            ElementName::CanPhysicalChannel => Ok(CanPhysicalChannel::try_from(element)?.into()),
            ElementName::EthernetPhysicalChannel => Ok(EthernetPhysicalChannel::try_from(element)?.into()),
            ElementName::FlexrayPhysicalChannel => Ok(FlexrayPhysicalChannel::try_from(element)?.into()),
            _ => Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "PhysicalChannel".to_string(),
            }),
        }
    }
}
