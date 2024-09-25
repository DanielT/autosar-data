use crate::*;
use autosar_data::ElementName;

mod clientserver;
mod senderreceiver;

pub use clientserver::*;
pub use senderreceiver::*;

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModeSwitchInterface(Element);
abstraction_element!(ModeSwitchInterface, ModeSwitchInterface);

impl ModeSwitchInterface {
    /// Create a new ModeSwitchInterface
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let mode_switch_interface = elements.create_named_sub_element(ElementName::ModeSwitchInterface, name)?;

        Ok(Self(mode_switch_interface))
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParameterInterface(Element);
abstraction_element!(ParameterInterface, ParameterInterface);

impl ParameterInterface {
    /// Create a new ParameterInterface
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let parameter_interface = elements.create_named_sub_element(ElementName::ParameterInterface, name)?;

        Ok(Self(parameter_interface))
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NvDataInterface(Element);
abstraction_element!(NvDataInterface, NvDataInterface);

impl NvDataInterface {
    /// Create a new NvDataInterface
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let nv_data_interface = elements.create_named_sub_element(ElementName::NvDataInterface, name)?;

        Ok(Self(nv_data_interface))
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TriggerInterface(Element);
abstraction_element!(TriggerInterface, TriggerInterface);

impl TriggerInterface {
    /// Create a new TriggerInterface
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let trigger_interface = elements.create_named_sub_element(ElementName::TriggerInterface, name)?;

        Ok(Self(trigger_interface))
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PortInterface {
    SenderReceiverInterface(SenderReceiverInterface),
    ClientServerInterface(ClientServerInterface),
    ModeSwitchInterface(ModeSwitchInterface),
    ParameterInterface(ParameterInterface),
    NvDataInterface(NvDataInterface),
    TriggerInterface(TriggerInterface),
}

impl AbstractionElement for PortInterface {
    fn element(&self) -> &Element {
        match self {
            PortInterface::SenderReceiverInterface(sender_receiver_interface) => sender_receiver_interface.element(),
            PortInterface::ClientServerInterface(client_server_interface) => client_server_interface.element(),
            PortInterface::ModeSwitchInterface(mode_switch_interface) => mode_switch_interface.element(),
            PortInterface::ParameterInterface(parameter_interface) => parameter_interface.element(),
            PortInterface::NvDataInterface(nv_data_interface) => nv_data_interface.element(),
            PortInterface::TriggerInterface(trigger_interface) => trigger_interface.element(),
        }
    }
}

impl From<SenderReceiverInterface> for PortInterface {
    fn from(sender_receiver_interface: SenderReceiverInterface) -> Self {
        PortInterface::SenderReceiverInterface(sender_receiver_interface)
    }
}

impl From<ClientServerInterface> for PortInterface {
    fn from(client_server_interface: ClientServerInterface) -> Self {
        PortInterface::ClientServerInterface(client_server_interface)
    }
}

impl From<ModeSwitchInterface> for PortInterface {
    fn from(mode_switch_interface: ModeSwitchInterface) -> Self {
        PortInterface::ModeSwitchInterface(mode_switch_interface)
    }
}

impl From<ParameterInterface> for PortInterface {
    fn from(parameter_interface: ParameterInterface) -> Self {
        PortInterface::ParameterInterface(parameter_interface)
    }
}

impl From<NvDataInterface> for PortInterface {
    fn from(nv_data_interface: NvDataInterface) -> Self {
        PortInterface::NvDataInterface(nv_data_interface)
    }
}

impl From<TriggerInterface> for PortInterface {
    fn from(trigger_interface: TriggerInterface) -> Self {
        PortInterface::TriggerInterface(trigger_interface)
    }
}

impl TryFrom<Element> for PortInterface {
    type Error = AutosarAbstractionError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        match element.element_name() {
            ElementName::SenderReceiverInterface => {
                Ok(PortInterface::SenderReceiverInterface(SenderReceiverInterface(element)))
            }
            ElementName::ClientServerInterface => {
                Ok(PortInterface::ClientServerInterface(ClientServerInterface(element)))
            }
            ElementName::ModeSwitchInterface => Ok(PortInterface::ModeSwitchInterface(ModeSwitchInterface(element))),
            ElementName::ParameterInterface => Ok(PortInterface::ParameterInterface(ParameterInterface(element))),
            ElementName::NvDataInterface => Ok(PortInterface::NvDataInterface(NvDataInterface(element))),
            ElementName::TriggerInterface => Ok(PortInterface::TriggerInterface(TriggerInterface(element))),
            _ => Err(AutosarAbstractionError::ConversionError {
                element: element,
                dest: "PortInterface".to_string(),
            }),
        }
    }
}
