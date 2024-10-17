use crate::{abstraction_element, element_iterator, AbstractionElement, ArPackage, AutosarAbstractionError, ByteOrder};
use autosar_data::{AutosarVersion, Element, ElementName, EnumItem};

/// A [`DataTransformationSet`] contains `DataTransformation`s and `TransformationTechnology`s used in communication
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataTransformationSet(Element);
abstraction_element!(DataTransformationSet, DataTransformationSet);

impl DataTransformationSet {
    /// Create a new DataTransformationSet
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let transformation_set = elements.create_named_sub_element(ElementName::DataTransformationSet, name)?;

        Ok(Self(transformation_set))
    }

    /// Create a new `DataTransformation` in the `DataTransformationSet`
    pub fn create_data_transformation(
        &self,
        name: &str,
        transformations: &[&TransformationTechnology],
        execute_despite_data_unavailability: bool,
    ) -> Result<DataTransformation, AutosarAbstractionError> {
        let data_transformations = self
            .element()
            .get_or_create_sub_element(ElementName::DataTransformations)?;
        let transformation = DataTransformation::new(
            &data_transformations,
            name,
            transformations,
            execute_despite_data_unavailability,
        )?;
        Ok(transformation)
    }

    pub fn data_transformations(&self) -> impl Iterator<Item = DataTransformation> {
        DataTransformationIterator::new(self.element().get_sub_element(ElementName::DataTransformations))
    }

    /// Create a new `TransformationTechnology` in the `DataTransformationSet`
    pub fn create_transformation_technology(
        &self,
        name: &str,
        config: &TransformationTechnologyConfig,
    ) -> Result<TransformationTechnology, AutosarAbstractionError> {
        let transtechs = self
            .element()
            .get_or_create_sub_element(ElementName::TransformationTechnologys)?;
        TransformationTechnology::new(&transtechs, name, config)
    }

    pub fn transformation_technologies(&self) -> impl Iterator<Item = TransformationTechnology> {
        TransformationTechnologyIterator::new(self.element().get_sub_element(ElementName::TransformationTechnologys))
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataTransformation(Element);
abstraction_element!(DataTransformation, DataTransformation);

impl DataTransformation {
    /// Create a new DataTransformation
    fn new(
        parent: &Element,
        name: &str,
        transformations: &[&TransformationTechnology],
        execute_despite_data_unavailability: bool,
    ) -> Result<Self, AutosarAbstractionError> {
        // an empty chain is not allowed
        if transformations.is_empty() {
            return Err(AutosarAbstractionError::InvalidParameter(
                "A DataTransformation must contain at least one TransformationTechnology".to_string(),
            ));
        }

        // only the first transformation in a chain may have TransformerClass 'Serializer'
        for transformation in &transformations[1..] {
            if transformation.transformer_class() == Some(EnumItem::Serializer) {
                return Err(AutosarAbstractionError::InvalidParameter(
                    "A DataTransformation may only contain a TransformationTechnology with TransformerClass 'Serializer' at the start of the chain".to_string(),
                ));
            }
        }

        // every transformation in the chain must be part of the same DataTransformationSet as the DataTransformation
        let dts = parent
            .named_parent()?
            .and_then(|dts| DataTransformationSet::try_from(dts).ok());
        if !transformations
            .iter()
            .all(|ttech| ttech.data_transformation_set() == dts)
        {
            return Err(AutosarAbstractionError::InvalidParameter(
                "All TransformationTechnologies in a DataTransformation must be part of the same DataTransformationSet"
                    .to_string(),
            ));
        }

        // if any of the transformations is an E2E transformation, then executeDespiteDataUnavailability must be true
        if transformations
            .iter()
            .any(|ttech| ttech.protocol().as_deref() == Some("E2E"))
            && !execute_despite_data_unavailability
        {
            return Err(AutosarAbstractionError::InvalidParameter(
                "If a DataTransformation contains an E2E transformation, executeDespiteDataUnavailability must be true"
                    .to_string(),
            ));
        }

        let transformation = parent.create_named_sub_element(ElementName::DataTransformation, name)?;
        transformation
            .create_sub_element(ElementName::ExecuteDespiteDataUnavailability)?
            .set_character_data(execute_despite_data_unavailability.to_string())?;
        let chain_refs = transformation.create_sub_element(ElementName::TransformerChainRefs)?;
        for transformation in transformations {
            chain_refs
                .create_sub_element(ElementName::TransformerChainRef)?
                .set_reference_target(transformation.element())?;
        }

        Ok(Self(transformation))
    }

    /// get the DataTransformationSet that contains this DataTransformation
    pub fn data_transformation_set(&self) -> Option<DataTransformationSet> {
        self.element()
            .named_parent()
            .ok()?
            .and_then(|dts| DataTransformationSet::try_from(dts).ok())
    }

    /// Get the TransformationTechnologies in the DataTransformation
    pub fn transformation_technologies(&self) -> impl Iterator<Item = TransformationTechnology> {
        TransformerChainRefIterator::new(self.element().get_sub_element(ElementName::TransformerChainRefs))
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransformationTechnology(Element);
abstraction_element!(TransformationTechnology, TransformationTechnology);

impl TransformationTechnology {
    /// Create a new TransformationTechnology
    fn new(
        parent: &Element,
        name: &str,
        config: &TransformationTechnologyConfig,
    ) -> Result<Self, AutosarAbstractionError> {
        let version = parent.min_version()?;
        let ttech = parent.create_named_sub_element(ElementName::TransformationTechnology, name)?;
        let buffer_props = ttech.create_sub_element(ElementName::BufferProperties)?;

        match config {
            TransformationTechnologyConfig::Generic(generic_config) => {
                ttech
                    .create_sub_element(ElementName::Protocol)?
                    .set_character_data(generic_config.protocol_name.as_str())?;
                ttech
                    .create_sub_element(ElementName::Version)?
                    .set_character_data(generic_config.protocol_version.as_str())?;
                ttech
                    .create_sub_element(ElementName::TransformerClass)?
                    .set_character_data(EnumItem::Custom)?;
                buffer_props
                    .create_sub_element(ElementName::HeaderLength)?
                    .set_character_data(generic_config.header_length as u64)?;
                buffer_props
                    .create_sub_element(ElementName::InPlace)?
                    .set_character_data(generic_config.in_place.to_string())?;
            }
            TransformationTechnologyConfig::Com(com_config) => {
                ttech
                    .create_sub_element(ElementName::Protocol)?
                    .set_character_data("COMBased")?;
                ttech
                    .create_sub_element(ElementName::Version)?
                    .set_character_data("1")?;
                ttech
                    .create_sub_element(ElementName::TransformerClass)?
                    .set_character_data(EnumItem::Serializer)?;

                // comxf does not have a header
                buffer_props
                    .create_sub_element(ElementName::HeaderLength)?
                    .set_character_data(0)?;
                // comxf is always the first transformer in a chain, and the first transformer is not allowed to be in place
                buffer_props
                    .create_sub_element(ElementName::InPlace)?
                    .set_character_data("false")?;

                if version <= AutosarVersion::Autosar_00049 {
                    // only in versions up to AUTOSAR R20-11 (AUTOSAR_00049): a COM transformer must have a BUFFER-COMPUTATION
                    let bufcomp_compu = buffer_props
                        .create_sub_element(ElementName::BufferComputation)?
                        .create_sub_element(ElementName::CompuRationalCoeffs)?;
                    let numerator = bufcomp_compu.create_sub_element(ElementName::CompuNumerator)?;
                    numerator
                        .create_sub_element(ElementName::V)?
                        .set_character_data(com_config.isignal_ipdu_length as u64)?;
                    numerator.create_sub_element(ElementName::V)?.set_character_data(1)?;
                    bufcomp_compu
                        .create_sub_element(ElementName::CompuDenominator)?
                        .create_sub_element(ElementName::V)?
                        .set_character_data(1)?;
                }
            }
            TransformationTechnologyConfig::E2E(e2e_config) => {
                ttech
                    .create_sub_element(ElementName::Protocol)?
                    .set_character_data("E2E")?;
                ttech
                    .create_sub_element(ElementName::Version)?
                    .set_character_data("1.0.0")?;
                ttech
                    .create_sub_element(ElementName::TransformerClass)?
                    .set_character_data(EnumItem::Safety)?;
                if version >= AutosarVersion::Autosar_4_3_0 {
                    ttech
                        .create_sub_element(ElementName::HasInternalState)?
                        .set_character_data("true")?;
                }
                ttech
                    .create_sub_element(ElementName::NeedsOriginalData)?
                    .set_character_data("false")?;

                // select the profile name and header length based on the chosen E2E profile
                let (profile_name, header_length) = match e2e_config.profile {
                    E2EProfile::P01 => ("PROFILE_01", 16),
                    E2EProfile::P02 => ("PROFILE_02", 16),
                    E2EProfile::P04 => ("PROFILE_04", 96),
                    E2EProfile::P04m => ("PROFILE_04m", 128),
                    E2EProfile::P05 => ("PROFILE_05", 24),
                    E2EProfile::P06 => ("PROFILE_06", 40),
                    E2EProfile::P07 => ("PROFILE_07", 160),
                    E2EProfile::P07m => ("PROFILE_07m", 192),
                    E2EProfile::P08 => ("PROFILE_08", 128),
                    E2EProfile::P08m => ("PROFILE_08m", 160),
                    E2EProfile::P11 => ("PROFILE_11", 16),
                    E2EProfile::P22 => ("PROFILE_22", 16),
                    E2EProfile::P44 => ("PROFILE_44", 96),
                    E2EProfile::P44m => ("PROFILE_44m", 128),
                };

                // when E2E is used in a transformer chain after COM, the header length must be zero
                // since we don#t know how this transformer will be used, the user must set zero_header_length to true if the header length should be zero
                let real_header_length = if e2e_config.zero_header_length {
                    0u32
                } else {
                    header_length
                };

                buffer_props
                    .create_sub_element(ElementName::HeaderLength)?
                    .set_character_data(real_header_length as u64)?;
                buffer_props
                    .create_sub_element(ElementName::InPlace)?
                    .set_character_data(e2e_config.transform_in_place.to_string())?;

                let trans_desc = ttech.create_sub_element(ElementName::TransformationDescriptions)?;
                let e2e_desc = trans_desc.create_sub_element(ElementName::EndToEndTransformationDescription)?;

                // create the E2E profile description, with the mandatory fields
                e2e_desc
                    .create_sub_element(ElementName::ProfileName)?
                    .set_character_data(profile_name)?;
                e2e_desc
                    .create_sub_element(ElementName::UpperHeaderBitsToShift)?
                    .set_character_data(e2e_config.offset as u64)?;
                e2e_desc
                    .create_sub_element(ElementName::MaxDeltaCounter)?
                    .set_character_data(e2e_config.max_delta_counter as u64)?;
                e2e_desc
                    .create_sub_element(ElementName::MaxErrorStateInit)?
                    .set_character_data(e2e_config.max_error_state_init as u64)?;
                e2e_desc
                    .create_sub_element(ElementName::MaxErrorStateInvalid)?
                    .set_character_data(e2e_config.max_error_state_invalid as u64)?;
                e2e_desc
                    .create_sub_element(ElementName::MaxErrorStateValid)?
                    .set_character_data(e2e_config.max_error_state_valid as u64)?;
                e2e_desc
                    .create_sub_element(ElementName::MaxNoNewOrRepeatedData)?
                    .set_character_data(e2e_config.max_no_new_or_repeated_data as u64)?;
                e2e_desc
                    .create_sub_element(ElementName::MinOkStateInit)?
                    .set_character_data(e2e_config.min_ok_state_init as u64)?;
                e2e_desc
                    .create_sub_element(ElementName::MinOkStateInvalid)?
                    .set_character_data(e2e_config.min_ok_state_invalid as u64)?;
                e2e_desc
                    .create_sub_element(ElementName::MinOkStateValid)?
                    .set_character_data(e2e_config.min_ok_state_valid as u64)?;

                // window size is one value in AUTOSAR 4.4.0 (AUTOSAR_00047) and older, and three values in AUTOSAR 4.5.0 (AUTOSAR_00048) and newer
                if version <= AutosarVersion::Autosar_00047 {
                    // window size is only valid in AUTOSAR 4.4.0 (AUTOSAR_00047) and older
                    e2e_desc
                        .create_sub_element(ElementName::WindowSize)?
                        .set_character_data(e2e_config.window_size as u64)?;
                } else {
                    // new (Autosar 4.5.0+): window size can be set for each state
                    e2e_desc
                        .create_sub_element(ElementName::WindowSizeInit)?
                        .set_character_data(e2e_config.window_size_init.unwrap_or(e2e_config.window_size) as u64)?;
                    e2e_desc
                        .create_sub_element(ElementName::WindowSizeInvalid)?
                        .set_character_data(e2e_config.window_size_invalid.unwrap_or(e2e_config.window_size) as u64)?;
                    e2e_desc
                        .create_sub_element(ElementName::WindowSizeValid)?
                        .set_character_data(e2e_config.window_size_valid.unwrap_or(e2e_config.window_size) as u64)?;
                }

                // special handling for E2E profiles 01 and 11
                if matches!(e2e_config.profile, E2EProfile::P01 | E2EProfile::P11) {
                    // data id mode
                    let Some(data_id_mode) = e2e_config.data_id_mode else {
                        return Err(AutosarAbstractionError::InvalidParameter(
                            "Data ID mode is required for E2E profiles 01 and 11".to_string(),
                        ));
                    };
                    e2e_desc
                        .create_sub_element(ElementName::DataIdMode)?
                        .set_character_data::<EnumItem>(data_id_mode.into())?;

                    // counter offset
                    let Some(counter_offset) = e2e_config.counter_offset else {
                        return Err(AutosarAbstractionError::InvalidParameter(
                            "Counter offset is required for E2E profiles 01 and 11".to_string(),
                        ));
                    };
                    e2e_desc
                        .create_sub_element(ElementName::CounterOffset)?
                        .set_character_data(counter_offset as u64)?;

                    // crc offset
                    let Some(crc_offset) = e2e_config.crc_offset else {
                        return Err(AutosarAbstractionError::InvalidParameter(
                            "CRC offset is required for E2E profiles 01 and 11".to_string(),
                        ));
                    };
                    e2e_desc
                        .create_sub_element(ElementName::CrcOffset)?
                        .set_character_data(crc_offset as u64)?;

                    // data id nibble offset
                    if data_id_mode == DataIdMode::Lower12Bit {
                        let Some(data_id_nibble_offset) = e2e_config.data_id_nibble_offset else {
                            return Err(AutosarAbstractionError::InvalidParameter(
                                "Data ID nibble offset is required for E2E profiles 01 and 11 with DataIdMode::Lower12Bit".to_string(),
                            ));
                        };
                        e2e_desc
                            .create_sub_element(ElementName::DataIdNibbleOffset)?
                            .set_character_data(data_id_nibble_offset as u64)?;
                    }
                } else {
                    // offset may only be set if the profile is not 01 or 11
                    e2e_desc
                        .create_sub_element(ElementName::Offset)?
                        .set_character_data(e2e_config.offset as u64)?;
                }

                // optional fields
                // profile behavior
                if let Some(profile_behavior) = e2e_config.profile_behavior {
                    e2e_desc
                        .create_sub_element(ElementName::ProfileBehavior)?
                        .set_character_data::<EnumItem>(profile_behavior.into())?;
                }

                // sync counter init
                if let Some(sync_counter_init) = e2e_config.sync_counter_init {
                    e2e_desc
                        .create_sub_element(ElementName::SyncCounterInit)?
                        .set_character_data(sync_counter_init as u64)?;
                }
            }
            TransformationTechnologyConfig::SomeIp(someip_config) => {
                ttech
                    .create_sub_element(ElementName::Protocol)?
                    .set_character_data("SOMEIP")?;
                ttech
                    .create_sub_element(ElementName::TransformerClass)?
                    .set_character_data(EnumItem::Serializer)?;

                // someip header length is always 64
                buffer_props
                    .create_sub_element(ElementName::HeaderLength)?
                    .set_character_data(64)?;
                // someip is always the first transformer in a chain, and the first transformer is not allowed to be in place
                buffer_props
                    .create_sub_element(ElementName::InPlace)?
                    .set_character_data("false")?;

                let trans_desc = ttech.create_sub_element(ElementName::TransformationDescriptions)?;
                let someip_desc = trans_desc.create_sub_element(ElementName::SomeipTransformationDescription)?;
                someip_desc
                    .create_sub_element(ElementName::Alignment)?
                    .set_character_data(someip_config.alignment as u64)?;
                someip_desc
                    .create_sub_element(ElementName::ByteOrder)?
                    .set_character_data::<EnumItem>(someip_config.byte_order.into())?;
                someip_desc
                    .create_sub_element(ElementName::InterfaceVersion)?
                    .set_character_data(someip_config.interface_version as u64)?;

                // the someip transformer must currently always use version "1.0.0"
                ttech
                    .create_sub_element(ElementName::Version)?
                    .set_character_data("1.0.0")?;
            }
        }

        Ok(Self(ttech))
    }

    /// Get the protocol of the TransformationTechnology
    pub fn protocol(&self) -> Option<String> {
        self.element()
            .get_sub_element(ElementName::Protocol)?
            .character_data()?
            .string_value()
    }

    /// Get the transformer class of the TransformationTechnology
    pub fn transformer_class(&self) -> Option<EnumItem> {
        self.element()
            .get_sub_element(ElementName::TransformerClass)?
            .character_data()?
            .enum_value()
    }

    /// get the DataTransformationSet that contains this TransformationTechnology
    pub fn data_transformation_set(&self) -> Option<DataTransformationSet> {
        self.element()
            .named_parent()
            .ok()?
            .and_then(|dts| DataTransformationSet::try_from(dts).ok())
    }

    /// Get the configuration of the TransformationTechnology
    pub fn config(&self) -> Option<TransformationTechnologyConfig> {
        let protocol = self
            .element()
            .get_sub_element(ElementName::Protocol)?
            .character_data()?
            .string_value()?;

        let opt_e2e_desc = self
            .element()
            .get_sub_element(ElementName::TransformationDescriptions)
            .and_then(|tdesc| tdesc.get_sub_element(ElementName::EndToEndTransformationDescription));
        let opt_someip_desc = self
            .element()
            .get_sub_element(ElementName::TransformationDescriptions)
            .and_then(|tdesc| tdesc.get_sub_element(ElementName::SomeipTransformationDescription));

        if let Some(e2e_desc) = opt_e2e_desc {
            // E2E transformation
            let profile_name = e2e_desc
                .get_sub_element(ElementName::ProfileName)?
                .character_data()?
                .string_value()?;
            let profile = match profile_name.as_str() {
                "PROFILE_01" => E2EProfile::P01,
                "PROFILE_02" => E2EProfile::P02,
                "PROFILE_04" => E2EProfile::P04,
                "PROFILE_04m" => E2EProfile::P04m,
                "PROFILE_05" => E2EProfile::P05,
                "PROFILE_06" => E2EProfile::P06,
                "PROFILE_07" => E2EProfile::P07,
                "PROFILE_07m" => E2EProfile::P07m,
                "PROFILE_08" => E2EProfile::P08,
                "PROFILE_08m" => E2EProfile::P08m,
                "PROFILE_11" => E2EProfile::P11,
                "PROFILE_22" => E2EProfile::P22,
                "PROFILE_44" => E2EProfile::P44,
                "PROFILE_44m" => E2EProfile::P44m,
                _ => return None,
            };

            let buffer_props = self.element().get_sub_element(ElementName::BufferProperties)?;
            let in_place_str = buffer_props
                .get_sub_element(ElementName::InPlace)?
                .character_data()?
                .string_value()?;
            let in_place = in_place_str == "true" || in_place_str == "1";
            let buffer_header_length = buffer_props
                .get_sub_element(ElementName::HeaderLength)?
                .character_data()?
                .parse_integer::<u32>()?;

            let opt_window_size_init = e2e_desc
                .get_sub_element(ElementName::WindowSizeInit)
                .and_then(|elem| elem.character_data())
                .and_then(|cdata| cdata.parse_integer::<u32>());
            let opt_window_size_invalid = e2e_desc
                .get_sub_element(ElementName::WindowSizeInvalid)
                .and_then(|elem| elem.character_data())
                .and_then(|cdata| cdata.parse_integer::<u32>());
            let opt_window_size_valid = e2e_desc
                .get_sub_element(ElementName::WindowSizeValid)
                .and_then(|elem| elem.character_data())
                .and_then(|cdata| cdata.parse_integer::<u32>());
            // window size is one value in AUTOSAR 4.4.0 (AUTOSAR_00047) and older, and three values in AUTOSAR 4.5.0 (AUTOSAR_00048) and newer
            // when getting the config, first try to use the WINDOW-SIZE, then fall back to the three separate values
            let opt_window_size = e2e_desc
                .get_sub_element(ElementName::WindowSize)
                .and_then(|elem| elem.character_data())
                .and_then(|cdata| cdata.parse_integer())
                .or(opt_window_size_init)
                .or(opt_window_size_invalid)
                .or(opt_window_size_valid);

            let config = E2ETransformationTechnologyConfig {
                profile,
                zero_header_length: buffer_header_length == 0,
                transform_in_place: in_place,
                offset: 0,
                max_delta_counter: e2e_desc
                    .get_sub_element(ElementName::MaxDeltaCounter)?
                    .character_data()?
                    .parse_integer()?,
                max_error_state_init: e2e_desc
                    .get_sub_element(ElementName::MaxErrorStateInit)?
                    .character_data()?
                    .parse_integer()?,
                max_error_state_invalid: e2e_desc
                    .get_sub_element(ElementName::MaxErrorStateInvalid)?
                    .character_data()?
                    .parse_integer()?,
                max_error_state_valid: e2e_desc
                    .get_sub_element(ElementName::MaxErrorStateValid)?
                    .character_data()?
                    .parse_integer()?,
                max_no_new_or_repeated_data: e2e_desc
                    .get_sub_element(ElementName::MaxNoNewOrRepeatedData)?
                    .character_data()?
                    .parse_integer()?,
                min_ok_state_init: e2e_desc
                    .get_sub_element(ElementName::MinOkStateInit)?
                    .character_data()?
                    .parse_integer()?,
                min_ok_state_invalid: e2e_desc
                    .get_sub_element(ElementName::MinOkStateInvalid)?
                    .character_data()?
                    .parse_integer()?,
                min_ok_state_valid: e2e_desc
                    .get_sub_element(ElementName::MinOkStateValid)?
                    .character_data()?
                    .parse_integer()?,
                window_size: opt_window_size?,
                window_size_init: e2e_desc
                    .get_sub_element(ElementName::WindowSizeInit)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cd| cd.parse_integer()),
                window_size_invalid: e2e_desc
                    .get_sub_element(ElementName::WindowSizeInvalid)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cd| cd.parse_integer()),
                window_size_valid: e2e_desc
                    .get_sub_element(ElementName::WindowSizeValid)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cd| cd.parse_integer()),
                profile_behavior: e2e_desc
                    .get_sub_element(ElementName::ProfileBehavior)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cd| cd.enum_value())
                    .and_then(|enumitem| enumitem.try_into().ok()),
                sync_counter_init: e2e_desc
                    .get_sub_element(ElementName::SyncCounterInit)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cd| cd.parse_integer()),
                data_id_mode: e2e_desc
                    .get_sub_element(ElementName::DataIdMode)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cd| cd.enum_value())
                    .and_then(|enumitem| enumitem.try_into().ok()),
                data_id_nibble_offset: e2e_desc
                    .get_sub_element(ElementName::DataIdNibbleOffset)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cd| cd.parse_integer()),
                crc_offset: e2e_desc
                    .get_sub_element(ElementName::CrcOffset)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cd| cd.parse_integer()),
                counter_offset: e2e_desc
                    .get_sub_element(ElementName::CounterOffset)
                    .and_then(|elem| elem.character_data())
                    .and_then(|cd| cd.parse_integer()),
            };

            Some(TransformationTechnologyConfig::E2E(config))
        } else if let Some(someip_desc) = opt_someip_desc {
            // SOMEIP transformation
            let someip_config = SomeIpTransformationTechnologyConfig {
                alignment: someip_desc
                    .get_sub_element(ElementName::Alignment)?
                    .character_data()?
                    .parse_integer()?,
                byte_order: someip_desc
                    .get_sub_element(ElementName::ByteOrder)?
                    .character_data()?
                    .enum_value()
                    .and_then(|enumitem| enumitem.try_into().ok())?,
                interface_version: someip_desc
                    .get_sub_element(ElementName::InterfaceVersion)?
                    .character_data()?
                    .parse_integer()?,
            };
            Some(TransformationTechnologyConfig::SomeIp(someip_config))
        } else if protocol == "COMBased" || protocol == "ComBased" {
            // COM transformation
            let com_config = ComTransformationTechnologyConfig {
                isignal_ipdu_length: self
                    .element()
                    .get_sub_element(ElementName::BufferProperties)?
                    .get_sub_element(ElementName::BufferComputation)?
                    .get_sub_element(ElementName::CompuRationalCoeffs)?
                    .get_sub_element(ElementName::CompuNumerator)?
                    .character_data()?
                    .unsigned_integer_value()? as u32,
            };
            Some(TransformationTechnologyConfig::Com(com_config))
        } else {
            // generic transformation
            let buffer_props = self.element().get_sub_element(ElementName::BufferProperties)?;
            let in_place_str = buffer_props
                .get_sub_element(ElementName::InPlace)?
                .character_data()?
                .string_value()?;
            let in_place = in_place_str == "true" || in_place_str == "1";

            let generic_config = GenericTransformationTechnologyConfig {
                protocol_name: self
                    .element()
                    .get_sub_element(ElementName::Protocol)?
                    .character_data()?
                    .string_value()?,
                protocol_version: self
                    .element()
                    .get_sub_element(ElementName::Version)?
                    .character_data()?
                    .string_value()?,
                header_length: buffer_props
                    .get_sub_element(ElementName::HeaderLength)?
                    .character_data()?
                    .parse_integer()?,
                in_place,
            };
            Some(TransformationTechnologyConfig::Generic(generic_config))
        }
    }
}

