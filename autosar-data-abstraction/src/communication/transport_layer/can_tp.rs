use crate::communication::{CanCluster, CanCommunicationConnector, NPdu, Pdu};
use crate::{abstraction_element, AbstractionElement, ArPackage, AutosarAbstractionError, EcuInstance};
use autosar_data::{Element, ElementName, EnumItem};

//#########################################################

/// Container for CanTp configuration
///
/// There should be one CanTpConfig for each CAN network in the system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanTpConfig(Element);
abstraction_element!(CanTpConfig, CanTpConfig);

impl CanTpConfig {
    pub(crate) fn new(name: &str, package: &ArPackage, cluster: &CanCluster) -> Result<Self, AutosarAbstractionError> {
        let pkg_elem = package.element().get_or_create_sub_element(ElementName::Elements)?;

        let tp_config_elem = pkg_elem.create_named_sub_element(ElementName::CanTpConfig, name)?;
        tp_config_elem
            .create_sub_element(ElementName::CommunicationClusterRef)?
            .set_reference_target(cluster.element())?;

        Ok(Self(tp_config_elem))
    }

    /// add a CanTp ECU to the configuration
    pub fn add_ecu(&self, tp_ecu: CanTpEcu) -> Result<(), AutosarAbstractionError> {
        let ecu_collection = self.element().get_or_create_sub_element(ElementName::TpEcus)?;
        let ecu_elem = ecu_collection.create_sub_element(ElementName::CanTpEcu)?;
        tp_ecu.set(&ecu_elem)?;

        Ok(())
    }

    /// get all of the ECUs in the configuration
    pub fn ecus(&self) -> impl Iterator<Item = CanTpEcu> {
        self.element()
            .get_sub_element(ElementName::TpEcus)
            .into_iter()
            .flat_map(|ecu_collection| {
                ecu_collection
                    .sub_elements()
                    .filter_map(|tp_ecu_elem| CanTpEcu::get(&tp_ecu_elem))
            })
    }

    /// create a new CanTpAddress in the configuration
    pub fn create_address(&self, name: &str, address: u32) -> Result<CanTpAddress, AutosarAbstractionError> {
        let addresses_container = self.element().get_or_create_sub_element(ElementName::TpAddresss)?;
        CanTpAddress::new(name, &addresses_container, address)
    }

    /// get all of the Can Tp addresses in the configuration
    pub fn addresses(&self) -> impl Iterator<Item = CanTpAddress> {
        self.element()
            .get_sub_element(ElementName::TpAddresss)
            .into_iter()
            .flat_map(|addresses_container| {
                addresses_container
                    .sub_elements()
                    .filter_map(|address_elem| CanTpAddress::try_from(address_elem).ok())
            })
    }

    /// create a new CanTpChannel in the configuration
    pub fn create_can_tp_channel(
        &self,
        name: &str,
        channel_id: u32,
        mode: CanTpChannelMode,
    ) -> Result<CanTpChannel, AutosarAbstractionError> {
        let channels_container = self.element().get_or_create_sub_element(ElementName::TpChannels)?;
        CanTpChannel::new(name, &channels_container, channel_id, mode)
    }

    /// create a new CanTpConnection in the configuration
    pub fn create_can_tp_connection(
        &self,
        name: Option<&str>,
        addressing_format: CanTpAddressingFormat,
        can_tp_channel: &CanTpChannel,
        data_pdu: &NPdu,
        tp_sdu: &Pdu,
        padding_activation: bool,
    ) -> Result<CanTpConnection, AutosarAbstractionError> {
        let connections_container = self.element().get_or_create_sub_element(ElementName::TpConnections)?;
        CanTpConnection::new(
            name,
            &connections_container,
            addressing_format,
            can_tp_channel,
            data_pdu,
            tp_sdu,
            padding_activation,
        )
    }

