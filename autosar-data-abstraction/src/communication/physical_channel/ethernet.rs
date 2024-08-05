use crate::communication::{
    EthernetCluster, NetworkEndpoint, NetworkEndpointAddress, Pdu, PduCollectionTrigger, PduTriggering,
    PhysicalChannel, SocketAddress, SocketAddressType, TpConfig,
};
use crate::{
    abstraction_element, element_iterator, AbstractionElement, ArPackage, AutosarAbstractionError, EcuInstance,
};
use autosar_data::{AutosarVersion, Element, ElementName, EnumItem};

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
    #[must_use]
    pub fn network_endpoints(&self) -> NetworkEndpointIterator {
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
    pub fn socket_addresses(&self) -> SocketAddressIterator {
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
    /// Static socket connections must always be created as a pair, one on each socket involved on the connection.
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
    /// let (connection_a, connection_b) = channel.create_static_socket_connection("Connection", &server_socket, &client_socket, None).unwrap();
    /// ```
    pub fn create_static_socket_connection(
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
}

//##################################################################

/// A SocketConnectionBundle describes a connection between a server port and multiple client ports.
/// It contains multiple bundled connections, each transporting one or more PDUs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SocketConnectionBundle(Element);
abstraction_element!(SocketConnectionBundle, SocketConnectionBundle);

impl SocketConnectionBundle {
    pub(crate) fn new(
        name: &str,
        server_port: &SocketAddress,
        connections_elem: &Element,
    ) -> Result<Self, AutosarAbstractionError> {
        let scb = connections_elem.create_named_sub_element(ElementName::SocketConnectionBundle, name)?;

        scb.create_sub_element(ElementName::ServerPortRef)?
            .set_reference_target(server_port.element())?;

        Ok(Self(scb))
    }

    /// get the physical channel containing this socket connection bundle
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
    /// # let server_endpoint = channel.create_network_endpoint("ServerAddress", NetworkEndpointAddress::IPv4 {
    /// #    address: Some("192.168.0.1".to_string()),
    /// #    address_source: Some(IPv4AddressSource::Fixed),
    /// #    default_gateway: None,
    /// #    network_mask: None
    /// # }, None).unwrap();
    /// # let server_socket = channel.create_socket_address("ServerSocket", &server_endpoint, &TpConfig::TcpTp { port_number: Some(1234), port_dynamically_assigned: None }, SocketAddressType::Unicast(None)).unwrap();
    /// # let client_endpoint = channel.create_network_endpoint("ClientAddress", NetworkEndpointAddress::IPv4 {
    /// #    address: Some("192.168.0.2".to_string()),
    /// #    address_source: Some(IPv4AddressSource::Fixed),
    /// #    default_gateway: None,
    /// #    network_mask: None
    /// # }, None).unwrap();
    /// # let client_socket = channel.create_socket_address("ClientSocket", &client_endpoint, &TpConfig::TcpTp { port_number: Some(1235), port_dynamically_assigned: None }, SocketAddressType::Unicast(None)).unwrap();
    /// let bundle = channel.create_socket_connection_bundle("Bundle", &server_socket).unwrap();
    /// assert_eq!(channel, bundle.get_physical_channel().unwrap());
    /// ```
    pub fn get_physical_channel(&self) -> Result<EthernetPhysicalChannel, AutosarAbstractionError> {
        let channel = self.element().named_parent()?.unwrap();
        EthernetPhysicalChannel::try_from(channel)
    }

    /// get the server port of this socket connection bundle
    pub fn server_port(&self) -> Option<SocketAddress> {
        self.element()
            .get_sub_element(ElementName::ServerPortRef)
            .and_then(|spr| spr.get_reference_target().ok())
            .and_then(|sp| SocketAddress::try_from(sp).ok())
    }

    /// create a bundled SocketConnection between the server port and a client port
    pub fn create_bundled_connection(
        &self,
        client_port: &SocketAddress,
    ) -> Result<SocketConnection, AutosarAbstractionError> {
        let server_port = self.server_port().ok_or_else(|| {
            AutosarAbstractionError::InvalidParameter("SocketConnectionBundle has no server port".to_string())
        })?;
        let own_tp_config = server_port.get_tp_config();
        let remote_tp_config = client_port.get_tp_config();
        match (own_tp_config, remote_tp_config) {
            (Some(TpConfig::TcpTp { .. }), Some(TpConfig::TcpTp { .. }))
            | (Some(TpConfig::UdpTp { .. }), Some(TpConfig::UdpTp { .. }))
            | (None, None) => { /* ok */ }
            _ => {
                return Err(AutosarAbstractionError::InvalidParameter(
                    "Both SocketAddresses must use the same transport protocol".to_string(),
                ))
            }
        }
        let conn = self
            .0
            .get_or_create_sub_element(ElementName::BundledConnections)?
            .get_or_create_sub_element(ElementName::SocketConnection)?;

        SocketConnection::new(conn, client_port)
    }
}

//##################################################################

/// A socketConnection inside a SocketConnectionBundle describes a single connection to a specific client port.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SocketConnection(Element);
abstraction_element!(SocketConnection, SocketConnection);

impl SocketConnection {
    pub(crate) fn new(conn: Element, client_port: &SocketAddress) -> Result<Self, AutosarAbstractionError> {
        conn.create_sub_element(ElementName::ClientPortRef)?
            .set_reference_target(client_port.element())?;

        Ok(Self(conn))
    }

    /// get the socket connection bundle containing this socket connection
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
    /// # let server_endpoint = channel.create_network_endpoint("ServerAddress", NetworkEndpointAddress::IPv4 {
    /// #    address: Some("192.168.0.1".to_string()),
    /// #    address_source: Some(IPv4AddressSource::Fixed),
    /// #    default_gateway: None,
    /// #    network_mask: None
    /// # }, None).unwrap();
    /// # let server_socket = channel.create_socket_address("ServerSocket", &server_endpoint, &TpConfig::TcpTp { port_number: Some(1234), port_dynamically_assigned: None }, SocketAddressType::Unicast(None)).unwrap();
    /// # let client_endpoint = channel.create_network_endpoint("ClientAddress", NetworkEndpointAddress::IPv4 {
    /// #    address: Some("192.168.0.2".to_string()),
    /// #    address_source: Some(IPv4AddressSource::Fixed),
    /// #    default_gateway: None,
    /// #    network_mask: None
    /// # }, None).unwrap();
    /// # let client_socket = channel.create_socket_address("ClientSocket", &client_endpoint, &TpConfig::TcpTp { port_number: Some(1235), port_dynamically_assigned: None }, SocketAddressType::Unicast(None)).unwrap();
    /// let bundle = channel.create_socket_connection_bundle("Bundle", &server_socket).unwrap();
    /// let connection = bundle.create_bundled_connection(&client_socket).unwrap();
    /// assert_eq!(bundle, connection.get_socket_connection_bundle().unwrap());
    /// ```
    pub fn get_socket_connection_bundle(&self) -> Result<SocketConnectionBundle, AutosarAbstractionError> {
        let bundle = self.element().named_parent()?.unwrap();
        SocketConnectionBundle::try_from(bundle)
    }

    /// add a PDU to the socket connection, returning a PduTriggering
    pub fn add_pdu(
        &self,
        pdu: &Pdu,
        header_id: u32,
        timeout: Option<f64>,
        collection_trigger: Option<PduCollectionTrigger>,
    ) -> Result<PduTriggering, AutosarAbstractionError> {
        let scii = self
            .0
            .get_or_create_sub_element(ElementName::Pdus)?
            .create_sub_element(ElementName::SocketConnectionIpduIdentifier)?;
        scii.create_sub_element(ElementName::HeaderId)?
            .set_character_data(header_id.to_string())?;
        if let Some(timeout) = timeout {
            scii.create_sub_element(ElementName::PduCollectionPduTimeout)?
                .set_character_data(timeout)?;
        }
        if let Some(collection_trigger) = collection_trigger {
            scii.create_sub_element(ElementName::PduCollectionTrigger)?
                .set_character_data::<EnumItem>(collection_trigger.into())?;
        }

        let pt = PduTriggering::new(
            pdu,
            &self.get_socket_connection_bundle()?.get_physical_channel()?.into(),
        )?;
        scii.create_sub_element(ElementName::PduTriggeringRef)?
            .set_reference_target(pt.element())?;

        Ok(pt)
    }

    /// set the header id for a PDU in this socket connection
    pub fn set_header_id(&self, pdu_triggering: &PduTriggering, header_id: u64) -> Result<(), AutosarAbstractionError> {
        for scii in self
            .element()
            .get_or_create_sub_element(ElementName::Pdus)?
            .sub_elements()
        {
            if let Some(pt_ref) = scii
                .get_sub_element(ElementName::PduTriggeringRef)
                .and_then(|ptref| ptref.get_reference_target().ok())
                .and_then(|pt| PduTriggering::try_from(pt).ok())
            {
                if pt_ref == *pdu_triggering {
                    scii.get_or_create_sub_element(ElementName::HeaderId)?
                        .set_character_data(header_id)?;
                }
            }
        }
        Ok(())
    }

    /// get the header id for a PDU in this socket connection
    pub fn header_id(&self, pdu_triggering: &PduTriggering) -> Option<u64> {
        for scii in self.element().get_sub_element(ElementName::Pdus)?.sub_elements() {
            if let Some(pt_ref) = scii
                .get_sub_element(ElementName::PduTriggeringRef)
                .and_then(|ptref| ptref.get_reference_target().ok())
                .and_then(|pt| PduTriggering::try_from(pt).ok())
            {
                if pt_ref == *pdu_triggering {
                    return scii
                        .get_sub_element(ElementName::HeaderId)?
                        .character_data()?
                        .parse_integer();
                }
            }
        }
        None
    }

    /// set the timeout for a PDU in this socket connection
    pub fn set_timeout(&self, pdu_triggering: &PduTriggering, timeout: f64) -> Result<(), AutosarAbstractionError> {
        for scii in self
            .element()
            .get_or_create_sub_element(ElementName::Pdus)?
            .sub_elements()
        {
            if let Some(pt_ref) = scii
                .get_sub_element(ElementName::PduTriggeringRef)
                .and_then(|ptref| ptref.get_reference_target().ok())
                .and_then(|pt| PduTriggering::try_from(pt).ok())
            {
                if pt_ref == *pdu_triggering {
                    scii.get_or_create_sub_element(ElementName::PduCollectionPduTimeout)?
                        .set_character_data(timeout)?;
                }
            }
        }
        Ok(())
    }

    /// get the timeout for a PDU in this socket connection
    pub fn timeout(&self, pdu_triggering: &PduTriggering) -> Option<f64> {
        for scii in self.element().get_sub_element(ElementName::Pdus)?.sub_elements() {
            if let Some(pt_ref) = scii
                .get_sub_element(ElementName::PduTriggeringRef)
                .and_then(|ptref| ptref.get_reference_target().ok())
                .and_then(|pt| PduTriggering::try_from(pt).ok())
            {
                if pt_ref == *pdu_triggering {
                    return scii
                        .get_sub_element(ElementName::PduCollectionPduTimeout)?
                        .character_data()?
                        .float_value();
                }
            }
        }
        None
    }

    /// set the collection trigger for a PDU in this socket connection
    pub fn set_collection_trigger(
        &self,
        pdu_triggering: &PduTriggering,
        trigger: PduCollectionTrigger,
    ) -> Result<(), AutosarAbstractionError> {
        for scii in self
            .element()
            .get_or_create_sub_element(ElementName::Pdus)?
            .sub_elements()
        {
            if let Some(pt_ref) = scii
                .get_sub_element(ElementName::PduTriggeringRef)
                .and_then(|ptref| ptref.get_reference_target().ok())
                .and_then(|pt| PduTriggering::try_from(pt).ok())
            {
                if pt_ref == *pdu_triggering {
                    scii.get_or_create_sub_element(ElementName::PduCollectionTrigger)?
                        .set_character_data::<EnumItem>(trigger.into())?;
                }
            }
        }
        Ok(())
    }

    /// get the collection trigger for a PDU in this socket connection
    pub fn collection_trigger(&self, pdu_triggering: &PduTriggering) -> Option<PduCollectionTrigger> {
        for scii in self.element().get_sub_element(ElementName::Pdus)?.sub_elements() {
            if let Some(pt_ref) = scii
                .get_sub_element(ElementName::PduTriggeringRef)
                .and_then(|ptref| ptref.get_reference_target().ok())
                .and_then(|pt| PduTriggering::try_from(pt).ok())
            {
                if pt_ref == *pdu_triggering {
                    return scii
                        .get_sub_element(ElementName::PduCollectionTrigger)?
                        .character_data()?
                        .enum_value()?
                        .try_into()
                        .ok();
                }
            }
        }
        None
    }

    /// create an iterator over all PDU triggerings in this socket connection
    pub fn pdu_triggerings(&self) -> SCPduTriggeringsIterator {
        SCPduTriggeringsIterator::new(self.element().get_sub_element(ElementName::Pdus))
    }
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

    /// get the remote port of this connection
    pub fn remote_port(&self) -> Option<SocketAddress> {
        let remote_port = self.element().get_sub_element(ElementName::RemoteAddresss)?;
        SocketAddress::try_from(remote_port).ok()
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
    pub fn i_pdu_identifiers(&self) -> SoConIPduIdentifiersIterator {
        SoConIPduIdentifiersIterator::new(self.element().get_sub_element(ElementName::IPduIdentifiers))
    }
}

//##################################################################

/// A SocketConnectionIpduIdentifierSet contains a set of SoConIPduIdentifiers, which are used in static socket connections.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SocketConnectionIpduIdentifierSet(Element);
abstraction_element!(SocketConnectionIpduIdentifierSet, SocketConnectionIpduIdentifierSet);

impl SocketConnectionIpduIdentifierSet {
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let sci = package
            .element()
            .get_or_create_sub_element(ElementName::Elements)?
            .create_named_sub_element(ElementName::SocketConnectionIpduIdentifierSet, name)?;
        Ok(Self(sci))
    }

    /// create a new SoConIPduIdentifier in this set
    pub fn create_socon_ipdu_identifier(
        &self,
        name: &str,
        pdu: &Pdu,
        channel: &EthernetPhysicalChannel,
        header_id: Option<u64>,
        timeout: Option<f64>,
        collection_trigger: Option<PduCollectionTrigger>,
    ) -> Result<SoConIPduIdentifier, AutosarAbstractionError> {
        let ipdu_identifiers = self.element().get_or_create_sub_element(ElementName::IPduIdentifiers)?;
        SoConIPduIdentifier::new(
            name,
            &ipdu_identifiers,
            pdu,
            channel,
            header_id,
            timeout,
            collection_trigger,
        )
    }
}

//##################################################################

/// A SoConIPduIdentifier describes a PDU that is transported over a static socket connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SoConIPduIdentifier(Element);
abstraction_element!(SoConIPduIdentifier, SoConIPduIdentifier);

impl SoConIPduIdentifier {
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
    pub fn set_pdu(&self, pdu: &Pdu, channel: &EthernetPhysicalChannel) -> Result<(), AutosarAbstractionError> {
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

element_iterator!(NetworkEndpointIterator, NetworkEndpoint, Some);

//##################################################################

element_iterator!(SocketAddressIterator, SocketAddress, Some);

//##################################################################

element_iterator!(
    SCPduTriggeringsIterator,
    PduTriggering,
    (|scii: Element| scii
        .get_sub_element(ElementName::PduTriggeringRef)
        .and_then(|pt| pt.get_reference_target().ok()))
);

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
