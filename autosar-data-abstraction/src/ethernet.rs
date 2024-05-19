use crate::{abstraction_element, AbstractionElement, ArPackage, AutosarAbstractionError, EcuInstance};
use autosar_data::{
    AutosarDataError, AutosarModel, AutosarVersion, CharacterData, Element, ElementName, ElementsIterator, EnumItem,
    WeakElement,
};

/// Provides information about the VLAN of an [`EthernetPhysicalChannel`]
#[derive(Debug, Clone, PartialEq)]
pub struct EthernetVlanInfo {
    pub vlan_name: String,
    pub vlan_id: u16,
}

/// An `EthernetCluster` contains all configuration items associated with an ethernet network.
/// The cluster connects multiple ECUs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EthernetCluster(Element);
abstraction_element!(EthernetCluster, EthernetCluster);

impl EthernetCluster {
    // create a new EthernetCluster - for internal use. User code should call System::create_ethernet_cluster
    pub(crate) fn new(cluster_name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elem_pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_cluster = elem_pkg_elements.create_named_sub_element(ElementName::EthernetCluster, cluster_name)?;
        if let Ok(cluster_content) = elem_cluster
            .create_sub_element(ElementName::EthernetClusterVariants)
            .and_then(|ecv| ecv.create_sub_element(ElementName::EthernetClusterConditional))
        {
            let _ = cluster_content.create_sub_element(ElementName::PhysicalChannels);
        }

        Ok(EthernetCluster(elem_cluster))
    }

    /// Create a new physical channel for the cluster
    ///
    /// The supplied VLAN info must be unique - there cannot be two VLANs with the same vlan identifier.
    /// One channel may be created without VLAN information; it carries untagged traffic.
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
    /// let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// let vlan_info = EthernetVlanInfo {
    ///     vlan_name: "VLAN_1".to_string(),
    ///     vlan_id: 1,
    /// };
    /// let channel = cluster.create_physical_channel("Channel", Some(vlan_info)).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ItemAlreadyExists`] There is already a physical channel for this VLAN
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn create_physical_channel(
        &self,
        channel_name: &str,
        vlan_info: Option<EthernetVlanInfo>,
    ) -> Result<EthernetPhysicalChannel, AutosarAbstractionError> {
        let phys_channels = self
            .0
            .get_or_create_sub_element(ElementName::EthernetClusterVariants)?
            .get_or_create_sub_element(ElementName::EthernetClusterConditional)?
            .get_or_create_sub_element(ElementName::PhysicalChannels)?;

        // make sure there is no other channel with the same VLAN info
        // If vlan_info is None, then there must be no existing channel without VLAN info
        for existing_channel in phys_channels.sub_elements() {
            let existing_vlan_info = EthernetPhysicalChannel(existing_channel).get_vlan_info();
            if let (Some(v1), Some(v2)) = (&vlan_info, &existing_vlan_info) {
                if v1.vlan_id == v2.vlan_id {
                    // the vlan identifier of an existing channel matches the new vlan identifier
                    return Err(AutosarAbstractionError::ItemAlreadyExists);
                }
            } else if existing_vlan_info.is_none() && vlan_info.is_none() {
                // the new channel is for untagged traffic (no VLAN), but there is already a channel for untagged traffic
                return Err(AutosarAbstractionError::ItemAlreadyExists);
            }
        }

        let channel = phys_channels.create_named_sub_element(ElementName::EthernetPhysicalChannel, channel_name)?;
        // set the vlan info
        if let Some(vlan_info) = vlan_info {
            let _ = channel
                .create_named_sub_element(ElementName::Vlan, &vlan_info.vlan_name)
                .and_then(|vlan| vlan.create_sub_element(ElementName::VlanIdentifier))
                .and_then(|vlan_id| vlan_id.set_character_data(CharacterData::String(vlan_info.vlan_id.to_string())));
        }
        // always set CATEGORY = WIRED, since this is the common case
        let _ = channel
            .create_sub_element(ElementName::Category)
            .and_then(|cat| cat.set_character_data(CharacterData::String("WIRED".to_string())));

        Ok(EthernetPhysicalChannel(channel))
    }

