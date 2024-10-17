use crate::{
    abstraction_element, communication::CanPhysicalChannel, AbstractionElement, AutosarAbstractionError, EcuInstance,
};
use autosar_data::{AutosarDataError, AutosarModel, Element, ElementName, ElementsIterator, WeakElement};

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
    /// # use autosar_data_abstraction::communication::*;
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
    pub fn connected_channels(&self) -> impl Iterator<Item = CanPhysicalChannel> {
        if let Ok(ecu) = self.ecu_instance().map(|ecuinstance| ecuinstance.element().clone()) {
            CanCtrlChannelsIterator::new(self, &ecu)
        } else {
            CanCtrlChannelsIterator {
                connector_iter: None,
                comm_controller: self.0.clone(),
                model: Err(AutosarDataError::ElementNotFound),
            }
        }
    }

    /// get the [`EcuInstance`] that contains this `CanCommunicationController`
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
    /// # use autosar_data_abstraction::communication::*;
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
        let comm_controller = controller.element().clone();
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
                            .filter_map(WeakElement::upgrade)
                            .filter_map(|elem| elem.named_parent().ok().flatten())
                        {
                            // This assumes that each connector will only ever be referenced by at most one
                            // PhysicalChannel, which is true for well-formed files.
                            if ref_origin.element_name() == ElementName::CanPhysicalChannel {
                                return CanPhysicalChannel::try_from(ref_origin).ok();
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
    use crate::{communication::CanClusterSettings, ArPackage, System, SystemCategory};
    use autosar_data::AutosarVersion;

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
