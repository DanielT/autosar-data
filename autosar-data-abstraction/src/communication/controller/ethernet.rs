use crate::communication::{EthernetPhysicalChannel, EthernetVlanInfo};
use crate::{abstraction_element, AbstractionElement, AutosarAbstractionError, EcuInstance};
use autosar_data::{AutosarDataError, AutosarModel, Element, ElementName, ElementsIterator, WeakElement};

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
                .and_then(|mua| mua.set_character_data(mac_address));
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
    pub fn connected_channels(&self) -> impl Iterator<Item = EthernetPhysicalChannel> {
        if let Ok(ecu) = self.ecu_instance().map(|ecuinstance| ecuinstance.element().clone()) {
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
                .and_then(|cat| cat.set_character_data(category));
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
                    .and_then(|vr| vr.set_reference_target(eth_channel.element()))?;
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
        let comm_controller = controller.element().clone();
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
                                return EthernetPhysicalChannel::try_from(ref_origin).ok();
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{communication::EthernetVlanInfo, ArPackage, System, SystemCategory};
    use autosar_data::{AutosarModel, AutosarVersion};

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
        let ctrl_parent = controller.element().parent().unwrap().unwrap();
        ctrl_parent.remove_sub_element(controller.element().clone()).unwrap();
        let count = controller.connected_channels().count();
        assert_eq!(count, 0);
    }
}
