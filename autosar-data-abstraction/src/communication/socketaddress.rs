use crate::communication::{EthernetPhysicalChannel, NetworkEndpoint};
use crate::{abstraction_element, AbstractionElement, AutosarAbstractionError, EcuInstance};
use autosar_data::{Element, ElementName};

use super::{StaticSocketConnection, TcpRole};

//##################################################################

/// A socket address estapblishes the link between one or more ECUs and a NetworkEndpoint.
/// It contains all settings that are relevant for this combination.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SocketAddress(Element);
abstraction_element!(SocketAddress, SocketAddress);

impl SocketAddress {
    pub(crate) fn new(
        name: &str,
        channel: &EthernetPhysicalChannel,
        network_endpoint: &NetworkEndpoint,
        tp_config: &TpConfig,
        sa_type: SocketAddressType,
    ) -> Result<Self, AutosarAbstractionError> {
        let channel_elem = channel.element();
        let (unicast, ecu_instances) = match sa_type {
            SocketAddressType::Unicast(Some(ecu_instance)) => (true, vec![ecu_instance]),
            SocketAddressType::Unicast(None) => (true, vec![]),
            SocketAddressType::Multicast(ecu_instances) => (false, ecu_instances),
        };

        // TCP connections don't work using multicast IP addresses
        if !unicast && matches!(tp_config, TpConfig::TcpTp { .. }) {
            return Err(AutosarAbstractionError::InvalidParameter(
                "TCP is incomptible with multicasting".to_string(),
            ));
        }
        // extension: check if the address is valid for multicasting?
        // IPv4: 224.0.0.0 - 239.255.255.255
        // IPv6: FFxx:/12

        // get the connector for each ECU in advance, so that nothing needs to be cleaned up if there is a problem here
        let connectors = ecu_instances
            .iter()
            .filter_map(|ecu_instance| channel.get_ecu_connector(ecu_instance))
            .collect::<Vec<_>>();
        if connectors.len() != ecu_instances.len() {
            return Err(AutosarAbstractionError::InvalidParameter(
                "All EcuInstances must be connected to the EthernetPhysicalChannel".to_string(),
            ));
        }

        let elem = channel_elem
            .get_or_create_sub_element(ElementName::SoAdConfig)?
            .get_or_create_sub_element(ElementName::SocketAddresss)?
            .create_named_sub_element(ElementName::SocketAddress, name)?;

        if unicast {
            if !connectors.is_empty() {
                elem.create_sub_element(ElementName::ConnectorRef)
                    .unwrap()
                    .set_reference_target(&connectors[0])
                    .unwrap();
            }
        } else {
            let mc_connectors = elem.create_sub_element(ElementName::MulticastConnectorRefs)?;
            for conn in &connectors {
                mc_connectors
                    .create_sub_element(ElementName::MulticastConnectorRef)?
                    .set_reference_target(conn)?;
            }
        }

        let ae_name = format!("{name}_AE");
        let ae = elem.create_named_sub_element(ElementName::ApplicationEndpoint, &ae_name)?;
        ae.create_sub_element(ElementName::NetworkEndpointRef)?
            .set_reference_target(network_endpoint.element())?;
        let tp_configuration = ae.create_sub_element(ElementName::TpConfiguration)?;
        match tp_config {
            TpConfig::TcpTp {
                port_number,
                port_dynamically_assigned,
            } => {
                let tcptp = tp_configuration.create_sub_element(ElementName::TcpTp)?;
                let tcptp_port = tcptp.create_sub_element(ElementName::TcpTpPort)?;
                if let Some(portnum) = port_number {
                    tcptp_port
                        .create_sub_element(ElementName::PortNumber)?
                        .set_character_data(portnum.to_string())?;
                }
                if let Some(dyn_assign) = port_dynamically_assigned {
                    let boolstr = if *dyn_assign { "true" } else { "false" };
                    tcptp_port
                        .create_sub_element(ElementName::DynamicallyAssigned)?
                        .set_character_data(boolstr)?;
                }
            }
            TpConfig::UdpTp {
                port_number,
                port_dynamically_assigned,
            } => {
                let udptp_port = tp_configuration
                    .create_sub_element(ElementName::UdpTp)?
                    .create_sub_element(ElementName::UdpTpPort)?;
                if let Some(portnum) = port_number {
                    udptp_port
                        .create_sub_element(ElementName::PortNumber)?
                        .set_character_data(portnum.to_string())?;
                }
                if let Some(dyn_assign) = port_dynamically_assigned {
                    let boolstr = if *dyn_assign { "true" } else { "false" };
                    udptp_port
                        .create_sub_element(ElementName::DynamicallyAssigned)?
                        .set_character_data(boolstr)?;
                }
            }
        }

        Ok(Self(elem))
    }

    /// get the socket address type: unicast / multicast, as well as the connected ecus
    pub fn get_type(&self) -> Option<SocketAddressType> {
        if let Some(connector_ref) = self.0.get_sub_element(ElementName::ConnectorRef) {
            let ecu = EcuInstance::try_from(connector_ref.get_reference_target().ok()?.named_parent().ok()??).ok()?;
            Some(SocketAddressType::Unicast(Some(ecu)))
        } else if let Some(mcr) = self.0.get_sub_element(ElementName::MulticastConnectorRefs) {
            let ecus = mcr
                .sub_elements()
                .filter_map(|cr| {
                    cr.get_reference_target()
                        .ok()
                        .and_then(|conn| conn.named_parent().ok().flatten())
                })
                .filter_map(|ecu_elem| EcuInstance::try_from(ecu_elem).ok())
                .collect::<Vec<_>>();
            Some(SocketAddressType::Multicast(ecus))
        } else {
            None
        }
    }