//#########################################################

/// The configuration of any kind of `TransformationTechnology`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformationTechnologyConfig {
    Generic(GenericTransformationTechnologyConfig),
    Com(ComTransformationTechnologyConfig),
    E2E(E2ETransformationTechnologyConfig),
    SomeIp(SomeIpTransformationTechnologyConfig),
}

//#########################################################

/// Configuration for a generic transformation technology
/// For a generic trasformation, the mandatory values must be chosen by the user
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericTransformationTechnologyConfig {
    protocol_name: String,
    protocol_version: String,
    header_length: u32,
    in_place: bool,
}

//#########################################################

/// Configuration for a COM transformation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComTransformationTechnologyConfig {
    /// The length of the ISignalIpdu tha will be transformed by this Com transformer.
    /// The value is only used up to AUTOSAR R20-11 (AUTOSAR_00049), where it is needed to calculate the buffer size.
    isignal_ipdu_length: u32,
}

//#########################################################

/// Configuration for an E2E transformation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct E2ETransformationTechnologyConfig {
    /// E2E profile to use
    pub profile: E2EProfile,
    /// When E2E is used in a transformer chain after COM, the header length must be zero.
    /// In this configuration you are expected to provide space for the E2E data inside the signal group layout, and zero_header_length should be set to true.
    /// If zero_header_length is set to false, the appropriate header length for the chosen E2E profile will be used (e.g. 24 bits for PROFILE_05)
    pub zero_header_length: bool,
    /// Should the E2E transformation take place in the existing buffer or in a separate buffer?
    pub transform_in_place: bool,
    /// The offset in bits from the start of the buffer where the E2E data should be placed
    /// If E2E is used after COM, the offset should be 0; if E2E is used after SOMEIP, the offset should be 64
    pub offset: u32,
    /// Maximum jump in the counter value between two consecutive messages
    pub max_delta_counter: u32,
    pub max_error_state_init: u32,
    pub max_error_state_invalid: u32,
    pub max_error_state_valid: u32,
    /// The maximum allowed number of consecutive failed counter checks
    pub max_no_new_or_repeated_data: u32,
    pub min_ok_state_init: u32,
    pub min_ok_state_invalid: u32,
    pub min_ok_state_valid: u32,
    /// window size: Size of the monitoring window for the E2E state machine.
    /// This can be directly set up to AUTOSAR 4.4.0 (AUTOSAR_00047).
    /// For newer files this only provides the default if window_size_init, window_size_invalid and window_size_valid are not set
    pub window_size: u32,
    /// window size in the init state - only valid in AUTOSAR 4.5.0 (AUTOSAR_00048) and newer. if it is not set, this will default to window_size
    pub window_size_init: Option<u32>,
    /// window size in the invalid state - only valid in AUTOSAR 4.5.0 (AUTOSAR_00048) and newer. if it is not set, this will default to window_size
    pub window_size_invalid: Option<u32>,
    /// window size in the valid state - only valid in AUTOSAR 4.5.0 (AUTOSAR_00048) and newer. if it is not set, this will default to window_size
    pub window_size_valid: Option<u32>,
    pub profile_behavior: Option<E2EProfileBehavior>,
    pub sync_counter_init: Option<u32>,
    /// The data ID mode to use; required for E2E profiles 01 and 11, unused otherwise
    pub data_id_mode: Option<DataIdMode>,
    /// Offset of the data ID in the Data[] array in bits. Required for E2E profiles 01 and 11 when data_id_mode is Lower12Bit, unused otherwise
    pub data_id_nibble_offset: Option<u32>,
    /// Offset of the crc in the Data[] array in bits. Required for E2E profiles 01 and 11, unused otherwise
    pub crc_offset: Option<u32>,
    /// Offset of the counter in the Data[] array in bits. Required for E2E profiles 01 and 11, unused otherwise
    pub counter_offset: Option<u32>,
}

