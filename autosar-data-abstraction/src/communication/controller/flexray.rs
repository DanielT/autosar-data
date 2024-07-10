use crate::{
    abstraction_element, communication::FlexrayPhysicalChannel, AbstractionElement, AutosarAbstractionError,
    EcuInstance,
};
use autosar_data::{AutosarDataError, AutosarModel, Element, ElementName, ElementsIterator, WeakElement};

/// An `EcuInstance` needs a `FlexrayCommunicationController` in order to connect to a Flexray cluster.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlexrayCommunicationController(Element);
abstraction_element!(FlexrayCommunicationController, FlexrayCommunicationController);

impl FlexrayCommunicationController {
    // create a new FlexrayCommunicationController - called by EcuInstance::create_flexray_communication_controller
    pub(crate) fn new(name: &str, ecu: &EcuInstance) -> Result<Self, AutosarAbstractionError> {
        let commcontrollers = ecu.element().get_or_create_sub_element(ElementName::CommControllers)?;
        let ctrl = commcontrollers.create_named_sub_element(ElementName::FlexrayCommunicationController, name)?;
        let _flxccc = ctrl
            .create_sub_element(ElementName::FlexrayCommunicationControllerVariants)?
            .create_sub_element(ElementName::FlexrayCommunicationControllerConditional)?;

        Ok(Self(ctrl))
    }

    /// return an iterator over the [`FlexrayPhysicalChannel`]s connected to this controller
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
    /// # let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let flexray_controller = ecu_instance.create_flexray_communication_controller("FRCtrl").unwrap();
    /// # let cluster = system.create_flexray_cluster("Cluster", &package, &FlexrayClusterSettings::default()).unwrap();
    /// # let physical_channel = cluster.create_physical_channel("Channel", FlexrayChannelName::A).unwrap();
    /// flexray_controller.connect_physical_channel("connection", &physical_channel).unwrap();
    /// for channel in flexray_controller.connected_channels() {
    ///     // ...
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model
    #[must_use]
    pub fn connected_channels(&self) -> FlexrayCtrlChannelsIterator {
        if let Ok(ecu) = self.ecu_instance().map(|ecuinstance| ecuinstance.element().clone()) {
            FlexrayCtrlChannelsIterator::new(self, &ecu)
        } else {
            FlexrayCtrlChannelsIterator {
                connector_iter: None,
                comm_controller: self.0.clone(),
                model: Err(AutosarDataError::ElementNotFound),
            }
        }
    }

    /// get the `EcuInstance` that contains this `FlexrayCommunicationController`
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
    /// # let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let flexray_controller = ecu_instance.create_flexray_communication_controller("FRCtrl").unwrap();
    /// assert_eq!(ecu_instance, flexray_controller.ecu_instance().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn ecu_instance(&self) -> Result<EcuInstance, AutosarAbstractionError> {
        // unwrapping is safe here - self.0.named_parent() cannot return Ok(None).
        // the FlexrayCommunicationController is always a child of an EcuInstance,
        // or else it is deleted and named_parent() return Err(...), which is handled by the ?
        let ecu: Element = self.0.named_parent()?.unwrap();
        EcuInstance::try_from(ecu)
    }

    /// Connect this [`FlexrayCommunicationController`] inside an [`EcuInstance`] to a [`FlexrayPhysicalChannel`] in the [`crate::System`]
    ///
    /// Creates a FlexrayCommunicationConnector in the [`EcuInstance`] that contains this [`FlexrayCommunicationController`].
    ///
    /// This function establishes the relationships:
    ///  - [`FlexrayPhysicalChannel`] -> FlexrayCommunicationConnector
    ///  - FlexrayCommunicationConnector -> [`FlexrayCommunicationController`]
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
    /// # let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let flexray_controller = ecu_instance.create_flexray_communication_controller("FlxCtrl").unwrap();
    /// # let cluster = system.create_flexray_cluster("Cluster", &package, &FlexrayClusterSettings::default()).unwrap();
    /// # let physical_channel = cluster.create_physical_channel("Channel", FlexrayChannelName::A).unwrap();
    /// flexray_controller.connect_physical_channel("connection", &physical_channel).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn connect_physical_channel(
        &self,
        connection_name: &str,
        flx_channel: &FlexrayPhysicalChannel,
    ) -> Result<(), AutosarAbstractionError> {
        let ecu = self.0.named_parent()?.unwrap();

        for existing_channel in self.connected_channels() {
            if existing_channel == *flx_channel {
                return Err(AutosarAbstractionError::ItemAlreadyExists);
            }
        }

        // create a new connector
        let connectors = ecu.get_or_create_sub_element(ElementName::Connectors)?;
        let connector =
            connectors.create_named_sub_element(ElementName::FlexrayCommunicationConnector, connection_name)?;
        connector
            .create_sub_element(ElementName::CommControllerRef)
            .and_then(|refelem| refelem.set_reference_target(&self.0))?;

        let channel_connctor_refs = flx_channel
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
pub struct FlexrayCtrlChannelsIterator {
    connector_iter: Option<ElementsIterator>,
    comm_controller: Element,
    model: Result<AutosarModel, AutosarDataError>,
}

impl FlexrayCtrlChannelsIterator {
    fn new(controller: &FlexrayCommunicationController, ecu: &Element) -> Self {
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

impl Iterator for FlexrayCtrlChannelsIterator {
    type Item = FlexrayPhysicalChannel;

    fn next(&mut self) -> Option<Self::Item> {
        let model = self.model.as_ref().ok()?;
        let connector_iter = self.connector_iter.as_mut()?;
        for connector in connector_iter.by_ref() {
            if connector.element_name() == ElementName::FlexrayCommunicationConnector {
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
                            if ref_origin.element_name() == ElementName::FlexrayPhysicalChannel {
                                return FlexrayPhysicalChannel::try_from(ref_origin).ok();
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
    use crate::{
        communication::{FlexrayChannelName, FlexrayClusterSettings},
        ArPackage, System, SystemCategory,
    };

    use super::*;
    use autosar_data::AutosarVersion;

    #[test]
    fn controller() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();
        let ecu = system.create_ecu_instance("ECU", &pkg).unwrap();

        // create a controller
        let result = ecu.create_flexray_communication_controller("Controller");
        let controller = result.unwrap();

        // create some physical channels
        let settings = FlexrayClusterSettings::default();
        let cluster = system.create_flexray_cluster("FlxCluster", &pkg, &settings).unwrap();
        let channel1 = cluster.create_physical_channel("C1", FlexrayChannelName::A).unwrap();

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