    /// get all of the CanTpConnections in the configuration
    pub fn connections(&self) -> impl Iterator<Item = CanTpConnection> {
        self.element()
            .get_sub_element(ElementName::TpConnections)
            .into_iter()
            .flat_map(|connections_container| {
                connections_container
                    .sub_elements()
                    .filter_map(|connection_elem| CanTpConnection::try_from(connection_elem).ok())
            })
    }

    /// create a new CanTpNode in the configuration
    pub fn create_can_tp_node(&self, name: &str) -> Result<CanTpNode, AutosarAbstractionError> {
        let nodes_container = self.element().get_or_create_sub_element(ElementName::TpNodes)?;
        CanTpNode::new(name, &nodes_container)
    }

    /// get all of the CanTpNodes in the configuration
    pub fn nodes(&self) -> impl Iterator<Item = CanTpNode> {
        self.element()
            .get_sub_element(ElementName::TpNodes)
            .into_iter()
            .flat_map(|nodes_container| {
                nodes_container
                    .sub_elements()
                    .filter_map(|node_elem| CanTpNode::try_from(node_elem).ok())
            })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanTpEcu {
    pub ecu_instance: EcuInstance,
    pub cycle_time_main_function: Option<f64>,
}

impl CanTpEcu {
    fn get(element: &Element) -> Option<Self> {
        let ecu_instance = element
            .get_sub_element(ElementName::EcuInstanceRef)?
            .get_reference_target()
            .ok()?;
        let cycle_time_main_function = element
            .get_sub_element(ElementName::CycleTimeMainFunction)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_float());

        Some(Self {
            ecu_instance: EcuInstance::try_from(ecu_instance).unwrap(),
            cycle_time_main_function,
        })
    }

    fn set(&self, element: &Element) -> Result<(), AutosarAbstractionError> {
        element
            .get_or_create_sub_element(ElementName::EcuInstanceRef)?
            .set_reference_target(self.ecu_instance.element())?;

        if let Some(cycle_time) = self.cycle_time_main_function {
            element
                .get_or_create_sub_element(ElementName::CycleTimeMainFunction)?
                .set_character_data(cycle_time.to_string())?;
        }

        Ok(())
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanTpAddress(Element);
abstraction_element!(CanTpAddress, CanTpAddress);

impl CanTpAddress {
    pub(crate) fn new(name: &str, parent: &Element, tp_address: u32) -> Result<Self, AutosarAbstractionError> {
        let address_elem = parent.create_named_sub_element(ElementName::CanTpAddress, name)?;
        address_elem
            .create_sub_element(ElementName::TpAddress)?
            .set_character_data(tp_address as u64)?;
        Ok(Self(address_elem))
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanTpChannel(Element);
abstraction_element!(CanTpChannel, CanTpChannel);

impl CanTpChannel {
    pub(crate) fn new(
        name: &str,
        parent: &Element,
        channel_id: u32,
        mode: CanTpChannelMode,
    ) -> Result<Self, AutosarAbstractionError> {
        let channel_elem = parent.create_named_sub_element(ElementName::CanTpChannel, name)?;
        channel_elem
            .create_sub_element(ElementName::ChannelId)?
            .set_character_data(channel_id as u64)?;
        channel_elem
            .create_sub_element(ElementName::ChannelMode)?
            .set_character_data::<EnumItem>(mode.into())?;
        Ok(Self(channel_elem))
    }

    pub fn channel_id(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::ChannelId)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_integer())
    }

    pub fn channel_mode(&self) -> Option<CanTpChannelMode> {
        self.element()
            .get_sub_element(ElementName::ChannelMode)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.enum_value())
            .and_then(|enumitem| enumitem.try_into().ok())
    }
}

//#########################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanTpChannelMode {
    FullDuplex,
    HalfDuplex,
}

impl From<CanTpChannelMode> for EnumItem {
    fn from(mode: CanTpChannelMode) -> Self {
        match mode {
            CanTpChannelMode::FullDuplex => EnumItem::FullDuplexMode,
            CanTpChannelMode::HalfDuplex => EnumItem::HalfDuplexMode,
        }
    }
}

impl TryFrom<EnumItem> for CanTpChannelMode {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<Self, Self::Error> {
        match value {
            EnumItem::FullDuplexMode => Ok(CanTpChannelMode::FullDuplex),
            EnumItem::HalfDuplexMode => Ok(CanTpChannelMode::HalfDuplex),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "CanTpChannelMode".to_string(),
            }),
        }
    }
}

