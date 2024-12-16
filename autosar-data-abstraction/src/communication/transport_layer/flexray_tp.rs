use crate::communication::{FlexrayCluster, FlexrayCommunicationConnector, NPdu, Pdu, TpAddress};
use crate::{abstraction_element, AbstractionElement, ArPackage, AutosarAbstractionError, EcuInstance};
use autosar_data::{Element, ElementName};

/// FlexrayTpConfig defines exactly one FlexRay ISO TP Configuration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlexrayTpConfig(Element);
abstraction_element!(FlexrayTpConfig, FlexrayTpConfig);

impl FlexrayTpConfig {
    pub(crate) fn new(
        name: &str,
        package: &ArPackage,
        cluster: &FlexrayCluster,
    ) -> Result<Self, AutosarAbstractionError> {
        let pkg_elem = package.element().get_or_create_sub_element(ElementName::Elements)?;

        let tp_config_elem = pkg_elem.create_named_sub_element(ElementName::FlexrayTpConfig, name)?;
        tp_config_elem
            .create_sub_element(ElementName::CommunicationClusterRef)?
            .set_reference_target(cluster.element())?;

        Ok(Self(tp_config_elem))
    }

    /// create a new FlexrayTpPduPool
    pub fn create_flexray_tp_pdu_pool(&self, name: &str) -> Result<FlexrayTpPduPool, AutosarAbstractionError> {
        let pdu_pools_elem = self.element().get_or_create_sub_element(ElementName::PduPools)?;
        FlexrayTpPduPool::new(name, &pdu_pools_elem)
    }

    /// iterate over all FlexrayTpPduPools
    pub fn flexray_tp_pdu_pools(&self) -> impl Iterator<Item = FlexrayTpPduPool> {
        self.element()
            .get_sub_element(ElementName::PduPools)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .map(FlexrayTpPduPool)
    }

    /// create a new TpAddress
    pub fn create_tp_address(&self, name: &str, address: u32) -> Result<TpAddress, AutosarAbstractionError> {
        let tp_addresses_elem = self.element().get_or_create_sub_element(ElementName::TpAddresss)?;
        TpAddress::new(name, &tp_addresses_elem, address)
    }

    /// iterate over all TpAddresses
    pub fn tp_addresses(&self) -> impl Iterator<Item = TpAddress> {
        self.element()
            .get_sub_element(ElementName::TpAddresss)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .filter_map(|elem| elem.try_into().ok())
    }

    /// create a new FlexrayTpConnection
    pub fn create_flexray_tp_connection(
        &self,
        name: Option<&str>,
        transmitter: &FlexrayTpNode,
        direct_tp_sdu: &Pdu,
        connection_control: &FlexrayTpConnectionControl,
    ) -> Result<FlexrayTpConnection, AutosarAbstractionError> {
        let tp_connections_elem = self.element().get_or_create_sub_element(ElementName::TpConnections)?;
        FlexrayTpConnection::new(
            name,
            &tp_connections_elem,
            transmitter,
            direct_tp_sdu,
            connection_control,
        )
    }

    /// iterate over all FlexrayTpConnections
    pub fn flexray_tp_connections(&self) -> impl Iterator<Item = FlexrayTpConnection> {
        self.element()
            .get_sub_element(ElementName::TpConnections)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .map(FlexrayTpConnection)
    }

    /// create a new FlexrayTpConnectionControl
    pub fn create_flexray_tp_connection_control(
        &self,
        name: &str,
    ) -> Result<FlexrayTpConnectionControl, AutosarAbstractionError> {
        let connection_controls_elem = self
            .element()
            .get_or_create_sub_element(ElementName::TpConnectionControls)?;
        FlexrayTpConnectionControl::new(name, &connection_controls_elem)
    }

    /// iterate over all FlexrayTpConnectionControls
    pub fn flexray_tp_connection_controls(&self) -> impl Iterator<Item = FlexrayTpConnectionControl> {
        self.element()
            .get_sub_element(ElementName::TpConnectionControls)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .map(FlexrayTpConnectionControl)
    }

