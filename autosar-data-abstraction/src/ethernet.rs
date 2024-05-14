use crate::{abstraction_element, AbstactionElement, ArPackage, AutosarAbstractionError, EcuInstance};
use autosar_data::{
    AutosarDataError, AutosarModel, CharacterData, Element, ElementName, ElementsIterator, WeakElement,
};

/// Provides information about the VLAN of an [`EthernetPhysicalChannel`]
#[derive(Debug, Clone, PartialEq)]
pub struct EthernetVlanInfo {
    pub vlan_name: String,
    pub vlan_id: u16,
}

/// An EthernetCluster contains all configuration items associated with an ethernet network.
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