//#########################################################

/// A connection identifies the sender and the receiver of this particular communication.
/// The CanTp module routes a Pdu through this connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanTpConnection(Element);
abstraction_element!(CanTpConnection, CanTpConnection);

impl CanTpConnection {
    pub(crate) fn new(
        name: Option<&str>,
        parent: &Element,
        addressing_format: CanTpAddressingFormat,
        can_tp_channel: &CanTpChannel,
        data_pdu: &NPdu,
        tp_sdu: &Pdu,
        padding_activation: bool,
    ) -> Result<Self, AutosarAbstractionError> {
        let connection_elem = parent.create_sub_element(ElementName::CanTpConnection)?;
        if let Some(name) = name {
            connection_elem.create_named_sub_element(ElementName::Ident, name)?;
        }
        connection_elem
            .create_sub_element(ElementName::AddressingFormat)?
            .set_character_data::<EnumItem>(addressing_format.into())?;
        connection_elem
            .create_sub_element(ElementName::CanTpChannelRef)?
            .set_reference_target(can_tp_channel.element())?;
        connection_elem
            .create_sub_element(ElementName::DataPduRef)?
            .set_reference_target(data_pdu.element())?;
        connection_elem
            .create_sub_element(ElementName::TpSduRef)?
            .set_reference_target(tp_sdu.element())?;
        connection_elem
            .create_sub_element(ElementName::PaddingActivation)?
            .set_character_data(padding_activation)?;

        Ok(Self(connection_elem))
    }

    /// get the name of the connection
    ///
    /// In early versions of the Autosar standard, CanTpConnection was not identifiable.
    /// This was fixed later by adding the Ident sub-element. This method returns the name
    /// provied in the Ident element, if it exists.
    pub fn name(&self) -> Option<String> {
        self.element()
            .get_sub_element(ElementName::Ident)
            .and_then(|elem| elem.item_name())
    }

    /// get the CanTpChannel associated with this connection
    pub fn channel(&self) -> Option<CanTpChannel> {
        self.element()
            .get_sub_element(ElementName::CanTpChannelRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|target| CanTpChannel::try_from(target).ok())
    }

    /// get the NPdu associated with this connection
    ///
    /// This is the Pdu that is sent over the CAN network
    pub fn data_pdu(&self) -> Option<NPdu> {
        self.element()
            .get_sub_element(ElementName::DataPduRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|target| NPdu::try_from(target).ok())
    }

    /// get the IPdu associated with this connection
    ///
    /// This is the Pdu that is sent over the transport protocol
    pub fn tp_sdu(&self) -> Option<Pdu> {
        self.element()
            .get_sub_element(ElementName::TpSduRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|target| Pdu::try_from(target).ok())
    }

