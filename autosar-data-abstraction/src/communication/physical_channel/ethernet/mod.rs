use crate::communication::{
    CommunicationDirection, EthernetCluster, GeneralPurposePdu, Pdu, PduCollectionTrigger, PduTriggering,
    PhysicalChannel,
};
use crate::{
    abstraction_element, element_iterator, AbstractionElement, ArPackage, AutosarAbstractionError, EcuInstance,
};
use autosar_data::{AutosarVersion, Element, ElementName, EnumItem};

mod networkendpoint;
mod soad_old;
mod socketaddress;
mod someip;
mod someip_old;

pub use networkendpoint::*;
pub use soad_old::*;
pub use socketaddress::*;
pub use someip::*;
pub use someip_old::*;

//##################################################################

/// Provides information about the VLAN of an [`EthernetPhysicalChannel`]
#[derive(Debug, Clone, PartialEq)]
pub struct EthernetVlanInfo {
    pub vlan_name: String,
    pub vlan_id: u16,
}

//##################################################################

/// The `EthernetPhysicalChannel` represents a VLAN or untagged traffic
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EthernetPhysicalChannel(Element);
abstraction_element!(EthernetPhysicalChannel, EthernetPhysicalChannel);

impl EthernetPhysicalChannel {
    /// Retrieves the VLAN information from a channel
    ///
    /// An ethernet physical channel that represents untagged traffic has no VLAN information and returns None.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # use autosar_data_abstraction::communication::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// let vlan_info = EthernetVlanInfo {
    ///     vlan_name: "VLAN_1".to_string(),
    ///     vlan_id: 1,
    /// };
    /// let channel = cluster.create_physical_channel("Channel", Some(vlan_info)).unwrap();
    /// let info = channel.get_vlan_info().unwrap();
    /// assert_eq!(info.vlan_id, 1);
    /// ```
    #[must_use]
    pub fn get_vlan_info(&self) -> Option<EthernetVlanInfo> {
        let vlan = self.0.get_sub_element(ElementName::Vlan)?;
        let vlan_id = vlan
            .get_sub_element(ElementName::VlanIdentifier)?
            .character_data()?
            .parse_integer::<u16>()?;
        Some(EthernetVlanInfo {
            vlan_name: vlan.item_name()?,
            vlan_id,
        })
    }

    /// get the cluster containing this physical channel
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// let channel = cluster.create_physical_channel("Channel", None).unwrap();
    /// let cluster_2 = channel.cluster().unwrap();
    /// assert_eq!(cluster, cluster_2);
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model
    pub fn cluster(&self) -> Result<EthernetCluster, AutosarAbstractionError> {
        let cluster_elem = self.0.named_parent()?.unwrap();
        EthernetCluster::try_from(cluster_elem)
    }

    /// create a network endpoint - IPv4 or IPv6 address - for this channel
    ///
    /// In older versions of the Autosar standard, up to version 4.4.0, the `NetworkEndpoint` could be linked to an Ecu.
    /// The parameter `ecu` specifies the target.
    /// The link is obsoleted in newer versions, and will only be created if the file version allows it.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # use autosar_data_abstraction::communication::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// # let channel = cluster.create_physical_channel("Channel", None).unwrap();
    /// let endpoint_address = NetworkEndpointAddress::IPv4 {
    ///     address: Some("192.168.0.1".to_string()),
    ///     address_source: Some(IPv4AddressSource::Fixed),
    ///     default_gateway: Some("192.168.0.2".to_string()),
    ///     network_mask: Some("255.255.255.0".to_string()),
    /// };
    /// let network_endpoint = channel.create_network_endpoint("Address1", endpoint_address, None).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model
    pub fn create_network_endpoint(
        &self,
        name: &str,
        address: NetworkEndpointAddress,
        ecu: Option<&EcuInstance>,
    ) -> Result<NetworkEndpoint, AutosarAbstractionError> {
        let network_endpoint = NetworkEndpoint::new(name, self, address)?;

        if let Some(ecu_instance) = ecu {
            let version = self.0.min_version()?;
            if version <= AutosarVersion::Autosar_00046 {
                let ne_element = network_endpoint.element().clone();

                // get a connector referenced by this physical channel which is contained in the ecu_instance
                if let Some(connector) = self.get_ecu_connector(ecu_instance) {
                    let _ = connector
                        .get_or_create_sub_element(ElementName::NetworkEndpointRefs)
                        .and_then(|ner| ner.create_sub_element(ElementName::NetworkEndpointRef))
                        .and_then(|ner| ner.set_reference_target(&ne_element));
                } else {
                    // no connector between the ECU and this channel -> abort
                    self.0
                        .get_sub_element(ElementName::NetworkEndpoints)
                        .and_then(|endpoints| endpoints.remove_sub_element(ne_element).ok());
                }
            }
        }

        Ok(network_endpoint)
    }

    /// create an iterator over all [`NetworkEndpoint`]s in this channel
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # use autosar_data_abstraction::communication::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// # let channel = cluster.create_physical_channel("Channel", None).unwrap();
    /// # let endpoint_address = NetworkEndpointAddress::IPv4 {
    /// #     address: Some("192.168.0.1".to_string()),
    /// #     address_source: Some(IPv4AddressSource::Fixed),
    /// #     default_gateway: Some("192.168.0.2".to_string()),
    /// #     network_mask: Some("255.255.255.0".to_string()),
    /// # };
    /// # channel.create_network_endpoint("Address1", endpoint_address, None).unwrap();
    /// for network_endpoint in channel.network_endpoints() {
    ///     // ...
    /// }
    /// # assert_eq!(channel.network_endpoints().count(), 1)
    /// ```
    pub fn network_endpoints(&self) -> impl Iterator<Item = NetworkEndpoint> {
        NetworkEndpointIterator::new(self.element().get_sub_element(ElementName::NetworkEndpoints))
    }

