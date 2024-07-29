use crate::communication::{
    CanCluster, CanClusterSettings, CanFrame, Cluster, ContainerIPdu, DcmIPdu, EthernetCluster, FlexrayCluster,
    FlexrayClusterSettings, FlexrayFrame, GeneralPurposeIPdu, GeneralPurposePdu, ISignal, ISignalGroup, ISignalIPdu,
    MultiplexedIPdu, NPdu, NmPdu, SecuredIPdu, SystemSignal, SystemSignalGroup,
};
use crate::software_component::{
    AbstractSwComponentType, ComponentPrototype, CompositionSwComponentType, RootSwCompositionPrototype,
    SwComponentPrototype,
};
use crate::{abstraction_element, AbstractionElement, ArPackage, AutosarAbstractionError, EcuInstance};
use autosar_data::{AutosarDataError, AutosarModel, Element, ElementName, WeakElement};
use std::iter::FusedIterator;

/// The System is the top level of a system template
///
/// It defines how ECUs communicate with each other over various networks.
/// It also contains the mapping of software components to ECUs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct System(Element);
abstraction_element!(System, System);

impl System {
    /// find an existing \<SYSTEM\> in the model, or return None
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/my/pkg").unwrap();
    /// let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// let sys_2 = System::find(&model).unwrap();
    /// assert_eq!(system, sys_2);
    /// ```
    #[must_use]
    pub fn find(model: &AutosarModel) -> Option<Self> {
        let elem = model
            .identifiable_elements()
            .filter_map(|(_, weak)| weak.upgrade())
            .find(|elem| elem.element_name() == ElementName::System)?;
        Some(Self(elem))
    }

    /// Create a new SYSTEM in the given AR-PACKAGE
    ///
    /// Note that an Autosar model should ony contain one SYSTEM. This is not checked here.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// let package = ArPackage::get_or_create(&model, "/my/pkg").unwrap();
    /// let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// assert!(model.get_element_by_path("/my/pkg/System").is_some())
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the SYSTEM element
    pub fn new(name: &str, package: &ArPackage, category: SystemCategory) -> Result<Self, AutosarAbstractionError> {
        let pkg_elem_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;

        let system_elem = pkg_elem_elements.create_named_sub_element(ElementName::System, name)?;
        system_elem
            .create_sub_element(ElementName::Category)
            .and_then(|cat| cat.set_character_data(category.to_string()))?;

        Ok(System(system_elem))
    }

    /// create an `EcuInstance` that is connected to this System
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package1 = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// let system = System::new("System", &package1, SystemCategory::SystemExtract).unwrap();
    /// # let package2 = ArPackage::get_or_create(&model, "/pkg2").unwrap();
    /// let ecu_instance = system.create_ecu_instance("ecu_name", &package2).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn create_ecu_instance(&self, name: &str, package: &ArPackage) -> Result<EcuInstance, AutosarAbstractionError> {
        let ecu_instance = EcuInstance::new(name, package)?;
        self.create_fibex_element_ref_unchecked(&ecu_instance.element())?;