//#########################################################

/// Configuration for a SOMEIP transformation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SomeIpTransformationTechnologyConfig {
    /// The alignment of the data in bits
    pub alignment: u32,
    pub byte_order: ByteOrder,
    pub interface_version: u32,
}

//#########################################################

/// enumeration of the possible E2E profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum E2EProfile {
    P01,
    P02,
    P04,
    P04m,
    P05,
    P06,
    P07,
    P07m,
    P08,
    P08m,
    P11,
    P22,
    P44,
    P44m,
}

//#########################################################

/// there are two standardized behaviors for E2E profiles, which can be selected for each E2E transformation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum E2EProfileBehavior {
    PreR4_2,
    R4_2,
}

impl From<E2EProfileBehavior> for EnumItem {
    fn from(e2e_profile_behavior: E2EProfileBehavior) -> EnumItem {
        match e2e_profile_behavior {
            E2EProfileBehavior::PreR4_2 => EnumItem::PreR4_2,
            E2EProfileBehavior::R4_2 => EnumItem::R4_2,
        }
    }
}

impl TryFrom<EnumItem> for E2EProfileBehavior {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<E2EProfileBehavior, AutosarAbstractionError> {
        match value {
            EnumItem::PreR4_2 => Ok(E2EProfileBehavior::PreR4_2),
            EnumItem::R4_2 => Ok(E2EProfileBehavior::R4_2),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "E2EProfileBehavior".to_string(),
            }),
        }
    }
}