    /// get the addressing format of the connection
    pub fn addressing_format(&self) -> Option<CanTpAddressingFormat> {
        self.element()
            .get_sub_element(ElementName::AddressingFormat)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.enum_value())
            .and_then(|enumitem| enumitem.try_into().ok())
    }

    /// get the padding activation of the connection
    pub fn padding_activation(&self) -> Option<bool> {
        self.element()
            .get_sub_element(ElementName::PaddingActivation)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_bool())
    }

    /// set the name of the connection
    pub fn set_name(&self, name: &str) -> Result<(), AutosarAbstractionError> {
        if let Some(ident_elem) = self.element().get_sub_element(ElementName::Ident) {
            ident_elem.set_item_name(name)?;
        } else {
            self.element().create_named_sub_element(ElementName::Ident, name)?;
        }
        Ok(())
    }

    /// set the CanTpChannel associated with this connection
    pub fn set_channel(&self, channel: &CanTpChannel) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::CanTpChannelRef)?
            .set_reference_target(channel.element())?;
        Ok(())
    }

    /// set the NPdu associated with this connection
    pub fn set_data_pdu(&self, data_pdu: &NPdu) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::DataPduRef)?
            .set_reference_target(data_pdu.element())?;
        Ok(())
    }

    /// set the IPdu associated with this connection
    pub fn set_tp_sdu(&self, tp_sdu: &Pdu) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::TpSduRef)?
            .set_reference_target(tp_sdu.element())?;
        Ok(())
    }

    /// set the addressing format of the connection
    pub fn set_addressing_format(
        &self,
        addressing_format: CanTpAddressingFormat,
    ) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::AddressingFormat)?
            .set_character_data::<EnumItem>(addressing_format.into())?;
        Ok(())
    }

    /// set the padding activation of the connection
    pub fn set_padding_activation(&self, padding_activation: bool) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::PaddingActivation)?
            .set_character_data(padding_activation)?;
        Ok(())
    }

    /// set the transmitter of the connection
    ///
    /// This is a CanTpNode representing an ECU that will send the data
    pub fn set_transmitter(&self, transmitter: &CanTpNode) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::TransmitterRef)?
            .set_reference_target(transmitter.element())?;
        Ok(())
    }

    /// get the transmitter of the connection
    pub fn transmitter(&self) -> Option<CanTpNode> {
        self.element()
            .get_sub_element(ElementName::TransmitterRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|target| CanTpNode::try_from(target).ok())
    }

    /// add a receiver to the connection
    ///
    /// This is a CanTpNode representing an ECU that will receive the data
    pub fn add_receiver(&self, receiver: &CanTpNode) -> Result<(), AutosarAbstractionError> {
        let receivers = self.element().get_or_create_sub_element(ElementName::ReceiverRefs)?;
        let receiver_ref_elem = receivers.create_sub_element(ElementName::ReceiverRef)?;
        receiver_ref_elem.set_reference_target(receiver.element())?;
        Ok(())
    }

    /// get all of the receivers of the connection
    pub fn receivers(&self) -> impl Iterator<Item = CanTpNode> {
        self.element()
            .get_sub_element(ElementName::ReceiverRefs)
            .into_iter()
            .flat_map(|receivers| {
                receivers.sub_elements().filter_map(|receiver_ref_elem| {
                    receiver_ref_elem
                        .get_reference_target()
                        .ok()
                        .and_then(|target| CanTpNode::try_from(target).ok())
                })
            })
    }
}

//#########################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanTpAddressingFormat {
    Extended,
    Mixed,
    Mixed29Bit,
    NormalFixed,
    Standard,
}

impl From<CanTpAddressingFormat> for EnumItem {
    fn from(format: CanTpAddressingFormat) -> Self {
        match format {
            CanTpAddressingFormat::Extended => EnumItem::Extended,
            CanTpAddressingFormat::Mixed => EnumItem::Mixed,
            CanTpAddressingFormat::Mixed29Bit => EnumItem::Mixed29Bit,
            CanTpAddressingFormat::NormalFixed => EnumItem::Normalfixed,
            CanTpAddressingFormat::Standard => EnumItem::Standard,
        }
    }
}

impl TryFrom<EnumItem> for CanTpAddressingFormat {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<Self, Self::Error> {
        match value {
            EnumItem::Extended => Ok(CanTpAddressingFormat::Extended),
            EnumItem::Mixed => Ok(CanTpAddressingFormat::Mixed),
            EnumItem::Mixed29Bit => Ok(CanTpAddressingFormat::Mixed29Bit),
            EnumItem::Normalfixed => Ok(CanTpAddressingFormat::NormalFixed),
            EnumItem::Standard => Ok(CanTpAddressingFormat::Standard),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "CanTpAddressingFormat".to_string(),
            }),
        }
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanTpNode(Element);
abstraction_element!(CanTpNode, CanTpNode);

