use crate::{abstraction_element, AbstactionElement, ArPackage, AutosarAbstractionError, EcuInstance};
use autosar_data::{AutosarDataError, AutosarModel, CharacterData, Element, ElementName, ElementsIterator};

/// Settings for a CAN cluster
#[derive(Debug, Clone, PartialEq)]
pub struct CanClusterSettings {
    pub baudrate: u32,
    pub can_fd_baudrate: Option<u32>,
    pub can_xl_baudrate: Option<u32>,
}

/// A `CanCluster` contains all configuration items associated with a CAN network.
/// The cluster connects multiple ECUs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanCluster(Element);
abstraction_element!(CanCluster, CanCluster);

impl CanCluster {
    // create a new CanCluster - for internal use. User code should call System::create_can_cluster
    pub(crate) fn new(
        cluster_name: &str,
        package: &ArPackage,
        settings: &CanClusterSettings,
    ) -> Result<Self, AutosarAbstractionError> {
        let elem_pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_cluster = elem_pkg_elements.create_named_sub_element(ElementName::CanCluster, cluster_name)?;
        if let Ok(cluster_content) = elem_cluster
            .create_sub_element(ElementName::CanClusterVariants)
            .and_then(|ccv| ccv.create_sub_element(ElementName::CanClusterConditional))
        {
            let _ = cluster_content
                .create_sub_element(ElementName::ProtocolName)
                .and_then(|pn| pn.set_character_data(autosar_data::CharacterData::String("CAN".to_string())));

            let _ = cluster_content.create_sub_element(ElementName::PhysicalChannels);
        }

        let can_cluster = CanCluster(elem_cluster);
        can_cluster.update_settings(settings);

        Ok(can_cluster)
    }

    /// Update the settings of this `CanCluster` with new values for the baudrates
    ///
    /// The baudrates for CanFD and CanXL are optional.
    /// If they are set to None in the settings, then corresponding elements in the model will be removed.
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
    /// let cluster = system.create_can_cluster("Cluster", &package, &CanClusterSettings::default()).unwrap();
    /// let mut settings = cluster.get_settings();
    /// settings.can_fd_baudrate = Some(2000000);
    /// cluster.update_settings(&settings);
    /// ```
    pub fn update_settings(&self, settings: &CanClusterSettings) {
        if let Ok(cluster_content) = self
            .0
            .get_or_create_sub_element(ElementName::CanClusterVariants)
            .and_then(|ccv| ccv.get_or_create_sub_element(ElementName::CanClusterConditional))
        {
            let _ = cluster_content
                .create_sub_element(ElementName::Baudrate)
                .and_then(|br| br.set_character_data(CharacterData::String(settings.baudrate.to_string())));
            if let Some(can_fd_baudrate) = settings.can_fd_baudrate {
                let _ = cluster_content
                    .create_sub_element(ElementName::CanFdBaudrate)
                    .and_then(|cfbr| cfbr.set_character_data(CharacterData::String(can_fd_baudrate.to_string())));
            } else if let Some(cfbr) = cluster_content.get_sub_element(ElementName::CanFdBaudrate) {
                let _ = cluster_content.remove_sub_element(cfbr);
            }
            if let Some(can_xl_baudrate) = settings.can_xl_baudrate {
                cluster_content
                    .create_sub_element(ElementName::CanXlBaudrate)
                    .and_then(|cxbr| cxbr.set_character_data(CharacterData::String(can_xl_baudrate.to_string())))
                    .unwrap();
            } else if let Some(cxbr) = cluster_content.get_sub_element(ElementName::CanXlBaudrate) {
                let _ = cluster_content.remove_sub_element(cxbr);
            }
        }
    }

    /// get the setings of this `CanCluster`
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
    /// let settings = CanClusterSettings {baudrate: 500000, can_fd_baudrate: None, can_xl_baudrate: None};
    /// let cluster = system.create_can_cluster("Cluster", &package, &settings).unwrap();
    /// let settings2 = cluster.get_settings();
    /// assert_eq!(settings, settings2);
    /// ```
    pub fn get_settings(&self) -> CanClusterSettings {
        let mut settings = CanClusterSettings {
            baudrate: 0,
            can_fd_baudrate: None,
            can_xl_baudrate: None,
        };
        if let Some(cluster_content) = self
            .0
            .get_sub_element(ElementName::CanClusterVariants)
            .and_then(|ccv| ccv.get_sub_element(ElementName::CanClusterConditional))
        {
            if let Some(baudrate) = cluster_content
                .get_sub_element(ElementName::Baudrate)
                .and_then(|br| br.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.baudrate = baudrate;
            }

            settings.can_fd_baudrate = cluster_content
                .get_sub_element(ElementName::CanFdBaudrate)
                .and_then(|br| br.character_data())
                .and_then(|cdata| cdata.parse_integer());

            settings.can_xl_baudrate = cluster_content
                .get_sub_element(ElementName::CanXlBaudrate)
                .and_then(|br| br.character_data())
                .and_then(|cdata| cdata.parse_integer());
        }
        settings
    }

