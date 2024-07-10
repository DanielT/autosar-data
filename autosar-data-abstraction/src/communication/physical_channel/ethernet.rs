use crate::communication::{
    EthernetCluster, NetworkEndpoint, NetworkEndpointAddress, Pdu, PduCollectionTrigger, PduTriggering,
    PhysicalChannel, SocketAddress, SocketAddressType, TpConfig,
};
use crate::{abstraction_element, element_iterator, AbstractionElement, AutosarAbstractionError, EcuInstance};
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

    /// create a static connection between two sockets (v1)
    ///
    /// The static connection transports PDUs between two sockets.
    ///
    /// The Autosar standard specified two different ways to establish a connection.
    /// This function creates a connection using the first method, using SOCKET-CONNECTION-BUNDLEs.
    ///
    /// Using SOCKET-CONNECTION-BUNDLEs was deprecated after Autosar 4.4.0, but remains available for compatibility.
    /// If the file version is <= AUTOSAR_00046 then this is the only available method.
    /// In newer files it is important to never mix the two methods, since this is explicitly forbidden by the standard.
    pub fn create_static_connection_v1(
        &self,
        name: &str,
        server: &SocketAddress,
        client: &SocketAddress,
        pdu_config: &Vec<SoConnIPdu>,
    ) -> Result<(), AutosarAbstractionError> {
        let soadcfg = self.0.get_or_create_sub_element(ElementName::SoAdConfig)?;
        let connections = soadcfg.get_or_create_sub_element(ElementName::ConnectionBundles)?;
        let scb = connections.create_named_sub_element(ElementName::SocketConnectionBundle, name)?;

        scb.create_sub_element(ElementName::ServerPortRef)?
            .set_reference_target(server.element())?;
        let conn = scb
            .create_sub_element(ElementName::BundledConnections)?
            .create_sub_element(ElementName::SocketConnection)?;
        conn.create_sub_element(ElementName::ClientPortRef)?
            .set_reference_target(client.element())?;

        let channel = PhysicalChannel::Ethernet(self.clone());
        let pdus = conn.create_sub_element(ElementName::Pdus)?;
        for pdu_cfg in pdu_config {
            let scii = pdus.create_sub_element(ElementName::SocketConnectionIpduIdentifier)?;
            scii.create_sub_element(ElementName::HeaderId)?
                .set_character_data(pdu_cfg.header_id.to_string())?;
            if let Some(timeout) = &pdu_cfg.timeout {
                scii.create_sub_element(ElementName::PduCollectionPduTimeout)?
                    .set_character_data(*timeout)?;
            }
            if let Some(collection_trigger) = &pdu_cfg.collection_trigger {
                scii.create_sub_element(ElementName::PduCollectionTrigger)?
                    .set_character_data::<EnumItem>((*collection_trigger).into())?;
            }

            let pt = PduTriggering::new(&pdu_cfg.pdu, &channel)?;
            scii.create_sub_element(ElementName::PduTriggeringRef)?
                .set_reference_target(pt.element())?;

            todo!("Pdu ports")
        }

        todo!()
    }

    /// create a static connection between two sockets (v2)
    ///
    /// The static connection transports PDUs between two sockets.
    ///
    /// The Autosar standard specified two different ways to establish a connection.
    /// This function creates a connection using the second method, using STATIC-SOCKET-CONNECTIONs.
    ///
    /// STATIC-SOCKET-CONNECTIONs were introduced in Autosar 4.5.0 (AUTOSAR_00048).
    /// If the file version is <= AUTOSAR_00048 then this method always fails.
    pub fn create_static_connection_v2(
        &self,
        name: &str,
        server: &SocketAddress,
        client: &SocketAddress,
        pdu_config: &Vec<SoConnIPdu>,
    ) -> Result<(), AutosarAbstractionError> {
        //
        todo!()
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq)]
pub struct SoConnIPdu {
    pub pdu: Pdu,
    pub header_id: u32,
    pub timeout: Option<f64>,
    pub collection_trigger: Option<PduCollectionTrigger>,
}

//##################################################################

element_iterator!(NetworkEndpointIterator, NetworkEndpoint, Some);

//##################################################################

element_iterator!(SocketAddressIterator, SocketAddress, Some);

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
