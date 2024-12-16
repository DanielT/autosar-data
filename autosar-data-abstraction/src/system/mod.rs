use crate::communication::{
    CanCluster, CanClusterSettings, CanFrame, CanTpConfig, Cluster, ContainerIPdu, DcmIPdu, DoIpTpConfig,
    EthernetCluster, EventGroupControlType, FlexrayArTpConfig, FlexrayCluster, FlexrayClusterSettings, FlexrayFrame,
    FlexrayTpConfig, GeneralPurposeIPdu, GeneralPurposeIPduCategory, GeneralPurposePdu, GeneralPurposePduCategory,
    ISignal, ISignalGroup, ISignalIPdu, MultiplexedIPdu, NPdu, NmPdu, SecuredIPdu, ServiceInstanceCollectionSet,
    SoAdRoutingGroup, SocketConnectionIpduIdentifierSet, SomeipTpConfig, SystemSignal, SystemSignalGroup,
};
use crate::datatype::SwBaseType;
use crate::software_component::{CompositionSwComponentType, RootSwCompositionPrototype};
use crate::{abstraction_element, AbstractionElement, ArPackage, AutosarAbstractionError, EcuInstance};
use autosar_data::{AutosarDataError, AutosarModel, Element, ElementName, WeakElement};
use std::iter::FusedIterator;

mod mapping;