    /// Create a new physical channel for the cluster
    ///
    /// A can cluster must contain exactly one physical channel; trying to add a second one triggers an error.
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
    /// let cluster = system.create_can_cluster("Cluster", &package, &CanClusterSettings::default()).unwrap();
    /// let channel = cluster.create_physical_channel("Channel").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ItemAlreadyExists`] There is already a physical channel in this CAN cluster
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn create_physical_channel(&self, channel_name: &str) -> Result<CanPhysicalChannel, AutosarAbstractionError> {
        let phys_channels = self
            .0
            .get_or_create_sub_element(ElementName::CanClusterVariants)?
            .get_or_create_sub_element(ElementName::CanClusterConditional)?
            .get_or_create_sub_element(ElementName::PhysicalChannels)?;

        if phys_channels.sub_elements().count() != 0 {
            return Err(AutosarAbstractionError::ItemAlreadyExists);
        }

        let channel = phys_channels.create_named_sub_element(ElementName::CanPhysicalChannel, channel_name)?;

        Ok(CanPhysicalChannel(channel))
    }

    /// return the CanPhysicalChannel of the Cluster, if it has been created
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
    /// # let cluster = system.create_can_cluster("Cluster", &package, &CanClusterSettings::default()).unwrap();
    /// # let can_channel = cluster.create_physical_channel("Channel").unwrap();
    /// let channel = cluster.physical_channel().unwrap();
    /// # assert_eq!(channel, can_channel);
    /// ```
    pub fn physical_channel(&self) -> Option<CanPhysicalChannel> {
        let channel = self
            .0
            .get_sub_element(ElementName::CanClusterVariants)?
            .get_sub_element(ElementName::CanClusterConditional)?
            .get_sub_element(ElementName::PhysicalChannels)?
            .get_sub_element(ElementName::CanPhysicalChannel)?;
        Some(CanPhysicalChannel(channel))
    }
}

//##################################################################

/// The CanPhysicalChannel contains all of the communication on a CAN network
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanPhysicalChannel(Element);
abstraction_element!(CanPhysicalChannel, CanPhysicalChannel);

impl CanPhysicalChannel {
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
    /// # let cluster = system.create_can_cluster("Cluster", &package, &CanClusterSettings::default()).unwrap();
    /// let channel = cluster.create_physical_channel("Channel").unwrap();
    /// let cluster_2 = channel.cluster().unwrap();
    /// assert_eq!(cluster, cluster_2);
    /// ```
    pub fn cluster(&self) -> Result<CanCluster, AutosarAbstractionError> {
        let cluster_elem = self.0.named_parent()?.unwrap();
        CanCluster::try_from(cluster_elem)
    }
}

//##################################################################

/// An `EcuInstance` needs a `CanCommunicationController` in order to connect to a CAN cluster.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanCommunicationController(Element);
abstraction_element!(CanCommunicationController, CanCommunicationController);

impl CanCommunicationController {
    // create a new CanCommunicationController - called by EcuInstance::create_can_communication_controller
    pub(crate) fn new(name: &str, ecu: &EcuInstance) -> Result<Self, AutosarAbstractionError> {
        let commcontrollers = ecu.element().get_or_create_sub_element(ElementName::CommControllers)?;
        let ctrl = commcontrollers.create_named_sub_element(ElementName::CanCommunicationController, name)?;
        let _canccc = ctrl
            .create_sub_element(ElementName::CanCommunicationControllerVariants)?
            .create_sub_element(ElementName::CanCommunicationControllerConditional)?;

        Ok(Self(ctrl))
    }