    /// returns an iterator over all [`EthernetPhysicalChannel`]s in the cluster
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
    /// cluster.create_physical_channel("Channel", None).unwrap();
    /// for channel in cluster.physical_channels() {
    ///     // ...
    /// }
    /// ```
    #[must_use]
    pub fn physical_channels(&self) -> EthernetClusterChannelsIterator {
        EthernetClusterChannelsIterator::new(self)
    }
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
                let ne_element = network_endpoint.element();

                // get a connector referenced by this physical channel which is contained in the ecu_instance
                if let Some(connector) = self.0.get_sub_element(ElementName::CommConnectors).and_then(|cc| {
                    cc.sub_elements().find(|ccrc| {
                        ccrc.get_sub_element(ElementName::CommunicationConnectorRef)
                            .and_then(|ccr| {
                                ccr.get_reference_target()
                                    .and_then(|r| r.named_parent())
                                    .ok()
                                    .flatten()
                                    .map(|p| p == ecu_instance.element())
                            })
                            .unwrap_or(false)
                    })
                }) {
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

    /// create an iterator over all `NetworkEnpoint`s in this channel
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
        NetworkEndpointIterator::new(self)
    }
}

//##################################################################

/// An `EcuInstance` needs an `EthernetCommunicationController` in order to connect to an ethernet cluster.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EthernetCommunicationController(Element);
abstraction_element!(EthernetCommunicationController, EthernetCommunicationController);

impl EthernetCommunicationController {
    // create an EthernetCommunicationController
    pub(crate) fn new(
        name: &str,
        ecu: &EcuInstance,
        mac_address: Option<String>,
    ) -> Result<Self, AutosarAbstractionError> {
        let commcontrollers = ecu.element().get_or_create_sub_element(ElementName::CommControllers)?;
        let ctrl = commcontrollers.create_named_sub_element(ElementName::EthernetCommunicationController, name)?;
        let ethccc = ctrl
            .create_sub_element(ElementName::EthernetCommunicationControllerVariants)?
            .create_sub_element(ElementName::EthernetCommunicationControllerConditional)?;
        if let Some(mac_address) = mac_address {
            // creating the mac address element fails if the supplied string has an invalid format
            let result = ethccc
                .create_sub_element(ElementName::MacUnicastAddress)
                .and_then(|mua| mua.set_character_data(CharacterData::String(mac_address)));
            if let Err(mac_address_error) = result {
                let _ = commcontrollers.remove_sub_element(ctrl);
                return Err(mac_address_error.into());
            }
        }
        let coupling_port_name = format!("{name}_CouplingPort");
        let _ = ethccc
            .create_sub_element(ElementName::CouplingPorts)
            .and_then(|cps| cps.create_named_sub_element(ElementName::CouplingPort, &coupling_port_name));

        Ok(Self(ctrl))
    }

    /// return an iterator over the [`EthernetPhysicalChannel`]s connected to this controller
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
    /// # let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let ethernet_controller = ecu_instance.create_ethernet_communication_controller("EthCtrl", None).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// # let physical_channel = cluster.create_physical_channel("Channel", None).unwrap();
    /// ethernet_controller.connect_physical_channel("connection", &physical_channel).unwrap();
    /// for channel in ethernet_controller.connected_channels() {
    ///     // ...
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    #[must_use]
    pub fn connected_channels(&self) -> EthernetCtrlChannelsIterator {
        if let Ok(ecu) = self.ecu_instance().map(|ecuinstance| ecuinstance.element()) {
            EthernetCtrlChannelsIterator::new(self, &ecu)
        } else {
            EthernetCtrlChannelsIterator {
                connector_iter: None,
                comm_controller: self.0.clone(),
                model: Err(AutosarDataError::ElementNotFound),
            }
        }
    }

