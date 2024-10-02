use crate::communication::{CommunicationDirection, ISignal, ISignalGroup, ISignalTriggering, PhysicalChannel};
use crate::{
    abstraction_element, element_iterator, make_unique_name, reflist_iterator, AbstractionElement, ArPackage,
    AutosarAbstractionError, EcuInstance,
};
use autosar_data::{AutosarDataError, Element, ElementName, EnumItem};

mod isignal_idpu;

pub use isignal_idpu::*;

//##################################################################

/// Network Management Pdu
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NmPdu(Element);
abstraction_element!(NmPdu, NmPdu);

impl NmPdu {
    pub(crate) fn new(name: &str, package: &ArPackage, length: u32) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_pdu = pkg_elements.create_named_sub_element(ElementName::NmPdu, name)?;
        elem_pdu
            .create_sub_element(ElementName::Length)?
            .set_character_data(length.to_string())?;

        Ok(Self(elem_pdu))
    }

    pub fn length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    pub fn pdu_triggerings(&self) -> impl Iterator<Item = PduTriggering> {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            PduTriggeringsIterator::new(reflist)
        } else {
            PduTriggeringsIterator::new(vec![])
        }
    }
}

impl From<NmPdu> for Pdu {
    fn from(value: NmPdu) -> Self {
        Pdu::NmPdu(value)
    }
}

//##################################################################

/// This is a Pdu of the transport layer. The main purpose of the TP layer is to segment and reassemble IPdus.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NPdu(Element);
abstraction_element!(NPdu, NPdu);

impl NPdu {
    pub(crate) fn new(name: &str, package: &ArPackage, length: u32) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_pdu = pkg_elements.create_named_sub_element(ElementName::NPdu, name)?;
        elem_pdu
            .create_sub_element(ElementName::Length)?
            .set_character_data(length.to_string())?;

        Ok(Self(elem_pdu))
    }

    pub fn length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    pub fn pdu_triggerings(&self) -> impl Iterator<Item = PduTriggering> {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            PduTriggeringsIterator::new(reflist)
        } else {
            PduTriggeringsIterator::new(vec![])
        }
    }
}

impl From<NPdu> for Pdu {
    fn from(value: NPdu) -> Self {
        Pdu::NPdu(value)
    }
}

//##################################################################

/// Represents the IPdus handled by Dcm
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DcmIPdu(Element);
abstraction_element!(DcmIPdu, DcmIPdu);

impl DcmIPdu {
    pub(crate) fn new(name: &str, package: &ArPackage, length: u32) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_pdu = pkg_elements.create_named_sub_element(ElementName::DcmIPdu, name)?;
        elem_pdu
            .create_sub_element(ElementName::Length)?
            .set_character_data(length.to_string())?;

        Ok(Self(elem_pdu))
    }

    pub fn length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    pub fn pdu_triggerings(&self) -> impl Iterator<Item = PduTriggering> {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            PduTriggeringsIterator::new(reflist)
        } else {
            PduTriggeringsIterator::new(vec![])
        }
    }
}

impl From<DcmIPdu> for Pdu {
    fn from(value: DcmIPdu) -> Self {
        Pdu::DcmIPdu(value)
    }
}

//##################################################################

/// This element is used for AUTOSAR Pdus without additional attributes that are routed by a bus interface
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneralPurposePdu(Element);
abstraction_element!(GeneralPurposePdu, GeneralPurposePdu);

impl GeneralPurposePdu {
    pub(crate) fn new(name: &str, package: &ArPackage, length: u32) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_pdu = pkg_elements.create_named_sub_element(ElementName::GeneralPurposePdu, name)?;
        elem_pdu
            .create_sub_element(ElementName::Length)?
            .set_character_data(length.to_string())?;

        Ok(Self(elem_pdu))
    }

    pub fn length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    pub fn pdu_triggerings(&self) -> impl Iterator<Item = PduTriggering> {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            PduTriggeringsIterator::new(reflist)
        } else {
            PduTriggeringsIterator::new(vec![])
        }
    }
}

impl From<GeneralPurposePdu> for Pdu {
    fn from(value: GeneralPurposePdu) -> Self {
        Pdu::GeneralPurposePdu(value)
    }
}

//##################################################################

/// This element is used for AUTOSAR Pdus without attributes that are routed by the PduR
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneralPurposeIPdu(Element);
abstraction_element!(GeneralPurposeIPdu, GeneralPurposeIPdu);

impl GeneralPurposeIPdu {
    pub(crate) fn new(name: &str, package: &ArPackage, length: u32) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_pdu = pkg_elements.create_named_sub_element(ElementName::GeneralPurposeIPdu, name)?;
        elem_pdu
            .create_sub_element(ElementName::Length)?
            .set_character_data(length.to_string())?;