    /// return an iterator over the [`CanPhysicalChannel`]s connected to this controller
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
    /// let can_controller = ecu_instance.create_can_communication_controller("CanCtrl").unwrap();
    /// # let cluster = system.create_can_cluster("Cluster", &package, &CanClusterSettings::default()).unwrap();
    /// # let physical_channel = cluster.create_physical_channel("Channel").unwrap();
    /// can_controller.connect_physical_channel("connection", &physical_channel).unwrap();
    /// for channel in can_controller.connected_channels() {
    ///     // ...
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn connected_channels(&self) -> CanCtrlChannelsIterator {
        if let Ok(ecu) = self.ecu_instance().map(|ecuinstance| ecuinstance.element()) {
            CanCtrlChannelsIterator::new(self, &ecu)
        } else {
            CanCtrlChannelsIterator {
                connector_iter: None,
                comm_controller: self.0.clone(),
                model: Err(AutosarDataError::ElementNotFound),
            }
        }
    }

    /// get the EcuInstance that contains this CanCommunicationController
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
    /// let can_controller = ecu_instance.create_can_communication_controller("CanCtrl").unwrap();
    /// assert_eq!(ecu_instance, can_controller.ecu_instance().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn ecu_instance(&self) -> Result<EcuInstance, AutosarAbstractionError> {
        // unwrapping is safe here - self.0.named_parent() cannot return Ok(None).
        // the CanCommunicationController is always a child of an EcuInstance,
        // or else it is deleted and named_parent() return Err(...), which is handled by the ?
        let ecu: Element = self.0.named_parent()?.unwrap();
        EcuInstance::try_from(ecu)
    }

    /// Connect this [`CanCommunicationController`] inside an [`EcuInstance`] to a [`CanPhysicalChannel`] in the [`crate::System`]
    ///
    /// Creates a CanCommunicationConnector in the [`EcuInstance`] that contains this [`CanCommunicationController`].
    ///
    /// This function establishes the relationships:
    ///  - [`CanPhysicalChannel`] -> CanCommunicationConnector
    ///  - CanCommunicationConnector -> [`CanCommunicationController`]
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
    /// let can_controller = ecu_instance.create_can_communication_controller("CanCtrl").unwrap();
    /// # let cluster = system.create_can_cluster("Cluster", &package, &CanClusterSettings::default()).unwrap();
    /// # let physical_channel = cluster.create_physical_channel("Channel").unwrap();
    /// can_controller.connect_physical_channel("connection", &physical_channel).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn connect_physical_channel(
        &self,
        connection_name: &str,
        can_channel: &CanPhysicalChannel,
    ) -> Result<(), AutosarAbstractionError> {
        let ecu = self.0.named_parent()?.unwrap();
        // check that there is no existing connector for this CanCommunicationController
        if let Some(connectors) = ecu.get_sub_element(ElementName::Connectors) {
            for connector in connectors.sub_elements() {
                // Does the existing connector reference this CanCommunicationController?
                // A CanCommunicationController can only connect to a single CAN cluster, so a second
                // connector cannot be created.
                if let Some(ccref) = connector.get_sub_element(ElementName::CommControllerRef) {
                    if let Ok(commcontroller_of_connector) = ccref.get_reference_target() {
                        if commcontroller_of_connector == self.0 {
                            return Err(AutosarAbstractionError::ItemAlreadyExists);
                        }
                    }
                }
            }
        }
        // create a new connector
        let connectors = ecu.get_or_create_sub_element(ElementName::Connectors)?;
        let connector = connectors.create_named_sub_element(ElementName::CanCommunicationConnector, connection_name)?;
        connector
            .create_sub_element(ElementName::CommControllerRef)
            .and_then(|refelem| refelem.set_reference_target(&self.0))?;

        let channel_connctor_refs = can_channel
            .element()
            .get_or_create_sub_element(ElementName::CommConnectors)?;
        channel_connctor_refs
            .create_sub_element(ElementName::CommunicationConnectorRefConditional)
            .and_then(|ccrc| ccrc.create_sub_element(ElementName::CommunicationConnectorRef))
            .and_then(|ccr| ccr.set_reference_target(&connector))?;

        Ok(())
    }
}

//##################################################################

#[doc(hidden)]
pub struct CanCtrlChannelsIterator {
    connector_iter: Option<ElementsIterator>,
    comm_controller: Element,
    model: Result<AutosarModel, AutosarDataError>,
}