    /// get the `EcuInstance` that contains this `EthernetCommunicationController`
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
    /// # let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let ethernet_controller = ecu_instance.create_ethernet_communication_controller("EthCtrl", None).unwrap();
    /// assert_eq!(ecu_instance, ethernet_controller.ecu_instance().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn ecu_instance(&self) -> Result<EcuInstance, AutosarAbstractionError> {
        // unwrapping is safe here - self.0.named_parent() cannot return Ok(None).
        // the EthernetCommunicationController is always a child of an EcuInstance,
        // or else it is deleted and named_parent() return Err(...), which is handled by the ?
        let ecu: Element = self.0.named_parent()?.unwrap();
        EcuInstance::try_from(ecu)
    }

    /// Connect this [`EthernetCommunicationController`] inside an [`EcuInstance`] to an [`EthernetPhysicalChannel`] in the [`crate::System`]
    ///
    /// Creates an EthernetCommunicationConnector in the [`EcuInstance`] that contains this [`EthernetCommunicationController`].
    ///
    /// This function establishes the relationships:
    ///  - [`EthernetPhysicalChannel`] -> EthernetCommunicationConnector
    ///  - EthernetCommunicationConnector -> [`EthernetCommunicationController`]
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
    /// # let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let ethernet_controller = ecu_instance.create_ethernet_communication_controller("EthCtrl", None).unwrap();
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// # let physical_channel = cluster.create_physical_channel("Channel", None).unwrap();
    /// ethernet_controller.connect_physical_channel("connection", &physical_channel).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn connect_physical_channel(
        &self,
        connection_name: &str,
        eth_channel: &EthernetPhysicalChannel,
    ) -> Result<(), AutosarAbstractionError> {
        let ecu: Element = self.0.named_parent()?.unwrap();
        let cluster_of_channel = eth_channel.cluster()?;

        // There can be multiple connectors referring to a single EthernetCommunicationController,
        // but all of these connectors must refer to different PhysicalChannels
        // (= VLANs) of the same EthernetCluster.
        for phys_channel in self.connected_channels() {
            if phys_channel == *eth_channel {
                return Err(AutosarAbstractionError::ItemAlreadyExists);
            }

            if phys_channel.cluster()? != cluster_of_channel {
                return Err(AutosarAbstractionError::InvalidParameter(
                    "The EthernetCommunicationController may only refer to different channels within the same cluster"
                        .to_string(),
                ));
            }
        }

        // create a new connector
        let connectors = ecu.get_or_create_sub_element(ElementName::Connectors)?;
        let connector =
            connectors.create_named_sub_element(ElementName::EthernetCommunicationConnector, connection_name)?;
        connector
            .create_sub_element(ElementName::CommControllerRef)
            .and_then(|refelem| refelem.set_reference_target(&self.0))?;

        // if the ethernet physical channel has a category (WIRED / WIRELESS / CANXL) then
        // set the category of the connector to the same value
        if let Some(category) = eth_channel
            .element()
            .get_sub_element(ElementName::Category)
            .and_then(|cat| cat.character_data())
            .and_then(|cdata| cdata.string_value())
        {
            let _ = connector
                .create_sub_element(ElementName::Category)
                .and_then(|cat| cat.set_character_data(CharacterData::String(category)));
        }

        // create a communication connector ref in the ethernet channel that refers to this connector
        let channel_connctor_refs = eth_channel
            .element()
            .get_or_create_sub_element(ElementName::CommConnectors)?;
        channel_connctor_refs
            .create_sub_element(ElementName::CommunicationConnectorRefConditional)
            .and_then(|ccrc| ccrc.create_sub_element(ElementName::CommunicationConnectorRef))
            .and_then(|ccr| ccr.set_reference_target(&connector))?;

        // if the PhysicalChannel has VLAN info AND if there is a coupling port in this CommunicationController
        // then the coupling port should link to the PhysicalChannel / VLAN
        if let Some(EthernetVlanInfo { .. }) = eth_channel.get_vlan_info() {
            if let Some(coupling_port) = self
                .0
                .get_sub_element(ElementName::EthernetCommunicationControllerVariants)
                .and_then(|eccv| eccv.get_sub_element(ElementName::EthernetCommunicationControllerConditional))
                .and_then(|eccc| eccc.get_sub_element(ElementName::CouplingPorts))
                .and_then(|cps| cps.get_sub_element(ElementName::CouplingPort))
            {
                coupling_port
                    .get_or_create_sub_element(ElementName::VlanMemberships)
                    .and_then(|vms| vms.create_sub_element(ElementName::VlanMembership))
                    .and_then(|vm| vm.create_sub_element(ElementName::VlanRef))
                    .and_then(|vr| vr.set_reference_target(&eth_channel.element()))?;
            }
        }

        Ok(())
    }
}

