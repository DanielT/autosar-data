use crate::{
    abstraction_element, can::CanCommunicationController, ethernet::EthernetCommunicationController,
    flexray::FlexrayCommunicationController, AbstactionElement, ArPackage, AutosarAbstractionError,
};
use autosar_data::{Element, ElementName};

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
}
