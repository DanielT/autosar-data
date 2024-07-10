use crate::communication::{AbstractCluster, EthernetPhysicalChannel, EthernetVlanInfo};
use crate::{abstraction_element, element_iterator, AbstractionElement, ArPackage, AutosarAbstractionError};
use autosar_data::{Element, ElementName};

/// An `EthernetCluster` contains all configuration items associated with an ethernet network.
/// The cluster connects multiple ECUs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EthernetCluster(Element);
abstraction_element!(EthernetCluster, EthernetCluster);

impl EthernetCluster {
    // create a new EthernetCluster - for internal use. User code should call System::create_ethernet_cluster
    pub(crate) fn new(cluster_name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elem_pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_cluster = elem_pkg_elements.create_named_sub_element(ElementName::EthernetCluster, cluster_name)?;
        if let Ok(cluster_content) = elem_cluster
            .create_sub_element(ElementName::EthernetClusterVariants)
            .and_then(|ecv| ecv.create_sub_element(ElementName::EthernetClusterConditional))
        {
            let _ = cluster_content.create_sub_element(ElementName::PhysicalChannels);
        }

        Ok(EthernetCluster(elem_cluster))
    }

    /// Create a new physical channel for the cluster
    ///
    /// The supplied VLAN info must be unique - there cannot be two VLANs with the same vlan identifier.
    /// One channel may be created without VLAN information; it carries untagged traffic.
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
    /// let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// let vlan_info = EthernetVlanInfo {
    ///     vlan_name: "VLAN_1".to_string(),
    ///     vlan_id: 1,
    /// };
    /// let channel = cluster.create_physical_channel("Channel", Some(vlan_info)).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ItemAlreadyExists`] There is already a physical channel for this VLAN
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn create_physical_channel(
        &self,
        channel_name: &str,
        vlan_info: Option<EthernetVlanInfo>,
    ) -> Result<EthernetPhysicalChannel, AutosarAbstractionError> {
        let phys_channels = self
            .0
            .get_or_create_sub_element(ElementName::EthernetClusterVariants)?
            .get_or_create_sub_element(ElementName::EthernetClusterConditional)?
            .get_or_create_sub_element(ElementName::PhysicalChannels)?;

        // make sure there is no other channel with the same VLAN info
        // If vlan_info is None, then there must be no existing channel without VLAN info
        for existing_channel in phys_channels.sub_elements() {
            let existing_vlan_info = EthernetPhysicalChannel::try_from(existing_channel)
                .ok()
                .and_then(|channel| channel.get_vlan_info());
            if let (Some(v1), Some(v2)) = (&vlan_info, &existing_vlan_info) {
                if v1.vlan_id == v2.vlan_id {
                    // the vlan identifier of an existing channel matches the new vlan identifier
                    return Err(AutosarAbstractionError::ItemAlreadyExists);
                }
            } else if existing_vlan_info.is_none() && vlan_info.is_none() {
                // the new channel is for untagged traffic (no VLAN), but there is already a channel for untagged traffic
                return Err(AutosarAbstractionError::ItemAlreadyExists);
            }
        }

        let channel = phys_channels.create_named_sub_element(ElementName::EthernetPhysicalChannel, channel_name)?;
        // set the vlan info
        if let Some(vlan_info) = vlan_info {
            let _ = channel
                .create_named_sub_element(ElementName::Vlan, &vlan_info.vlan_name)
                .and_then(|vlan| vlan.create_sub_element(ElementName::VlanIdentifier))
                .and_then(|vlan_id| vlan_id.set_character_data(vlan_info.vlan_id.to_string()));
        }
        // always set CATEGORY = WIRED, since this is the common case
        let _ = channel
            .create_sub_element(ElementName::Category)
            .and_then(|cat| cat.set_character_data("WIRED"));

        EthernetPhysicalChannel::try_from(channel)
    }

    /// returns an iterator over all [`EthernetPhysicalChannel`]s in the cluster
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
    /// # let cluster = system.create_ethernet_cluster("Cluster", &package).unwrap();
    /// cluster.create_physical_channel("Channel", None).unwrap();
    /// for channel in cluster.physical_channels() {
    ///     // ...
    /// }
    /// ```
    #[must_use]
    pub fn physical_channels(&self) -> EthernetClusterChannelsIterator {
        EthernetClusterChannelsIterator::new(
            self.element()
                .get_sub_element(ElementName::EthernetClusterVariants)
                .and_then(|ecv| ecv.get_sub_element(ElementName::EthernetClusterConditional))
                .and_then(|ecc| ecc.get_sub_element(ElementName::PhysicalChannels)),
        )
    }
}

impl AbstractCluster for EthernetCluster {}

//##################################################################

element_iterator!(EthernetClusterChannelsIterator, EthernetPhysicalChannel, Some);

//##################################################################

#[cfg(test)]
mod test {
    use crate::{
        communication::{AbstractCluster, EthernetVlanInfo},
        ArPackage, System, SystemCategory,
    };
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn cluster() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();

        let pkg2 = ArPackage::get_or_create(&model, "/ethernet").unwrap();
        // create the ethernet cluster EthCluster
        let result = system.create_ethernet_cluster("EthCluster", &pkg2);
        assert!(result.is_ok());
        let cluster = result.unwrap();
        // creating the same cluster again is not possible
        let result = system.create_ethernet_cluster("EthCluster", &pkg2);
        assert!(result.is_err());

        // system link
        let linked_system = cluster.system().unwrap();
        assert_eq!(linked_system, system);

        // create an untagged channel
        let result = cluster.create_physical_channel("Channel1", None);
        assert!(result.is_ok());
        // can't create a second untagged channel
        let result = cluster.create_physical_channel("Channel2", None);
        assert!(result.is_err());

        // create a channel for VLAN 1
        let vlan_info = EthernetVlanInfo {
            vlan_name: "VLAN_1".to_string(),
            vlan_id: 1,
        };
        let result = cluster.create_physical_channel("Channel3", Some(vlan_info));
        assert!(result.is_ok());

        // can't create a second channel called Channel3
        let vlan_info = EthernetVlanInfo {
            vlan_name: "VLAN_2".to_string(),
            vlan_id: 2,
        };
        let result = cluster.create_physical_channel("Channel3", Some(vlan_info));
        assert!(result.is_err());

        // create a channel for VLAN 2
        let vlan_info = EthernetVlanInfo {
            vlan_name: "VLAN_2".to_string(),
            vlan_id: 2,
        };
        let result = cluster.create_physical_channel("Channel4", Some(vlan_info));
        assert!(result.is_ok());

        // can't create a second channel for VLAN 2
        let vlan_info = EthernetVlanInfo {
            vlan_name: "VLAN_2".to_string(),
            vlan_id: 2,
        };
        let result = cluster.create_physical_channel("Channel5", Some(vlan_info));
        assert!(result.is_err());

        let count = cluster.physical_channels().count();
        assert_eq!(count, 3);
    }
}