//##################################################################

/// A network endpoint contains address information for a connection
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkEndpoint(Element);
abstraction_element!(NetworkEndpoint, NetworkEndpoint);

impl NetworkEndpoint {
    fn new(
        name: &str,
        channel: &EthernetPhysicalChannel,
        address: NetworkEndpointAddress,
    ) -> Result<Self, AutosarAbstractionError> {
        let el_network_endpoint = channel
            .0
            .get_or_create_sub_element(ElementName::NetworkEndpoints)?
            .create_named_sub_element(ElementName::NetworkEndpoint, name)?;

        let network_endpoint = Self(el_network_endpoint);
        let result = network_endpoint.add_network_endpoint_address(address);
        if let Err(error) = result {
            let _ = channel.0.remove_sub_element(network_endpoint.0);
            return Err(error);
        }

        Ok(network_endpoint)
    }

    /// add a network endpoint address to this `NetworkEndpoint`
    ///
    /// A `NetworkEndpoint` may have multiple sets of address information. The following restrictions apply:
    ///
    /// - all addresses must have the same type, i.e. all IPv4 or all IPv6
    /// - only one of them may be a `Fixed` address, all others must be dynamic (DHCP, automatic link local, etc.)
    pub fn add_network_endpoint_address(&self, address: NetworkEndpointAddress) -> Result<(), AutosarAbstractionError> {
        let mut fixedcount = 0;
        if matches!(address, NetworkEndpointAddress::IPv4 { address_source, .. } if address_source == Some(IPv4AddressSource::Fixed))
            || matches!(address, NetworkEndpointAddress::IPv6 { address_source, .. } if address_source == Some(IPv6AddressSource::Fixed))
        {
            fixedcount = 1;
        }
        for existing_address in self.addresses() {
            if std::mem::discriminant(&existing_address) != std::mem::discriminant(&address) {
                return Err(AutosarAbstractionError::InvalidParameter(
                    "you cannot mix IPv4 and IPv6 inside one NetworkEndpoint".to_string(),
                ));
            }
            if matches!(existing_address, NetworkEndpointAddress::IPv4 { address_source, .. } if address_source == Some(IPv4AddressSource::Fixed))
                || matches!(existing_address, NetworkEndpointAddress::IPv6 { address_source, .. } if address_source == Some(IPv6AddressSource::Fixed))
            {
                fixedcount += 1;
            }
        }
        if fixedcount > 1 {
            return Err(AutosarAbstractionError::InvalidParameter(
                "Only one NetworkEndpointAddress can be a fixed address".to_string(),
            ));
        }

        let addresses = self
            .0
            .get_or_create_sub_element(ElementName::NetworkEndpointAddresses)?;
        match address {
            NetworkEndpointAddress::IPv4 {
                address,
                address_source,
                default_gateway,
                network_mask,
            } => {
                let cfg = addresses.create_sub_element(ElementName::Ipv4Configuration)?;
                if let Some(addr) = address {
                    cfg.create_sub_element(ElementName::Ipv4Address)?
                        .set_character_data(CharacterData::String(addr))?;
                }
                if let Some(addr_src) = address_source {
                    cfg.create_sub_element(ElementName::Ipv4AddressSource)?
                        .set_character_data(addr_src.to_cdata())?;
                }
                if let Some(defgw) = default_gateway {
                    cfg.create_sub_element(ElementName::DefaultGateway)?
                        .set_character_data(CharacterData::String(defgw))?;
                }
                if let Some(netmask) = network_mask {
                    cfg.create_sub_element(ElementName::NetworkMask)?
                        .set_character_data(CharacterData::String(netmask))?;
                }
            }
            NetworkEndpointAddress::IPv6 {
                address,
                address_source,
                default_router,
            } => {
                let cfg = addresses.create_sub_element(ElementName::Ipv6Configuration)?;
                if let Some(addr) = address {
                    cfg.create_sub_element(ElementName::Ipv6Address)?
                        .set_character_data(CharacterData::String(addr))?;
                }
                if let Some(addr_src) = address_source {
                    cfg.create_sub_element(ElementName::Ipv6AddressSource)?
                        .set_character_data(addr_src.to_cdata())?;
                }
                if let Some(dr) = default_router {
                    cfg.create_sub_element(ElementName::DefaultRouter)?
                        .set_character_data(CharacterData::String(dr))?;
                }
            }
        }
        Ok(())
    }