        Ok(Self(elem_pdu))
    }

    pub fn length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    pub fn pdu_triggerings(&self) -> impl Iterator<Item = PduTriggering> {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            PduTriggeringsIterator::new(reflist)
        } else {
            PduTriggeringsIterator::new(vec![])
        }
    }
}

impl From<GeneralPurposeIPdu> for Pdu {
    fn from(value: GeneralPurposeIPdu) -> Self {
        Pdu::GeneralPurposeIPdu(value)
    }
}

//##################################################################

/// Several IPdus can be collected in one ContainerIPdu based on the headerType
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerIPdu(Element);
abstraction_element!(ContainerIPdu, ContainerIPdu);

impl ContainerIPdu {
    pub(crate) fn new(name: &str, package: &ArPackage, length: u32) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_pdu = pkg_elements.create_named_sub_element(ElementName::ContainerIPdu, name)?;
        elem_pdu
            .create_sub_element(ElementName::Length)?
            .set_character_data(length.to_string())?;

        Ok(Self(elem_pdu))
    }

    pub fn length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    pub fn pdu_triggerings(&self) -> impl Iterator<Item = PduTriggering> {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            PduTriggeringsIterator::new(reflist)
        } else {
            PduTriggeringsIterator::new(vec![])
        }
    }
}

impl From<ContainerIPdu> for Pdu {
    fn from(value: ContainerIPdu) -> Self {
        Pdu::ContainerIPdu(value)
    }
}

//##################################################################

/// Wraps an IPdu to protect it from unauthorized manipulation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecuredIPdu(Element);
abstraction_element!(SecuredIPdu, SecuredIPdu);

impl SecuredIPdu {
    pub(crate) fn new(name: &str, package: &ArPackage, length: u32) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_pdu = pkg_elements.create_named_sub_element(ElementName::SecuredIPdu, name)?;
        elem_pdu
            .create_sub_element(ElementName::Length)?
            .set_character_data(length.to_string())?;

        Ok(Self(elem_pdu))
    }

    pub fn length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    pub fn pdu_triggerings(&self) -> impl Iterator<Item = PduTriggering> {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            PduTriggeringsIterator::new(reflist)
        } else {
            PduTriggeringsIterator::new(vec![])
        }
    }
}

impl From<SecuredIPdu> for Pdu {
    fn from(value: SecuredIPdu) -> Self {
        Pdu::SecuredIPdu(value)
    }
}

//##################################################################

/// The multiplexed pdu contains one of serveral signal pdus
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultiplexedIPdu(Element);
abstraction_element!(MultiplexedIPdu, MultiplexedIPdu);

impl MultiplexedIPdu {
    pub(crate) fn new(name: &str, package: &ArPackage, length: u32) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_pdu = pkg_elements.create_named_sub_element(ElementName::MultiplexedIPdu, name)?;
        elem_pdu
            .create_sub_element(ElementName::Length)?
            .set_character_data(length.to_string())?;

        Ok(Self(elem_pdu))
    }

    pub fn length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    pub fn pdu_triggerings(&self) -> impl Iterator<Item = PduTriggering> {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            PduTriggeringsIterator::new(reflist)
        } else {
            PduTriggeringsIterator::new(vec![])
        }
    }
}

impl From<MultiplexedIPdu> for Pdu {
    fn from(value: MultiplexedIPdu) -> Self {
        Pdu::MultiplexedIPdu(value)
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pdu {
    ISignalIPdu(ISignalIPdu),
    NmPdu(NmPdu),
    NPdu(NPdu),
    DcmIPdu(DcmIPdu),
    GeneralPurposePdu(GeneralPurposePdu),
    GeneralPurposeIPdu(GeneralPurposeIPdu),
    ContainerIPdu(ContainerIPdu),
    SecuredIPdu(SecuredIPdu),
    MultiplexedIPdu(MultiplexedIPdu),
}

impl AbstractionElement for Pdu {
    fn element(&self) -> &Element {
        match self {
            Pdu::ISignalIPdu(pdu) => pdu.element(),
            Pdu::NmPdu(pdu) => pdu.element(),
            Pdu::NPdu(pdu) => pdu.element(),
            Pdu::DcmIPdu(pdu) => pdu.element(),
            Pdu::GeneralPurposePdu(pdu) => pdu.element(),
            Pdu::GeneralPurposeIPdu(pdu) => pdu.element(),
            Pdu::ContainerIPdu(pdu) => pdu.element(),
            Pdu::SecuredIPdu(pdu) => pdu.element(),
            Pdu::MultiplexedIPdu(pdu) => pdu.element(),
        }
    }
}

impl TryFrom<Element> for Pdu {
    type Error = AutosarAbstractionError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        match element.element_name() {
            ElementName::ISignalIPdu => Ok(ISignalIPdu::try_from(element)?.into()),
            ElementName::NmPdu => Ok(NmPdu::try_from(element)?.into()),
            ElementName::NPdu => Ok(NPdu::try_from(element)?.into()),
            ElementName::DcmIPdu => Ok(DcmIPdu::try_from(element)?.into()),
            ElementName::GeneralPurposePdu => Ok(GeneralPurposePdu::try_from(element)?.into()),
            ElementName::GeneralPurposeIPdu => Ok(GeneralPurposeIPdu::try_from(element)?.into()),
            ElementName::ContainerIPdu => Ok(ContainerIPdu::try_from(element)?.into()),
            ElementName::SecuredIPdu => Ok(SecuredIPdu::try_from(element)?.into()),
            ElementName::MultiplexedIPdu => Ok(MultiplexedIPdu::try_from(element)?.into()),
            _ => Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "Pdu".to_string(),
            }),
        }
    }
}