//#########################################################

/// data ID modes for E2E profiles 01 and 11
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataIdMode {
    All16Bit,
    Alternating8Bit,
    Lower12Bit,
    Lower8Bit,
}

impl From<DataIdMode> for EnumItem {
    fn from(data_id_mode: DataIdMode) -> EnumItem {
        match data_id_mode {
            DataIdMode::All16Bit => EnumItem::All16Bit,
            DataIdMode::Alternating8Bit => EnumItem::Alternating8Bit,
            DataIdMode::Lower12Bit => EnumItem::Lower12Bit,
            DataIdMode::Lower8Bit => EnumItem::Lower8Bit,
        }
    }
}

impl TryFrom<EnumItem> for DataIdMode {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<DataIdMode, AutosarAbstractionError> {
        match value {
            EnumItem::All16Bit => Ok(DataIdMode::All16Bit),
            EnumItem::Alternating8Bit => Ok(DataIdMode::Alternating8Bit),
            EnumItem::Lower12Bit => Ok(DataIdMode::Lower12Bit),
            EnumItem::Lower8Bit => Ok(DataIdMode::Lower8Bit),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "DataIdMode".to_string(),
            }),
        }
    }
}

//#########################################################

/// Properties of a transformation of an ISignal(Group)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformationISignalPropsConfig {
    E2E(E2ETransformationISignalPropsConfig),
    SomeIp(SomeIpTransformationISignalPropsConfig),
}