        Ok(ecu_instance)
    }

    /// get an iterator over all ECU-INSTANCEs in this SYSTEM
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// system.create_ecu_instance("ecu_name1", &package).unwrap();
    /// system.create_ecu_instance("ecu_name2", &package).unwrap();
    /// for ecu in system.ecu_instances() {
    ///     // do something
    /// }
    /// assert_eq!(system.ecu_instances().count(), 2);
    /// ```
    #[must_use]
    pub fn ecu_instances(&self) -> EcuInstanceIterator {
        EcuInstanceIterator::new(self)
    }

    /// create a new CAN-CLUSTER
    ///
    /// The cluster must have a channel to be valid, but this channel is not created automatically.
    /// Call [`CanCluster::create_physical_channel`] to create it.
    ///
    /// Use the [`CanClusterSettings`] to define the baudrates for Can, CanFD, and CanXL.
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
    /// let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// let cluster = system.create_can_cluster("can_cluster", &package, &CanClusterSettings::default()).unwrap();
    /// cluster.create_physical_channel("can_channel");
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the can cluster
    pub fn create_can_cluster(
        &self,
        cluster_name: &str,
        package: &ArPackage,
        settings: &CanClusterSettings,
    ) -> Result<CanCluster, AutosarAbstractionError> {
        let cluster = CanCluster::new(cluster_name, package, settings)?;
        self.create_fibex_element_ref_unchecked(&cluster.element())?;

        Ok(cluster)
    }

    /// create a new ETHERNET-CLUSTER and connect it to the SYSTEM
    ///
    /// The cluster must have at least one channel to be valid.
    /// Call [`EthernetCluster::create_physical_channel`] to create it.
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
    /// let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// let cluster = system.create_ethernet_cluster("ethernet_cluster", &package).unwrap();
    /// let vlan_info = EthernetVlanInfo { vlan_name: "VLAN_1".to_string(), vlan_id: 1};
    /// cluster.create_physical_channel("ethernet_channel", Some(vlan_info));
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ethernet cluster
    pub fn create_ethernet_cluster(
        &self,
        cluster_name: &str,
        package: &ArPackage,
    ) -> Result<EthernetCluster, AutosarAbstractionError> {
        let cluster = EthernetCluster::new(cluster_name, package)?;
        self.create_fibex_element_ref_unchecked(&cluster.element())?;

        Ok(cluster)
    }

    /// create a new FLEXRAY-CLUSTER and connect it to the SYSTEM
    ///
    /// A `FlexrayClusterSettings` structure containing the timings and parameters for the Flexray cluster must be provided.
    ///
    /// The cluster must have at least one channel to be valid.
    /// Call [`FlexrayCluster::create_physical_channel`] to create it.
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
    /// let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// let cluster = system.create_flexray_cluster("flexray_cluster", &package, &FlexrayClusterSettings::default()).unwrap();
    /// cluster.create_physical_channel("flexray_channel", FlexrayChannelName::A);
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the flexray cluster
    pub fn create_flexray_cluster(
        &self,
        cluster_name: &str,
        package: &ArPackage,
        settings: &FlexrayClusterSettings,
    ) -> Result<FlexrayCluster, AutosarAbstractionError> {
        let cluster = FlexrayCluster::new(cluster_name, package, settings)?;
        self.create_fibex_element_ref_unchecked(&cluster.element())?;

        Ok(cluster)
    }

    /// create a new [`CanFrame`]
    ///
    /// This new frame needs to be linked to a `CanPhysicalChannel`
    pub fn create_can_frame(
        &self,
        name: &str,
        byte_length: u64,
        package: &ArPackage,
    ) -> Result<CanFrame, AutosarAbstractionError> {
        let can_frame = CanFrame::new(name, byte_length, package)?;
        self.create_fibex_element_ref_unchecked(can_frame.element())?;

        Ok(can_frame)
    }

    /// create a new [`FlexrayFrame`]
    ///
    /// This new frame needs to be linked to a `FlexrayPhysicalChannel`
    pub fn create_flexray_frame(
        &self,
        name: &str,
        byte_length: u64,
        package: &ArPackage,
    ) -> Result<FlexrayFrame, AutosarAbstractionError> {
        let flexray_frame = FlexrayFrame::new(name, byte_length, package)?;
        self.create_fibex_element_ref_unchecked(flexray_frame.element())?;

        Ok(flexray_frame)
    }

    /// create a new isignal in the [`System`]
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
    /// let sig_package = ArPackage::get_or_create(&model, "/ISignals").unwrap();
    /// let sys_package = ArPackage::get_or_create(&model, "/SystemSignals").unwrap();
    /// let system_signal = SystemSignal::new("signal1", &sys_package).unwrap();
    /// system.create_isignal("signal1", 32, &system_signal, &sig_package).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::InvalidParameter`] sig_package and sys_package may not be identical
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_isignal(
        &self,
        name: &str,
        bit_length: u64,
        syssignal: &SystemSignal,
        package: &ArPackage,
    ) -> Result<ISignal, AutosarAbstractionError> {
        let i_signal = ISignal::new(name, bit_length, syssignal, package)?;

        self.create_fibex_element_ref_unchecked(i_signal.element())?;

        Ok(i_signal)
    }

    /// create a new signal group in the [`System`]
    ///
    /// `I-SIGNAL-GROUP` and `SYSTEM-SIGNAL-GROUP` are created using the same name; therefore they must be placed in
    /// different packages: sig_package and sys_package may not be identical.
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
    /// let sig_package = ArPackage::get_or_create(&model, "/ISignals").unwrap();
    /// let sys_package = ArPackage::get_or_create(&model, "/SystemSignals").unwrap();
    /// let system_signal_group = SystemSignalGroup::new("signalgroup", &sys_package).unwrap();
    /// system.create_i_signal_group("signal_group", &system_signal_group, &sig_package).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::InvalidParameter`] sig_package and sys_package may not be identical
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_i_signal_group(
        &self,
        name: &str,
        system_signal_group: &SystemSignalGroup,
        package: &ArPackage,
    ) -> Result<ISignalGroup, AutosarAbstractionError> {
        let i_signal_group = ISignalGroup::new(name, system_signal_group, package)?;

        self.create_fibex_element_ref_unchecked(i_signal_group.element())?;

        Ok(i_signal_group)
    }

    /// create an [`ISignalIPdu`] in the [`System`]
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
    /// let package = ArPackage::get_or_create(&model, "/Pdus").unwrap();
    /// system.create_isignal_ipdu("pdu", &package, 42).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_isignal_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
    ) -> Result<ISignalIPdu, AutosarAbstractionError> {
        let pdu = ISignalIPdu::new(name, package, length)?;
        self.create_fibex_element_ref_unchecked(pdu.element())?;

        Ok(pdu)
    }

    /// create an [`NmPdu`] in the [`System`]
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
    /// let package = ArPackage::get_or_create(&model, "/Pdus").unwrap();
    /// system.create_nm_pdu("pdu", &package, 42).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_nm_pdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
    ) -> Result<NmPdu, AutosarAbstractionError> {
        let pdu = NmPdu::new(name, package, length)?;
        self.create_fibex_element_ref_unchecked(pdu.element())?;

        Ok(pdu)
    }

    /// create an [`NPdu`] in the [`System`]
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
    /// let package = ArPackage::get_or_create(&model, "/Pdus").unwrap();
    /// system.create_n_pdu("pdu", &package, 42).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_n_pdu(&self, name: &str, package: &ArPackage, length: u32) -> Result<NPdu, AutosarAbstractionError> {
        let pdu = NPdu::new(name, package, length)?;
        self.create_fibex_element_ref_unchecked(pdu.element())?;

        Ok(pdu)
    }

    /// create a [`DcmIPdu`] in the [`System`]
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
    /// let package = ArPackage::get_or_create(&model, "/Pdus").unwrap();
    /// system.create_dcm_ipdu("pdu", &package, 42).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_dcm_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
    ) -> Result<DcmIPdu, AutosarAbstractionError> {
        let pdu = DcmIPdu::new(name, package, length)?;
        self.create_fibex_element_ref_unchecked(pdu.element())?;

        Ok(pdu)
    }

    /// create a [`GeneralPurposePdu`] in the [`System`]
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
    /// let package = ArPackage::get_or_create(&model, "/Pdus").unwrap();
    /// system.create_general_purpose_pdu("pdu", &package, 42).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_general_purpose_pdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
    ) -> Result<GeneralPurposePdu, AutosarAbstractionError> {
        let pdu = GeneralPurposePdu::new(name, package, length)?;
        self.create_fibex_element_ref_unchecked(pdu.element())?;

        Ok(pdu)
    }

    /// create a [`GeneralPurposeIPdu`] in the [`System`]
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
    /// let package = ArPackage::get_or_create(&model, "/Pdus").unwrap();
    /// system.create_general_purpose_ipdu("pdu", &package, 42).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_general_purpose_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
    ) -> Result<GeneralPurposeIPdu, AutosarAbstractionError> {
        let pdu = GeneralPurposeIPdu::new(name, package, length)?;
        self.create_fibex_element_ref_unchecked(pdu.element())?;

        Ok(pdu)
    }

    /// create a [`ContainerIPdu`] in the [`System`]
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
    /// let package = ArPackage::get_or_create(&model, "/Pdus").unwrap();
    /// system.create_container_ipdu("pdu", &package, 42).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_container_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
    ) -> Result<ContainerIPdu, AutosarAbstractionError> {
        let pdu = ContainerIPdu::new(name, package, length)?;
        self.create_fibex_element_ref_unchecked(pdu.element())?;

        Ok(pdu)
    }

    /// create a [`SecuredIPdu`] in the [`System`]
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
    /// let package = ArPackage::get_or_create(&model, "/Pdus").unwrap();
    /// system.create_secured_ipdu("pdu", &package, 42).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_secured_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
    ) -> Result<SecuredIPdu, AutosarAbstractionError> {
        let pdu = SecuredIPdu::new(name, package, length)?;
        self.create_fibex_element_ref_unchecked(pdu.element())?;

        Ok(pdu)
    }

    /// create a [`MultiplexedIPdu`] in the [`System`]
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
    /// let package = ArPackage::get_or_create(&model, "/Pdus").unwrap();
    /// system.create_multiplexed_ipdu("pdu", &package, 42).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create elements
    pub fn create_multiplexed_ipdu(
        &self,
        name: &str,
        package: &ArPackage,
        length: u32,
    ) -> Result<MultiplexedIPdu, AutosarAbstractionError> {
        let pdu = MultiplexedIPdu::new(name, package, length)?;
        self.create_fibex_element_ref_unchecked(pdu.element())?;

        Ok(pdu)
    }

    /// Create an iterator over all clusters connected to the SYSTEM
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
    /// let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// system.create_can_cluster("can_cluster", &package, &CanClusterSettings::default()).unwrap();
    /// system.create_flexray_cluster("flexray_cluster", &package, &FlexrayClusterSettings::default()).unwrap();
    /// for cluster in system.clusters() {
    ///     // do something
    /// }
    /// assert_eq!(system.clusters().count(), 2);
    /// ```
    #[must_use]
    pub fn clusters(&self) -> ClusterIterator {
        ClusterIterator::new(self)
    }

    /// connect an element to the SYSTEM by creating a FIBEX-ELEMENT-REF
    ///
    /// If there is already a FIBEX-ELEMENT-REF, this function does nothing, successfully.
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
    /// let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let pkg_elements = package.element().get_sub_element(ElementName::Elements).unwrap();
    /// let can_cluster = pkg_elements.create_named_sub_element(ElementName::CanCluster, "Cluster").unwrap();
    /// system.create_fibex_element_ref(&can_cluster).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model
    pub fn create_fibex_element_ref(&self, elem: &Element) -> Result<(), AutosarDataError> {
        let model = elem.model()?;
        let refs = model.get_references_to(&elem.path()?);
        for reference in refs.iter().filter_map(WeakElement::upgrade) {
            if reference.element_name() == ElementName::FibexElementRef {
                // a FIBEX-ELEMENT-REF for this elem already exists.
                return Ok(());
            }
        }
        self.create_fibex_element_ref_unchecked(elem)
    }

    fn create_fibex_element_ref_unchecked(&self, elem: &Element) -> Result<(), AutosarDataError> {
        let fibex_elements = self.0.get_or_create_sub_element(ElementName::FibexElements)?;
        let fibex_element_ref = fibex_elements
            .create_sub_element(ElementName::FibexElementRefConditional)?
            .create_sub_element(ElementName::FibexElementRef)?;
        fibex_element_ref.set_reference_target(elem)
    }

    /// set the root software composition of the system
    ///
    /// This function will remove any existing root software composition
    pub fn set_root_sw_composition(
        &self,
        name: &str,
        composition_type: &CompositionSwComponentType,
    ) -> Result<RootSwCompositionPrototype, AutosarAbstractionError> {
        let root_compositions = self
            .0
            .get_or_create_sub_element(ElementName::RootSoftwareCompositions)?;

        if let Some(existing_composition) = root_compositions.get_sub_element(ElementName::RootSwCompositionPrototype) {
            root_compositions.remove_sub_element(existing_composition)?;
        }
        RootSwCompositionPrototype::new(name, &root_compositions, composition_type)
    }

    /// get the root software composition of the system
    pub fn root_composition(&self) -> Option<RootSwCompositionPrototype> {
        let root_compositions = self.element().get_sub_element(ElementName::RootSoftwareCompositions)?;
        let root_composition = root_compositions.get_sub_element(ElementName::RootSwCompositionPrototype)?;
        RootSwCompositionPrototype::try_from(root_composition).ok()
    }

    /// get or create a mapping for this system
    ///
    /// There does not seem to be any benefit to having multiple mappings for a single system, so this function
    /// will return the first mapping if it exists. Otherwise a new mapping will be created with the provided name.
    pub fn get_or_create_mapping(&self, name: &str) -> Result<SystemMapping, AutosarAbstractionError> {
        if let Some(mapping) = self.0.get_sub_element(ElementName::Mappings) {
            if let Some(mapping) = mapping.get_sub_element(ElementName::SystemMapping) {
                return SystemMapping::try_from(mapping);
            }
        }
        SystemMapping::new(name, self)
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemMapping(Element);
abstraction_element!(SystemMapping, SystemMapping);

impl SystemMapping {
    pub(crate) fn new(name: &str, system: &System) -> Result<Self, AutosarAbstractionError> {
        let element = system
            .element()
            .get_or_create_sub_element(ElementName::Mappings)?
            .create_named_sub_element(ElementName::SystemMapping, name)?;

        Ok(Self(element))
    }

    /// get the system that contains this mapping
    pub fn system(&self) -> Result<System, AutosarAbstractionError> {
        let sys_elem = self.element().named_parent()?.unwrap();
        System::try_from(sys_elem)
    }

    /// create a new mapping between a SWC and an ECU
    pub fn map_swc_to_ecu(
        &self,
        name: &str,
        component_prototype: &SwComponentPrototype,
        ecu: &EcuInstance,
    ) -> Result<SwcToEcuMapping, AutosarAbstractionError> {
        let root_composition_prototype =
            self.system()?
                .root_composition()
                .ok_or(AutosarAbstractionError::InvalidParameter(
                    "The root compositon must be set before mapping any swc".to_string(),
                ))?;
        let root_composition_type =
            root_composition_prototype
                .composition()
                .ok_or(AutosarAbstractionError::InvalidParameter(
                    "Incomplete root composition prototype".to_string(),
                ))?;

        let mut context_composition_prototypes = vec![];
        let mut current_composition = component_prototype.parent_composition()?;

        // check if the composition is a child of the root composition; this is needed to ensure that the loop can terminate
        if root_composition_type != current_composition && !root_composition_type.is_parent_of(&current_composition) {
            return Err(AutosarAbstractionError::InvalidParameter(
                "The composition is not a child of the root composition".to_string(),
            ));
        }

        // find all compositions between the root composition and the current composition
        while current_composition != root_composition_type {
            // typical case is that each component is only in one composition, so the for loop should only run once
            for comp_proto in current_composition.instances() {
                // this condition should never fail - it only returns none if comp_proto is the root
                // composition, which we already know is not true
                if let Ok(Some(comp_type)) = comp_proto.parent_composition() {
                    if root_composition_type == comp_type || root_composition_type.is_parent_of(&comp_type) {
                        context_composition_prototypes.push(comp_proto.clone());
                        current_composition = comp_type;
                        break;
                    }
                }
            }
        }

        // the items were collected in reverse order, so we need to reverse them again
        context_composition_prototypes.reverse();

        SwcToEcuMapping::new(
            name,
            component_prototype,
            &context_composition_prototypes,
            &root_composition_prototype,
            ecu,
            self,
        )
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwcToEcuMapping(Element);
abstraction_element!(SwcToEcuMapping, SwcToEcuMapping);

impl SwcToEcuMapping {
    pub(crate) fn new(
        name: &str,
        component_prototype: &SwComponentPrototype,
        context_composition_prototypes: &[ComponentPrototype],
        root_composition_prototype: &RootSwCompositionPrototype,
        ecu: &EcuInstance,
        mapping: &SystemMapping,
    ) -> Result<Self, AutosarAbstractionError> {
        let sw_mappings_elem = mapping.element().get_or_create_sub_element(ElementName::SwMappings)?;
        let swc_to_ecu_mapping = sw_mappings_elem.create_named_sub_element(ElementName::SwcToEcuMapping, name)?;

        let iref = swc_to_ecu_mapping
            .create_sub_element(ElementName::ComponentIrefs)?
            .create_sub_element(ElementName::ComponentIref)?;

        // create the references to root composition and context compositions
        iref.create_sub_element(ElementName::ContextCompositionRef)?
            .set_reference_target(root_composition_prototype.element())?;
        for context_comp in context_composition_prototypes {
            iref.create_sub_element(ElementName::ContextComponentRef)?
                .set_reference_target(context_comp.element())?;
        }
        // create the reference to the target component prototype
        iref.create_sub_element(ElementName::TargetComponentRef)?
            .set_reference_target(component_prototype.element())?;

        swc_to_ecu_mapping
            .create_sub_element(ElementName::EcuInstanceRef)?
            .set_reference_target(ecu.element())?;

        Ok(Self(swc_to_ecu_mapping))
    }

    /// get the component prototype that is mapped here
    pub fn target_component(&self) -> Option<SwComponentPrototype> {
        self.element()
            .get_sub_element(ElementName::ComponentIrefs)
            .and_then(|irefs| irefs.get_sub_element(ElementName::ComponentIref))
            .and_then(|iref| iref.get_sub_element(ElementName::TargetComponentRef))
            .and_then(|target| target.get_reference_target().ok())
            .and_then(|target| SwComponentPrototype::try_from(target).ok())
    }

    /// get the ECU instance which is the target of this mapping
    pub fn ecu_instance(&self) -> Option<EcuInstance> {
        self.element()
            .get_sub_element(ElementName::EcuInstanceRef)
            .and_then(|r| r.get_reference_target().ok())
            .and_then(|target| EcuInstance::try_from(target).ok())
    }
}

//#########################################################

/// The category of a System
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemCategory {
    /// The `System` is used to describe system constraints
    SystemConstraints,
    /// The `System` is used to describe the system configuration of a complete AUTOSAR system
    SystemDescription,
    /// The `System` is used to describe a subsystem specific view on the complete system description
    SystemExtract,
    /// The `System` is used to describe the ECU specific view on the complete system description
    EcuExtract,
    /// The `System` is used to describe a functional (solution-independent/abstract) system design
    AbstractSystemDescription,
    /// The `System` is used to describe the closed view on one ECU
    EcuSystemDescription,
    /// The `System` describes the content of one CpSoftwareCluster
    SwClusterSystemDescription,
    /// `System` which describes the rapid prototyping algorithm in the format of AUTOSAR Software Components
    RptSystem,
}

impl ToString for SystemCategory {
    fn to_string(&self) -> String {
        match self {
            SystemCategory::SystemConstraints => "SYSTEM_CONSTRAINTS".to_string(),
            SystemCategory::SystemDescription => "SYSTEM_DESCRIPTION".to_string(),
            SystemCategory::SystemExtract => "SYSTEM_EXTRACT".to_string(),
            SystemCategory::EcuExtract => "ECU_EXTRACT".to_string(),
            SystemCategory::AbstractSystemDescription => "ABSTRACT_SYSTEM_DESCRIPTION".to_string(),
            SystemCategory::EcuSystemDescription => "ECU_SYSTEM_DESCRIPTION".to_string(),
            SystemCategory::SwClusterSystemDescription => "SW_CLUSTER_SYSTEM_DESCRIPTION".to_string(),
            SystemCategory::RptSystem => "RPT_SYSTEM".to_string(),
        }
    }
}

//#########################################################

pub struct EcuInstanceIterator {
    fibex_elements: Option<Element>,
    position: usize,
}

impl EcuInstanceIterator {
    pub(crate) fn new(system: &System) -> Self {
        let fibex_elements = system.0.get_sub_element(ElementName::FibexElements);

        EcuInstanceIterator {
            fibex_elements,
            position: 0,
        }
    }
}

impl Iterator for EcuInstanceIterator {
    type Item = EcuInstance;

    fn next(&mut self) -> Option<Self::Item> {
        let fibelem = self.fibex_elements.as_ref()?;

        while let Some(fibrefcond) = fibelem.get_sub_element_at(self.position) {
            self.position += 1;
            if let Some(ecuinstance) = fibrefcond
                .get_sub_element(ElementName::FibexElementRef)
                .and_then(|r| r.get_reference_target().ok())
                .and_then(|target| EcuInstance::try_from(target).ok())
            {
                return Some(ecuinstance);
            }
        }
        self.fibex_elements = None;
        None
    }
}

impl FusedIterator for EcuInstanceIterator {}

//#########################################################

#[doc(hidden)]
pub struct ClusterIterator {
    fibex_elements: Option<Element>,
    position: usize,
}

impl ClusterIterator {
    pub(crate) fn new(system: &System) -> Self {
        let fibex_elements = system.0.get_sub_element(ElementName::FibexElements);

        ClusterIterator {
            fibex_elements,
            position: 0,
        }
    }
}

impl Iterator for ClusterIterator {
    type Item = Cluster;

    fn next(&mut self) -> Option<Self::Item> {
        let fibelem = self.fibex_elements.as_ref()?;

        while let Some(fibrefcond) = fibelem.get_sub_element_at(self.position) {
            self.position += 1;
            if let Some(ref_element) = fibrefcond
                .get_sub_element(ElementName::FibexElementRef)
                .and_then(|r| r.get_reference_target().ok())
            {
                match ref_element.element_name() {
                    ElementName::CanCluster => {
                        if let Ok(can_cluster) = CanCluster::try_from(ref_element) {
                            return Some(Cluster::Can(can_cluster));
                        }
                    }
                    ElementName::EthernetCluster => {
                        if let Ok(can_cluster) = EthernetCluster::try_from(ref_element) {
                            return Some(Cluster::Ethernet(can_cluster));
                        }
                    }
                    ElementName::FlexrayCluster => {
                        if let Ok(can_cluster) = FlexrayCluster::try_from(ref_element) {
                            return Some(Cluster::FlexRay(can_cluster));
                        }
                    }
                    // missing: LIN, TTCAN, J1939
                    _ => {}
                }
            }
        }
        self.fibex_elements = None;
        None
    }
}

impl FusedIterator for ClusterIterator {}

//#########################################################

#[cfg(test)]
mod test {
    use crate::{
        communication::{CanClusterSettings, FlexrayClusterSettings},
        software_component::CompositionSwComponentType,
        system::SystemCategory,
        AbstractionElement, ArPackage, System,
    };
    use autosar_data::{AutosarModel, AutosarVersion, ElementName};

    #[test]
    fn system() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();

        // try to find a system in the empty model
        let result = System::find(&model);
        assert!(result.is_none());

        // create a System
        let package = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();

        // find the newly created system
        let system_2 = System::find(&model).unwrap();
        assert_eq!(system, system_2);
    }

    #[test]
    fn system_category() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        System::new("System", &package, SystemCategory::AbstractSystemDescription).unwrap();

        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        System::new("System", &package, SystemCategory::EcuExtract).unwrap();

        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        System::new("System", &package, SystemCategory::EcuSystemDescription).unwrap();

        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        System::new("System", &package, SystemCategory::RptSystem).unwrap();

        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        System::new("System", &package, SystemCategory::SwClusterSystemDescription).unwrap();

        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        System::new("System", &package, SystemCategory::SystemConstraints).unwrap();

        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        System::new("System", &package, SystemCategory::SystemDescription).unwrap();

        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    }

    #[test]
    fn fibex_ref() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        let system = System::new("System", &package, SystemCategory::SystemDescription).unwrap();

        let el_elements = package
            .element()
            .get_or_create_sub_element(ElementName::Elements)
            .unwrap();
        let el_ecuinst = el_elements
            .create_named_sub_element(ElementName::EcuInstance, "Ecu")
            .unwrap();

        let el_fibex_elements = system
            .element()
            .get_or_create_sub_element(ElementName::FibexElements)
            .unwrap();
        assert_eq!(el_fibex_elements.sub_elements().count(), 0);

        // create one reference
        system.create_fibex_element_ref(&el_ecuinst).unwrap();
        assert_eq!(el_fibex_elements.sub_elements().count(), 1);
        // find the existing reference and do nothing
        system.create_fibex_element_ref(&el_ecuinst).unwrap();
        assert_eq!(el_fibex_elements.sub_elements().count(), 1);
    }

    #[test]
    fn ecu_instance_iterator() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package_1 = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        let system = System::new("System", &package_1, SystemCategory::SystemExtract).unwrap();
        let package_2 = ArPackage::get_or_create(&model, "/ECU").unwrap();
        system.create_ecu_instance("Ecu_1", &package_2).unwrap();
        system.create_ecu_instance("Ecu_2", &package_2).unwrap();
        system.create_ecu_instance("Ecu_3", &package_2).unwrap();

        let mut iter = system.ecu_instances();
        let item = iter.next().unwrap();
        assert_eq!(item.name().unwrap(), "Ecu_1");
        assert_eq!(model.get_element_by_path("/ECU/Ecu_1").unwrap(), *item.element());
        let item = iter.next().unwrap();
        assert_eq!(item.name().unwrap(), "Ecu_2");
        assert_eq!(model.get_element_by_path("/ECU/Ecu_2").unwrap(), *item.element());
        let item = iter.next().unwrap();
        assert_eq!(item.name().unwrap(), "Ecu_3");
        assert_eq!(model.get_element_by_path("/ECU/Ecu_3").unwrap(), *item.element());

        assert!(iter.next().is_none());
        // after returning none the iterator continues to return none
        assert!(iter.next().is_none());
    }

    #[test]
    fn cluster_iterator() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package_1 = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        let system = System::new("System", &package_1, SystemCategory::SystemExtract).unwrap();
        let package_2 = ArPackage::get_or_create(&model, "/Clusters").unwrap();

        let settings = CanClusterSettings::new();
        system.create_can_cluster("CanCluster", &package_2, &settings).unwrap();

        let settings = FlexrayClusterSettings::new();
        system
            .create_flexray_cluster("FlexrayCluster", &package_2, &settings)
            .unwrap();

        system.create_ethernet_cluster("EthernetCluster", &package_2).unwrap();

        // the ecu-instance is a fourth item in the FIBEX-ELEMENTS of the system, which should not be picked up by the iterator
        let package_3 = ArPackage::get_or_create(&model, "/ECU").unwrap();
        system.create_ecu_instance("Ecu_1", &package_3).unwrap();

        assert_eq!(system.clusters().count(), 3);
    }

    #[test]
    fn sw_mapping() {
        let model = AutosarModel::new();
        let _file = model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package_1 = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        let system = System::new("System", &package_1, SystemCategory::SystemExtract).unwrap();
        let package_2 = ArPackage::get_or_create(&model, "/SWC").unwrap();
        let package_3 = ArPackage::get_or_create(&model, "/ECU").unwrap();

        let root_composition = CompositionSwComponentType::new("RootComposition", &package_2).unwrap();
        let context_composition = CompositionSwComponentType::new("ContextComposition", &package_2).unwrap();
        let ecu_composition = CompositionSwComponentType::new("EcuComposition", &package_2).unwrap();
        let _root_proto = system
            .set_root_sw_composition("RootComposition", &root_composition)
            .unwrap();
        assert_eq!(system.root_composition().unwrap(), _root_proto);

        let context_proto = root_composition
            .add_component("ContextComposition", &context_composition.clone().into())
            .unwrap();
        let ecu_proto = context_composition
            .add_component("EcuComposition", &ecu_composition.into())
            .unwrap();
        let ecu = system.create_ecu_instance("Ecu", &package_3).unwrap();

        let mapping = system.get_or_create_mapping("Mapping").unwrap();
        mapping.map_swc_to_ecu("SwcToEcu1", &context_proto, &ecu).unwrap();
        let swc_to_ecu = mapping.map_swc_to_ecu("SwcToEcu2", &ecu_proto, &ecu).unwrap();

        assert_eq!(swc_to_ecu.target_component().unwrap(), ecu_proto);
        assert_eq!(swc_to_ecu.ecu_instance().unwrap(), ecu);

        // println!("{}", _file.serialize().unwrap());
    }
}