    pub fn addresses(&self) -> NetworkEndpointAddressIterator {
        NetworkEndpointAddressIterator::new(self)
    }
}

//##################################################################

/// address information for a network endpoint
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkEndpointAddress {
    /// IPv4 addressing information
    IPv4 {
        /// IPv4 address in the form "a.b.c.d". This is used if the address source is FIXED
        address: Option<String>,
        /// defines how the address is obtained
        address_source: Option<IPv4AddressSource>,
        /// ip address of the default gateway
        default_gateway: Option<String>,
        /// Network mask in the form "a.b.c.d"
        network_mask: Option<String>,
    },
    /// IPv6 addressing information
    IPv6 {
        /// IPv6 address, without abbreviation
        address: Option<String>,
        /// defines how the address is obtained
        address_source: Option<IPv6AddressSource>,
        /// IP address of the default router
        default_router: Option<String>,
    },
}

impl TryFrom<Element> for NetworkEndpointAddress {
    type Error = AutosarAbstractionError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        match element.element_name() {
            ElementName::Ipv4Configuration => {
                let address = element
                    .get_sub_element(ElementName::Ipv4Address)
                    .and_then(|i4a| i4a.character_data())
                    .and_then(|cdata| cdata.string_value());
                let address_source = element
                    .get_sub_element(ElementName::Ipv4AddressSource)
                    .and_then(|i4as| i4as.character_data())
                    .and_then(IPv4AddressSource::from_cdata);
                let default_gateway = element
                    .get_sub_element(ElementName::DefaultGateway)
                    .and_then(|dg| dg.character_data())
                    .and_then(|cdata| cdata.string_value());
                let network_mask = element
                    .get_sub_element(ElementName::NetworkMask)
                    .and_then(|nm| nm.character_data())
                    .and_then(|cdata| cdata.string_value());

                Ok(NetworkEndpointAddress::IPv4 {
                    address,
                    address_source,
                    default_gateway,
                    network_mask,
                })
            }
            ElementName::Ipv6Configuration => {
                let address = element
                    .get_sub_element(ElementName::Ipv6Address)
                    .and_then(|i6a| i6a.character_data())
                    .and_then(|cdata| cdata.string_value());
                let address_source = element
                    .get_sub_element(ElementName::Ipv6AddressSource)
                    .and_then(|i6as| i6as.character_data())
                    .and_then(IPv6AddressSource::from_cdata);
                let default_router = element
                    .get_sub_element(ElementName::DefaultRouter)
                    .and_then(|dr| dr.character_data())
                    .and_then(|cdata| cdata.string_value());

                Ok(NetworkEndpointAddress::IPv6 {
                    address,
                    address_source,
                    default_router,
                })
            }
            _ => Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "NetwworkEndpointAddress".to_string(),
            }),
        }
    }
}