impl CanTpNode {
    pub(crate) fn new(name: &str, parent: &Element) -> Result<Self, AutosarAbstractionError> {
        let node_elem = parent.create_named_sub_element(ElementName::CanTpNode, name)?;
        Ok(Self(node_elem))
    }

    /// set the CanTpAddress of this Node
    pub fn set_address(&self, address: &CanTpAddress) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::TpAddressRef)?
            .set_reference_target(address.element())?;
        Ok(())
    }

    /// get the CanTpAddress of this Node
    pub fn address(&self) -> Option<CanTpAddress> {
        self.element()
            .get_sub_element(ElementName::TpAddressRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|target| CanTpAddress::try_from(target).ok())
    }

    /// set the reference to a CanCommunicationConnector of an ECU and a CanPhysicalChannel
    ///
    /// The connector connects the ECU to the physical channel, so by setting this reference, the
    /// ECU is also connected to the CanTpNode
    pub fn set_connector(&self, connector: &CanCommunicationConnector) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::ConnectorRef)?
            .set_reference_target(connector.element())?;
        Ok(())
    }

    /// get the CanCommunicationConnector of this Node
    pub fn connector(&self) -> Option<CanCommunicationConnector> {
        self.element()
            .get_sub_element(ElementName::ConnectorRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|target| CanCommunicationConnector::try_from(target).ok())
    }
}

//#########################################################

#[cfg(test)]
mod test {
    use super::*;
    use crate::{communication::CanClusterSettings, System, SystemCategory};
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn can_transport_protocol() {
        let model = AutosarModel::new();
        let _file = model.create_file("CanTp.arxml", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();

        let system = System::new("system", &package, SystemCategory::EcuExtract).unwrap();
        let can_cluster = system
            .create_can_cluster("can_cluster", &package, &CanClusterSettings::default())
            .unwrap();
        let can_channel = can_cluster.create_physical_channel("can_channel").unwrap();
        let ecu_instance = system.create_ecu_instance("ecu_instance", &package).unwrap();
        let communication_controller = ecu_instance.create_can_communication_controller("can_ctrl").unwrap();
        let connector = communication_controller
            .connect_physical_channel("name", &can_channel)
            .unwrap();

        let can_tp_config = CanTpConfig::new("can_tp_config", &package, &can_cluster).unwrap();
        can_tp_config
            .add_ecu(CanTpEcu {
                ecu_instance,
                cycle_time_main_function: None,
            })
            .unwrap();

        let address = can_tp_config.create_address("address", 0x1234).unwrap();
        let channel = can_tp_config
            .create_can_tp_channel("channel", 1, CanTpChannelMode::FullDuplex)
            .unwrap();

        let data_pdu = system.create_n_pdu("data_pdu", &package, 8).unwrap();
        let tp_sdu = system.create_dcm_ipdu("ipdu", &package, 4096).unwrap();

        let connection = can_tp_config
            .create_can_tp_connection(
                Some("connection"),
                CanTpAddressingFormat::Standard,
                &channel,
                &data_pdu,
                &tp_sdu.into(),
                false,
            )
            .unwrap();

        let node = can_tp_config.create_can_tp_node("node").unwrap();
        node.set_address(&address).unwrap();
        node.set_connector(&connector).unwrap();
        assert_eq!(node.address().unwrap(), address);
        assert_eq!(node.connector().unwrap(), connector);
        connection.set_transmitter(&node).unwrap();
        assert_eq!(connection.transmitter().unwrap(), node);

        connection.add_receiver(&node).unwrap();
        assert_eq!(connection.receivers().count(), 1);
    }
}
