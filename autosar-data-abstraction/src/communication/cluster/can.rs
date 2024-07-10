use crate::communication::{AbstractCluster, CanPhysicalChannel};
use crate::{abstraction_element, AbstractionElement, ArPackage, AutosarAbstractionError};
use autosar_data::{Element, ElementName};

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
                .and_then(|pn| pn.set_character_data("CAN"));

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
    /// # use autosar_data_abstraction::communication::*;
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
                .and_then(|br| br.set_character_data(settings.baudrate.to_string()));
            if let Some(can_fd_baudrate) = settings.can_fd_baudrate {
                let _ = cluster_content
                    .create_sub_element(ElementName::CanFdBaudrate)
                    .and_then(|cfbr| cfbr.set_character_data(can_fd_baudrate.to_string()));
            } else if let Some(cfbr) = cluster_content.get_sub_element(ElementName::CanFdBaudrate) {
                let _ = cluster_content.remove_sub_element(cfbr);
            }
            if let Some(can_xl_baudrate) = settings.can_xl_baudrate {
                cluster_content
                    .create_sub_element(ElementName::CanXlBaudrate)
                    .and_then(|cxbr| cxbr.set_character_data(can_xl_baudrate.to_string()))
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
    /// # use autosar_data_abstraction::communication::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// let settings = CanClusterSettings {baudrate: 500000, can_fd_baudrate: None, can_xl_baudrate: None};
    /// let cluster = system.create_can_cluster("Cluster", &package, &settings).unwrap();
    /// let settings2 = cluster.get_settings();
    /// assert_eq!(settings, settings2);
    /// ```
    #[must_use]
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
    /// # use autosar_data_abstraction::communication::*;
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

        CanPhysicalChannel::try_from(channel)
    }

    /// return the CanPhysicalChannel of the Cluster, if it has been created
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
    /// # let cluster = system.create_can_cluster("Cluster", &package, &CanClusterSettings::default()).unwrap();
    /// # let can_channel = cluster.create_physical_channel("Channel").unwrap();
    /// let channel = cluster.physical_channel().unwrap();
    /// # assert_eq!(channel, can_channel);
    /// ````
    #[must_use]
    pub fn physical_channel(&self) -> Option<CanPhysicalChannel> {
        let channel = self
            .0
            .get_sub_element(ElementName::CanClusterVariants)?
            .get_sub_element(ElementName::CanClusterConditional)?
            .get_sub_element(ElementName::PhysicalChannels)?
            .get_sub_element(ElementName::CanPhysicalChannel)?;
        CanPhysicalChannel::try_from(channel).ok()
    }
}

impl AbstractCluster for CanCluster {}

//##################################################################

/// Settings for a CAN cluster
#[derive(Debug, Clone, PartialEq)]
pub struct CanClusterSettings {
    pub baudrate: u32,
    pub can_fd_baudrate: Option<u32>,
    pub can_xl_baudrate: Option<u32>,
}

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
    use crate::{
        communication::{AbstractCluster, CanClusterSettings},
        ArPackage, System, SystemCategory,
    };
    use autosar_data::{AutosarModel, AutosarVersion};

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

        // system link
        let linked_system = cluster.system().unwrap();
        assert_eq!(linked_system, system);

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
}