    /// create a socket address in the ethernet channel
    ///
    /// It contains the settings of the TCP/UDP port and links to a [`NetworkEndpoint`] which contains the IP address.
    /// The socket address can either be a unicast adress which is associated with a single ECU, or a multicast address
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # use autosar_data_abstraction::communication::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let ecu_instance = system.create_ecu_instance("Ecu", &package).unwrap();
    /// # let controller = ecu_instance.create_ethernet_communication_controller("EthCtrl", None).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// # let channel = cluster.create_physical_channel("Channel", None).unwrap();
    /// # controller.connect_physical_channel("connection", &channel).unwrap();
    /// # let endpoint_address = NetworkEndpointAddress::IPv4 {
    /// #     address: Some("192.168.0.1".to_string()),
    /// #     address_source: Some(IPv4AddressSource::Fixed),
    /// #     default_gateway: Some("192.168.0.2".to_string()),
    /// #     network_mask: Some("255.255.255.0".to_string()),
    /// # };
    /// # let network_endpoint = channel.create_network_endpoint("Address", endpoint_address, None).unwrap();
    /// let tcp_port = TpConfig::TcpTp {
    ///     port_number: Some(1234),
    ///     port_dynamically_assigned: None,
    /// };
    /// let socket_type = SocketAddressType::Unicast(Some(ecu_instance));
    /// channel.create_socket_address("SocketName", &network_endpoint, &tcp_port, socket_type).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::InvalidParameter`] `sa_type` contains a reference to an EcuInstance which is not connected to this channel
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model
    pub fn create_socket_address(
        &self,
        name: &str,
        network_endpoint: &NetworkEndpoint,
        tp_config: &TpConfig,
        sa_type: SocketAddressType,
    ) -> Result<SocketAddress, AutosarAbstractionError> {
        SocketAddress::new(name, self, network_endpoint, tp_config, sa_type)
    }

    /// create an iterator over all [`SocketAddress`]es in this channel
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # use autosar_data_abstraction::communication::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let ecu_instance = system.create_ecu_instance("Ecu", &package).unwrap();
    /// # let controller = ecu_instance.create_ethernet_communication_controller("EthCtrl", None).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// # let channel = cluster.create_physical_channel("Channel", None).unwrap();
    /// # controller.connect_physical_channel("connection", &channel).unwrap();
    /// # let endpoint_address = NetworkEndpointAddress::IPv4 {
    /// #     address: Some("192.168.0.1".to_string()),
    /// #     address_source: Some(IPv4AddressSource::Fixed),
    /// #     default_gateway: Some("192.168.0.2".to_string()),
    /// #     network_mask: Some("255.255.255.0".to_string()),
    /// # };
    /// # let network_endpoint = channel.create_network_endpoint("Address", endpoint_address, None).unwrap();
    /// let tcp_port = TpConfig::TcpTp {
    ///     port_number: Some(1234),
    ///     port_dynamically_assigned: None,
    /// };
    /// let socket_type = SocketAddressType::Unicast(Some(ecu_instance));
    /// channel.create_socket_address("SocketName", &network_endpoint, &tcp_port, socket_type).unwrap();
    /// assert_eq!(channel.socket_addresses().count(), 1);
    /// ```
    pub fn socket_addresses(&self) -> impl Iterator<Item = SocketAddress> {
        SocketAddressIterator::new(
            self.element()
                .get_sub_element(ElementName::SoAdConfig)
                .and_then(|sc| sc.get_sub_element(ElementName::SocketAddresss)),
        )
    }

    pub(crate) fn get_ecu_connector(&self, ecu_instance: &EcuInstance) -> Option<Element> {
        PhysicalChannel::Ethernet(self.clone()).get_ecu_connector(ecu_instance)
    }

    /// create a socket connection bundle
    ///
    /// The `SocketConnectionBundle` is the "old" way to establish a connection between two sockets.
    /// It is deprecated in newer versions of the Autosar standard, but remains available for compatibility.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # use autosar_data_abstraction::communication::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00046).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// # let channel = cluster.create_physical_channel("Channel", None).unwrap();
    /// # let server_endpoint = channel.create_network_endpoint("ServerAddress", NetworkEndpointAddress::IPv4 {
    /// #    address: Some("192.16.0.1".to_string()),
    /// #    address_source: Some(IPv4AddressSource::Fixed),
    /// #    default_gateway: None,
    /// #    network_mask: None
    /// # }, None).unwrap();
    /// # let server_socket = channel.create_socket_address("ServerSocket", &server_endpoint, &TpConfig::TcpTp {
    /// #    port_number: Some(1234),
    /// #    port_dynamically_assigned: None
    /// # }, SocketAddressType::Unicast(None)).unwrap();
    /// let bundle = channel.create_socket_connection_bundle("Bundle", &server_socket).unwrap();
    /// ```
    pub fn create_socket_connection_bundle(
        &self,
        name: &str,
        server_port: &SocketAddress,
    ) -> Result<SocketConnectionBundle, AutosarAbstractionError> {
        let soadcfg = self.0.get_or_create_sub_element(ElementName::SoAdConfig)?;
        let connections = soadcfg.get_or_create_sub_element(ElementName::ConnectionBundles)?;

        SocketConnectionBundle::new(name, server_port, &connections)
    }