impl Pdu {
    pub fn length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }
}

//##################################################################

/// a PduTriggering triggers a PDU in a frame or ethernet connection
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PduTriggering(Element);
abstraction_element!(PduTriggering, PduTriggering);

impl PduTriggering {
    pub(crate) fn new(pdu: &Pdu, channel: &PhysicalChannel) -> Result<Self, AutosarAbstractionError> {
        let model = channel.element().model()?;
        let base_path = channel.element().path()?;
        let pdu_name = pdu
            .name()
            .ok_or(AutosarAbstractionError::InvalidParameter("invalid pdu".to_string()))?;
        let pt_name = format!("PT_{pdu_name}");
        let pt_name = make_unique_name(&model, base_path, pt_name);

        let triggerings = channel
            .element()
            .get_or_create_sub_element(ElementName::PduTriggerings)?;
        let pt_elem = triggerings.create_named_sub_element(ElementName::PduTriggering, &pt_name)?;
        pt_elem
            .create_sub_element(ElementName::IPduRef)?
            .set_reference_target(pdu.element())?;

        let pt = Self(pt_elem);

        if let Pdu::ISignalIPdu(isignal_ipdu) = pdu {
            for signal_mapping in isignal_ipdu.mapped_signals() {
                if let Some(signal) = signal_mapping.signal() {
                    pt.add_signal_triggering(&signal)?;
                } else if let Some(signal_group) = signal_mapping.signal_group() {
                    pt.add_signal_group_triggering(&signal_group)?;
                }
            }
        }

        Ok(pt)
    }

    /// get the Pdu that is triggered by this pdu triggering
    pub fn pdu(&self) -> Option<Pdu> {
        let pdu_elem = self
            .element()
            .get_sub_element(ElementName::IPduRef)?
            .get_reference_target()
            .ok()?;
        Pdu::try_from(pdu_elem).ok()
    }

    /// get the physical channel that contains this pdu triggering
    pub fn physical_channel(&self) -> Result<PhysicalChannel, AutosarAbstractionError> {
        let channel_elem = self.element().named_parent()?.ok_or(AutosarDataError::ItemDeleted)?;
        PhysicalChannel::try_from(channel_elem)
    }

    /// create an IPduPort to connect a PduTriggering to an EcuInstance
    pub fn create_pdu_port(
        &self,
        ecu: &EcuInstance,
        direction: CommunicationDirection,
    ) -> Result<IPduPort, AutosarAbstractionError> {
        for pdu_port in self.pdu_ports() {
            if let (Some(existing_ecu), Some(existing_direction)) = (pdu_port.ecu(), pdu_port.communication_direction())
            {
                if existing_ecu == *ecu && existing_direction == direction {
                    return Ok(pdu_port);
                }
            }
        }

        let channel = self.physical_channel()?;
        let connector = channel
            .get_ecu_connector(ecu)
            .ok_or(AutosarAbstractionError::InvalidParameter(
                "The ECU is not connected to the channel".to_string(),
            ))?;

        let name = self.name().ok_or(AutosarDataError::ItemDeleted)?;
        let suffix = match direction {
            CommunicationDirection::In => "Rx",
            CommunicationDirection::Out => "Tx",
        };
        let port_name = format!("{name}_{suffix}",);
        let pp_elem = connector
            .get_or_create_sub_element(ElementName::EcuCommPortInstances)?
            .create_named_sub_element(ElementName::IPduPort, &port_name)?;
        pp_elem
            .create_sub_element(ElementName::CommunicationDirection)?
            .set_character_data::<EnumItem>(direction.into())?;

        self.element()
            .get_or_create_sub_element(ElementName::IPduPortRefs)?
            .create_sub_element(ElementName::IPduPortRef)?
            .set_reference_target(&pp_elem)?;

        for st in self.signal_triggerings() {
            st.connect_to_ecu(ecu, direction)?;
        }

        Ok(IPduPort(pp_elem))
    }