impl CanCtrlChannelsIterator {
    fn new(controller: &CanCommunicationController, ecu: &Element) -> Self {
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

impl Iterator for CanCtrlChannelsIterator {
    type Item = CanPhysicalChannel;

    fn next(&mut self) -> Option<Self::Item> {
        let model = self.model.as_ref().ok()?;
        let connector_iter = self.connector_iter.as_mut()?;
        for connector in connector_iter.by_ref() {
            if connector.element_name() == ElementName::CanCommunicationConnector {
                if let Some(commcontroller_of_connector) = connector
                    .get_sub_element(ElementName::CommControllerRef)
                    .and_then(|ccr| ccr.get_reference_target().ok())
                {
                    if commcontroller_of_connector == self.comm_controller {
                        for ref_origin in model
                            .get_references_to(&connector.path().ok()?)
                            .iter()
                            .filter_map(|weakelem| weakelem.upgrade())
                            .filter_map(|elem| elem.named_parent().ok().flatten())
                        {
                            // This assumes that each connector will only ever be referenced by at most one
                            // PhysicalChannel, which is true for well-formed files.
                            if ref_origin.element_name() == ElementName::CanPhysicalChannel {
                                return Some(CanPhysicalChannel(ref_origin));
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

/// settings for a CAN cluster
impl CanClusterSettings {
    /// create a new `CanClusterSettings` object
    #[must_use]
    pub fn new() -> Self {
        Self {
            baudrate: 500_000,
            can_fd_baudrate: None,
            can_xl_baudrate: None,
        }
    }
}

impl Default for CanClusterSettings {
    fn default() -> Self {
        Self::new()
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
        model.create_file("filename", AutosarVersion::Autosar_00051).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();

        let pkg2 = ArPackage::get_or_create(&model, "/can").unwrap();
        // create the CAN cluster CanCluster
        let settings = CanClusterSettings::default();
        let result = system.create_can_cluster("CanCluster", &pkg2, &settings);
        assert!(result.is_ok());
        let cluster = result.unwrap();
        // creating the same cluster again is not possible
        let settings = CanClusterSettings::default();
        let result = system.create_can_cluster("CanCluster", &pkg2, &settings);
        assert!(result.is_err());

        // settings for CanFd
        let mut settings = cluster.get_settings();
        assert!(settings.can_fd_baudrate.is_none());
        settings.can_fd_baudrate = Some(2_000_000);
        cluster.update_settings(&settings);
        let mut settings = cluster.get_settings();
        assert!(settings.can_fd_baudrate.is_some());
        // add setings for CanXL, remove CanFd
        settings.can_fd_baudrate = None;
        settings.can_xl_baudrate = Some(10_000_000);
        cluster.update_settings(&settings);
        let mut settings = cluster.get_settings();
        assert!(settings.can_fd_baudrate.is_none());
        assert!(settings.can_xl_baudrate.is_some());
        // remove CanXl settings
        settings.can_xl_baudrate = None;
        cluster.update_settings(&settings);
        let settings = cluster.get_settings();
        assert!(settings.can_fd_baudrate.is_none());
        assert!(settings.can_xl_baudrate.is_none());

        // create a channel
        let result = cluster.create_physical_channel("Channel1");
        assert!(result.is_ok());
        // can't create a second channel
        let result = cluster.create_physical_channel("Channel2");
        assert!(result.is_err());

        let pc = cluster.physical_channel();
        assert!(pc.is_some());
    }

    #[test]
    fn channel() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();
        let settings = CanClusterSettings::default();
        let cluster = system.create_can_cluster("CanCluster", &pkg, &settings).unwrap();

        let channel = cluster.create_physical_channel("channel_name").unwrap();
        let c2 = channel.cluster().unwrap();
        assert_eq!(cluster, c2);
    }

    #[test]
    fn controller() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();
        let ecu = system.create_ecu_instance("ECU", &pkg).unwrap();

        // create a controller
        let result = ecu.create_can_communication_controller("Controller");
        let controller = result.unwrap();

        // create some physical channels
        let settings = CanClusterSettings::default();
        let cluster = system.create_can_cluster("CanCluster", &pkg, &settings).unwrap();
        let channel1 = cluster.create_physical_channel("C1").unwrap();

        // connect the controller to channel1
        let result = controller.connect_physical_channel("connection_name1", &channel1);
        assert!(result.is_ok());
        // can't connect to the same channel again
        let result = controller.connect_physical_channel("connection_name2", &channel1);
        assert!(result.is_err());

        let count = controller.connected_channels().count();
        assert_eq!(count, 1);

        // remove the controller and try to list its connected channels again
        let ctrl_parent = controller.0.parent().unwrap().unwrap();
        ctrl_parent.remove_sub_element(controller.0.clone()).unwrap();
        let count = controller.connected_channels().count();
        assert_eq!(count, 0);
    }
}