    /// create a pair of static socket connections
    ///
    /// Static socket connections are usually created as a pair, one on each socket involved on the connection.
    /// This helper function creates both at once. To create a single connection, use [`SocketAddress::create_static_socket_connection`].
    ///
    /// If the connection is a TCP connection, the first port connects to the second port, and the second port listens for incoming connection.
    /// The ordering of `port_1` and `port_2` has no impact on the direction of the transported PDUs. This is defined in the PduTriggering.
    ///
    /// `StaticSocketConnection`s are the "new" way to establish a connection between two sockets.
    /// It was introduced in Autosar 4.5.0 (AUTOSAR_00048) and is the recommended way to create connections.
    ///
    /// SocketConnectionBundles (old) and StaticSocketConnections (new) may never be used in the same file.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # use autosar_data_abstraction::communication::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// # let channel = cluster.create_physical_channel("Channel", None).unwrap();
    /// # let endpoint = channel.create_network_endpoint("ServerAddress", NetworkEndpointAddress::IPv4 {
    /// #    address: Some("192.168.0.1".to_string()),
    /// #    address_source: Some(IPv4AddressSource::Fixed),
    /// #    default_gateway: None,
    /// #    network_mask: None
    /// # }, None).unwrap();
    /// # let server_socket = channel.create_socket_address("ServerSocket", &endpoint, &TpConfig::TcpTp {
    /// #    port_number: Some(1234),
    /// #    port_dynamically_assigned: None
    /// # }, SocketAddressType::Unicast(None)).unwrap();
    /// # let client_socket = channel.create_socket_address("ClientSocket", &endpoint, &TpConfig::TcpTp {
    /// #    port_number: Some(1235),
    /// #    port_dynamically_assigned: None
    /// # }, SocketAddressType::Unicast(None)).unwrap();
    /// let (connection_a, connection_b) = channel.create_static_socket_connection_pair("Connection", &server_socket, &client_socket, None).unwrap();
    /// ```
    pub fn create_static_socket_connection_pair(
        &self,
        name: &str,
        port_1: &SocketAddress,
        port_2: &SocketAddress,
        tcp_connect_timeout: Option<f64>,
    ) -> Result<(StaticSocketConnection, StaticSocketConnection), AutosarAbstractionError> {
        let ssc1 = port_1.create_static_socket_connection(name, port_2, Some(TcpRole::Connect), tcp_connect_timeout)?;
        let ssc2 = port_2.create_static_socket_connection(name, port_1, Some(TcpRole::Listen), tcp_connect_timeout)?;
        Ok((ssc1, ssc2))
    }