    /// create a new FlexrayTpEcu
    pub fn create_flexray_tp_ecu(
        &self,
        name: &str,
        ecu_instance: &EcuInstance,
        full_duplex_enabled: bool,
    ) -> Result<FlexrayTpEcu, AutosarAbstractionError> {
        let ecus_elem = self.element().get_or_create_sub_element(ElementName::TpEcus)?;
        FlexrayTpEcu::new(name, &ecus_elem, ecu_instance, full_duplex_enabled)
    }

    /// iterate over all FlexrayTpEcus
    pub fn flexray_tp_ecus(&self) -> impl Iterator<Item = FlexrayTpEcu> {
        self.element()
            .get_sub_element(ElementName::TpEcus)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .map(FlexrayTpEcu)
    }

    /// create a new FlexrayTpNode
    pub fn create_flexray_tp_node(&self, name: &str) -> Result<FlexrayTpNode, AutosarAbstractionError> {
        let nodes_elem = self.element().get_or_create_sub_element(ElementName::TpNodes)?;
        FlexrayTpNode::new(name, &nodes_elem)
    }

    /// iterate over all FlexrayTpNodes
    pub fn flexray_tp_nodes(&self) -> impl Iterator<Item = FlexrayTpNode> {
        self.element()
            .get_sub_element(ElementName::TpNodes)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .map(FlexrayTpNode)
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlexrayTpPduPool(Element);
abstraction_element!(FlexrayTpPduPool, FlexrayTpPduPool);

impl FlexrayTpPduPool {
    pub(crate) fn new(name: &str, parent: &Element) -> Result<Self, AutosarAbstractionError> {
        let pdu_pool_elem = parent.create_named_sub_element(ElementName::FlexrayTpPduPool, name)?;
        Ok(Self(pdu_pool_elem))
    }

    /// add an NPdu to the PduPool
    pub fn add_npdu(&self, n_pdu: &NPdu) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::NPduRefs)?
            .create_sub_element(ElementName::NPduRef)?
            .set_reference_target(n_pdu.element())?;
        Ok(())
    }

    /// iterate over all referenced NPdus
    pub fn npdus(&self) -> impl Iterator<Item = NPdu> {
        self.element()
            .get_sub_element(ElementName::NPduRefs)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .filter_map(|ref_elem| {
                ref_elem
                    .get_reference_target()
                    .ok()
                    .and_then(|elem| elem.try_into().ok())
            })
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlexrayTpConnection(Element);
abstraction_element!(FlexrayTpConnection, FlexrayTpConnection);

impl FlexrayTpConnection {
    pub(crate) fn new(
        name: Option<&str>,
        parent: &Element,
        transmitter: &FlexrayTpNode,
        direct_tp_sdu: &Pdu,
        connection_control: &FlexrayTpConnectionControl,
    ) -> Result<Self, AutosarAbstractionError> {
        let connection_elem = parent.create_sub_element(ElementName::FlexrayTpConnection)?;
        if let Some(name) = name {
            connection_elem.create_named_sub_element(ElementName::Ident, name)?;
        }
        connection_elem
            .create_sub_element(ElementName::TransmitterRef)?
            .set_reference_target(transmitter.element())?;
        connection_elem
            .create_sub_element(ElementName::DirectTpSduRef)?
            .set_reference_target(direct_tp_sdu.element())?;
        connection_elem
            .create_sub_element(ElementName::TpConnectionControlRef)?
            .set_reference_target(connection_control.element())?;
        Ok(Self(connection_elem))
    }

    /// get the transmitter of the connection
    pub fn transmitter(&self) -> Option<FlexrayTpNode> {
        self.element()
            .get_sub_element(ElementName::TransmitterRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| elem.try_into().ok())
    }

    /// get the direct TP SDU of the connection
    pub fn direct_tp_sdu(&self) -> Option<Pdu> {
        self.element()
            .get_sub_element(ElementName::DirectTpSduRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| elem.try_into().ok())
    }

    /// get the connection control of the connection
    pub fn connection_control(&self) -> Option<FlexrayTpConnectionControl> {
        self.element()
            .get_sub_element(ElementName::TpConnectionControlRef)?
            .try_into()
            .ok()
    }

    /// add a receiver to the connection
    pub fn add_receiver(&self, receiver: &FlexrayTpNode) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::ReceiverRefs)?
            .create_sub_element(ElementName::ReceiverRef)?
            .set_reference_target(receiver.element())?;
        Ok(())
    }