    /// create an iterator over the IPduPorts that are connected to this PduTriggering
    pub fn pdu_ports(&self) -> IPduPortIterator {
        IPduPortIterator::new(self.element().get_sub_element(ElementName::IPduPortRefs))
    }

    /// create an iterator over the ISignalTriggerings that are triggered by this PduTriggering
    pub fn signal_triggerings(&self) -> PtSignalTriggeringsIterator {
        PtSignalTriggeringsIterator::new(self.element().get_sub_element(ElementName::ISignalTriggerings))
    }

    /// add a signal triggering for a signal to this PduTriggering
    pub fn add_signal_triggering(&self, signal: &ISignal) -> Result<ISignalTriggering, AutosarAbstractionError> {
        let channel = self.physical_channel()?;
        let st = ISignalTriggering::new(signal, &channel)?;
        let triggerings = self
            .element()
            .get_or_create_sub_element(ElementName::ISignalTriggerings)?;
        triggerings
            .create_sub_element(ElementName::ISignalTriggeringRefConditional)?
            .create_sub_element(ElementName::ISignalTriggeringRef)?
            .set_reference_target(st.element())?;

        for pdu_port in self.pdu_ports() {
            if let (Some(ecu), Some(direction)) = (pdu_port.ecu(), pdu_port.communication_direction()) {
                st.connect_to_ecu(&ecu, direction)?;
            }
        }

        Ok(st)
    }

    /// add a signal triggering for a signal group to this PduTriggering
    pub fn add_signal_group_triggering(
        &self,
        signal_group: &ISignalGroup,
    ) -> Result<ISignalTriggering, AutosarAbstractionError> {
        let channel = self.physical_channel()?;
        let st = ISignalTriggering::new_group(signal_group, &channel)?;
        let triggerings = self
            .element()
            .get_or_create_sub_element(ElementName::ISignalTriggerings)?;
        triggerings
            .create_sub_element(ElementName::ISignalTriggeringRefConditional)?
            .create_sub_element(ElementName::ISignalTriggeringRef)?
            .set_reference_target(st.element())?;

        for pdu_port in self.pdu_ports() {
            if let (Some(ecu), Some(direction)) = (pdu_port.ecu(), pdu_port.communication_direction()) {
                st.connect_to_ecu(&ecu, direction)?;
            }
        }

        Ok(st)
    }
}

//##################################################################

/// The IPduPort allows an ECU to send or receive a PDU
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IPduPort(Element);
abstraction_element!(IPduPort, IPduPort);

impl IPduPort {
    pub fn ecu(&self) -> Option<EcuInstance> {
        let comm_connector_elem = self.element().named_parent().ok()??;
        let ecu_elem = comm_connector_elem.named_parent().ok()??;
        EcuInstance::try_from(ecu_elem).ok()
    }

    pub fn communication_direction(&self) -> Option<CommunicationDirection> {
        self.element()
            .get_sub_element(ElementName::CommunicationDirection)?
            .character_data()?
            .enum_value()?
            .try_into()
            .ok()
    }
}

//##################################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PduCollectionTrigger {
    Always,
    Never,
}

impl From<PduCollectionTrigger> for EnumItem {
    fn from(value: PduCollectionTrigger) -> Self {
        match value {
            PduCollectionTrigger::Always => EnumItem::Always,
            PduCollectionTrigger::Never => EnumItem::Never,
        }
    }
}

impl TryFrom<EnumItem> for PduCollectionTrigger {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<Self, Self::Error> {
        match value {
            EnumItem::Always => Ok(PduCollectionTrigger::Always),
            EnumItem::Never => Ok(PduCollectionTrigger::Never),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "PduCollectionTrigger".to_string(),
            }),
        }
    }
}

//##################################################################

element_iterator!(
    IPduPortIterator,
    IPduPort,
    (|element: Element| element.get_reference_target().ok())
);

//##################################################################

reflist_iterator!(PduTriggeringsIterator, PduTriggering);

//##################################################################

element_iterator!(
    PtSignalTriggeringsIterator,
    ISignalTriggering,
    (|element: Element| element
        .get_sub_element(ElementName::ISignalTriggeringRef)
        .and_then(|str| str.get_reference_target().ok()))
);