    /// configure SOME/IP service discovery (SD) for an ECU connected to this channel
    ///
    /// SD is used to broadcast service offers on the network and subscribe to services offered by other ECUs.
    /// This function configures the ECU to use the SOME/IP SD protocol.
    ///
    /// SD uses either socket connection bundles or static socket connections to communicate.
    ///
    /// `ecu` is the ECU that should be configured for SD.
    /// `unicast_socket` is the socket address used for unicast rt/tx communication by the ECU.
    /// `unicast_rx_pdu` and `unicast_tx_pdu` are the GeneralPurposePdus used for the unicast communication.
    /// `common_config` contains common configuration settings that can be used for all SD ECUs.
    ///  - `multicast_rx_socket` is the socket address used for multicast communication by all SD ECUs.
    ///  - `remote_socket` is a socket whose IP is set to ANY with UDP port 0, acting as the remote address in the SD communication.
    ///  - `name_prefix` is an optional prefix for the names of the created elements.
    ///  - `prefer_static_socket_connections` is a flag that determines if SocketConnectionBundles should be used instead of StaticSocketConnections.
    ///     This is only relevant if the type can't be detected automatically.
    ///  - `ipdu_identifier_set` is contains the IPduIdentifiers that are used in StaticSocketConnections.
    ///
    /// Note:
    /// Usually SomeIP SD is expected to use port 30490, but this is not mandatory.
    /// The port number is set in the sockets, and must be the same for all SD sockets.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # use autosar_data_abstraction::communication::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let ecu_instance = system.create_ecu_instance("Ecu", &package).unwrap();
    /// # let controller = ecu_instance.create_ethernet_communication_controller("EthCtrl", None).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// # let channel = cluster.create_physical_channel("Channel", None).unwrap();
    /// # controller.connect_physical_channel("connection", &channel).unwrap();
    ///
    /// let unicast_endpoint = channel.create_network_endpoint("UnicastEndpoint", NetworkEndpointAddress::IPv4 {
    ///    address: Some("192.168.0.1".to_string()),
    ///    address_source: Some(IPv4AddressSource::Fixed),
    ///    default_gateway: None,
    ///    network_mask: None
    /// }, None).unwrap();
    /// let unicast_socket = channel.create_socket_address("UnicastSocket", &unicast_endpoint, &TpConfig::UdpTp {
    ///    port_number: Some(30490),
    ///    port_dynamically_assigned: None
    /// }, SocketAddressType::Unicast(Some(ecu_instance.clone()))).unwrap();
    /// let multicast_rx_endpoint = channel.create_network_endpoint("MulticastEndpoint", NetworkEndpointAddress::IPv4 {
    ///    address: Some("239.0.0.1".to_string()),
    ///    address_source: Some(IPv4AddressSource::Fixed),
    ///    default_gateway: None,
    ///    network_mask: None
    /// }, None).unwrap();
    /// let multicast_rx_socket = channel.create_socket_address("MulticastSocket", &multicast_rx_endpoint, &TpConfig::UdpTp {
    ///    port_number: Some(30490),
    ///    port_dynamically_assigned: None
    /// }, SocketAddressType::Multicast(vec![ecu_instance.clone()])).unwrap();
    /// let remote_endpoint = channel.create_network_endpoint("RemoteEndpoint", NetworkEndpointAddress::IPv4 {
    ///    address: Some("ANY".to_string()),
    ///    address_source: None,
    ///    default_gateway: None,
    ///    network_mask: None
    /// }, None).unwrap();
    /// let remote_socket = channel.create_socket_address("RemoteSocket", &remote_endpoint, &TpConfig::UdpTp {
    ///   port_number: Some(0),
    ///   port_dynamically_assigned: None
    /// }, SocketAddressType::Unicast(None)).unwrap();
    /// let unicast_rx_pdu = system.create_general_purpose_pdu("UnicastRxPdu", &package, 0, GeneralPurposePduCategory::Sd).unwrap();
    /// let unicast_tx_pdu = system.create_general_purpose_pdu("UnicastTxPdu", &package, 0, GeneralPurposePduCategory::Sd).unwrap();
    /// let multicast_rx_pdu = system.create_general_purpose_pdu("MulticastRxPdu", &package, 0, GeneralPurposePduCategory::Sd).unwrap();
    /// let common_config = CommonServiceDiscoveryConfig {
    ///   multicast_rx_socket: &multicast_rx_socket,
    ///   multicast_rx_pdu: &multicast_rx_pdu,
    ///   remote_socket: &remote_socket,
    ///   name_prefix: None,
    ///   prefer_static_socket_connections: false,
    ///   ipdu_identifier_set: None,
    /// };
    ///
    /// channel.configure_service_discovery_for_ecu(&ecu_instance, &unicast_socket, &unicast_rx_pdu, &unicast_tx_pdu, &common_config).unwrap();
    /// ```
    pub fn configure_service_discovery_for_ecu(
        &self,
        ecu: &EcuInstance,
        unicast_socket: &SocketAddress,
        unicast_rx_pdu: &GeneralPurposePdu,
        unicast_tx_pdu: &GeneralPurposePdu,
        common_config: &CommonServiceDiscoveryConfig,
    ) -> Result<(), AutosarAbstractionError> {
        let version = self.0.min_version()?;

        // check: the ECU must be connected to this channel
        if self.get_ecu_connector(ecu).is_none() {
            return Err(AutosarAbstractionError::InvalidParameter(
                "The ECU must be connected to the channel".to_string(),
            ));
        };

        // build the SD configuration using SocketConnectionBundles if the file verion is old,
        // or if SocketConnectionBundles already exist, or if the user prefers it
        let use_scb = version < AutosarVersion::Autosar_00048
            || self.has_socket_connections()
            || !common_config.prefer_static_socket_connections;

        // check: each socket must be part of the channel
        if unicast_socket.physical_channel()? != *self
            || common_config.multicast_rx_socket.physical_channel()? != *self
            || common_config.remote_socket.physical_channel()? != *self
        {
            return Err(AutosarAbstractionError::InvalidParameter(
                "All sockets must be part of the channel".to_string(),
            ));
        }

        // the "unicast socket" must be configured as Unicast
        match unicast_socket.get_type() {
            Some(SocketAddressType::Unicast(opt_socket_ecu)) => {
                if let Some(socket_ecu) = opt_socket_ecu {
                    if &socket_ecu != ecu {
                        return Err(AutosarAbstractionError::InvalidParameter(
                            "The unicast socket belongs to a different ECU".to_string(),
                        ));
                    }
                }
            }
            None => {
                // set the ECU for the unicast socket
                unicast_socket.set_unicast_ecu(ecu)?;
            }
            _ => {
                return Err(AutosarAbstractionError::InvalidParameter(
                    "The unicast socket is not configured as Unicast".to_string(),
                ));
            }
        }

        // the "multicast rx socket" must be configured as Multicast
        // the ecu will be added to the list of multicast ECUs in the socket
        match common_config.multicast_rx_socket.get_type() {
            Some(SocketAddressType::Multicast(_)) | None => {
                common_config.multicast_rx_socket.add_multicast_ecu(ecu)?;
            }
            _ => {
                return Err(AutosarAbstractionError::InvalidParameter(
                    "The multicast rx socket is not configured as Multicast".to_string(),
                ));
            }
        }

        // each socket must use UDP
        let Some(TpConfig::UdpTp {
            port_number: unicast_port,
            ..
        }) = unicast_socket.get_tp_config()
        else {
            return Err(AutosarAbstractionError::InvalidParameter(
                "The unicast port must use UDP".to_string(),
            ));
        };
        let Some(TpConfig::UdpTp {
            port_number: multicast_rx_port,
            ..
        }) = common_config.multicast_rx_socket.get_tp_config()
        else {
            return Err(AutosarAbstractionError::InvalidParameter(
                "The multicast rx port must use UDP".to_string(),
            ));
        };
        let Some(TpConfig::UdpTp {
            port_number: remote_port,
            port_dynamically_assigned: remote_dynamically_assigned,
        }) = common_config.remote_socket.get_tp_config()
        else {
            return Err(AutosarAbstractionError::InvalidParameter(
                "The remote port must use UDP".to_string(),
            ));
        };
        if unicast_port.is_none() || unicast_port != multicast_rx_port {
            return Err(AutosarAbstractionError::InvalidParameter(
                "All local UDP ports must use the same port number".to_string(),
            ));
        }
        // required the remote port to either be 0 or dynamically_assigned = true
        // the attribute dynamically_assigned is obsolete in AUTOSAR 4.5.0 and newer
        if remote_port != Some(0) && remote_dynamically_assigned != Some(true) {
            return Err(AutosarAbstractionError::InvalidParameter(
                "The remote UDP port must be 0 / dynamically assigned".to_string(),
            ));
        }

        // the IP address (ipv4 or ipv6) of the remote socket must be set to ANY
        let Some(remote_network_endpoint) = common_config.remote_socket.network_endpoint() else {
            return Err(AutosarAbstractionError::InvalidParameter(
                "The remote socket must have a network endpoint".to_string(),
            ));
        };
        if !remote_network_endpoint.addresses().all(|neaddr| match neaddr {
            NetworkEndpointAddress::IPv4 { address, .. } => address == Some("ANY".to_string()),
            NetworkEndpointAddress::IPv6 { address, .. } => address == Some("ANY".to_string()),
        }) {
            return Err(AutosarAbstractionError::InvalidParameter(
                "The IP (v4/v6) address of the remote socket must be set to ANY".to_string(),
            ));
        }

        // create the SD configuration
        if use_scb {
            self.configure_sd_socket_connection_bundle(
                ecu,
                unicast_socket,
                unicast_tx_pdu,
                unicast_rx_pdu,
                common_config,
            )?;
        } else {
            self.configure_sd_static_socket_connection(
                common_config,
                unicast_socket,
                unicast_rx_pdu,
                ecu,
                unicast_tx_pdu,
            )?;
        }

        Ok(())
    }

