use std::iter::FusedIterator;

use crate::{
    abstraction_element, can::CanClusterSettings, flexray::FlexrayClusterSettings, AbstractionElement, ArPackage,
    AutosarAbstractionError, CanCluster, EcuInstance, EthernetCluster, FlexrayCluster,
};
use autosar_data::{AutosarDataError, AutosarModel, Element, ElementName, WeakElement};

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
            .and_then(|cat| cat.set_character_data(autosar_data::CharacterData::String(category.to_string())))?;

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

    /// Create an iterator over all clusters connected to the SYSTEM
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

/// An `AbstractCluster` is returned by [`System::clusters`].
/// It can contain any supported communication cluster.
#[non_exhaustive]
pub enum AbstractCluster {
    Can(CanCluster),
    Ethernet(EthernetCluster),
    FlexRay(FlexrayCluster),
    // missing: Lin, TTCAN, J1939, CDD (aka user defined)
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
    type Item = AbstractCluster;

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
                            return Some(AbstractCluster::Can(can_cluster));
                        }
                    }
                    ElementName::EthernetCluster => {
                        if let Ok(can_cluster) = EthernetCluster::try_from(ref_element) {
                            return Some(AbstractCluster::Ethernet(can_cluster));
                        }
                    }
                    ElementName::FlexrayCluster => {
                        if let Ok(can_cluster) = FlexrayCluster::try_from(ref_element) {
                            return Some(AbstractCluster::FlexRay(can_cluster));
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
        can::CanClusterSettings, flexray::FlexrayClusterSettings, system::SystemCategory, AbstractionElement, ArPackage,
        System,
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
        assert_eq!(item.name(), "Ecu_1");
        assert_eq!(model.get_element_by_path("/ECU/Ecu_1").unwrap(), item.element());
        let item = iter.next().unwrap();
        assert_eq!(item.name(), "Ecu_2");
        assert_eq!(model.get_element_by_path("/ECU/Ecu_2").unwrap(), item.element());
        let item = iter.next().unwrap();
        assert_eq!(item.name(), "Ecu_3");
        assert_eq!(model.get_element_by_path("/ECU/Ecu_3").unwrap(), item.element());

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
}
