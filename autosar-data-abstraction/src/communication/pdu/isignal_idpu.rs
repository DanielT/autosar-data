use crate::communication::{ISignal, ISignalGroup, Pdu, PduTriggering, PduTriggeringsIterator, TransferProperty};
use crate::{
    abstraction_element, element_iterator, make_unique_name, AbstractionElement, ArPackage, AutosarAbstractionError,
    ByteOrder,
};
use autosar_data::{Element, ElementName, EnumItem};

//##################################################################

/// Represents the IPdus handled by Com
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISignalIPdu(Element);
abstraction_element!(ISignalIPdu, ISignalIPdu);

impl ISignalIPdu {
    pub(crate) fn new(name: &str, package: &ArPackage, length: u32) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_pdu = pkg_elements.create_named_sub_element(ElementName::ISignalIPdu, name)?;
        elem_pdu
            .create_sub_element(ElementName::Length)?
            .set_character_data(length.to_string())?;

        Ok(Self(elem_pdu))
    }

    pub fn length(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    /// returns an iterator over all signals mapped to the PDU
    pub fn mapped_signals(&self) -> ISIgnalToIPduMappingsIterator {
        ISIgnalToIPduMappingsIterator::new(self.element().get_sub_element(ElementName::ISignalToPduMappings))
    }

    /// map a signal to the ISignalIPdu
    ///
    /// If this signal is part of a signal group, then the group must be mapped first
    pub fn map_signal(
        &self,
        signal: &ISignal,
        start_position: u32,
        byte_order: ByteOrder,
        update_bit: Option<u32>,
        transfer_property: TransferProperty,
    ) -> Result<ISignalToIPduMapping, AutosarAbstractionError> {
        let signal_name = signal
            .name()
            .ok_or(AutosarAbstractionError::InvalidParameter("invalid signal".to_string()))?;

        let length = self.length().unwrap_or(0);
        let mut validator = SignalMappingValidator::new(length);
        // build a bitmap of all signals that are already mapped in this pdu
        for mapping in self.mapped_signals() {
            if let (Some(m_signal), Some(m_start_pos), Some(m_byte_order)) =
                (mapping.signal(), mapping.start_position(), mapping.byte_order())
            {
                let len = m_signal.length().unwrap_or(0);
                validator.add_signal(m_start_pos, len, m_byte_order, mapping.update_bit());
            }
        }
        // add the new signal to the validator bitmap to see if it overlaps any exisiting signals
        if !validator.add_signal(start_position, signal.length().unwrap_or(0), byte_order, update_bit) {
            return Err(AutosarAbstractionError::InvalidParameter(format!(
                "Cannot map signal {signal_name} to an overalapping position in the pdu"
            )));
        }

        if let Some(signal_group) = signal.signal_group() {
            if self
                .mapped_signals()
                .filter_map(|mapping| mapping.signal_group())
                .any(|grp| grp == signal_group)
            {
                return Err(AutosarAbstractionError::InvalidParameter(
                    "Cannot map signal to pdu, because it is part of an unmapped signal group.".to_string(),
                ));
            }
        }

        // add a pdu triggering for the newly mapped PDU to each frame triggering of this frame
        for pt in self.pdu_triggerings() {
            let st = pt.add_signal_triggering(signal)?;
            for pdu_port in pt.pdu_ports() {
                if let (Some(ecu), Some(direction)) = (pdu_port.ecu(), pdu_port.communication_direction()) {
                    st.connect_to_ecu(&ecu, direction)?;
                }
            }
        }

        // create and return the new mapping
        let model = self.element().model()?;
        let base_path = self.element().path()?;
        let name = make_unique_name(&model, base_path, signal_name);

        let mappings = self
            .element()
            .get_or_create_sub_element(ElementName::ISignalToPduMappings)?;

        ISignalToIPduMapping::new(
            &name,
            &mappings,
            signal,
            start_position,
            byte_order,
            update_bit,
            transfer_property,
        )
    }

    /// map a signal group to the PDU
    pub fn map_signal_group(
        &self,
        signal_group: &ISignalGroup,
    ) -> Result<ISignalToIPduMapping, AutosarAbstractionError> {
        let signal_group_name = signal_group.name().ok_or(AutosarAbstractionError::InvalidParameter(
            "invalid signal group".to_string(),
        ))?;

        // add a pdu triggering for the newly mapped PDU to each frame triggering of this frame
        for pt in self.pdu_triggerings() {
            let st = pt.add_signal_group_triggering(signal_group)?;
            for pdu_port in pt.pdu_ports() {
                if let (Some(ecu), Some(direction)) = (pdu_port.ecu(), pdu_port.communication_direction()) {
                    st.connect_to_ecu(&ecu, direction)?;
                }
            }
        }

        // create and return the new mapping
        let model = self.element().model()?;
        let base_path = self.element().path()?;
        let name = make_unique_name(&model, base_path, signal_group_name);

        let mappings = self
            .element()
            .get_or_create_sub_element(ElementName::ISignalToPduMappings)?;

        ISignalToIPduMapping::new_group(&name, &mappings, signal_group)
    }

    /// returns an iterator over all PduTriggerings that use this PDU
    pub fn pdu_triggerings(&self) -> impl Iterator<Item = PduTriggering> {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            PduTriggeringsIterator::new(reflist)
        } else {
            PduTriggeringsIterator::new(vec![])
        }
    }

    /// set the transmission timing of the PDU
    pub fn set_timing(&self, timing_spec: &IpduTiming) -> Result<(), AutosarAbstractionError> {
        if let Some(timing_elem) = self.element().get_sub_element(ElementName::IPduTimingSpecifications) {
            self.element().remove_sub_element(timing_elem)?;
        }

        let timing_elem = self
            .element()
            .create_sub_element(ElementName::IPduTimingSpecifications)?
            .create_sub_element(ElementName::IPduTiming)?;
        if let Some(min_delay) = timing_spec.minimum_delay {
            timing_elem
                .create_sub_element(ElementName::MinimumDelay)?
                .set_character_data(min_delay)?;
        }
        if let Some(transmission_mode_true_timing) = &timing_spec.transmission_mode_true_timing {
            let tmtt_elem = timing_elem
                .get_or_create_sub_element(ElementName::TransmissionModeDeclaration)?
                .create_sub_element(ElementName::TransmissionModeTrueTiming)?;
            Self::set_transmission_mode_timinig(tmtt_elem, transmission_mode_true_timing)?;
        }
        if let Some(transmission_mode_false_timing) = &timing_spec.transmission_mode_false_timing {
            let tmtf_elem = timing_elem
                .get_or_create_sub_element(ElementName::TransmissionModeDeclaration)?
                .create_sub_element(ElementName::TransmissionModeFalseTiming)?;
            Self::set_transmission_mode_timinig(tmtf_elem, transmission_mode_false_timing)?;
        }

        Ok(())
    }

    /// Helper function to set the transmission mode timing, used by ISignalIPdu::set_timing for both true and false timing
    fn set_transmission_mode_timinig(
        timing_element: Element,
        transmission_mode_timing: &TransmissionModeTiming,
    ) -> Result<(), AutosarAbstractionError> {
        if let Some(cyclic_timing) = &transmission_mode_timing.cyclic_timing {
            let ct_elem = timing_element.create_sub_element(ElementName::CyclicTiming)?;
            ct_elem
                .create_sub_element(ElementName::TimePeriod)?
                .create_sub_element(ElementName::Value)?
                .set_character_data(cyclic_timing.time_period)?;
            if let Some(time_offset) = cyclic_timing.time_offset {
                ct_elem
                    .create_sub_element(ElementName::TimeOffset)?
                    .create_sub_element(ElementName::Value)?
                    .set_character_data(time_offset)?;
            }
        }
        if let Some(event_controlled_timing) = &transmission_mode_timing.event_controlled_timing {
            let ect_elem = timing_element.create_sub_element(ElementName::EventControlledTiming)?;
            ect_elem
                .create_sub_element(ElementName::NumberOfRepetitions)?
                .set_character_data(event_controlled_timing.number_of_repetitions as u64)?;
            if let Some(repetition_period) = event_controlled_timing.repetition_period {
                ect_elem
                    .create_sub_element(ElementName::RepetitionPeriod)?
                    .create_sub_element(ElementName::Value)?
                    .set_character_data(repetition_period)?;
            }
        }

        Ok(())
    }

    /// get the transmission timing of the PDU
    pub fn timing(&self) -> Option<IpduTiming> {
        let timing_elem = self
            .element()
            .get_sub_element(ElementName::IPduTimingSpecifications)?
            .get_sub_element(ElementName::IPduTiming)?;
        let minimum_delay = timing_elem
            .get_sub_element(ElementName::MinimumDelay)
            .and_then(|md| md.character_data())
            .and_then(|cdata| cdata.parse_float());
        let transmission_mode_true_timing = timing_elem
            .get_sub_element(ElementName::TransmissionModeDeclaration)
            .and_then(|tmd| tmd.get_sub_element(ElementName::TransmissionModeTrueTiming))
            .and_then(|tmtt| Self::transmission_mode_timing(&tmtt));
        let transmission_mode_false_timing = timing_elem
            .get_sub_element(ElementName::TransmissionModeDeclaration)
            .and_then(|tmd| tmd.get_sub_element(ElementName::TransmissionModeFalseTiming))
            .and_then(|tmtf| Self::transmission_mode_timing(&tmtf));

        Some(IpduTiming {
            minimum_delay,
            transmission_mode_true_timing,
            transmission_mode_false_timing,
        })
    }

    /// Helper function to get the transmission mode timing, used by ISignalIPdu::timing for both true and false modes
    fn transmission_mode_timing(timing_elem: &Element) -> Option<TransmissionModeTiming> {
        let cyclic_timing = timing_elem.get_sub_element(ElementName::CyclicTiming).and_then(|ct| {
            let time_period = ct
                .get_sub_element(ElementName::TimePeriod)
                .and_then(|tp| tp.get_sub_element(ElementName::Value))
                .and_then(|val| val.character_data())
                .and_then(|cdata| cdata.parse_float());
            let time_offset = ct
                .get_sub_element(ElementName::TimeOffset)
                .and_then(|to| to.get_sub_element(ElementName::Value))
                .and_then(|val| val.character_data())
                .and_then(|cdata| cdata.parse_float());
            time_period.map(|tp| CyclicTiming {
                time_period: tp,
                time_offset,
            })
        });
        let event_controlled_timing = timing_elem
            .get_sub_element(ElementName::EventControlledTiming)
            .and_then(|ect| {
                let number_of_repetitions = ect
                    .get_sub_element(ElementName::NumberOfRepetitions)
                    .and_then(|nr| nr.character_data())
                    .and_then(|cdata| cdata.parse_integer());
                let repetition_period = ect
                    .get_sub_element(ElementName::RepetitionPeriod)
                    .and_then(|rp| rp.get_sub_element(ElementName::Value))
                    .and_then(|val| val.character_data())
                    .and_then(|cdata| cdata.parse_float());
                number_of_repetitions.map(|nr| EventControlledTiming {
                    number_of_repetitions: nr,
                    repetition_period,
                })
            });

        Some(TransmissionModeTiming {
            cyclic_timing,
            event_controlled_timing,
        })
    }
}

