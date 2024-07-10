use crate::{
    abstraction_element,
    communication::{FlexrayCluster, FlexrayCommunicationCycle, FlexrayFrame, FlexrayFrameTriggering},
    AbstractionElement, AutosarAbstractionError,
};
use autosar_data::{Element, ElementName, EnumItem};

/// the `FlexrayPhysicalChannel` represents either channel A or B of Flexray cluster
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlexrayPhysicalChannel(Element);
abstraction_element!(FlexrayPhysicalChannel, FlexrayPhysicalChannel);

impl FlexrayPhysicalChannel {
    /// get the channel name of a `FlexrayPhysicalChannel`
    #[must_use]
    pub fn channel_name(&self) -> Option<FlexrayChannelName> {
        let cn = self
            .0
            .get_sub_element(ElementName::ChannelName)?
            .character_data()?
            .enum_value()?;
        match cn {
            EnumItem::ChannelA => Some(FlexrayChannelName::A),
            EnumItem::ChannelB => Some(FlexrayChannelName::B),
            _ => None,
        }
    }

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
    /// # let cluster = system.create_flexray_cluster("Cluster", &package, &FlexrayClusterSettings::default()).unwrap();
    /// let channel = cluster.create_physical_channel("Channel", FlexrayChannelName::A).unwrap();
    /// let cluster_2 = channel.cluster().unwrap();
    /// assert_eq!(cluster, cluster_2);
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model
    pub fn cluster(&self) -> Result<FlexrayCluster, AutosarAbstractionError> {
        let cluster_elem = self.0.named_parent()?.unwrap();
        FlexrayCluster::try_from(cluster_elem)
    }

    /// add a trigger for a flexray frame in this physical channel
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
    /// # let cluster = system.create_flexray_cluster("Cluster", &package, &FlexrayClusterSettings::default()).unwrap();
    /// let channel = cluster.create_physical_channel("Channel", FlexrayChannelName::A).unwrap();
    /// let frame = system.create_flexray_frame("Frame", 64, &frame_package).unwrap();
    /// let timing = FlexrayCommunicationCycle::Repetition {base_cycle: 1, cycle_repetition: CycleRepetition::C1};
    /// channel.trigger_frame(&frame, 1, &timing).unwrap();
    /// ```
    pub fn trigger_frame(
        &self,
        frame: &FlexrayFrame,
        slot_id: u16,
        timing: &FlexrayCommunicationCycle,
    ) -> Result<FlexrayFrameTriggering, AutosarAbstractionError> {
        FlexrayFrameTriggering::new(self, frame, slot_id, timing)
    }
}

//##################################################################

/// A flexray cluster may contain the channels A and/or B.
///
/// This enum is an abstraction over the \<CHANNEL-NAME\> element.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexrayChannelName {
    A,
    B,
}

//##################################################################

#[cfg(test)]
mod test {
    use crate::{
        communication::{FlexrayChannelName, FlexrayClusterSettings},
        AbstractionElement, ArPackage, System, SystemCategory,
    };
    use autosar_data::{AutosarModel, AutosarVersion, ElementName};

    #[test]
    fn channel() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();
        let settings = FlexrayClusterSettings::default();
        let cluster = system.create_flexray_cluster("FlxCluster", &pkg, &settings).unwrap();

        let channel = cluster
            .create_physical_channel("channel_name", FlexrayChannelName::A)
            .unwrap();
        let c2 = channel.cluster().unwrap();
        assert_eq!(cluster, c2);

        // damage the channel info by removing the channel name
        let elem_channelname = channel.element().get_sub_element(ElementName::ChannelName).unwrap();
        elem_channelname.remove_character_data().unwrap();
        assert!(channel.channel_name().is_none());

        // now there is no longer a channel A
        let channel2 = cluster.create_physical_channel("channel_name2", FlexrayChannelName::A);
        assert!(channel2.is_ok())
    }
}
