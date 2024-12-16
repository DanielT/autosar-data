use crate::communication::{EthernetCluster, PduTriggering};
use crate::{abstraction_element, AbstractionElement, ArPackage, AutosarAbstractionError};
use autosar_data::{Element, ElementName};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoIpTpConfig(Element);
abstraction_element!(DoIpTpConfig, DoIpTpConfig);

impl DoIpTpConfig {
    pub(crate) fn new(
        name: &str,
        package: &ArPackage,
        cluster: &EthernetCluster,
    ) -> Result<Self, AutosarAbstractionError> {
        let pkg_elem = package.element().get_or_create_sub_element(ElementName::Elements)?;

        let tp_config_elem = pkg_elem.create_named_sub_element(ElementName::DoIpTpConfig, name)?;
        tp_config_elem
            .create_sub_element(ElementName::CommunicationClusterRef)?
            .set_reference_target(cluster.element())?;

        Ok(Self(tp_config_elem))
    }

    /// create a new DoIpLogicAddress
    pub fn create_doip_logic_address(
        &self,
        name: &str,
        address: u32,
    ) -> Result<DoIpLogicAddress, AutosarAbstractionError> {
        let logic_addresses_elem = self
            .element()
            .get_or_create_sub_element(ElementName::DoIpLogicAddresss)?;
        DoIpLogicAddress::new(name, &logic_addresses_elem, address)
    }

    /// iterate over all DoIpLogicAddresss
    pub fn doip_logic_addresses(&self) -> impl Iterator<Item = DoIpLogicAddress> {
        self.element()
            .get_sub_element(ElementName::DoIpLogicAddresss)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .map(DoIpLogicAddress)
    }

    /// create a new DoIpTpConnection
    pub fn create_doip_tp_connection(
        &self,
        name: Option<&str>,
        source: &DoIpLogicAddress,
        target: &DoIpLogicAddress,
        tp_sdu_triggering: &PduTriggering,
    ) -> Result<DoIpTpConnection, AutosarAbstractionError> {
        let tp_connections_elem = self.element().get_or_create_sub_element(ElementName::TpConnections)?;
        DoIpTpConnection::new(name, &tp_connections_elem, source, target, tp_sdu_triggering)
    }

