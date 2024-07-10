use crate::{
    abstraction_element,
    communication::{CanAddressingMode, CanCluster, CanFrame, CanFrameTriggering, CanFrameType},
    AbstractionElement, AutosarAbstractionError,
};
use autosar_data::Element;

/// The `CanPhysicalChannel contains all of the communication on a CAN network
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
    /// # use autosar_data_abstraction::communication::*;
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

    /// add a trigger for a CAN frame in this physical channel
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
    /// # let frame_package = ArPackage::get_or_create(&model, "/Frames").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let cluster = system.create_can_cluster("Cluster", &package, &CanClusterSettings::default()).unwrap();
    /// let channel = cluster.create_physical_channel("Channel").unwrap();
    /// let frame = system.create_can_frame("Frame", 8, &frame_package).unwrap();
    /// channel.trigger_frame(&frame, 0x100, CanAddressingMode::Standard, CanFrameType::Can20).unwrap();
    /// ```
    pub fn trigger_frame(
        &self,
        frame: &CanFrame,
        identifier: u32,
        addressing_mode: CanAddressingMode,
        frame_type: CanFrameType,
    ) -> Result<CanFrameTriggering, AutosarAbstractionError> {
        CanFrameTriggering::new(self, frame, identifier, addressing_mode, frame_type)
    }
}

//##################################################################

#[cfg(test)]
mod test {
    use crate::{communication::CanClusterSettings, ArPackage, System, SystemCategory};
    use autosar_data::{AutosarModel, AutosarVersion};

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
}