//#########################################################

/// Properties of an E2E transformation of an ISignal(Group)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct E2ETransformationISignalPropsConfig {
    pub data_ids: Vec<u32>,
    pub data_length: Option<u32>,
    pub max_data_length: Option<u32>,
    pub min_data_length: Option<u32>,
    /// source_id was added in AUTOSAR R20-11 (AUTOSAR_00049), and setting it to Some(x) causes an error in older files
    pub source_id: Option<u32>,
}

//#########################################################

/// Properties of a SOMEIP transformation of an ISignal(Group)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SomeIpTransformationISignalPropsConfig {
    pub legacy_strings: Option<bool>,
    pub interface_version: Option<u32>,
    pub dynamic_length: Option<bool>,
    pub message_type: Option<SomeIpMessageType>,
    pub size_of_array_length: Option<u32>,
    pub size_of_string_length: Option<u32>,
    pub size_of_struct_length: Option<u32>,
    pub size_of_union_length: Option<u32>,
}

//#########################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SomeIpMessageType {
    Notification,
    Request,
    RequestNoReturn,
    Response,
}

impl From<SomeIpMessageType> for EnumItem {
    fn from(someip_msg_type: SomeIpMessageType) -> EnumItem {
        match someip_msg_type {
            SomeIpMessageType::Notification => EnumItem::Notification,
            SomeIpMessageType::Request => EnumItem::Request,
            SomeIpMessageType::RequestNoReturn => EnumItem::RequestNoReturn,
            SomeIpMessageType::Response => EnumItem::Response,
        }
    }
}

impl TryFrom<EnumItem> for SomeIpMessageType {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<SomeIpMessageType, AutosarAbstractionError> {
        match value {
            EnumItem::Notification => Ok(SomeIpMessageType::Notification),
            EnumItem::Request => Ok(SomeIpMessageType::Request),
            EnumItem::RequestNoReturn => Ok(SomeIpMessageType::RequestNoReturn),
            EnumItem::Response => Ok(SomeIpMessageType::Response),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "SomeIpMessageType".to_string(),
            }),
        }
    }
}

//#########################################################