/// `IPv4AddressSource` defines how the address of an IPv4 `NetworkEndpoint` is obtained
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPv4AddressSource {
    /// use AutoIp (aka APIPA) to assign a link-local address
    AutoIp,
    /// use AutoIp with DoIp settings to assign a link-local address
    AutoIpDoIp,
    /// dynamic assignment using DHCP
    DHCPv4,
    /// static IP address configuration - the address must be specified in NetworkEndpointAddress
    Fixed,
}

impl IPv4AddressSource {
    fn from_cdata(cdata: CharacterData) -> Option<Self> {
        match cdata {
            CharacterData::Enum(EnumItem::AutoIp) => Some(Self::AutoIp),
            CharacterData::Enum(EnumItem::AutoIpDoip) => Some(Self::AutoIpDoIp),
            CharacterData::Enum(EnumItem::Dhcpv4) => Some(Self::DHCPv4),
            CharacterData::Enum(EnumItem::Fixed) => Some(Self::Fixed),
            _ => None,
        }
    }

    fn to_cdata(self) -> CharacterData {
        match self {
            Self::AutoIp => CharacterData::Enum(EnumItem::AutoIp),
            Self::AutoIpDoIp => CharacterData::Enum(EnumItem::AutoIpDoip),
            Self::DHCPv4 => CharacterData::Enum(EnumItem::Dhcpv4),
            Self::Fixed => CharacterData::Enum(EnumItem::Fixed),
        }
    }
}

/// `IPv6AddressSource` defines how the address of an IPv6 `NetworkEndpoint` is obtained
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPv6AddressSource {
    /// dynamic assignment using DHCP
    DHCPv6,
    /// static IP address configuration - the address must be specified in NetworkEndpointAddress
    Fixed,
    /// automatic link local address assignment
    LinkLocal,
    /// automatic link local address assignment using doip parameters
    LinkLocalDoIp,
    /// IPv6 stateless autoconfiguration
    RouterAdvertisement,
}

impl IPv6AddressSource {
    fn from_cdata(cdata: CharacterData) -> Option<Self> {
        match cdata {
            CharacterData::Enum(EnumItem::Dhcpv6) => Some(Self::DHCPv6),
            CharacterData::Enum(EnumItem::Fixed) => Some(Self::Fixed),
            CharacterData::Enum(EnumItem::LinkLocal) => Some(Self::LinkLocal),
            CharacterData::Enum(EnumItem::LinkLocalDoip) => Some(Self::LinkLocalDoIp),
            CharacterData::Enum(EnumItem::RouterAdvertisement) => Some(Self::RouterAdvertisement),
            _ => None,
        }
    }

    fn to_cdata(self) -> CharacterData {
        match self {
            Self::DHCPv6 => CharacterData::Enum(EnumItem::Dhcpv6),
            Self::Fixed => CharacterData::Enum(EnumItem::Fixed),
            Self::LinkLocal => CharacterData::Enum(EnumItem::LinkLocal),
            Self::LinkLocalDoIp => CharacterData::Enum(EnumItem::LinkLocalDoip),
            Self::RouterAdvertisement => CharacterData::Enum(EnumItem::RouterAdvertisement),
        }
    }
}

//##################################################################

#[doc(hidden)]
pub struct EthernetCtrlChannelsIterator {
    connector_iter: Option<ElementsIterator>,
    comm_controller: Element,
    model: Result<AutosarModel, AutosarDataError>,
}

impl EthernetCtrlChannelsIterator {
    fn new(controller: &EthernetCommunicationController, ecu: &Element) -> Self {
        let iter = ecu.get_sub_element(ElementName::Connectors).map(|c| c.sub_elements());
        let comm_controller = controller.element();
        let model = comm_controller.model();
        Self {
            connector_iter: iter,
            comm_controller,
            model,
        }
    }
}