    /// iterate over all receivers of the connection
    pub fn receivers(&self) -> impl Iterator<Item = FlexrayTpNode> {
        self.element()
            .get_sub_element(ElementName::ReceiverRefs)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .filter_map(|ref_elem| {
                ref_elem
                    .get_reference_target()
                    .ok()
                    .and_then(|elem| elem.try_into().ok())
            })
    }

    /// set the reversed TP SDU of the connection
    /// This is used if the connection supports both sending and receiving
    pub fn set_reversed_tp_sdu(&self, reversed_tp_sdu: &Pdu) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::ReversedTpSduRef)?
            .set_reference_target(reversed_tp_sdu.element())?;
        Ok(())
    }

    /// get the reversed TP SDU of the connection
    pub fn reversed_tp_sdu(&self) -> Option<Pdu> {
        self.element()
            .get_sub_element(ElementName::ReversedTpSduRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| elem.try_into().ok())
    }

    /// set the TX FlexrayTpPduPool of the connection
    pub fn set_tx_pdu_pool(&self, tx_pdu_pool: &FlexrayTpPduPool) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::TxPduPoolRef)?
            .set_reference_target(tx_pdu_pool.element())?;
        Ok(())
    }

    /// get the TX FlexrayTpPduPool of the connection
    pub fn tx_pdu_pool(&self) -> Option<FlexrayTpPduPool> {
        self.element()
            .get_sub_element(ElementName::TxPduPoolRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| elem.try_into().ok())
    }

    /// set the RX FlexrayTpPduPool of the connection
    pub fn set_rx_pdu_pool(&self, rx_pdu_pool: &FlexrayTpPduPool) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::RxPduPoolRef)?
            .set_reference_target(rx_pdu_pool.element())?;
        Ok(())
    }

    /// get the RX FlexrayTpPduPool of the connection
    pub fn rx_pdu_pool(&self) -> Option<FlexrayTpPduPool> {
        self.element()
            .get_sub_element(ElementName::RxPduPoolRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| elem.try_into().ok())
    }

    /// set the multicast TpAddress of the connection
    /// This element is optional; setting None will remove the element
    pub fn set_multicast_address(&self, multicast_address: Option<&TpAddress>) -> Result<(), AutosarAbstractionError> {
        if let Some(multicast_address) = multicast_address {
            // add or update the multicast address
            self.element()
                .get_or_create_sub_element(ElementName::MulticastRef)?
                .set_reference_target(multicast_address.element())?;
        } else {
            // remove the multicast address
            if let Some(multicast_elem) = self.element().get_sub_element(ElementName::MulticastRef) {
                self.element().remove_sub_element(multicast_elem)?;
            }
        }
        Ok(())
    }

    /// get the multicast TpAddress of the connection
    pub fn multicast_address(&self) -> Option<TpAddress> {
        self.element()
            .get_sub_element(ElementName::MulticastRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| elem.try_into().ok())
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlexrayTpConnectionControl(Element);
abstraction_element!(FlexrayTpConnectionControl, FlexrayTpConnectionControl);

impl FlexrayTpConnectionControl {
    pub(crate) fn new(name: &str, parent: &Element) -> Result<Self, AutosarAbstractionError> {
        let connection_control_elem = parent.create_named_sub_element(ElementName::FlexrayTpConnectionControl, name)?;
        Ok(Self(connection_control_elem))
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlexrayTpEcu(Element);
abstraction_element!(FlexrayTpEcu, FlexrayTpEcu);

impl FlexrayTpEcu {
    pub(crate) fn new(
        name: &str,
        parent: &Element,
        ecu_instance: &EcuInstance,
        full_duplex_enabled: bool,
    ) -> Result<Self, AutosarAbstractionError> {
        let ecu_elem = parent.create_named_sub_element(ElementName::FlexrayTpEcu, name)?;
        ecu_elem
            .create_sub_element(ElementName::EcuInstanceRef)?
            .set_reference_target(ecu_instance.element())?;
        ecu_elem
            .create_sub_element(ElementName::FullDuplexEnabled)?
            .set_character_data(full_duplex_enabled)?;
        Ok(Self(ecu_elem))
    }

    /// get the EcuInstance of the FlexrayTpEcu
    pub fn ecu_instance(&self) -> Option<EcuInstance> {
        self.element()
            .get_sub_element(ElementName::EcuInstanceRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| elem.try_into().ok())
    }

    /// get the full duplex enabled flag of the FlexrayTpEcu
    pub fn full_duplex_enabled(&self) -> Option<bool> {
        self.element()
            .get_sub_element(ElementName::FullDuplexEnabled)?
            .character_data()?
            .parse_bool()
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlexrayTpNode(Element);
abstraction_element!(FlexrayTpNode, FlexrayTpNode);

impl FlexrayTpNode {
    pub(crate) fn new(name: &str, parent: &Element) -> Result<Self, AutosarAbstractionError> {
        let node_elem = parent.create_named_sub_element(ElementName::FlexrayTpNode, name)?;
        Ok(Self(node_elem))
    }

    /// set or remove FlexrayTpAddress of the node
    /// A TP address is mandatory for unicast nodes, but optional for multicast nodes
    /// Setting None will remove the element
    pub fn set_tp_address(&self, tp_address: Option<&TpAddress>) -> Result<(), AutosarAbstractionError> {
        if let Some(tp_address) = tp_address {
            // add or update the TP address
            self.element()
                .get_or_create_sub_element(ElementName::TpAddressRef)?
                .set_reference_target(tp_address.element())?;
        } else {
            // remove the TP address
            if let Some(tp_address_elem) = self.element().get_sub_element(ElementName::TpAddressRef) {
                self.element().remove_sub_element(tp_address_elem)?;
            }
        }
        Ok(())
    }

    /// get the FlexrayTpAddress of the node
    pub fn tp_address(&self) -> Option<TpAddress> {
        self.element()
            .get_sub_element(ElementName::TpAddressRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| elem.try_into().ok())
    }

    /// add a FlexrayCommunicationConnector to the node
    /// The node can be associated with up to 2 connectors.
    /// In a system description this reference is mandatory.
    pub fn add_connector(&self, connector: &FlexrayCommunicationConnector) -> Result<(), AutosarAbstractionError> {
        let connector_refs = self.element().get_or_create_sub_element(ElementName::ConnectorRefs)?;

        if connector_refs.sub_elements().count() >= 2 {
            return Err(AutosarAbstractionError::InvalidParameter(
                "A FlexrayTpNode can only have up to 2 connectors".to_string(),
            ));
        }
        connector_refs
            .create_sub_element(ElementName::ConnectorRef)?
            .set_reference_target(connector.element())?;
        Ok(())
    }

    /// iterate over all FlexrayCommunicationConnectors of the node
    pub fn connectors(&self) -> impl Iterator<Item = FlexrayCommunicationConnector> {
        self.element()
            .get_sub_element(ElementName::ConnectorRefs)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .filter_map(|ref_elem| {
                ref_elem
                    .get_reference_target()
                    .ok()
                    .and_then(|elem| elem.try_into().ok())
            })
    }
}

//##################################################################

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        communication::{FlexrayChannelName, FlexrayClusterSettings},
        System, SystemCategory,
    };
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn test_flexray_iso_transport_protocol() {
        let model = AutosarModel::new();
        let _file = model.create_file("DoipTp.arxml", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();

        let system = System::new("system", &package, SystemCategory::EcuExtract).unwrap();
        let flexray_cluster = system
            .create_flexray_cluster("flexray_cluster", &package, &FlexrayClusterSettings::default())
            .unwrap();
        let flexray_channel = flexray_cluster
            .create_physical_channel("flexray_channel_a", FlexrayChannelName::A)
            .unwrap();
        let ecu_instance = system.create_ecu_instance("ecu_instance", &package).unwrap();
        let communication_controller = ecu_instance
            .create_flexray_communication_controller("can_ctrl")
            .unwrap();
        let connector = communication_controller
            .connect_physical_channel("name", &flexray_channel)
            .unwrap();

        // create a direct TP SDU (DCM-I-PDU)
        let tp_sdu = system.create_dcm_ipdu("diag", &package, 1024).unwrap();

        // create some NPdus
        let npdu1 = system.create_n_pdu("npdu1", &package, 64).unwrap();
        let npdu2 = system.create_n_pdu("npdu2", &package, 64).unwrap();

        // create a FlexrayTpConfig
        let fr_tp_config = system
            .create_flexray_tp_config("FrTpConfig", &package, &flexray_cluster)
            .unwrap();

        // create a FlexrayTpPduPool
        let fr_tp_pdu_pool_tx = fr_tp_config.create_flexray_tp_pdu_pool("FrTpPduPool_Tx").unwrap();
        fr_tp_pdu_pool_tx.add_npdu(&npdu1).unwrap();
        assert_eq!(fr_tp_pdu_pool_tx.npdus().next(), Some(npdu1));
        let fr_tp_pdu_pool_rx = fr_tp_config.create_flexray_tp_pdu_pool("FrTpPduPool_Rx").unwrap();
        fr_tp_pdu_pool_rx.add_npdu(&npdu2).unwrap();
        assert_eq!(fr_tp_pdu_pool_rx.npdus().next(), Some(npdu2));

        assert!(fr_tp_config.flexray_tp_pdu_pools().count() == 2);

        // create a FlexrayTpAddress
        let tp_address_1 = fr_tp_config.create_tp_address("TpAddress1", 0x1234).unwrap();
        assert_eq!(fr_tp_config.tp_addresses().next(), Some(tp_address_1.clone()));
        let tp_address_2 = fr_tp_config.create_tp_address("TpAddress2", 0x5678).unwrap();
        assert_eq!(fr_tp_config.tp_addresses().count(), 2);

        // create a FlexrayTpNode
        let tp_node_1 = fr_tp_config.create_flexray_tp_node("TpNode1").unwrap();
        tp_node_1.set_tp_address(Some(&tp_address_1)).unwrap();
        assert_eq!(tp_node_1.tp_address().unwrap(), tp_address_1);
        tp_node_1.add_connector(&connector).unwrap();
        assert_eq!(tp_node_1.connectors().next(), Some(connector));
        let tp_node_2 = fr_tp_config.create_flexray_tp_node("TpNode2").unwrap();
        tp_node_2.set_tp_address(Some(&tp_address_2)).unwrap();

        // create a FlexrayTpConnectionControl
        let connection_control = fr_tp_config
            .create_flexray_tp_connection_control("ConnectionControl")
            .unwrap();

        // create a FlexrayTpConnection
        let connection = fr_tp_config
            .create_flexray_tp_connection(None, &tp_node_1, &tp_sdu.into(), &connection_control)
            .unwrap();
        connection.add_receiver(&tp_node_2).unwrap();
        connection.set_tx_pdu_pool(&fr_tp_pdu_pool_tx).unwrap();
        connection.set_rx_pdu_pool(&fr_tp_pdu_pool_rx).unwrap();
        connection.set_multicast_address(Some(&tp_address_2)).unwrap();
        assert!(connection.receivers().count() == 1);
        assert_eq!(connection.receivers().next(), Some(tp_node_2));
        assert_eq!(connection.tx_pdu_pool().unwrap(), fr_tp_pdu_pool_tx);
        assert_eq!(connection.rx_pdu_pool().unwrap(), fr_tp_pdu_pool_rx);
        assert_eq!(connection.multicast_address().unwrap(), tp_address_2);
    }
}