    /// configure SOME/IP service discovery (SD) using SocketConnectionBundles
    fn configure_sd_socket_connection_bundle(
        &self,
        ecu: &EcuInstance,
        unicast_socket: &SocketAddress,
        unicast_tx_pdu: &GeneralPurposePdu,
        unicast_rx_pdu: &GeneralPurposePdu,
        common_config: &CommonServiceDiscoveryConfig<'_>,
    ) -> Result<(), AutosarAbstractionError> {
        let name_prefix = common_config.name_prefix.unwrap_or("");
        let ecu_name = ecu.name().unwrap_or("unnamed".to_string());

        let connection_bundles = self
            .element()
            .get_or_create_sub_element(ElementName::SoAdConfig)?
            .get_or_create_sub_element(ElementName::ConnectionBundles)?;

        // check if the unicast connection already exists
        let scb_unicast = connection_bundles
            .sub_elements()
            .filter_map(|elem| SocketConnectionBundle::try_from(elem).ok())
            .find(|scb| {
                scb.server_port().map(|sp| &sp == unicast_socket).unwrap_or(false)
                    && scb.bundled_connections().any(|sc| {
                        sc.client_ip_addr_from_connection_request() == Some(true)
                            && sc
                                .client_port()
                                .map(|cp| &cp == common_config.remote_socket)
                                .unwrap_or(false)
                            && sc.pdu_triggerings().count() == 2
                    })
            });

        if scb_unicast.is_none() {
            // create a new SocketConnectionBundle for the unicast connection
            let scb_name = format!("{name_prefix}SD_Unicast_{ecu_name}");
            let scb = SocketConnectionBundle::new(&scb_name, unicast_socket, &connection_bundles)?;
            let conn = scb.create_bundled_connection(common_config.remote_socket)?;
            conn.set_client_ip_addr_from_connection_request(Some(true))?;
            conn.set_client_port_from_connection_request(Some(true))?;
            let pt_tx = conn.trigger_pdu(
                unicast_tx_pdu,
                SocketConnection::SD_HEADER_ID,
                None,
                Some(PduCollectionTrigger::Always),
            )?;
            let pt_rx = conn.trigger_pdu(
                unicast_rx_pdu,
                SocketConnection::SD_HEADER_ID,
                None,
                Some(PduCollectionTrigger::Always),
            )?;
            pt_tx.create_pdu_port(ecu, CommunicationDirection::Out)?;
            pt_rx.create_pdu_port(ecu, CommunicationDirection::In)?;
        }

        // check if the multicast connection already exists
        let scb_multicast_opt = connection_bundles
            .sub_elements()
            .filter_map(|elem| SocketConnectionBundle::try_from(elem).ok())
            .find(|scb| {
                scb.server_port()
                    .map(|sp| &sp == common_config.multicast_rx_socket)
                    .unwrap_or(false)
                    && scb.bundled_connections().any(|sc| {
                        sc.client_ip_addr_from_connection_request() == Some(true)
                            && sc.client_port().map(|cp| &cp == unicast_socket).unwrap_or(false)
                            && sc.pdu_triggerings().count() == 1
                    })
            });

        let scb_multicast_pt = if let Some(pt) = scb_multicast_opt
            .and_then(|scb| scb.bundled_connections().next())
            .and_then(|sc| sc.pdu_triggerings().next())
        {
            // the PduTriggering in the multicast connection already exists, return it
            pt
        } else {
            // create a new SocketConnectionBundle for the multicast connection
            let scb_name = format!("{name_prefix}SD_Multicast_Rx");
            let scb = SocketConnectionBundle::new(&scb_name, common_config.multicast_rx_socket, &connection_bundles)?;
            let conn = scb.create_bundled_connection(common_config.remote_socket)?;
            conn.set_client_ip_addr_from_connection_request(Some(true))?;
            conn.set_client_port_from_connection_request(Some(true))?;
            // trigger the multicast PDU in the connection, which creates a PduTriggering
            conn.trigger_pdu(
                common_config.multicast_rx_pdu,
                SocketConnection::SD_HEADER_ID,
                None,
                Some(PduCollectionTrigger::Always),
            )?
        };
        // add a PduPort for the ecu to the PduTriggering
        scb_multicast_pt.create_pdu_port(ecu, CommunicationDirection::In)?;

        Ok(())
    }