impl Iterator for EthernetCtrlChannelsIterator {
    type Item = EthernetPhysicalChannel;

    fn next(&mut self) -> Option<Self::Item> {
        let model = self.model.as_ref().ok()?;
        let connector_iter = self.connector_iter.as_mut()?;
        for connector in connector_iter.by_ref() {
            if connector.element_name() == ElementName::EthernetCommunicationConnector {
                if let Some(commcontroller_of_connector) = connector
                    .get_sub_element(ElementName::CommControllerRef)
                    .and_then(|ccr| ccr.get_reference_target().ok())
                {
                    if commcontroller_of_connector == self.comm_controller {
                        for ref_origin in model
                            .get_references_to(&connector.path().ok()?)
                            .iter()
                            .filter_map(WeakElement::upgrade)
                            .filter_map(|elem| elem.named_parent().ok().flatten())
                        {
                            // This assumes that each connector will only ever be referenced by at most one
                            // PhysicalChannel, which is true for well-formed files.
                            if ref_origin.element_name() == ElementName::EthernetPhysicalChannel {
                                return Some(EthernetPhysicalChannel(ref_origin));
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

//##################################################################

#[doc(hidden)]
pub struct EthernetClusterChannelsIterator {
    elements_iter: Option<ElementsIterator>,
}

impl EthernetClusterChannelsIterator {
    fn new(cluster: &EthernetCluster) -> Self {
        let elements_iter = cluster
            .0
            .get_sub_element(ElementName::EthernetClusterVariants)
            .and_then(|ecv| ecv.get_sub_element(ElementName::EthernetClusterConditional))
            .and_then(|ecc| ecc.get_sub_element(ElementName::PhysicalChannels))
            .map(|pc| pc.sub_elements());

        Self { elements_iter }
    }
}

impl Iterator for EthernetClusterChannelsIterator {
    type Item = EthernetPhysicalChannel;

    fn next(&mut self) -> Option<Self::Item> {
        let elements_iter = self.elements_iter.as_mut()?;
        for channel in elements_iter.by_ref() {
            if let Ok(ethernet_physical_channel) = EthernetPhysicalChannel::try_from(channel) {
                return Some(ethernet_physical_channel);
            }
        }
        None
    }
}

//##################################################################

#[doc(hidden)]
pub struct NetworkEndpointIterator {
    elements_iter: Option<ElementsIterator>,
}

impl NetworkEndpointIterator {
    fn new(channel: &EthernetPhysicalChannel) -> Self {
        let elements_iter = channel
            .0
            .get_sub_element(ElementName::NetworkEndpoints)
            .map(|ne| ne.sub_elements());

        Self { elements_iter }
    }
}

impl Iterator for NetworkEndpointIterator {
    type Item = NetworkEndpoint;

    fn next(&mut self) -> Option<Self::Item> {
        let elements_iter = self.elements_iter.as_mut()?;
        for endpoint in elements_iter.by_ref() {
            if let Ok(network_endpoint) = NetworkEndpoint::try_from(endpoint) {
                return Some(network_endpoint);
            }
        }
        None
    }
}

//##################################################################

#[doc(hidden)]
pub struct NetworkEndpointAddressIterator {
    elements_iter: Option<ElementsIterator>,
}

impl NetworkEndpointAddressIterator {
    fn new(network_endpoint: &NetworkEndpoint) -> Self {
        let elements_iter = network_endpoint
            .0
            .get_sub_element(ElementName::NetworkEndpointAddresses)
            .map(|nea| nea.sub_elements());

        Self { elements_iter }
    }
}

impl Iterator for NetworkEndpointAddressIterator {
    type Item = NetworkEndpointAddress;

    fn next(&mut self) -> Option<Self::Item> {
        let elements_iter = self.elements_iter.as_mut()?;
        for channel in elements_iter.by_ref() {
            if let Ok(network_endpoint_address) = NetworkEndpointAddress::try_from(channel) {
                return Some(network_endpoint_address);
            }
        }
        None
    }
}

//##################################################################

#[cfg(test)]
mod test {
    use crate::{System, SystemCategory};

    use super::*;
    use autosar_data::AutosarVersion;

    #[test]
    fn cluster() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();

        let pkg2 = ArPackage::get_or_create(&model, "/ethernet").unwrap();
        // create the ethernet cluster EthCluster
        let result = system.create_ethernet_cluster("EthCluster", &pkg2);
        assert!(result.is_ok());
        let cluster = result.unwrap();
        // creating the same cluster again is not possible
        let result = system.create_ethernet_cluster("EthCluster", &pkg2);
        assert!(result.is_err());

        // create an untagged channel
        let result = cluster.create_physical_channel("Channel1", None);
        assert!(result.is_ok());
        // can't create a second untagged channel
        let result = cluster.create_physical_channel("Channel2", None);
        assert!(result.is_err());

        // create a channel for VLAN 1
        let vlan_info = EthernetVlanInfo {
            vlan_name: "VLAN_1".to_string(),
            vlan_id: 1,
        };
        let result = cluster.create_physical_channel("Channel3", Some(vlan_info));
        assert!(result.is_ok());

        // can't create a second channel called Channel3
        let vlan_info = EthernetVlanInfo {
            vlan_name: "VLAN_2".to_string(),
            vlan_id: 2,
        };
        let result = cluster.create_physical_channel("Channel3", Some(vlan_info));
        assert!(result.is_err());

        // create a channel for VLAN 2
        let vlan_info = EthernetVlanInfo {
            vlan_name: "VLAN_2".to_string(),
            vlan_id: 2,
        };
        let result = cluster.create_physical_channel("Channel4", Some(vlan_info));
        assert!(result.is_ok());

        // can't create a second channel for VLAN 2
        let vlan_info = EthernetVlanInfo {
            vlan_name: "VLAN_2".to_string(),
            vlan_id: 2,
        };
        let result = cluster.create_physical_channel("Channel5", Some(vlan_info));
        assert!(result.is_err());

        let count = cluster.physical_channels().count();
        assert_eq!(count, 3);
    }

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

        elem_vlanid
            .set_character_data(CharacterData::String(1.to_string()))
            .unwrap();
        let vi = channel.get_vlan_info().unwrap();
        assert_eq!(vi.vlan_id, 1);
    }

    #[test]
    fn controller() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();
        let ecu = system.create_ecu_instance("ECU", &pkg).unwrap();

        // can't create a controller with an invalid MAC address
        let result = ecu.create_ethernet_communication_controller("Controller", Some("abcdef".to_string()));
        assert!(result.is_err());

        // create a controller
        let result = ecu.create_ethernet_communication_controller("Controller", Some("01:02:03:04:05:06".to_string()));
        let controller = result.unwrap();

        // create some physical channels
        let cluster = system.create_ethernet_cluster("EthCluster", &pkg).unwrap();
        let channel1 = cluster.create_physical_channel("C1", None).unwrap();
        let vlan_info = EthernetVlanInfo {
            vlan_name: "VLAN_1".to_string(),
            vlan_id: 1,
        };
        let channel2 = cluster.create_physical_channel("C2", Some(vlan_info)).unwrap();

        // connect the controller to channel1
        let result = controller.connect_physical_channel("connection_name1", &channel1);
        assert!(result.is_ok());
        // can't connect to the same channel again
        let result = controller.connect_physical_channel("connection_name2", &channel1);
        assert!(result.is_err());
        // connect the controller to channel2
        let result = controller.connect_physical_channel("connection_name2", &channel2);
        assert!(result.is_ok());

        let count = controller.connected_channels().count();
        assert_eq!(count, 2);

        // remove the controller and try to list its connected channels again
        let ctrl_parent = controller.0.parent().unwrap().unwrap();
        ctrl_parent.remove_sub_element(controller.0.clone()).unwrap();
        let count = controller.connected_channels().count();
        assert_eq!(count, 0);
    }
}