    /// iterate over all DoIpTpConnections
    pub fn doip_tp_connections(&self) -> impl Iterator<Item = DoIpTpConnection> {
        self.element()
            .get_sub_element(ElementName::TpConnections)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .map(DoIpTpConnection)
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoIpLogicAddress(Element);
abstraction_element!(DoIpLogicAddress, DoIpLogicAddress);

impl DoIpLogicAddress {
    pub(crate) fn new(name: &str, parent: &Element, address: u32) -> Result<Self, AutosarAbstractionError> {
        let logic_address_elem = parent.create_named_sub_element(ElementName::DoIpLogicAddress, name)?;
        logic_address_elem
            .create_sub_element(ElementName::Address)?
            .set_character_data(address as u64)?;
        Ok(Self(logic_address_elem))
    }

    /// get the address of this DoIpLogicAddress
    pub fn address(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Address)
            .and_then(|elem| elem.character_data())
            .and_then(|data| data.parse_integer())
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoIpTpConnection(Element);
abstraction_element!(DoIpTpConnection, DoIpTpConnection);

impl DoIpTpConnection {
    pub(crate) fn new(
        name: Option<&str>,
        parent: &Element,
        source: &DoIpLogicAddress,
        target: &DoIpLogicAddress,
        tp_sdu_triggering: &PduTriggering,
    ) -> Result<Self, AutosarAbstractionError> {
        let tp_connection_elem = parent.create_sub_element(ElementName::DoIpTpConnection)?;
        if let Some(name) = name {
            tp_connection_elem.create_named_sub_element(ElementName::Ident, name)?;
        }
        tp_connection_elem
            .create_sub_element(ElementName::DoIpSourceAddressRef)?
            .set_reference_target(source.element())?;
        tp_connection_elem
            .create_sub_element(ElementName::DoIpTargetAddressRef)?
            .set_reference_target(target.element())?;
        tp_connection_elem
            .create_sub_element(ElementName::TpSduRef)?
            .set_reference_target(tp_sdu_triggering.element())?;
        Ok(Self(tp_connection_elem))
    }

    /// get the source DoIpLogicAddress
    pub fn source(&self) -> Option<DoIpLogicAddress> {
        self.element()
            .get_sub_element(ElementName::DoIpSourceAddressRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| DoIpLogicAddress::try_from(elem).ok())
    }

    /// get the target DoIpLogicAddress
    pub fn target(&self) -> Option<DoIpLogicAddress> {
        self.element()
            .get_sub_element(ElementName::DoIpTargetAddressRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| DoIpLogicAddress::try_from(elem).ok())
    }

    /// get the PduTriggering for this connection
    pub fn tp_sdu_triggering(&self) -> Option<PduTriggering> {
        self.element()
            .get_sub_element(ElementName::TpSduRef)
            .and_then(|elem| elem.get_reference_target().ok())
            .and_then(|elem| PduTriggering::try_from(elem).ok())
    }
}

//##################################################################

/// This element defines the DoIp configuration for a specific Ecu
///
/// Only available in Autosar_00048 and later
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoIpConfig(Element);
abstraction_element!(DoIpConfig, DoIpConfig);

//##################################################################

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        communication::{
            CommunicationDirection, IPv4AddressSource, NetworkEndpointAddress, SocketAddressType, TpConfig,
        },
        System, SystemCategory,
    };
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn test_doip_transport_protocol() {
        let model = AutosarModel::new();
        let _file = model.create_file("DoipTp.arxml", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();

        let system = System::new("system", &package, SystemCategory::EcuExtract).unwrap();
        let eth_cluster = system.create_ethernet_cluster("can_cluster", &package).unwrap();
        let eth_channel = eth_cluster.create_physical_channel("can_channel", None).unwrap();
        let ecu_instance = system.create_ecu_instance("ecu_instance", &package).unwrap();
        let communication_controller = ecu_instance
            .create_ethernet_communication_controller("can_ctrl", Some("ab:cd:ef:01:02:03".to_string()))
            .unwrap();
        let _connector = communication_controller
            .connect_physical_channel("name", &eth_channel)
            .unwrap();

        // create socket #1
        let network_address_1 = NetworkEndpointAddress::IPv4 {
            address: Some("192.168.0.1".to_string()),
            address_source: Some(IPv4AddressSource::Fixed),
            default_gateway: Some("192.168.0.200".to_string()),
            network_mask: Some("255.255.255.0".to_string()),
        };
        let network_endpoint_1 = eth_channel
            .create_network_endpoint("local_endpoint", network_address_1, None)
            .unwrap();
        let udp_port_1 = TpConfig::UdpTp {
            port_number: Some(1234),
            port_dynamically_assigned: None,
        };
        let socket_type_1 = SocketAddressType::Unicast(Some(ecu_instance.clone()));
        let socket_address_tcp_1 = eth_channel
            .create_socket_address("ServerSocket", &network_endpoint_1, &udp_port_1, socket_type_1)
            .unwrap();

        // ceate socket #2
        let network_address_2 = NetworkEndpointAddress::IPv4 {
            address: Some("192.168.0.2".to_string()),
            address_source: Some(IPv4AddressSource::Fixed),
            default_gateway: Some("192.168.0.200".to_string()),
            network_mask: Some("255.255.255.0".to_string()),
        };
        let network_endpoint_2 = eth_channel
            .create_network_endpoint("remote_endpoint", network_address_2, None)
            .unwrap();
        let udp_port_2 = TpConfig::UdpTp {
            port_number: Some(5678),
            port_dynamically_assigned: None,
        };
        let socket_type_2 = SocketAddressType::Unicast(None);
        let socket_address_tcp_2 = eth_channel
            .create_socket_address("ClientSocket", &network_endpoint_2, &udp_port_2, socket_type_2)
            .unwrap();

        // create a connection (V2)
        let (static_socket_connection_a, static_socket_connection_b) = eth_channel
            .create_static_socket_connection_pair(
                "StaticSocketConnection",
                &socket_address_tcp_1,
                &socket_address_tcp_2,
                None,
            )
            .unwrap();

        // create a DCM_I_Pdu
        let dcm_i_pdu = system.create_dcm_ipdu("Diag", &package, 1024).unwrap();

        // create an IPduIdentifier, which is used to map the PDU to both sides of the socket connection
        let ipdu_identifier_set_package = ArPackage::get_or_create(&model, "/Network/IpduIdentifierSets").unwrap();
        let socon_ipdu_identifier_set = system
            .create_socket_connection_ipdu_identifier_set("IpduIdentifierSet", &ipdu_identifier_set_package)
            .unwrap();
        let ipdu_identifier = socon_ipdu_identifier_set
            .create_socon_ipdu_identifier("IpduIdentifier", &dcm_i_pdu, &eth_channel, Some(0x1000), None, None)
            .unwrap();

        // trigger the DCM_I_Pdu on the connection
        static_socket_connection_a
            .add_ipdu_identifier(&ipdu_identifier)
            .unwrap();
        static_socket_connection_b
            .add_ipdu_identifier(&ipdu_identifier)
            .unwrap();
        let pdu_triggering = ipdu_identifier.pdu_triggering().unwrap();
        pdu_triggering
            .create_pdu_port(&ecu_instance, CommunicationDirection::Out)
            .unwrap();

        let doip_tp_config = system
            .create_doip_tp_config("doip_tp_config", &package, &eth_cluster)
            .unwrap();

        let doip_logic_address_source = doip_tp_config.create_doip_logic_address("addr_source", 1).unwrap();
        assert_eq!(doip_logic_address_source.address(), Some(1));
        let doip_logic_address_target = doip_tp_config.create_doip_logic_address("addr_target", 2).unwrap();
        assert_eq!(doip_logic_address_target.address(), Some(2));

        let doip_tp_connection = doip_tp_config
            .create_doip_tp_connection(
                None,
                &doip_logic_address_source,
                &doip_logic_address_target,
                &pdu_triggering,
            )
            .unwrap();
        assert_eq!(doip_tp_connection.source(), Some(doip_logic_address_source.clone()));
        assert_eq!(doip_tp_connection.target(), Some(doip_logic_address_target.clone()));
        assert_eq!(doip_tp_connection.tp_sdu_triggering(), Some(pdu_triggering.clone()));

        let doip_tp_connections: Vec<DoIpTpConnection> = doip_tp_config.doip_tp_connections().collect();
        assert_eq!(doip_tp_connections.len(), 1);
        assert_eq!(doip_tp_connections[0], doip_tp_connection);

        let doip_logic_addresses: Vec<DoIpLogicAddress> = doip_tp_config.doip_logic_addresses().collect();
        assert_eq!(doip_logic_addresses.len(), 2);
        assert_eq!(doip_logic_addresses[0], doip_logic_address_source);
    }
}