    /// configure SOME/IP service discovery (SD) using StaticSocketConnections
    fn configure_sd_static_socket_connection(
        &self,
        common_config: &CommonServiceDiscoveryConfig<'_>,
        unicast_socket: &SocketAddress,
        unicast_rx_pdu: &GeneralPurposePdu,
        ecu: &EcuInstance,
        unicast_tx_pdu: &GeneralPurposePdu,
    ) -> Result<(), AutosarAbstractionError> {
        let name_prefix = common_config.name_prefix.unwrap_or("");
        let ecu_name = ecu.name().unwrap_or("unnamed".to_string());

        let Some(ipdu_identifier_set) = common_config.ipdu_identifier_set else {
            return Err(AutosarAbstractionError::InvalidParameter(
                "An IPduIdentifierSet is required for StaticSocketConnections".to_string(),
            ));
        };
        let ssc_unicast = unicast_socket.static_socket_connections().find(|ssc| {
            ssc.remote_socket()
                .map(|rs| &rs == common_config.remote_socket)
                .unwrap_or(false)
                && ssc.i_pdu_identifiers().count() == 2
        });

        if ssc_unicast.is_none() {
            // create a new StaticSocketConnection for the unicast connection
            let name = format!("{name_prefix}SD_Unicast_{ecu_name}");
            let ssc = unicast_socket.create_static_socket_connection(&name, common_config.remote_socket, None, None)?;
            // create the IPduIdentifier for the unicast rx PDU
            let name = format!("{name_prefix}SD_Unicast_Rx_{ecu_name}");
            let idpu_identifier_rx = ipdu_identifier_set.create_socon_ipdu_identifier(
                &name,
                unicast_rx_pdu,
                self,
                Some(SoConIPduIdentifier::SD_HEADER_ID),
                None,
                Some(PduCollectionTrigger::Always),
            )?;
            // create a PduPort for the ecu in the new PduTriggering
            idpu_identifier_rx
                .pdu_triggering()
                .unwrap()
                .create_pdu_port(ecu, CommunicationDirection::In)?;
            // create the IPduIdentifier for the unicast tx PDU
            let name = format!("{name_prefix}SD_Unicast_Tx_{ecu_name}");
            let idpu_identifier_tx = ipdu_identifier_set.create_socon_ipdu_identifier(
                &name,
                unicast_tx_pdu,
                self,
                Some(SoConIPduIdentifier::SD_HEADER_ID),
                None,
                Some(PduCollectionTrigger::Always),
            )?;
            // create a PduPort for the ecu in the new PduTriggering
            idpu_identifier_tx
                .pdu_triggering()
                .unwrap()
                .create_pdu_port(ecu, CommunicationDirection::Out)?;
            ssc.add_ipdu_identifier(&idpu_identifier_rx)?;
            ssc.add_ipdu_identifier(&idpu_identifier_tx)?;
        }

        // create or extend the shared multicast connection
        let ssc_multicast = common_config
            .multicast_rx_socket
            .static_socket_connections()
            .find(|ssc| {
                ssc.remote_socket().map(|rs| &rs == unicast_socket).unwrap_or(false)
                    && ssc.i_pdu_identifiers().count() == 1
            });

        let pt_multicast_rx = if let Some(pt) = ssc_multicast
            .and_then(|ssc| ssc.i_pdu_identifiers().next())
            .and_then(|ipi| ipi.pdu_triggering())
        {
            // the PduTriggering already exists, return it
            pt
        } else {
            // create a new StaticSocketConnection for the multicast connection
            let name = format!("{name_prefix}SD_Multicast_Rx");
            let ssc = common_config.multicast_rx_socket.create_static_socket_connection(
                &name,
                common_config.remote_socket,
                None,
                None,
            )?;
            let idpu_identifier_mc_rx = ipdu_identifier_set.create_socon_ipdu_identifier(
                &name,
                common_config.multicast_rx_pdu,
                self,
                Some(SoConIPduIdentifier::SD_HEADER_ID),
                None,
                Some(PduCollectionTrigger::Always),
            )?;
            let pt = idpu_identifier_mc_rx.pdu_triggering().unwrap();
            ssc.add_ipdu_identifier(&idpu_identifier_mc_rx)?;
            pt
        };
        // add a PduPort for the ecu to the PduTriggering
        pt_multicast_rx.create_pdu_port(ecu, CommunicationDirection::In)?;
        Ok(())
    }

    /// check if the channel contains any SocketConnectionBundles (old) or SocketConnections (very old)
    pub fn has_socket_connections(&self) -> bool {
        if let Some(soad_config) = self.element().get_sub_element(ElementName::SoAdConfig) {
            if let Some(connection_bundles) = soad_config.get_sub_element(ElementName::ConnectionBundles) {
                // does at least one SocketConnectionBundle exist?
                if connection_bundles.sub_elements().count() > 0 {
                    return true;
                }
            }
            if let Some(connections) = soad_config.get_sub_element(ElementName::Connections) {
                // does at least one SocketConnection exist?
                return connections.sub_elements().count() > 0;
            }
        }
        false
    }
}

pub struct CommonServiceDiscoveryConfig<'a> {
    pub multicast_rx_socket: &'a SocketAddress,
    pub multicast_rx_pdu: &'a GeneralPurposePdu,
    pub remote_socket: &'a SocketAddress,
    pub prefer_static_socket_connections: bool,
    pub ipdu_identifier_set: Option<&'a SocketConnectionIpduIdentifierSet>,
    pub name_prefix: Option<&'a str>,
}

//##################################################################

/// A static socket connection is a connection between two sockets.
///
/// This is the new way to establish a connection. It was introduced in Autosar 4.5.0 (AUTOSAR_00048).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticSocketConnection(Element);
abstraction_element!(StaticSocketConnection, StaticSocketConnection);

impl StaticSocketConnection {
    pub(crate) fn new(
        name: &str,
        parent: &Element,
        remote_address: &SocketAddress,
        tcp_role: Option<TcpRole>,
        tcp_connect_timeout: Option<f64>,
    ) -> Result<Self, AutosarAbstractionError> {
        let connections = parent.get_or_create_sub_element(ElementName::StaticSocketConnections)?;
        let ssc = connections.create_named_sub_element(ElementName::StaticSocketConnection, name)?;

        ssc.create_sub_element(ElementName::RemoteAddresss)?
            .create_sub_element(ElementName::SocketAddressRefConditional)?
            .create_sub_element(ElementName::SocketAddressRef)?
            .set_reference_target(remote_address.element())?;

        if let Some(role) = tcp_role {
            ssc.create_sub_element(ElementName::TcpRole)?
                .set_character_data::<EnumItem>(role.into())?;
        }

        if let Some(timeout) = tcp_connect_timeout {
            ssc.create_sub_element(ElementName::TcpConnectTimeout)?
                .set_character_data(timeout)?;
        }

        Ok(Self(ssc))
    }