// Properties for the End to End transformation of an ISignal(Group)
//
// Implementation notes:
// - This is not an AbstractionElement, as it is not named.
// - It references the inner EndToEndTransformationISignalPropsConditional element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EndToEndTransformationISignalProps(pub(crate) Element);

impl EndToEndTransformationISignalProps {
    pub(crate) fn new(
        parent_element: Element,
        transformer: &TransformationTechnology,
        props: &E2ETransformationISignalPropsConfig,
    ) -> Result<Self, AutosarAbstractionError> {
        if transformer.protocol().as_deref() != Some("E2E") {
            return Err(AutosarAbstractionError::InvalidParameter(
                "EndToEndTransformationISignalProps must reference a E2E transformer".to_string(),
            ));
        }
        let e2e_props_elem = parent_element
            .create_sub_element(ElementName::EndToEndTransformationISignalProps)?
            .create_sub_element(ElementName::EndToEndTransformationISignalPropsVariants)?
            .create_sub_element(ElementName::EndToEndTransformationISignalPropsConditional)?;
        e2e_props_elem
            .create_sub_element(ElementName::TransformerRef)?
            .set_reference_target(transformer.element())?;

        if !props.data_ids.is_empty() {
            let data_ids_elem = e2e_props_elem.create_sub_element(ElementName::DataIds)?;
            for data_id in &props.data_ids {
                data_ids_elem
                    .create_sub_element(ElementName::DataId)?
                    .set_character_data(*data_id as u64)?;
            }
        }
        if let Some(data_length) = props.data_length {
            e2e_props_elem
                .create_sub_element(ElementName::DataLength)?
                .set_character_data(data_length.to_string())?;
        }
        if let Some(max_data_length) = props.max_data_length {
            e2e_props_elem
                .create_sub_element(ElementName::MaxDataLength)?
                .set_character_data(max_data_length.to_string())?;
        }
        if let Some(min_data_length) = props.min_data_length {
            e2e_props_elem
                .create_sub_element(ElementName::MinDataLength)?
                .set_character_data(min_data_length.to_string())?;
        }
        if let Some(source_id) = props.source_id {
            e2e_props_elem
                .create_sub_element(ElementName::SourceId)?
                .set_character_data(source_id.to_string())?;
        }
        Ok(Self(e2e_props_elem))
    }

    pub(crate) fn config(&self) -> E2ETransformationISignalPropsConfig {
        let elem = &self.0;
        let data_ids = elem
            .get_sub_element(ElementName::DataIds)
            .into_iter()
            .flat_map(|elem| elem.sub_elements())
            .filter_map(|elem| elem.character_data())
            .filter_map(|cdata| cdata.parse_integer())
            .collect();
        let data_length = elem
            .get_sub_element(ElementName::DataLength)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_integer());
        let max_data_length = elem
            .get_sub_element(ElementName::MaxDataLength)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_integer());
        let min_data_length = elem
            .get_sub_element(ElementName::MinDataLength)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_integer());
        let source_id = elem
            .get_sub_element(ElementName::SourceId)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_integer());
        E2ETransformationISignalPropsConfig {
            data_ids,
            data_length,
            max_data_length,
            min_data_length,
            source_id,
        }
    }
}

//#########################################################

// Properties for the SOMEIP transformation of an ISignal(Group)
//
// Implementation notes:
// - This is not an AbstractionElement, as it is not named.
// - It references the inner SomeIpTransformationISignalPropsConditional element.
pub(crate) struct SomeIpTransformationISignalProps(pub(crate) Element);

impl SomeIpTransformationISignalProps {
    pub(crate) fn new(
        parent_element: Element,
        transformer: &TransformationTechnology,
        props: &SomeIpTransformationISignalPropsConfig,
    ) -> Result<Self, AutosarAbstractionError> {
        if transformer.protocol().as_deref() != Some("SOMEIP") {
            return Err(AutosarAbstractionError::InvalidParameter(
                "SOMEIPTransformationISignalProps must reference a SOMEIP transformer".to_string(),
            ));
        }
        let someip_props_elem = parent_element
            .create_sub_element(ElementName::SomeipTransformationISignalProps)?
            .create_sub_element(ElementName::SomeipTransformationISignalPropsVariants)?
            .create_sub_element(ElementName::SomeipTransformationISignalPropsConditional)?;
        someip_props_elem
            .create_sub_element(ElementName::TransformerRef)?
            .set_reference_target(transformer.element())?;
        if let Some(legacy_strings) = &props.legacy_strings {
            someip_props_elem
                .create_sub_element(ElementName::ImplementsLegacyStringSerialization)?
                .set_character_data(legacy_strings.to_string())?;
        }
        if let Some(interface_version) = props.interface_version {
            someip_props_elem
                .create_sub_element(ElementName::InterfaceVersion)?
                .set_character_data(interface_version.to_string())?;
        }
        if let Some(dynamic_length) = props.dynamic_length {
            someip_props_elem
                .create_sub_element(ElementName::IsDynamicLengthFieldSize)?
                .set_character_data(dynamic_length.to_string())?;
        }
        if let Some(message_type) = props.message_type {
            someip_props_elem
                .create_sub_element(ElementName::MessageType)?
                .set_character_data::<EnumItem>(message_type.into())?;
        }
        if let Some(size_of_array_length) = props.size_of_array_length {
            someip_props_elem
                .create_sub_element(ElementName::SizeOfArrayLengthFields)?
                .set_character_data(size_of_array_length.to_string())?;
        }
        if let Some(size_of_string_length) = props.size_of_string_length {
            someip_props_elem
                .create_sub_element(ElementName::SizeOfStringLengthFields)?
                .set_character_data(size_of_string_length.to_string())?;
        }
        if let Some(size_of_struct_length) = props.size_of_struct_length {
            someip_props_elem
                .create_sub_element(ElementName::SizeOfStructLengthFields)?
                .set_character_data(size_of_struct_length.to_string())?;
        }
        if let Some(size_of_union_length) = props.size_of_union_length {
            someip_props_elem
                .create_sub_element(ElementName::SizeOfUnionLengthFields)?
                .set_character_data(size_of_union_length.to_string())?;
        }
        Ok(Self(someip_props_elem))
    }

    pub(crate) fn config(&self) -> SomeIpTransformationISignalPropsConfig {
        let elem = &self.0;
        let legacy_strings = elem
            .get_sub_element(ElementName::ImplementsLegacyStringSerialization)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.string_value())
            .map(|s| &s == "true" || s == "1");
        let interface_version = elem
            .get_sub_element(ElementName::InterfaceVersion)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_integer());
        let dynamic_length = elem
            .get_sub_element(ElementName::IsDynamicLengthFieldSize)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.string_value())
            .map(|s| &s == "true" || s == "1");
        let message_type = elem
            .get_sub_element(ElementName::MessageType)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.enum_value())
            .and_then(|enumitem| enumitem.try_into().ok());
        let size_of_array_length = elem
            .get_sub_element(ElementName::SizeOfArrayLengthFields)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_integer());
        let size_of_string_length = elem
            .get_sub_element(ElementName::SizeOfStringLengthFields)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_integer());
        let size_of_struct_length = elem
            .get_sub_element(ElementName::SizeOfStructLengthFields)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_integer());
        let size_of_union_length = elem
            .get_sub_element(ElementName::SizeOfUnionLengthFields)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_integer());
        SomeIpTransformationISignalPropsConfig {
            legacy_strings,
            interface_version,
            dynamic_length,
            message_type,
            size_of_array_length,
            size_of_string_length,
            size_of_struct_length,
            size_of_union_length,
        }
    }
}