    /// get the transport protocol settings for this `SocketAddress`
    pub fn get_tp_config(&self) -> Option<TpConfig> {
        let tp = self
            .0
            .get_sub_element(ElementName::ApplicationEndpoint)?
            .get_sub_element(ElementName::TpConfiguration)?;

        if let Some(tcp_tp) = tp.get_sub_element(ElementName::TcpTp) {
            let port = tcp_tp.get_sub_element(ElementName::TcpTpPort)?;
            let (port_number, port_dynamically_assigned) = Self::get_port_config(&port);
            Some(TpConfig::TcpTp {
                port_number,
                port_dynamically_assigned,
            })
        } else if let Some(udp_tp) = tp.get_sub_element(ElementName::UdpTp) {
            let port = udp_tp.get_sub_element(ElementName::UdpTpPort)?;
            let (port_number, port_dynamically_assigned) = Self::get_port_config(&port);
            Some(TpConfig::UdpTp {
                port_number,
                port_dynamically_assigned,
            })
        } else {
            None
        }
    }

    fn get_port_config(port_element: &Element) -> (Option<u16>, Option<bool>) {
        let port_number = port_element
            .get_sub_element(ElementName::PortNumber)
            .and_then(|pn| pn.character_data())
            .and_then(|cdata| cdata.parse_integer());
        let port_dynamically_assigned = port_element
            .get_sub_element(ElementName::DynamicallyAssigned)
            .and_then(|da| da.character_data())
            .and_then(|cdata| cdata.string_value())
            .map(|val| val == "true" || val == "1");
        (port_number, port_dynamically_assigned)
    }

    pub fn create_static_socket_connection(
        &self,
        name: &str,
        remote_address: &SocketAddress,
        tcp_role: Option<TcpRole>,
        tcp_connect_timeout: Option<f64>,
    ) -> Result<StaticSocketConnection, AutosarAbstractionError> {
        let own_tp_config = self.get_tp_config();
        let remote_tp_config = remote_address.get_tp_config();
        match (own_tp_config, remote_tp_config) {
            (Some(TpConfig::TcpTp { .. }), Some(TpConfig::TcpTp { .. })) => {
                StaticSocketConnection::new(name, self.element(), remote_address, tcp_role, tcp_connect_timeout)
            }
            (Some(TpConfig::UdpTp { .. }), Some(TpConfig::UdpTp { .. })) | (None, None) => {
                StaticSocketConnection::new(name, self.element(), remote_address, None, None)
            }
            _ => Err(AutosarAbstractionError::InvalidParameter(
                "Both SocketAddresses must use the same transport protocol".to_string(),
            )),
        }
    }
}

//##################################################################

/// transport protocol settings of a [`SocketAddress`]
#[derive(Debug, Clone, PartialEq)]
pub enum TpConfig {
    /// The socket uses TCP
    TcpTp {
        port_number: Option<u16>,
        port_dynamically_assigned: Option<bool>,
        // additional TCP options: currently not supported
    },
    // The socket uses UDP
    UdpTp {
        port_number: Option<u16>,
        port_dynamically_assigned: Option<bool>,
    },
    // RtpTp, Ieee1722Tp, HttpTp: currently not supported
}

//##################################################################

/// Describes if a [`SocketAddress`] is used for unicast or multicast
#[derive(Debug, Clone, PartialEq)]
pub enum SocketAddressType {
    Unicast(Option<EcuInstance>),
    Multicast(Vec<EcuInstance>),
}

//##################################################################

#[cfg(test)]
mod test {
    use super::*;
    use crate::communication::{IPv4AddressSource, NetworkEndpointAddress};
    use crate::{ArPackage, System, SystemCategory};
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn socket_address() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_4_3_0).unwrap();
        let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
        let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
        let ecu_instance = system.create_ecu_instance("Ecu", &package).unwrap();
        let controller = ecu_instance
            .create_ethernet_communication_controller("EthCtrl", None)
            .unwrap();
        let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
        let channel = cluster.create_physical_channel("Channel", None).unwrap();
        controller.connect_physical_channel("connection", &channel).unwrap();
        let endpoint_address = NetworkEndpointAddress::IPv4 {
            address: Some("192.168.0.1".to_string()),
            address_source: Some(IPv4AddressSource::Fixed),
            default_gateway: Some("192.168.0.2".to_string()),
            network_mask: Some("255.255.255.0".to_string()),
        };
        let network_endpoint = channel
            .create_network_endpoint("Address", endpoint_address, Some(&ecu_instance))
            .unwrap();
        let tcp_port = TpConfig::TcpTp {
            port_number: Some(1234),
            port_dynamically_assigned: None,
        };
        let socket_type = SocketAddressType::Unicast(Some(ecu_instance));
        channel
            .create_socket_address("SocketName", &network_endpoint, &tcp_port, socket_type)
            .unwrap();
        assert_eq!(channel.socket_addresses().count(), 1);
    }
}