    /// get the socket address containing this static socket connection
    pub fn socket_address(&self) -> Result<SocketAddress, AutosarAbstractionError> {
        let sa = self.element().named_parent()?.unwrap();
        SocketAddress::try_from(sa)
    }

    /// get the remote socket of this connection
    pub fn remote_socket(&self) -> Option<SocketAddress> {
        let remote_socket = self.element().get_sub_element(ElementName::RemoteAddresss)?;
        SocketAddress::try_from(remote_socket).ok()
    }

    pub fn add_ipdu_identifier(&self, identifier: &SoConIPduIdentifier) -> Result<(), AutosarAbstractionError> {
        let ipdu_identifiers = self.element().get_or_create_sub_element(ElementName::IPduIdentifiers)?;
        let scii = ipdu_identifiers
            .create_sub_element(ElementName::SoConIPduIdentifierRefConditional)?
            .create_sub_element(ElementName::SoConIPduIdentifierRef)?;
        scii.set_reference_target(identifier.element())?;
        Ok(())
    }

    /// create an iterator over all SoConIPduIdentifiers in this static socket connection
    pub fn i_pdu_identifiers(&self) -> impl Iterator<Item = SoConIPduIdentifier> {
        SoConIPduIdentifiersIterator::new(self.element().get_sub_element(ElementName::IPduIdentifiers))
    }
}

//##################################################################

/// A SocketConnectionIpduIdentifierSet contains a set of SoConIPduIdentifiers, which are used in static socket connections and in SomeIp events.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SocketConnectionIpduIdentifierSet(Element);
abstraction_element!(SocketConnectionIpduIdentifierSet, SocketConnectionIpduIdentifierSet);

impl SocketConnectionIpduIdentifierSet {
    /// create a new SocketConnectionIpduIdentifierSet
    ///
    /// The SocketConnectionIpduIdentifierSet is a Fibex element, so this function is not exported in the API.
    /// Users should call System::create_socket_connection_ipdu_identifier_set instead, which also creates the required FibexElementRef.
    pub(crate) fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let sci = package
            .element()
            .get_or_create_sub_element(ElementName::Elements)?
            .create_named_sub_element(ElementName::SocketConnectionIpduIdentifierSet, name)?;
        Ok(Self(sci))
    }

    /// create a new SoConIPduIdentifier in this set
    pub fn create_socon_ipdu_identifier<T: Into<Pdu> + Clone>(
        &self,
        name: &str,
        pdu: &T,
        channel: &EthernetPhysicalChannel,
        header_id: Option<u64>,
        timeout: Option<f64>,
        collection_trigger: Option<PduCollectionTrigger>,
    ) -> Result<SoConIPduIdentifier, AutosarAbstractionError> {
        let ipdu_identifiers = self.element().get_or_create_sub_element(ElementName::IPduIdentifiers)?;
        SoConIPduIdentifier::new(
            name,
            &ipdu_identifiers,
            &pdu.clone().into(),
            channel,
            header_id,
            timeout,
            collection_trigger,
        )
    }

    /// create an iterator over all SoConIPduIdentifiers in this set
    pub fn socon_ipdu_identifiers(&self) -> impl Iterator<Item = SoConIPduIdentifier> {
        self.element()
            .get_sub_element(ElementName::IPduIdentifiers)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .filter_map(|elem| SoConIPduIdentifier::try_from(elem).ok())
    }
}

//##################################################################

/// A SoConIPduIdentifier describes a PDU that is transported over a static socket connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SoConIPduIdentifier(Element);
abstraction_element!(SoConIPduIdentifier, SoConIPduIdentifier);

impl SoConIPduIdentifier {
    /// The PDU header id for SD messages must always be set to 0xFFFF_8100
    pub const SD_HEADER_ID: u64 = 0xFFFF_8100;

    // create a new SoConIPduIdentifier (internal)
    pub(crate) fn new(
        name: &str,
        parent: &Element,
        pdu: &Pdu,
        channel: &EthernetPhysicalChannel,
        header_id: Option<u64>,
        timeout: Option<f64>,
        collection_trigger: Option<PduCollectionTrigger>,
    ) -> Result<Self, AutosarAbstractionError> {
        let scii = Self(parent.create_named_sub_element(ElementName::SoConIPduIdentifier, name)?);

        let pt = PduTriggering::new(pdu, &channel.clone().into())?;
        scii.element()
            .create_sub_element(ElementName::PduTriggeringRef)?
            .set_reference_target(pt.element())?;

        if let Some(header_id) = header_id {
            scii.set_header_id(header_id)?;
        }
        if let Some(timeout) = timeout {
            scii.set_timeout(timeout)?;
        }
        if let Some(collection_trigger) = collection_trigger {
            scii.set_collection_trigger(collection_trigger)?;
        }
        Ok(scii)
    }

    /// create a new PduTriggering for the pdu and reference it in this SoConIPduIdentifier
    pub fn set_pdu<T: Into<Pdu> + Clone>(
        &self,
        pdu: &T,
        channel: &EthernetPhysicalChannel,
    ) -> Result<(), AutosarAbstractionError> {
        let pdu: Pdu = pdu.clone().into();
        self.set_pdu_internal(&pdu, channel)
    }

    fn set_pdu_internal(&self, pdu: &Pdu, channel: &EthernetPhysicalChannel) -> Result<(), AutosarAbstractionError> {
        if let Some(pt_old) = self
            .element()
            .get_sub_element(ElementName::PduTriggeringRef)
            .and_then(|pt| pt.get_reference_target().ok())
        {
            let pt_old = PduTriggering::try_from(pt_old)?;
            if let Some(old_pdu) = pt_old.pdu() {
                if old_pdu == *pdu {
                    return Ok(());
                }
                // remove old pdu
                // -> pt_old.remove();
                todo!("remove() is not implemented yet");
            }
        }
        let pt_new = PduTriggering::new(pdu, &channel.clone().into())?;
        self.element()
            .get_or_create_sub_element(ElementName::PduTriggeringRef)?
            .set_reference_target(pt_new.element())?;
        Ok(())
    }

