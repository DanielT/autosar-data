use crate::communication::{
    CanPhysicalChannel, CommunicationDirection, Frame, FramePort, FrameTriggering, Pdu, PduToFrameMapping,
    PduToFrameMappingIterator, PduTriggering,
};
use crate::{
    abstraction_element, make_unique_name, reflist_iterator, AbstractionElement, ArPackage, AutosarAbstractionError,
    ByteOrder, EcuInstance,
};
use autosar_data::{AutosarDataError, Element, ElementName, EnumItem};

/// A frame on a CAN bus
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanFrame(Element);
abstraction_element!(CanFrame, CanFrame);

impl CanFrame {
    pub(crate) fn new(name: &str, byte_length: u64, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let can_frame = pkg_elements.create_named_sub_element(ElementName::CanFrame, name)?;

        can_frame
            .create_sub_element(ElementName::FrameLength)?
            .set_character_data(byte_length.to_string())?;

        Ok(Self(can_frame))
    }

    /// returns an iterator over all PDUs in the frame
    pub fn mapped_pdus(&self) -> impl Iterator<Item = PduToFrameMapping> {
        PduToFrameMappingIterator::new(self.element().get_sub_element(ElementName::PduToFrameMappings))
    }

    /// Iterator over all [`CanFrameTriggering`]s using this frame
    pub fn frame_triggerings(&self) -> impl Iterator<Item = CanFrameTriggering> {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            CanFrameTriggeringsIterator::new(reflist)
        } else {
            CanFrameTriggeringsIterator::new(vec![])
        }
    }

    pub fn map_pdu<T: AbstractionElement + Clone + Into<Pdu>>(
        &self,
        pdu: &T,
        start_position: u32,
        byte_order: ByteOrder,
        update_bit: Option<u32>,
    ) -> Result<PduToFrameMapping, AutosarAbstractionError> {
        if self.mapped_pdus().count() != 0 {
            return Err(AutosarAbstractionError::InvalidParameter(
                "CAN only allows one PDU per frame".to_string(),
            ));
        }
        Frame::from(self.clone()).map_pdu(pdu, start_position, byte_order, update_bit)
    }
}

impl From<CanFrame> for Frame {
    fn from(frame: CanFrame) -> Self {
        Frame::Can(frame)
    }
}

//##################################################################

/// The frame triggering connects a frame to a physical channel
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanFrameTriggering(Element);
abstraction_element!(CanFrameTriggering, CanFrameTriggering);

impl CanFrameTriggering {
    pub(crate) fn new(
        channel: &CanPhysicalChannel,
        frame: &CanFrame,
        identifier: u32,
        addressing_mode: CanAddressingMode,
        frame_type: CanFrameType,
    ) -> Result<Self, AutosarAbstractionError> {
        let model = channel.element().model()?;
        let base_path = channel.element().path()?;
        let frame_name = frame
            .name()
            .ok_or(AutosarAbstractionError::InvalidParameter("invalid frame".to_string()))?;
        let ft_name = format!("FT_{frame_name}");
        let ft_name = make_unique_name(&model, base_path, ft_name);

        let frame_triggerings = channel
            .element()
            .get_or_create_sub_element(ElementName::FrameTriggerings)?;
        let can_triggering = frame_triggerings.create_named_sub_element(ElementName::CanFrameTriggering, &ft_name)?;

        can_triggering
            .create_sub_element(ElementName::FrameRef)?
            .set_reference_target(frame.element())?;

        let ft = Self(can_triggering);
        ft.set_addressing_mode(addressing_mode)?;
        ft.set_frame_type(frame_type)?;
        if let Err(error) = ft.set_identifier(identifier) {
            let _ = frame_triggerings.remove_sub_element(ft.0);
            return Err(error);
        }

        for pdu_mapping in frame.mapped_pdus() {
            if let Some(pdu) = pdu_mapping.pdu() {
                ft.add_pdu_triggering(&pdu)?;
            }
        }

        Ok(ft)
    }

    /// set the can id associated with this frame
    pub fn set_identifier(&self, identifier: u32) -> Result<(), AutosarAbstractionError> {
        let amode = self.get_addressing_mode().unwrap_or(CanAddressingMode::Standard);
        if amode == CanAddressingMode::Standard && identifier > 0x7ff {
            return Err(AutosarAbstractionError::InvalidParameter(format!(
                "CAN-ID {identifier} is outside the 11-bit range allowed by standard addressing"
            )));
        } else if identifier > 0x1fff_ffff {
            return Err(AutosarAbstractionError::InvalidParameter(format!(
                "CAN-ID {identifier} is outside the 29-bit range allowed by extended addressing"
            )));
        }
        self.element()
            .get_or_create_sub_element(ElementName::Identifier)?
            .set_character_data(identifier.to_string())?;

        Ok(())
    }