//#########################################################

element_iterator!(DataTransformationIterator, DataTransformation, Some);

//#########################################################

element_iterator!(TransformationTechnologyIterator, TransformationTechnology, Some);

//#########################################################

element_iterator!(
    TransformerChainRefIterator,
    TransformationTechnology,
    (|element: Element| element.get_reference_target().ok())
);

//#########################################################

#[cfg(test)]
mod test {
    use crate::{
        communication::{ISignal, SystemSignal},
        datatype::{BaseTypeEncoding, SwBaseType},
    };

    use super::*;
    use autosar_data::AutosarModel;

    #[test]
    fn transformation_technologies() {
        // Data Transformation Sets and transformation technologies were introduced in AUTOSAR 4.2.1
        // There have been several changes over time, so it makes sense to test with multiple versions
        let versions = vec![
            AutosarVersion::Autosar_4_2_1,
            AutosarVersion::Autosar_4_3_0,
            AutosarVersion::Autosar_00044,
            AutosarVersion::Autosar_00046,
            AutosarVersion::Autosar_00047,
            AutosarVersion::Autosar_00048,
            AutosarVersion::Autosar_00049,
            AutosarVersion::Autosar_00050,
            AutosarVersion::Autosar_00051,
            AutosarVersion::Autosar_00052,
        ];
        for version in versions {
            create_transformation_technologies(version);
        }
    }

    fn create_transformation_technologies(file_version: AutosarVersion) {
        let model = AutosarModel::new();
        let _file = model.create_file("test", file_version).unwrap();

        let package = ArPackage::get_or_create(&model, "/package").unwrap();
        let dts = DataTransformationSet::new("test", &package).unwrap();

        // create a generic transformation technology
        let config = TransformationTechnologyConfig::Generic(GenericTransformationTechnologyConfig {
            protocol_name: "test".to_string(),
            protocol_version: "1.0.0".to_string(),
            header_length: 16,
            in_place: true,
        });
        let ttech = dts.create_transformation_technology("generic", &config).unwrap();
        let config2 = ttech.config().unwrap();
        assert_eq!(config, config2);

        // create a COM transformation technology
        let config = TransformationTechnologyConfig::Com(ComTransformationTechnologyConfig { isignal_ipdu_length: 8 });
        dts.create_transformation_technology("com", &config).unwrap();

        // create an E2E transformation technology for each profile
        for profile in &[
            E2EProfile::P01,
            E2EProfile::P02,
            E2EProfile::P04,
            E2EProfile::P04m,
            E2EProfile::P05,
            E2EProfile::P06,
            E2EProfile::P07,
            E2EProfile::P07m,
            E2EProfile::P08,
            E2EProfile::P08m,
            E2EProfile::P11,
            E2EProfile::P22,
            E2EProfile::P44,
            E2EProfile::P44m,
        ] {
            let e2e_config = E2ETransformationTechnologyConfig {
                profile: *profile,
                zero_header_length: false,
                transform_in_place: true,
                offset: 0,
                max_delta_counter: 0,
                max_error_state_init: 0,
                max_error_state_invalid: 0,
                max_error_state_valid: 0,
                max_no_new_or_repeated_data: 0,
                min_ok_state_init: 0,
                min_ok_state_invalid: 0,
                min_ok_state_valid: 0,
                window_size: 10,
                window_size_init: Some(11),
                window_size_invalid: Some(12),
                window_size_valid: Some(13),
                profile_behavior: Some(E2EProfileBehavior::R4_2),
                sync_counter_init: Some(0),
                data_id_mode: Some(DataIdMode::All16Bit),
                data_id_nibble_offset: Some(1),
                crc_offset: Some(2),
                counter_offset: Some(3),
            };
            let config = TransformationTechnologyConfig::E2E(e2e_config.clone());
            let ttech = dts
                .create_transformation_technology(&format!("{:?}", profile), &config)
                .unwrap();
            let config2 = ttech.config().unwrap();
            let TransformationTechnologyConfig::E2E(e2e_config2) = config2 else {
                panic!("Expected E2E transformation technology");
            };
            assert_eq!(e2e_config.profile, e2e_config2.profile);
            assert_eq!(e2e_config.zero_header_length, e2e_config2.zero_header_length);
            assert_eq!(e2e_config.transform_in_place, e2e_config2.transform_in_place);
            assert_eq!(e2e_config.offset, e2e_config2.offset);
            assert_eq!(e2e_config.max_delta_counter, e2e_config2.max_delta_counter);
            assert_eq!(e2e_config.max_error_state_init, e2e_config2.max_error_state_init);
            assert_eq!(e2e_config.max_error_state_invalid, e2e_config2.max_error_state_invalid);
            assert_eq!(e2e_config.max_error_state_valid, e2e_config2.max_error_state_valid);
            assert_eq!(
                e2e_config.max_no_new_or_repeated_data,
                e2e_config2.max_no_new_or_repeated_data
            );
            assert_eq!(e2e_config.min_ok_state_init, e2e_config2.min_ok_state_init);
            assert_eq!(e2e_config.min_ok_state_invalid, e2e_config2.min_ok_state_invalid);
            assert_eq!(e2e_config.min_ok_state_valid, e2e_config2.min_ok_state_valid);
        }

        // create a SOMEIP transformation technology
        let config = TransformationTechnologyConfig::SomeIp(SomeIpTransformationTechnologyConfig {
            alignment: 8,
            byte_order: ByteOrder::MostSignificantByteFirst,
            interface_version: 1,
        });
        let ttech = dts.create_transformation_technology("someip", &config).unwrap();
        let config2 = ttech.config().unwrap();
        assert_eq!(config, config2);
    }