impl From<ISignalIPdu> for Pdu {
    fn from(value: ISignalIPdu) -> Self {
        Pdu::ISignalIPdu(value)
    }
}

//##################################################################

/// ISignalToIPduMapping connects an isignal to an isignalipdu
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISignalToIPduMapping(Element);
abstraction_element!(ISignalToIPduMapping, ISignalToIPduMapping);

impl ISignalToIPduMapping {
    fn new(
        name: &str,
        mappings: &Element,
        signal: &ISignal,
        start_position: u32,
        byte_order: ByteOrder,
        update_bit: Option<u32>,
        transfer_property: TransferProperty,
    ) -> Result<Self, AutosarAbstractionError> {
        let signal_mapping = mappings.create_named_sub_element(ElementName::ISignalToIPduMapping, name)?;
        signal_mapping
            .create_sub_element(ElementName::ISignalRef)?
            .set_reference_target(signal.element())?;
        signal_mapping
            .create_sub_element(ElementName::PackingByteOrder)?
            .set_character_data::<EnumItem>(byte_order.into())?;
        signal_mapping
            .create_sub_element(ElementName::StartPosition)?
            .set_character_data(start_position as u64)?;
        signal_mapping
            .create_sub_element(ElementName::TransferProperty)?
            .set_character_data::<EnumItem>(transfer_property.into())?;
        if let Some(update_bit_pos) = update_bit {
            signal_mapping
                .create_sub_element(ElementName::UpdateIndicationBitPosition)?
                .set_character_data(update_bit_pos as u64)?;
        }

        Ok(Self(signal_mapping))
    }