    pub fn get_identifier(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Identifier)?
            .character_data()?
            .parse_integer()
    }

    pub fn set_addressing_mode(&self, addressing_mode: CanAddressingMode) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::CanAddressingMode)?
            .set_character_data::<EnumItem>(addressing_mode.into())?;

        Ok(())
    }

    pub fn get_addressing_mode(&self) -> Option<CanAddressingMode> {
        self.element()
            .get_sub_element(ElementName::CanAddressingMode)?
            .character_data()?
            .enum_value()?
            .try_into()
            .ok()
    }

    pub fn set_frame_type(&self, frame_type: CanFrameType) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::CanFrameRxBehavior)?
            .set_character_data::<EnumItem>(frame_type.into())?;
        self.element()
            .get_or_create_sub_element(ElementName::CanFrameTxBehavior)?
            .set_character_data::<EnumItem>(frame_type.into())?;

        Ok(())
    }

    pub fn get_frame_type(&self) -> Option<CanFrameType> {
        self.element()
            .get_sub_element(ElementName::CanFrameTxBehavior)?
            .character_data()?
            .enum_value()?
            .try_into()
            .ok()
    }

    /// get the frame triggered by this FrameTriggering
    pub fn frame(&self) -> Option<CanFrame> {
        CanFrame::try_from(
            self.element()
                .get_sub_element(ElementName::FrameRef)?
                .get_reference_target()
                .ok()?,
        )
        .ok()
    }

    pub(crate) fn add_pdu_triggering(&self, pdu: &Pdu) -> Result<PduTriggering, AutosarAbstractionError> {
        FrameTriggering::Can(self.clone()).add_pdu_triggering(pdu)
    }

    /// get the physical channel that contains this frame triggering
    pub fn physical_channel(&self) -> Result<CanPhysicalChannel, AutosarAbstractionError> {
        let channel_elem = self.element().named_parent()?.ok_or(AutosarDataError::ItemDeleted)?;
        CanPhysicalChannel::try_from(channel_elem)
    }

    pub fn connect_to_ecu(
        &self,
        ecu: &EcuInstance,
        direction: CommunicationDirection,
    ) -> Result<FramePort, AutosarAbstractionError> {
        FrameTriggering::Can(self.clone()).connect_to_ecu(ecu, direction)
    }

    pub fn pdu_triggerings(&self) -> impl Iterator<Item = PduTriggering> {
        FrameTriggering::Can(self.clone()).pdu_triggerings()
    }
}

impl From<CanFrameTriggering> for FrameTriggering {
    fn from(cft: CanFrameTriggering) -> Self {
        FrameTriggering::Can(cft)
    }
}

//##################################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanAddressingMode {
    Standard,
    Extended,
}

impl TryFrom<EnumItem> for CanAddressingMode {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<Self, Self::Error> {
        match value {
            EnumItem::Standard => Ok(CanAddressingMode::Standard),
            EnumItem::Extended => Ok(CanAddressingMode::Extended),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "CanAddressingMode".to_string(),
            }),
        }
    }
}

impl From<CanAddressingMode> for EnumItem {
    fn from(value: CanAddressingMode) -> Self {
        match value {
            CanAddressingMode::Standard => EnumItem::Standard,
            CanAddressingMode::Extended => EnumItem::Extended,
        }
    }
}

//##################################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanFrameType {
    Can20,
    CanFd,
    Any,
}

impl TryFrom<EnumItem> for CanFrameType {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<Self, Self::Error> {
        match value {
            EnumItem::Can20 => Ok(CanFrameType::Can20),
            EnumItem::CanFd => Ok(CanFrameType::CanFd),
            EnumItem::Any => Ok(CanFrameType::Any),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "CanFrameType".to_string(),
            }),
        }
    }
}

impl From<CanFrameType> for EnumItem {
    fn from(value: CanFrameType) -> Self {
        match value {
            CanFrameType::Can20 => EnumItem::Can20,
            CanFrameType::CanFd => EnumItem::CanFd,
            CanFrameType::Any => EnumItem::Any,
        }
    }
}

//##################################################################

reflist_iterator!(CanFrameTriggeringsIterator, CanFrameTriggering);