    #[test]
    fn data_transformation() {
        let model = AutosarModel::new();
        let _file = model.create_file("test", AutosarVersion::Autosar_4_2_1).unwrap();

        let package = ArPackage::get_or_create(&model, "/package").unwrap();
        let dts = DataTransformationSet::new("test_dts", &package).unwrap();

        let e2e_transformation = dts
            .create_transformation_technology(
                "e2e",
                &TransformationTechnologyConfig::E2E(E2ETransformationTechnologyConfig {
                    profile: E2EProfile::P01,
                    zero_header_length: false,
                    transform_in_place: true,
                    offset: 0,
                    max_delta_counter: 0,
                    max_error_state_init: 0,
                    max_error_state_invalid: 0,
                    max_error_state_valid: 0,
                    max_no_new_or_repeated_data: 0,
                    min_ok_state_init: 0,
                    min_ok_state_invalid: 0,
                    min_ok_state_valid: 0,
                    window_size: 10,
                    window_size_init: Some(11),
                    window_size_invalid: Some(12),
                    window_size_valid: Some(13),
                    profile_behavior: Some(E2EProfileBehavior::R4_2),
                    sync_counter_init: Some(0),
                    data_id_mode: Some(DataIdMode::All16Bit),
                    data_id_nibble_offset: Some(1),
                    crc_offset: Some(2),
                    counter_offset: Some(3),
                }),
            )
            .unwrap();

        let com_transformation = dts
            .create_transformation_technology(
                "com",
                &TransformationTechnologyConfig::Com(ComTransformationTechnologyConfig { isignal_ipdu_length: 8 }),
            )
            .unwrap();

        let someip_transformation = dts
            .create_transformation_technology(
                "someip",
                &TransformationTechnologyConfig::SomeIp(SomeIpTransformationTechnologyConfig {
                    alignment: 8,
                    byte_order: ByteOrder::MostSignificantByteFirst,
                    interface_version: 1,
                }),
            )
            .unwrap();

        let generic_transformation = dts
            .create_transformation_technology(
                "generic",
                &TransformationTechnologyConfig::Generic(GenericTransformationTechnologyConfig {
                    protocol_name: "test".to_string(),
                    protocol_version: "1.0.0".to_string(),
                    header_length: 16,
                    in_place: true,
                }),
            )
            .unwrap();

        // not allowed: empty transformation chain
        let result = dts.create_data_transformation("test1", &[], true);
        assert!(result.is_err());

        // not allowed: multiple serializers Com + SomeIp
        let result = dts.create_data_transformation("test2", &[&com_transformation, &someip_transformation], true);
        assert!(result.is_err());

        // Ok: Com only
        let result = dts.create_data_transformation("test3", &[&com_transformation], true);
        assert!(result.is_ok());

        // Ok: E2E only
        let result = dts.create_data_transformation("test4", &[&e2e_transformation], true);
        assert!(result.is_ok());

        // Ok: SomeIp only
        let result = dts.create_data_transformation("test5", &[&someip_transformation], true);
        assert!(result.is_ok());

        // Ok: generic only
        let result = dts.create_data_transformation("test6", &[&generic_transformation], true);
        assert!(result.is_ok());

        // Ok: Com + E2E
        let result = dts.create_data_transformation("test7", &[&com_transformation, &e2e_transformation], true);
        assert!(result.is_ok());

        // Ok: SomeIp + E2E
        let result = dts.create_data_transformation("test8", &[&someip_transformation, &e2e_transformation], true);
        assert!(result.is_ok());

        // Ok: generic + E2E
        let result = dts.create_data_transformation("test9", &[&generic_transformation, &e2e_transformation], true);
        assert!(result.is_ok());

        let dts2 = DataTransformationSet::new("test_dts2", &package).unwrap();

        // not allowed: dts2 using transformation from dts
        let result = dts2.create_data_transformation("test10", &[&e2e_transformation], true);
        assert!(result.is_err());
    }

    #[test]
    fn data_transformation_chain_iter() {
        let model = AutosarModel::new();
        let _file = model.create_file("test", AutosarVersion::Autosar_4_2_1).unwrap();

        let package = ArPackage::get_or_create(&model, "/package").unwrap();
        let dts = DataTransformationSet::new("test_dts", &package).unwrap();

        let com_transformation = dts
            .create_transformation_technology(
                "com",
                &TransformationTechnologyConfig::Com(ComTransformationTechnologyConfig { isignal_ipdu_length: 8 }),
            )
            .unwrap();

        let dt = dts
            .create_data_transformation("test", &[&com_transformation], false)
            .unwrap();
        assert_eq!(dt.transformation_technologies().count(), 1);
        let com_transformation2 = dt.transformation_technologies().next().unwrap();
        assert_eq!(com_transformation, com_transformation2);
    }

    #[test]
    fn transformation_isignal_props() {
        let model = AutosarModel::new();
        let _file = model.create_file("test", AutosarVersion::Autosar_00049).unwrap();

        let package = ArPackage::get_or_create(&model, "/package").unwrap();
        let dts = DataTransformationSet::new("test_dts", &package).unwrap();

        let e2e_transformation = dts
            .create_transformation_technology(
                "e2e",
                &TransformationTechnologyConfig::E2E(E2ETransformationTechnologyConfig {
                    profile: E2EProfile::P01,
                    zero_header_length: false,
                    transform_in_place: true,
                    offset: 0,
                    max_delta_counter: 0,
                    max_error_state_init: 0,
                    max_error_state_invalid: 0,
                    max_error_state_valid: 0,
                    max_no_new_or_repeated_data: 0,
                    min_ok_state_init: 0,
                    min_ok_state_invalid: 0,
                    min_ok_state_valid: 0,
                    window_size: 10,
                    window_size_init: Some(11),
                    window_size_invalid: Some(12),
                    window_size_valid: Some(13),
                    profile_behavior: Some(E2EProfileBehavior::R4_2),
                    sync_counter_init: Some(0),
                    data_id_mode: Some(DataIdMode::All16Bit),
                    data_id_nibble_offset: Some(1),
                    crc_offset: Some(2),
                    counter_offset: Some(3),
                }),
            )
            .unwrap();
        let someip_transformation = dts
            .create_transformation_technology(
                "someip",
                &TransformationTechnologyConfig::SomeIp(SomeIpTransformationTechnologyConfig {
                    alignment: 8,
                    byte_order: ByteOrder::MostSignificantByteFirst,
                    interface_version: 1,
                }),
            )
            .unwrap();

        let sw_base_type =
            SwBaseType::new("sw_base_type", &package, 8, BaseTypeEncoding::None, None, None, None).unwrap();
        let signal = ISignal::new(
            "signal",
            8,
            &SystemSignal::new("sys_signal", &package).unwrap(),
            Some(&sw_base_type),
            &package,
        )
        .unwrap();

        let e2e_props = E2ETransformationISignalPropsConfig {
            data_ids: vec![1, 2, 3],
            data_length: Some(8),
            max_data_length: Some(16),
            min_data_length: Some(4),
            source_id: Some(0),
        };
        let someip_props = SomeIpTransformationISignalPropsConfig {
            legacy_strings: Some(true),
            interface_version: Some(1),
            dynamic_length: Some(true),
            message_type: Some(SomeIpMessageType::Request),
            size_of_array_length: Some(8),
            size_of_string_length: Some(16),
            size_of_struct_length: Some(32),
            size_of_union_length: Some(64),
        };

        signal
            .add_transformation_isignal_props(
                &e2e_transformation,
                &TransformationISignalPropsConfig::E2E(e2e_props.clone()),
            )
            .unwrap();
        signal
            .add_transformation_isignal_props(
                &someip_transformation,
                &TransformationISignalPropsConfig::SomeIp(someip_props.clone()),
            )
            .unwrap();
        assert_eq!(signal.transformation_isignal_props().count(), 2);
        let mut props_iter = signal.transformation_isignal_props();
        assert_eq!(
            props_iter.next().unwrap(),
            TransformationISignalPropsConfig::E2E(e2e_props)
        );
        assert_eq!(
            props_iter.next().unwrap(),
            TransformationISignalPropsConfig::SomeIp(someip_props)
        );
    }
}
