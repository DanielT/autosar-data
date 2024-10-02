use crate::communication::{
    CanCommunicationController, CommunicationController, EthernetCommunicationController,
    FlexrayCommunicationController,
};
use crate::{abstraction_element, AbstractionElement, ArPackage, AutosarAbstractionError};
use autosar_data::{Element, ElementName, ElementsIterator};

/// The `EcuInstance` represents one ECU in a `System`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EcuInstance(Element);
abstraction_element!(EcuInstance, EcuInstance);

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct EcuInstance(Element);

impl EcuInstance {
    // Create a new EcuInstance
    //
    // This new EcuInstance will not be connected to a System.
    pub(crate) fn new(name: &str, package: &ArPackage) -> Result<EcuInstance, AutosarAbstractionError> {
        let elem_pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_ecu_instance = elem_pkg_elements.create_named_sub_element(ElementName::EcuInstance, name)?;

        Ok(EcuInstance(elem_ecu_instance))
    }

    /// Create a CAN-COMMUNICATION-CONTROLLER for this ECU-INSTANCE
    ///
    /// The ECU must have one controller per bus it communicates on.
    /// For example, if it communicates on two CAN buses, then two CAN-COMMUNICATION-CONTROLLERs are needed.
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
    /// let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let can_controller = ecu_instance.create_can_communication_controller("CanCtrl").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn create_can_communication_controller(
        &self,
        name: &str,
    ) -> Result<CanCommunicationController, AutosarAbstractionError> {
        CanCommunicationController::new(name, self)
    }

    /// Create an ETHERNET-COMMUNICATION-CONTROLLER for this ECU-INSTANCE
    ///
    /// The ECU must have one controller per bus it communicates on.
    /// For example, if it communicates on two CAN buses, then two CAN-COMMUNICATION-CONTROLLERs are needed.
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
    /// let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let ethernet_controller = ecu_instance
    ///     .create_ethernet_communication_controller("EthCtrl", Some("ab:cd:ef:01:02:03".to_string()))
    ///     .unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn create_ethernet_communication_controller(
        &self,
        name: &str,
        mac_address: Option<String>,
    ) -> Result<EthernetCommunicationController, AutosarAbstractionError> {
        EthernetCommunicationController::new(name, self, mac_address)
    }

    /// Create a FLEXRAY-COMMUNICATION-CONTROLLER for this ECU-INSTANCE
    ///
    /// The ECU must have one controller per bus it communicates on.
    /// For example, if it communicates on two CAN buses, then two CAN-COMMUNICATION-CONTROLLERs are needed.
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
    /// let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let flexray_controller = ecu_instance
    ///     .create_flexray_communication_controller("FlexrayCtrl")
    ///     .unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn create_flexray_communication_controller(
        &self,
        name: &str,
    ) -> Result<FlexrayCommunicationController, AutosarAbstractionError> {
        FlexrayCommunicationController::new(name, self)
    }

    /// return an interator over all communication controllers in this `EcuInstance`
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
    /// let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// ecu_instance.create_flexray_communication_controller("FlexrayCtrl").unwrap();
    /// ecu_instance.create_can_communication_controller("CanCtrl").unwrap();
    /// for ctrl in ecu_instance.communication_controllers() {
    ///     // ...
    /// }
    /// # assert_eq!(ecu_instance.communication_controllers().count(), 2);
    /// ```
    #[must_use]
    pub fn communication_controllers(&self) -> impl Iterator<Item = CommunicationController> {
        EcuInstanceControllersIterator::new(self)
    }
}

//##################################################################

#[doc(hidden)]
pub struct EcuInstanceControllersIterator {
    iter: Option<ElementsIterator>,
}

impl EcuInstanceControllersIterator {
    fn new(ecu_instance: &EcuInstance) -> Self {
        let iter = ecu_instance
            .0
            .get_sub_element(ElementName::CommControllers)
            .map(|cc| cc.sub_elements());
        Self { iter }
    }
}

impl Iterator for EcuInstanceControllersIterator {
    type Item = CommunicationController;

    fn next(&mut self) -> Option<Self::Item> {
        let elements_iter = self.iter.as_mut()?;
        for comm_ctrl in elements_iter {
            match comm_ctrl.element_name() {
                ElementName::CanCommunicationController => {
                    if let Ok(can_ctrl) = CanCommunicationController::try_from(comm_ctrl) {
                        return Some(CommunicationController::Can(can_ctrl));
                    }
                }
                ElementName::EthernetCommunicationController => {
                    if let Ok(eth_ctrl) = EthernetCommunicationController::try_from(comm_ctrl) {
                        return Some(CommunicationController::Ethernet(eth_ctrl));
                    }
                }
                ElementName::FlexrayCommunicationController => {
                    if let Ok(flx_ctrl) = FlexrayCommunicationController::try_from(comm_ctrl) {
                        return Some(CommunicationController::Flexray(flx_ctrl));
                    }
                }
                _ => {}
            }
        }
        None
    }
}

//##################################################################

#[cfg(test)]
mod test {
    use crate::*;
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn ecu() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
        let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
        let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
        ecu_instance
            .create_flexray_communication_controller("FlexrayCtrl")
            .unwrap();
        ecu_instance.create_can_communication_controller("CanCtrl").unwrap();
        ecu_instance
            .create_ethernet_communication_controller("EthCtrl", None)
            .unwrap();
        assert_eq!(ecu_instance.communication_controllers().count(), 3);
    }
}