    /// set the header id for this SoConIPduIdentifier
    pub fn set_header_id(&self, header_id: u64) -> Result<(), AutosarAbstractionError> {
        self.element()
            .create_sub_element(ElementName::HeaderId)?
            .set_character_data(header_id)?;
        Ok(())
    }

    /// set the timeout for this SoConIPduIdentifier
    pub fn set_timeout(&self, timeout: f64) -> Result<(), AutosarAbstractionError> {
        self.element()
            .create_sub_element(ElementName::PduCollectionPduTimeout)?
            .set_character_data(timeout)?;
        Ok(())
    }

    /// set the collection trigger for this SoConIPduIdentifier
    pub fn set_collection_trigger(&self, trigger: PduCollectionTrigger) -> Result<(), AutosarAbstractionError> {
        self.element()
            .create_sub_element(ElementName::PduCollectionTrigger)?
            .set_character_data::<EnumItem>(trigger.into())?;
        Ok(())
    }

    /// get the PduTriggering referenced by this SoConIPduIdentifier
    pub fn pdu_triggering(&self) -> Option<PduTriggering> {
        let pt = self
            .element()
            .get_sub_element(ElementName::PduTriggeringRef)?
            .get_reference_target()
            .ok()?;
        PduTriggering::try_from(pt).ok()
    }

    /// get the header id for this SoConIPduIdentifier
    pub fn header_id(&self) -> Option<u64> {
        self.element()
            .get_sub_element(ElementName::HeaderId)?
            .character_data()?
            .parse_integer()
    }

    /// get the timeout for this SoConIPduIdentifier
    pub fn timeout(&self) -> Option<f64> {
        self.element()
            .get_sub_element(ElementName::PduCollectionPduTimeout)?
            .character_data()?
            .float_value()
    }

    /// get the collection trigger for this SoConIPduIdentifier
    pub fn collection_trigger(&self) -> Option<PduCollectionTrigger> {
        self.element()
            .get_sub_element(ElementName::PduCollectionTrigger)?
            .character_data()?
            .enum_value()?
            .try_into()
            .ok()
    }
}

//##################################################################

/// The role of a TCP connection in a static socket connection cna either be `Connect` (=client) or `Listen` (=server).
pub enum TcpRole {
    Connect,
    Listen,
}

impl TryFrom<EnumItem> for TcpRole {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<Self, Self::Error> {
        match value {
            EnumItem::Listen => Ok(Self::Listen),
            EnumItem::Connect => Ok(Self::Connect),

            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "TcpRole".to_string(),
            }),
        }
    }
}

impl From<TcpRole> for EnumItem {
    fn from(value: TcpRole) -> Self {
        match value {
            TcpRole::Listen => EnumItem::Listen,
            TcpRole::Connect => EnumItem::Connect,
        }
    }
}

//##################################################################

// control types used in routing groups for SOME/IP events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventGroupControlType {
    ActivationAndTriggerUnicast,
    ActivationMulticast,
    ActivationUnicast,
    TriggerUnicast,
}

impl From<EventGroupControlType> for EnumItem {
    fn from(value: EventGroupControlType) -> Self {
        match value {
            EventGroupControlType::ActivationAndTriggerUnicast => EnumItem::ActivationAndTriggerUnicast,
            EventGroupControlType::ActivationMulticast => EnumItem::ActivationMulticast,
            EventGroupControlType::ActivationUnicast => EnumItem::ActivationUnicast,
            EventGroupControlType::TriggerUnicast => EnumItem::TriggerUnicast,
        }
    }
}

impl TryFrom<EnumItem> for EventGroupControlType {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<Self, Self::Error> {
        match value {
            EnumItem::ActivationAndTriggerUnicast => Ok(Self::ActivationAndTriggerUnicast),
            EnumItem::ActivationMulticast => Ok(Self::ActivationMulticast),
            EnumItem::ActivationUnicast => Ok(Self::ActivationUnicast),
            EnumItem::TriggerUnicast => Ok(Self::TriggerUnicast),

            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "EventGroupControlType".to_string(),
            }),
        }
    }
}

//##################################################################

element_iterator!(NetworkEndpointIterator, NetworkEndpoint, Some);

//##################################################################

element_iterator!(SocketAddressIterator, SocketAddress, Some);

//##################################################################

element_iterator!(
    SoConIPduIdentifiersIterator,
    SoConIPduIdentifier,
    (|scirc: Element| scirc
        .get_sub_element(ElementName::SoConIPduIdentifierRef)
        .and_then(|sciir| sciir.get_reference_target().ok()))
);

//##################################################################

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ArPackage, System, SystemCategory};
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn channel() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();
        let cluster = system.create_ethernet_cluster("EthCluster", &pkg).unwrap();

        let channel = cluster.create_physical_channel("channel_name", None).unwrap();
        let c2 = channel.cluster().unwrap();
        assert_eq!(cluster, c2);

        let vi = channel.get_vlan_info();
        assert!(vi.is_none());

        let elem_vlan = channel
            .element()
            .create_named_sub_element(ElementName::Vlan, "VLAN_1")
            .unwrap();
        let vi = channel.get_vlan_info();
        assert!(vi.is_none());

        let elem_vlanid = elem_vlan.create_sub_element(ElementName::VlanIdentifier).unwrap();
        let vi = channel.get_vlan_info();
        assert!(vi.is_none());

        elem_vlanid.set_character_data(1).unwrap();
        let vi = channel.get_vlan_info().unwrap();
        assert_eq!(vi.vlan_id, 1);
    }
}