    fn new_group(name: &str, mappings: &Element, signal_group: &ISignalGroup) -> Result<Self, AutosarAbstractionError> {
        let signal_mapping = mappings.create_named_sub_element(ElementName::ISignalToIPduMapping, name)?;
        signal_mapping
            .create_sub_element(ElementName::ISignalGroupRef)?
            .set_reference_target(signal_group.element())?;

        Ok(Self(signal_mapping))
    }

    /// Reference to the signal that is mapped to the PDU.
    /// Every mapping contains either a signal or a signal group.
    pub fn signal(&self) -> Option<ISignal> {
        self.element()
            .get_sub_element(ElementName::ISignalRef)
            .and_then(|sigref| sigref.get_reference_target().ok())
            .and_then(|signal_elem| ISignal::try_from(signal_elem).ok())
    }

    /// Byte order of the data in the signal.
    pub fn byte_order(&self) -> Option<ByteOrder> {
        self.element()
            .get_sub_element(ElementName::PackingByteOrder)
            .and_then(|pbo| pbo.character_data())
            .and_then(|cdata| cdata.enum_value())
            .and_then(|enumval| enumval.try_into().ok())
    }

    /// Start position of the signal data within the PDU (bit position).
    /// The start position is mandatory if the mapping describes a signal.
    pub fn start_position(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::StartPosition)
            .and_then(|pbo| pbo.character_data())
            .and_then(|cdata| cdata.parse_integer())
    }

    /// Bit position of the update bit for the mapped signal. Not all signals use an update bit.
    /// This is never used for signal groups
    pub fn update_bit(&self) -> Option<u32> {
        self.element()
            .get_sub_element(ElementName::StartPosition)
            .and_then(|pbo| pbo.character_data())
            .and_then(|cdata| cdata.parse_integer())
    }

    /// Bit position of the update bit for the mapped signal. Not all signals use an update bit.
    /// This is never used for signal groups
    pub fn transfer_property(&self) -> Option<TransferProperty> {
        self.element()
            .get_sub_element(ElementName::TransferProperty)
            .and_then(|pbo| pbo.character_data())
            .and_then(|cdata| cdata.enum_value())
            .and_then(|enumval| enumval.try_into().ok())
    }

    /// Reference to the signal group that is mapped to the PDU.
    /// Every mapping contains either a signal or a signal group.
    pub fn signal_group(&self) -> Option<ISignalGroup> {
        self.element()
            .get_sub_element(ElementName::ISignalGroupRef)
            .and_then(|sgref| sgref.get_reference_target().ok())
            .and_then(|siggrp_elem| ISignalGroup::try_from(siggrp_elem).ok())
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq)]
pub struct IpduTiming {
    /// minimum delay in seconds between two transmissions of the PDU
    pub minimum_delay: Option<f64>,
    /// timing specification if the COM transmission mode is true
    pub transmission_mode_true_timing: Option<TransmissionModeTiming>,
    /// timing specification if the COM transmission mode is false
    pub transmission_mode_false_timing: Option<TransmissionModeTiming>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransmissionModeTiming {
    pub cyclic_timing: Option<CyclicTiming>,
    pub event_controlled_timing: Option<EventControlledTiming>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CyclicTiming {
    /// period of repetition in seconds
    pub time_period: f64,
    /// delay until the first transmission of the PDU in seconds
    pub time_offset: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventControlledTiming {
    /// The PDU will be sent (number of repetitions + 1) times. If number of repetitions is 0, then the PDU is sent exactly once.
    pub number_of_repetitions: u32,
    /// time in seconds between two transmissions of the PDU
    pub repetition_period: Option<f64>,
}

//##################################################################

/// Helper struct to validate signal mappings
pub struct SignalMappingValidator {
    bitmap: Vec<u8>,
}

impl SignalMappingValidator {
    pub fn new(length: u32) -> Self {
        Self {
            bitmap: vec![0; length as usize],
        }
    }

    pub fn add_signal(
        &mut self,
        bit_position: u32,
        bit_length: u64,
        byte_order: ByteOrder,
        update_bit: Option<u32>,
    ) -> bool {
        let bit_position = bit_position as u64;
        let first_byte = (bit_position / 8) as usize;
        let bit_offset = bit_position % 8; // bit position inside the first byte
        let first_byte_bits; // number of bits in the first byte
        let mut first_mask;

        if byte_order == ByteOrder::MostSignificantByteFirst {
            // MostSignificantByteFirst / big endian
            // bit-position: 5, length: 10
            // byte   |               0               |               1               |
            // bit    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
            // signal | 4   5   6   7   8   9                           0   1   2   3
            first_byte_bits = (bit_offset + 1).min(bit_length);
            first_mask = ((1u16 << (bit_offset + 1)) - 1) as u8;
            if bit_offset + 1 != first_byte_bits {
                let pos2 = bit_offset - first_byte_bits;
                let subtract_mask = (1u8 << pos2) - 1;
                first_mask -= subtract_mask;
            }
        } else {
            // MostSignificantByteLast / little endian
            // bit-position: 5, length: 10
            // byte   |               0               |               1               |
            // bit    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
            // signal |                     0   1   2   3   4   5   6   7   8   9
            first_byte_bits = (8 - bit_offset).min(bit_length); // value range 1 - 8
            first_mask = !((1u16 << bit_offset) - 1) as u8; // 0b1111_1110, 0b1111_1100, 0b1111_1000, ..., 0b1000_0000
            if bit_offset + first_byte_bits < 8 {
                let pos2 = bit_offset + first_byte_bits;
                let subtract_mask = !((1u8 << pos2) - 1);
                first_mask -= subtract_mask;
            }
        }
        let full_bytes = (bit_length - first_byte_bits) as usize / 8;
        let end_bits = (bit_length - first_byte_bits) % 8;

        let mut result = self.apply_mask(first_mask, first_byte);
        result &= self.apply_full_bytes(first_byte + 1, full_bytes);

        // handle any bits in a partial trailing byte
        if end_bits > 0 {
            let end_mask = if byte_order == ByteOrder::MostSignificantByteFirst {
                !((1u8 << end_bits) - 1)
            } else {
                (1u8 << end_bits) - 1
            };
            result &= self.apply_mask(end_mask, first_byte + full_bytes + 1);
        }

        // handle the update bit, if any
        if let Some(update_bit) = update_bit {
            let position = (update_bit / 8) as usize;
            let bit_pos = update_bit % 8;
            let mask = 1 << bit_pos;
            result &= self.apply_mask(mask, position)
        }

        result
    }

    fn apply_mask(&mut self, mask: u8, position: usize) -> bool {
        if position < self.bitmap.len() {
            // check if any of the masked bits were already set
            let result = self.bitmap[position] & mask == 0;
            // set the masked bits
            self.bitmap[position] |= mask;
            result
        } else {
            false
        }
    }

    fn apply_full_bytes(&mut self, position: usize, count: usize) -> bool {
        let mut result = true;
        if count > 0 {
            let limit = self.bitmap.len().min(position + count);
            for idx in position..limit {
                result &= self.apply_mask(0xff, idx);
            }
            // make sure all of the signal bytes are inside the pdu length
            result &= limit == position + count;
        }
        result
    }
}

//##################################################################

element_iterator!(ISIgnalToIPduMappingsIterator, ISignalToIPduMapping, Some);

//##################################################################

#[cfg(test)]
mod test {
    use super::*;
    use crate::ByteOrder;
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn validate_signal_mapping() {
        // create a validator and add a 2-bit signal
        let mut validator = SignalMappingValidator::new(4);
        let result = validator.add_signal(0, 2, ByteOrder::MostSignificantByteLast, None);
        assert!(result);
        assert_eq!(validator.bitmap[0], 0x03);

        // create a validator and add a little-endian 9-bit signal
        let mut validator = SignalMappingValidator::new(4);
        let result = validator.add_signal(5, 10, ByteOrder::MostSignificantByteLast, None);
        assert!(result);
        assert_eq!(validator.bitmap[0], 0xE0);
        assert_eq!(validator.bitmap[1], 0x7F);

        // create a validator and add a big-endian 9-bit signal
        let mut validator = SignalMappingValidator::new(4);
        let result = validator.add_signal(5, 10, ByteOrder::MostSignificantByteFirst, None);
        assert!(result);
        assert_eq!(validator.bitmap[0], 0x3F);
        assert_eq!(validator.bitmap[1], 0xF0);

        // add another signal to the existing validator
        let result = validator.add_signal(5, 10, ByteOrder::MostSignificantByteLast, None);
        // it returns false (the signals overlap), but the bitmap is still updated
        assert!(!result);
        assert_eq!(validator.bitmap[0], 0xFF);
        assert_eq!(validator.bitmap[1], 0xFF);

        // create a validator and add a 32-bit signal
        let mut validator = SignalMappingValidator::new(4);
        let result = validator.add_signal(0, 32, ByteOrder::MostSignificantByteLast, None);
        assert!(result);
        assert_eq!(validator.bitmap[0], 0xFF);
        assert_eq!(validator.bitmap[1], 0xFF);
        assert_eq!(validator.bitmap[2], 0xFF);
        assert_eq!(validator.bitmap[3], 0xFF);

        // create a validator and add a big-endian 32-bit signal
        let mut validator = SignalMappingValidator::new(4);
        let result = validator.add_signal(7, 32, ByteOrder::MostSignificantByteFirst, None);
        assert!(result);
        assert_eq!(validator.bitmap[0], 0xFF);
        assert_eq!(validator.bitmap[1], 0xFF);
        assert_eq!(validator.bitmap[2], 0xFF);
        assert_eq!(validator.bitmap[3], 0xFF);

        // multiple mixed signals
        let mut validator = SignalMappingValidator::new(8);
        let result = validator.add_signal(7, 16, ByteOrder::MostSignificantByteFirst, Some(60));
        assert!(result);
        let result = validator.add_signal(16, 3, ByteOrder::MostSignificantByteLast, Some(61));
        assert!(result);
        let result = validator.add_signal(19, 7, ByteOrder::MostSignificantByteLast, Some(62));
        assert!(result);
        let result = validator.add_signal(26, 30, ByteOrder::MostSignificantByteLast, Some(63));
        assert!(result);
        let result = validator.add_signal(59, 4, ByteOrder::MostSignificantByteFirst, None);
        assert!(result);
        assert_eq!(validator.bitmap, [0xFF; 8]);
    }

    #[test]
    fn ipdu_timing() {
        let model = AutosarModel::new();
        let _file = model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let package = ArPackage::get_or_create(&model, "/pkg").unwrap();
        let pdu = ISignalIPdu::new("pdu_name", &package, 8).unwrap();

        let timing_spec = IpduTiming {
            minimum_delay: Some(0.1),
            transmission_mode_true_timing: Some(TransmissionModeTiming {
                cyclic_timing: Some(CyclicTiming {
                    time_period: 0.2,
                    time_offset: Some(0.3),
                }),
                event_controlled_timing: Some(EventControlledTiming {
                    number_of_repetitions: 4,
                    repetition_period: Some(0.5),
                }),
            }),
            transmission_mode_false_timing: Some(TransmissionModeTiming {
                cyclic_timing: Some(CyclicTiming {
                    time_period: 0.6,
                    time_offset: Some(0.7),
                }),
                event_controlled_timing: Some(EventControlledTiming {
                    number_of_repetitions: 8,
                    repetition_period: Some(0.9),
                }),
            }),
        };
        pdu.set_timing(&timing_spec).unwrap();
        let timing_spec2 = pdu.timing().unwrap();
        assert_eq!(timing_spec, timing_spec2);
    }
}