pub use mapping::*;

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
        self.create_fibex_element_ref_unchecked(ecu_instance.element())?;

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
    pub fn ecu_instances(&self) -> impl Iterator<Item = EcuInstance> {
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
        self.create_fibex_element_ref_unchecked(cluster.element())?;

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
        self.create_fibex_element_ref_unchecked(cluster.element())?;

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
        self.create_fibex_element_ref_unchecked(cluster.element())?;

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
    /// system.create_isignal("signal1", 32, &system_signal, None, &sig_package).unwrap();
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
        datatype: Option<&SwBaseType>,
        package: &ArPackage,
    ) -> Result<ISignal, AutosarAbstractionError> {
        let i_signal = ISignal::new(name, bit_length, syssignal, datatype, package)?;

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
    /// system.create_general_purpose_pdu("pdu", &package, 42, GeneralPurposePduCategory::GlobalTime).unwrap();
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
        category: GeneralPurposePduCategory,
    ) -> Result<GeneralPurposePdu, AutosarAbstractionError> {
        let pdu = GeneralPurposePdu::new(name, package, length, category)?;
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
    /// system.create_general_purpose_ipdu("pdu", &package, 42, GeneralPurposeIPduCategory::Xcp).unwrap();
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
        category: GeneralPurposeIPduCategory,
    ) -> Result<GeneralPurposeIPdu, AutosarAbstractionError> {
        let pdu = GeneralPurposeIPdu::new(name, package, length, category)?;
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
    pub fn clusters(&self) -> impl Iterator<Item = Cluster> {
        ClusterIterator::new(self)
    }

    /// Create a SocketConnectionIpduIdentifierSet in the SYSTEM
    ///
    /// `SocketConnectionIpduIdentifierSet` are part of the new ethernet modeling that was introduced in Autosar 4.5.0 (AUTOSAR_00048).
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
    /// let set = system.create_socket_connection_ipdu_identifier_set("set", &package).unwrap();
    /// ```
    pub fn create_socket_connection_ipdu_identifier_set(
        &self,
        name: &str,
        package: &ArPackage,
    ) -> Result<SocketConnectionIpduIdentifierSet, AutosarAbstractionError> {
        let set = SocketConnectionIpduIdentifierSet::new(name, package)?;
        self.create_fibex_element_ref_unchecked(set.element())?;

        Ok(set)
    }

    /// Create a SoAdRoutingGroup in the SYSTEM
    ///
    /// `SoAdRoutingGroup` are part of the old ethernet modeling that was used prior to Autosar 4.5.0 (AUTOSAR_00048).
    /// The elements are still present (but obsolete) in newer versions of the standard.
    /// Old and new elements may not be mixed in the same model.
    pub fn create_so_ad_routing_group(
        &self,
        name: &str,
        package: &ArPackage,
        control_type: Option<EventGroupControlType>,
    ) -> Result<SoAdRoutingGroup, AutosarAbstractionError> {
        let group = SoAdRoutingGroup::new(name, package, control_type)?;
        self.create_fibex_element_ref_unchecked(group.element())?;

        Ok(group)
    }

    /// Create a ServiceInstanceCollectionSet in the SYSTEM
    ///
    /// `ServiceInstanceCollectionSet`s are part of the new ethernet modeling that was introduced in Autosar 4.5.0 (AUTOSAR_00048).
    pub fn create_service_instance_collection_set(
        &self,
        name: &str,
        package: &ArPackage,
    ) -> Result<ServiceInstanceCollectionSet, AutosarAbstractionError> {
        let set = ServiceInstanceCollectionSet::new(name, package)?;
        self.create_fibex_element_ref_unchecked(set.element())?;

        Ok(set)
    }

    /// Create a SomipTpConfig in the SYSTEM
    ///
    /// `SomeipTpConfig`s contain the configuration how to segment or reassemble large SomipTp PDUs.
    pub fn create_somip_tp_config<T: Into<Cluster> + Clone>(
        &self,
        name: &str,
        package: &ArPackage,
        cluster: &T,
    ) -> Result<SomeipTpConfig, AutosarAbstractionError> {
        let config = SomeipTpConfig::new(name, package, &cluster.clone().into())?;
        self.create_fibex_element_ref_unchecked(config.element())?;

        Ok(config)
    }

    /// Create a CanTpConfig in the SYSTEM
    ///
    /// `CanTpConfig`s contain the configuration how to segment or reassemble diagnostic messages on a CAN bus.
    pub fn create_can_tp_config(
        &self,
        name: &str,
        package: &ArPackage,
        can_cluster: &CanCluster,
    ) -> Result<CanTpConfig, AutosarAbstractionError> {
        let config = CanTpConfig::new(name, package, can_cluster)?;
        self.create_fibex_element_ref_unchecked(config.element())?;

        Ok(config)
    }

    /// Create a DoIpTpConfig in the SYSTEM
    ///
    /// `DoIpTpConfig`s contain the configuration how to transmit diagnostic messages over IP networks.
    pub fn create_doip_tp_config(
        &self,
        name: &str,
        package: &ArPackage,
        eth_cluster: &EthernetCluster,
    ) -> Result<DoIpTpConfig, AutosarAbstractionError> {
        let config = DoIpTpConfig::new(name, package, eth_cluster)?;
        self.create_fibex_element_ref_unchecked(config.element())?;

        Ok(config)
    }

    /// Create a FlexRayTpConfig in the SYSTEM
    ///
    /// `FlexRayTpConfig`s describe how to segment or reassemble diagnostic messages on a FlexRay bus.
    /// This configuration type is used for Flexray ISO TP communication.
    pub fn create_flexray_tp_config(
        &self,
        name: &str,
        package: &ArPackage,
        flexray_cluster: &FlexrayCluster,
    ) -> Result<FlexrayTpConfig, AutosarAbstractionError> {
        let config = FlexrayTpConfig::new(name, package, flexray_cluster)?;
        self.create_fibex_element_ref_unchecked(config.element())?;

        Ok(config)
    }

    /// Create a FlexrayArTpConfig in the SYSTEM
    ///
    /// `FlexrayArTpConfig`s describe how to segment or reassemble diagnostic messages on a FlexRay bus.
    /// This configuration type is used for Flexray AUTOSAR TP communication.
    pub fn create_flexray_ar_tp_config(
        &self,
        name: &str,
        package: &ArPackage,
        flexray_cluster: &FlexrayCluster,
    ) -> Result<FlexrayArTpConfig, AutosarAbstractionError> {
        let config = FlexrayArTpConfig::new(name, package, flexray_cluster)?;
        self.create_fibex_element_ref_unchecked(config.element())?;

        Ok(config)
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

/// The category of a System
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl std::fmt::Display for SystemCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemCategory::SystemConstraints => f.write_str("SYSTEM_CONSTRAINTS"),
            SystemCategory::SystemDescription => f.write_str("SYSTEM_DESCRIPTION"),
            SystemCategory::SystemExtract => f.write_str("SYSTEM_EXTRACT"),
            SystemCategory::EcuExtract => f.write_str("ECU_EXTRACT"),
            SystemCategory::AbstractSystemDescription => f.write_str("ABSTRACT_SYSTEM_DESCRIPTION"),
            SystemCategory::EcuSystemDescription => f.write_str("ECU_SYSTEM_DESCRIPTION"),
            SystemCategory::SwClusterSystemDescription => f.write_str("SW_CLUSTER_SYSTEM_DESCRIPTION"),
            SystemCategory::RptSystem => f.write_str("RPT_SYSTEM"),
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
            .create_component("ContextComposition", &context_composition.clone())
            .unwrap();
        let ecu_proto = context_composition
            .create_component("EcuComposition", &ecu_composition)
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
