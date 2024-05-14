use crate::{abstraction_element, AbstactionElement, ArPackage, AutosarAbstractionError, EcuInstance};
use autosar_data::{
    AutosarDataError, AutosarModel, CharacterData, Element, ElementName, ElementsIterator, EnumItem, WeakElement,
};

/// Configuration settings of the Flexray cluster.
///
/// Refer to `AUTOSAR_TPS_SystemTemplate.pdf`, chapter 3.3.4 and Flexay 2.1 standard, appendix A + B
/// for the meanings and ranges of these parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct FlexrayClusterSettings {
    /// baud rate of the Flexray cluster: one of 10000000 (10Mbit/s), 5000000 (5Mbit/s), 2500000 (2.5Mbit/s)
    pub baudrate: u32,
    /// gdActionPointOffset: Number of macroticks the action point is offset from the
    /// beginning of a static slot or symbol window: in the range 1 - 63 MT
    pub action_point_offset: u8,
    /// gdBit: Nominal bit time; inverse of the baudrate: 0.1µs, 0.2µs, or 0.4µs
    pub bit: f64,
    /// gdCASRxLowMax: Upper limit of the CAS acceptance window. Range 67 - 99 gdBit
    pub cas_rx_low_max: u8,
    /// gColdStartAttempts: Maximum number of times a node in the cluster is permitted to attempt to start the cluster.
    /// Range: 2 - 31
    pub cold_start_attempts: u8,
    /// gdCycle: Length of the cycle in seconds. Range: 10µs - 16000µs; typically 0.005s
    pub cycle: f64,
    /// cCycleCountMax. Must be 63 to conform with Flexray 2.1
    pub cycle_count_max: u8,
    /// Indicates whether network idle time (NIT) error status should be detected
    pub detect_nit_error: bool,
    /// gdDynamicSlotIdlePhase: Duration of the idle phase within a dynamic slot. Range 0 - 2
    pub dynamic_slot_idle_phase: u8,
    /// Duration for which the bitstrobing is paused after transmission
    pub ignore_after_tx: u16,
    /// gListenNoise: Upper limit for the startup listen timeout and wakeup listen timeout in the presence of noise.
    /// Range 2 - 16
    pub listen_noise: u8,
    /// gMacroPerCycle: Number of macroticks in a communication cycle. Range: 10 - 16000 MT
    pub macro_per_cycle: u16,
    /// macrotick_duration \[s\] = cycle \[s\] / macro_per_cycle
    pub macrotick_duration: f64,
    /// gMaxWithoutClockCorrectionFatal: Threshold used for testing the vClockCorrectionFailed counter.
    /// Range: <max_without_clock_correction_passive> - 15 even/odd cycle pairs
    pub max_without_clock_correction_fatal: u8,
    /// gMaxWithoutClockCorrectionPassive: Threshold used for testing the vClockCorrectionFailed counter.
    /// Range: 1 - 15 even/odd cycle pairs
    pub max_without_clock_correction_passive: u8,
    /// gdMinislotActionPointOffset: Number of macroticks the minislot action point is offset from the beginning of a minislot. Range: 1 - 31 MT
    pub minislot_action_point_offset: u8,
    /// gdMinislot: Duration of a minislot. Range: 2 - 63 MT
    pub minislot_duration: u8,
    /// gdNIT: Duration of the Network Idle Time. 2 - 805 MT
    pub network_idle_time: u16,
    /// gNetworkManagementVectorLength: Length of the Network Management vector in a cluster. Range: 0 - 12 bytes
    pub network_management_vector_length: u8,
    /// gNumberOfMinislots: Number of minislots in the dynamic segment. Range: 0 - 7986
    pub number_of_minislots: u16,
    /// gNumberOfStaticSlots: Number of static slots in the static segment. Range: 2 - 1023
    pub number_of_static_slots: u16,
    /// gOffsetCorrectionStart: Start of the offset correction phase within the NIT, as MT from the start of the cycle. Range: 9 - 15999 MT
    pub offset_correction_start: u16,
    /// gPayloadLengthStatic: Payload length of a static frame, in two byte words. Range: 0 - 127 words
    pub payload_length_static: u16,
    /// Additional timespan in macroticks which takes jitter into account
    pub safety_margin: u16,
    /// gdSampleClockPeriod: Sample clock period. One of [1.25e-8 s, 2.5e-8 s, 5e-8 s]
    pub sample_clock_period: Option<f64>,
    /// gdStaticSlot: Duration of a static slot. Range: 4 - 661 MT
    pub static_slot_duration: u16,
    /// gdSymbolWindow: Duration of the symbol window. Range: 0 - 142 MT
    pub symbol_window: u8,
    // according to the Flexray spec: "gdActionPointOffset parameter also specifies the action point in the symbol window"
    // so this parameter is useless?
    pub symbol_window_action_point_offset: Option<u8>,
    /// gSyncNodeMax: Maximum number of nodes that may send frames with the sync frame indicator bit set to one. Range: 2 - 15
    pub sync_frame_id_count_max: u8,
    /// The duration of timer t_TrcvStdbyDelay in seconds.
    pub transceiver_standby_delay: Option<f64>,
    /// gdTSSTransmitter: Number of bits in the Transmission Start Sequence. Range: 3 - 15 gdBit
    pub transmission_start_sequence_duration: u8,
    /// gdWakeupSymbolRxIdle: Number of bits used by the node to test the duration
    /// of the 'idle' portion of a received wakeup symbol. Range: 14 - 59 gdBit
    pub wakeup_rx_idle: u16,
    /// gdWakeupSymbolRxLow: Number of bits used by the node to test the LOW
    /// portion of a received wakeup symbol. Range 11 - 59 gdBit
    pub wakeup_rx_low: u8,
    /// gdWakeupSymbolRxWindow: The size of the window used to detect wakeups. Range: 76 - 301 gdBit
    pub wakeup_rx_window: u16,
    /// gdWakeupSymbolTxLow: Number of bits used by the node to transmit the LOW part of a wakeup symbol. Range: 15 - 60 gdBit
    pub wakeup_tx_active: u16,
    /// gdWakeupSymbolTxIdle: Number of bits used by the node to transmit the 'idle' part of a wakeup symbol. Range: 45 - 180 gdBit
    pub wakeup_tx_idle: u16,
}

/// Information about the flexray physical channels present inside a cluster
#[derive(Debug, Clone, PartialEq)]
pub struct FlexrayPhysicalChannelsInfo {
    /// channel A - it contains FlexrayChannelName::A
    pub channel_a: Option<FlexrayPhysicalChannel>,
    /// channel B - it contains FlexrayChannelName::B
    pub channel_b: Option<FlexrayPhysicalChannel>,
}

/// A flexray cluster may contain the channels A and/or B.
///
/// This enum is an abstraction over the \<CHANNEL-NAME\> element.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexrayChannelName {
    A,
    B,
}

/// A `FlexrayCluster` contains all configuration items associated with a Flexray network.
/// The cluster connects multiple ECUs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlexrayCluster(Element);
abstraction_element!(FlexrayCluster, FlexrayCluster);

impl FlexrayCluster {
    // create a new FlexrayCluster - internal use only. User code should call System::create_flexray_cluster
    pub(crate) fn new(
        name: &str,
        package: &ArPackage,
        settings: &FlexrayClusterSettings,
    ) -> Result<Self, AutosarAbstractionError> {
        let elem_pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_cluster = elem_pkg_elements.create_named_sub_element(ElementName::FlexrayCluster, name)?;
        if let Ok(cluster_content) = elem_cluster
            .create_sub_element(ElementName::FlexrayClusterVariants)
            .and_then(|fcv| fcv.create_sub_element(ElementName::FlexrayClusterConditional))
        {
            cluster_content
                .create_sub_element(ElementName::ProtocolName)
                .and_then(|pn| pn.set_character_data(CharacterData::String("FlexRay".to_string())))?;
            cluster_content
                .create_sub_element(ElementName::ProtocolVersion)
                .and_then(|pv| pv.set_character_data(CharacterData::String("2.1".to_string())))?;

            cluster_content.create_sub_element(ElementName::PhysicalChannels)?;
        }

        let flexray_cluster = FlexrayCluster(elem_cluster);
        flexray_cluster.update_settings(settings);

        Ok(flexray_cluster)
    }

    /// update the cluster settings
    ///
    /// The settings of a flexray cluster determine all the details of timing and slot layout.
    /// These settings are subject to multiple cross dependencies and constraints.
    ///
    /// You may check the validity of the settings by calling [`FlexrayClusterSettings::verify`].
    ///
    /// However, the update function does not require that the settings are valid, and will
    /// also update the model with invalid settings if desired.
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
    /// let cluster = system.create_flexray_cluster("Cluster", &package, &FlexrayClusterSettings::default()).unwrap();
    /// let mut settings = cluster.get_settings();
    /// settings.macro_per_cycle = 5000;
    /// cluster.update_settings(&settings);
    /// ```
    pub fn update_settings(&self, settings: &FlexrayClusterSettings) {
        if let Ok(cluster_content) = self
            .0
            .get_or_create_sub_element(ElementName::FlexrayClusterVariants)
            .and_then(|fcv| fcv.get_or_create_sub_element(ElementName::FlexrayClusterConditional))
        {
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::Baudrate)
                .and_then(|br| br.set_character_data(settings.baudrate.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::ActionPointOffset)
                .and_then(|apo| apo.set_character_data(settings.action_point_offset.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::Bit)
                .and_then(|bit| bit.set_character_data(settings.bit));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::CasRxLowMax)
                .and_then(|crlm| crlm.set_character_data(settings.cas_rx_low_max.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::ColdStartAttempts)
                .and_then(|csa| csa.set_character_data(settings.cold_start_attempts.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::Cycle)
                .and_then(|apo| apo.set_character_data(settings.cycle));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::CycleCountMax)
                .and_then(|ccm| ccm.set_character_data(settings.cycle_count_max.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::DetectNitError)
                .and_then(|dne| dne.set_character_data(settings.detect_nit_error.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::DynamicSlotIdlePhase)
                .and_then(|dsip| dsip.set_character_data(settings.dynamic_slot_idle_phase.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::IgnoreAfterTx)
                .and_then(|iat| iat.set_character_data(settings.ignore_after_tx.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::ListenNoise)
                .and_then(|ln| ln.set_character_data(settings.listen_noise.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::MacroPerCycle)
                .and_then(|mpc| mpc.set_character_data(settings.macro_per_cycle.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::MacrotickDuration)
                .and_then(|mpc| mpc.set_character_data(settings.macrotick_duration));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::MaxWithoutClockCorrectionFatal)
                .and_then(|mwccf| mwccf.set_character_data(settings.max_without_clock_correction_fatal.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::MaxWithoutClockCorrectionPassive)
                .and_then(|mwccp| mwccp.set_character_data(settings.max_without_clock_correction_passive.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::MinislotActionPointOffset)
                .and_then(|mapo| mapo.set_character_data(settings.minislot_action_point_offset.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::MinislotDuration)
                .and_then(|md| md.set_character_data(settings.minislot_duration.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::NetworkIdleTime)
                .and_then(|nit| nit.set_character_data(settings.network_idle_time.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::NetworkManagementVectorLength)
                .and_then(|nmvl| nmvl.set_character_data(settings.network_management_vector_length.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::NumberOfMinislots)
                .and_then(|nom| nom.set_character_data(settings.number_of_minislots.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::NumberOfStaticSlots)
                .and_then(|noss| noss.set_character_data(settings.number_of_static_slots.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::OffsetCorrectionStart)
                .and_then(|ocs| ocs.set_character_data(settings.offset_correction_start.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::PayloadLengthStatic)
                .and_then(|pls| pls.set_character_data(settings.payload_length_static.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::SafetyMargin)
                .and_then(|sm| sm.set_character_data(settings.safety_margin.to_string()));
            if let Some(sample_clock_period) = settings.sample_clock_period {
                let _ = cluster_content
                    .get_or_create_sub_element(ElementName::SampleClockPeriod)
                    .and_then(|scp| scp.set_character_data(sample_clock_period));
            } else if let Some(scp) = cluster_content.get_sub_element(ElementName::SampleClockPeriod) {
                let _ = cluster_content.remove_sub_element(scp);
            }
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::StaticSlotDuration)
                .and_then(|ssd| ssd.set_character_data(settings.static_slot_duration.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::SymbolWindow)
                .and_then(|sw| sw.set_character_data(settings.symbol_window.to_string()));

            if let Some(symbol_window_action_point_offset) = settings.symbol_window_action_point_offset {
                let _ = cluster_content
                    .get_or_create_sub_element(ElementName::SymbolWindowActionPointOffset)
                    .and_then(|swapo| swapo.set_character_data(symbol_window_action_point_offset.to_string()));
            } else if let Some(swapo) = cluster_content.get_sub_element(ElementName::SymbolWindowActionPointOffset) {
                let _ = cluster_content.remove_sub_element(swapo);
            }
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::SyncFrameIdCountMax)
                .and_then(|sficm| sficm.set_character_data(settings.sync_frame_id_count_max.to_string()));
            if let Some(transceiver_standby_delay) = settings.transceiver_standby_delay {
                let _ = cluster_content
                    .get_or_create_sub_element(ElementName::TranceiverStandbyDelay)
                    .and_then(|tsd| tsd.set_character_data(transceiver_standby_delay));
            } else if let Some(tsd) = cluster_content.get_sub_element(ElementName::TranceiverStandbyDelay) {
                let _ = cluster_content.remove_sub_element(tsd);
            }
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::TransmissionStartSequenceDuration)
                .and_then(|tssd| tssd.set_character_data(settings.transmission_start_sequence_duration.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::WakeupRxIdle)
                .and_then(|wri| wri.set_character_data(settings.wakeup_rx_idle.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::WakeupRxLow)
                .and_then(|wrl| wrl.set_character_data(settings.wakeup_rx_low.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::WakeupRxWindow)
                .and_then(|wrw| wrw.set_character_data(settings.wakeup_rx_window.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::WakeupTxActive)
                .and_then(|wta| wta.set_character_data(settings.wakeup_tx_active.to_string()));
            let _ = cluster_content
                .get_or_create_sub_element(ElementName::WakeupTxIdle)
                .and_then(|wti| wti.set_character_data(settings.wakeup_tx_idle.to_string()));
        }
    }

    /// retrieve the current flexray cluster settings from a [`FlexrayCluster`]
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
    /// # let mut settings_in = FlexrayClusterSettings::default();
    /// # settings_in.macro_per_cycle = 5000;
    /// let cluster = system.create_flexray_cluster("Cluster", &package, &settings_in).unwrap();
    /// let settings = cluster.get_settings();
    /// # assert_eq!(settings_in, settings);
    /// ```
    #[must_use]
    pub fn get_settings(&self) -> FlexrayClusterSettings {
        let mut settings = FlexrayClusterSettings {
            baudrate: 0,
            action_point_offset: 0,
            bit: 0.0,
            cas_rx_low_max: 0,
            cold_start_attempts: 0,
            cycle: 0.0,
            cycle_count_max: 0,
            detect_nit_error: false,
            dynamic_slot_idle_phase: 0,
            ignore_after_tx: 0,
            listen_noise: 0,
            macro_per_cycle: 0,
            macrotick_duration: 0.0,
            max_without_clock_correction_fatal: 0,
            max_without_clock_correction_passive: 0,
            minislot_action_point_offset: 0,
            minislot_duration: 0,
            network_idle_time: 0,
            network_management_vector_length: 0,
            number_of_minislots: 0,
            number_of_static_slots: 0,
            offset_correction_start: 0,
            payload_length_static: 0,
            safety_margin: 0,
            sample_clock_period: None,
            static_slot_duration: 0,
            symbol_window: 0,
            symbol_window_action_point_offset: None,
            sync_frame_id_count_max: 0,
            transceiver_standby_delay: None,
            transmission_start_sequence_duration: 0,
            wakeup_rx_idle: 0,
            wakeup_rx_low: 0,
            wakeup_rx_window: 0,
            wakeup_tx_active: 0,
            wakeup_tx_idle: 0,
        };

        if let Some(cluster_content) = self
            .0
            .get_sub_element(ElementName::FlexrayClusterVariants)
            .and_then(|fcv| fcv.get_sub_element(ElementName::FlexrayClusterConditional))
        {
            if let Some(baudrate) = cluster_content
                .get_sub_element(ElementName::Baudrate)
                .and_then(|br| br.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.baudrate = baudrate;
            }
            if let Some(action_point_offset) = cluster_content
                .get_sub_element(ElementName::ActionPointOffset)
                .and_then(|apo| apo.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.action_point_offset = action_point_offset;
            }
            if let Some(bit) = cluster_content
                .get_sub_element(ElementName::Bit)
                .and_then(|bit| bit.character_data())
                .and_then(|cdata| cdata.float_value())
            {
                settings.bit = bit;
            }
            if let Some(cas_rx_low_max) = cluster_content
                .get_sub_element(ElementName::CasRxLowMax)
                .and_then(|crlm| crlm.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.cas_rx_low_max = cas_rx_low_max;
            }
            if let Some(cold_start_attempts) = cluster_content
                .get_sub_element(ElementName::ColdStartAttempts)
                .and_then(|csa| csa.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.cold_start_attempts = cold_start_attempts;
            }
            if let Some(cycle) = cluster_content
                .get_sub_element(ElementName::Cycle)
                .and_then(|apo| apo.character_data())
                .and_then(|cdata| cdata.float_value())
            {
                settings.cycle = cycle;
            }
            if let Some(cycle_count_max) = cluster_content
                .get_sub_element(ElementName::CycleCountMax)
                .and_then(|ccm| ccm.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.cycle_count_max = cycle_count_max;
            }
            if let Some(detect_nit_error) = cluster_content
                .get_sub_element(ElementName::DetectNitError)
                .and_then(|dne| dne.character_data())
                .and_then(|cdata| cdata.string_value())
            {
                settings.detect_nit_error = (detect_nit_error == "true") || (detect_nit_error == "1");
            }
            if let Some(dynamic_slot_idle_phase) = cluster_content
                .get_sub_element(ElementName::DynamicSlotIdlePhase)
                .and_then(|dsip| dsip.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.dynamic_slot_idle_phase = dynamic_slot_idle_phase;
            }
            if let Some(ignore_after_tx) = cluster_content
                .get_sub_element(ElementName::IgnoreAfterTx)
                .and_then(|iat| iat.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.ignore_after_tx = ignore_after_tx;
            }
            if let Some(listen_noise) = cluster_content
                .get_sub_element(ElementName::ListenNoise)
                .and_then(|ln| ln.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.listen_noise = listen_noise;
            }
            if let Some(macro_per_cycle) = cluster_content
                .get_sub_element(ElementName::MacroPerCycle)
                .and_then(|mpc| mpc.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.macro_per_cycle = macro_per_cycle;
            }
            if let Some(macrotick_duration) = cluster_content
                .get_sub_element(ElementName::MacrotickDuration)
                .and_then(|mpc| mpc.character_data())
                .and_then(|cdata| cdata.float_value())
            {
                settings.macrotick_duration = macrotick_duration;
            }
            if let Some(max_without_clock_correction_fatal) = cluster_content
                .get_sub_element(ElementName::MaxWithoutClockCorrectionFatal)
                .and_then(|mwccf| mwccf.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.max_without_clock_correction_fatal = max_without_clock_correction_fatal;
            }
            if let Some(max_without_clock_correction_passive) = cluster_content
                .get_sub_element(ElementName::MaxWithoutClockCorrectionPassive)
                .and_then(|mwccp| mwccp.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.max_without_clock_correction_passive = max_without_clock_correction_passive;
            }
            if let Some(minislot_action_point_offset) = cluster_content
                .get_sub_element(ElementName::MinislotActionPointOffset)
                .and_then(|mapo| mapo.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.minislot_action_point_offset = minislot_action_point_offset;
            }
            if let Some(minislot_duration) = cluster_content
                .get_sub_element(ElementName::MinislotDuration)
                .and_then(|md| md.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.minislot_duration = minislot_duration;
            }
            if let Some(network_idle_time) = cluster_content
                .get_sub_element(ElementName::NetworkIdleTime)
                .and_then(|nit| nit.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.network_idle_time = network_idle_time;
            }
            if let Some(network_management_vector_length) = cluster_content
                .get_sub_element(ElementName::NetworkManagementVectorLength)
                .and_then(|nmvl| nmvl.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.network_management_vector_length = network_management_vector_length;
            }
            if let Some(number_of_minislots) = cluster_content
                .get_sub_element(ElementName::NumberOfMinislots)
                .and_then(|nom| nom.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.number_of_minislots = number_of_minislots;
            }
            if let Some(number_of_static_slots) = cluster_content
                .get_sub_element(ElementName::NumberOfStaticSlots)
                .and_then(|noss| noss.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.number_of_static_slots = number_of_static_slots;
            }
            if let Some(offset_correction_start) = cluster_content
                .get_sub_element(ElementName::OffsetCorrectionStart)
                .and_then(|ocs| ocs.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.offset_correction_start = offset_correction_start;
            }
            if let Some(payload_length_static) = cluster_content
                .get_sub_element(ElementName::PayloadLengthStatic)
                .and_then(|pls| pls.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.payload_length_static = payload_length_static;
            }
            if let Some(safety_margin) = cluster_content
                .get_sub_element(ElementName::SafetyMargin)
                .and_then(|sm| sm.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.safety_margin = safety_margin;
            }

            settings.sample_clock_period = cluster_content
                .get_sub_element(ElementName::SampleClockPeriod)
                .and_then(|scp| scp.character_data())
                .and_then(|cdata| cdata.float_value());

            if let Some(static_slot_duration) = cluster_content
                .get_sub_element(ElementName::StaticSlotDuration)
                .and_then(|ssd| ssd.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.static_slot_duration = static_slot_duration;
            }
            if let Some(symbol_window) = cluster_content
                .get_sub_element(ElementName::SymbolWindow)
                .and_then(|sw| sw.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.symbol_window = symbol_window;
            }
            settings.symbol_window_action_point_offset = cluster_content
                .get_sub_element(ElementName::SymbolWindowActionPointOffset)
                .and_then(|swapo| swapo.character_data())
                .and_then(|cdata| cdata.parse_integer());

            if let Some(sync_frame_id_count_max) = cluster_content
                .get_sub_element(ElementName::SyncFrameIdCountMax)
                .and_then(|sficm| sficm.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.sync_frame_id_count_max = sync_frame_id_count_max;
            }

            settings.transceiver_standby_delay = cluster_content
                .get_sub_element(ElementName::TranceiverStandbyDelay)
                .and_then(|tsd| tsd.character_data())
                .and_then(|cdata| cdata.float_value());

            if let Some(transmission_start_sequence_duration) = cluster_content
                .get_sub_element(ElementName::TransmissionStartSequenceDuration)
                .and_then(|tssd| tssd.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.transmission_start_sequence_duration = transmission_start_sequence_duration;
            }
            if let Some(wakeup_rx_idle) = cluster_content
                .get_sub_element(ElementName::WakeupRxIdle)
                .and_then(|wri| wri.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.wakeup_rx_idle = wakeup_rx_idle;
            }
            if let Some(wakeup_rx_low) = cluster_content
                .get_sub_element(ElementName::WakeupRxLow)
                .and_then(|wrl| wrl.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.wakeup_rx_low = wakeup_rx_low;
            }
            if let Some(wakeup_rx_window) = cluster_content
                .get_sub_element(ElementName::WakeupRxWindow)
                .and_then(|wrw| wrw.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.wakeup_rx_window = wakeup_rx_window;
            }
            if let Some(wakeup_tx_active) = cluster_content
                .get_sub_element(ElementName::WakeupTxActive)
                .and_then(|wta| wta.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.wakeup_tx_active = wakeup_tx_active;
            }
            if let Some(wakeup_tx_idle) = cluster_content
                .get_sub_element(ElementName::WakeupTxIdle)
                .and_then(|wti| wti.character_data())
                .and_then(|cdata| cdata.parse_integer())
            {
                settings.wakeup_tx_idle = wakeup_tx_idle;
            }
        }

        settings
    }

    /// Create a new physical channel for the cluster
    ///
    /// A cluster may contain channel A, channel B, or both A and B.
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
    /// # let settings = FlexrayClusterSettings::default();
    /// let cluster = system.create_flexray_cluster("Cluster", &package, &settings).unwrap();
    /// let channel = cluster.create_physical_channel("Channel", FlexrayChannelName::A).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ItemAlreadyExists`] There is already a physical channel in this CAN cluster
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn create_physical_channel(
        &self,
        name: &str,
        channel_name: FlexrayChannelName,
    ) -> Result<FlexrayPhysicalChannel, AutosarAbstractionError> {
        let phys_channels = self
            .0
            .get_or_create_sub_element(ElementName::FlexrayClusterVariants)?
            .get_or_create_sub_element(ElementName::FlexrayClusterConditional)?
            .get_or_create_sub_element(ElementName::PhysicalChannels)?;

        for existing_channel in phys_channels.sub_elements() {
            if let Some(existing_channel_name) = FlexrayPhysicalChannel(existing_channel).channel_name() {
                if existing_channel_name == channel_name {
                    return Err(AutosarAbstractionError::ItemAlreadyExists);
                }
            }
        }

        let channel = phys_channels.create_named_sub_element(ElementName::FlexrayPhysicalChannel, name)?;

        let cn_item = match channel_name {
            FlexrayChannelName::A => EnumItem::ChannelA,
            FlexrayChannelName::B => EnumItem::ChannelB,
        };

        let _ = channel
            .create_sub_element(ElementName::ChannelName)
            .and_then(|cn| cn.set_character_data(cn_item));

        Ok(FlexrayPhysicalChannel(channel))
    }

    /// get the physical channels of this cluster
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
    /// let cluster = system.create_can_cluster("Cluster", &package, &CanClusterSettings::default()).unwrap();
    /// let channel = cluster.create_physical_channel("Channel").unwrap();
    /// ```
    #[must_use]
    pub fn physical_channels(&self) -> FlexrayPhysicalChannelsInfo {
        let mut channel_info = FlexrayPhysicalChannelsInfo {
            channel_a: None,
            channel_b: None,
        };
        if let Some(phys_channels) = self
            .0
            .get_sub_element(ElementName::FlexrayClusterVariants)
            .and_then(|fcv| fcv.get_sub_element(ElementName::FlexrayClusterConditional))
            .and_then(|fcc| fcc.get_sub_element(ElementName::PhysicalChannels))
        {
            for channel_elem in phys_channels.sub_elements() {
                if let Ok(channel) = FlexrayPhysicalChannel::try_from(channel_elem) {
                    match channel.channel_name() {
                        Some(FlexrayChannelName::A) => channel_info.channel_a = Some(channel),
                        Some(FlexrayChannelName::B) => channel_info.channel_b = Some(channel),
                        None => {}
                    }
                }
            }
        }
        channel_info
    }
}

//##################################################################

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
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// # let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// # let system = System::new("System", &package, SystemCategory::SystemExtract).unwrap();
    /// # let cluster = system.create_can_cluster("Cluster", &package, &CanClusterSettings::default()).unwrap();
    /// let channel = cluster.create_physical_channel("Channel").unwrap();
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
}

//##################################################################

/// An `EcuInstance` needs a `FlexrayCommunicationController` in order to connect to a Flexray cluster.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlexrayCommunicationController(Element);
abstraction_element!(FlexrayCommunicationController, FlexrayCommunicationController);

impl FlexrayCommunicationController {
    // create a new FlexrayCommunicationController - called by EcuInstance::create_flexray_communication_controller
    pub(crate) fn new(name: &str, ecu: &EcuInstance) -> Result<Self, AutosarAbstractionError> {
        let commcontrollers = ecu.element().get_or_create_sub_element(ElementName::CommControllers)?;
        let ctrl = commcontrollers.create_named_sub_element(ElementName::FlexrayCommunicationController, name)?;
        let _flxccc = ctrl
            .create_sub_element(ElementName::FlexrayCommunicationControllerVariants)?
            .create_sub_element(ElementName::FlexrayCommunicationControllerConditional)?;

        Ok(Self(ctrl))
    }

    /// return an iterator over the [`FlexrayPhysicalChannel`]s connected to this controller
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
    /// # let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let flexray_controller = ecu_instance.create_flexray_communication_controller("FRCtrl").unwrap();
    /// # let cluster = system.create_flexray_cluster("Cluster", &package, &FlexrayClusterSettings::default()).unwrap();
    /// # let physical_channel = cluster.create_physical_channel("Channel", FlexrayChannelName::A).unwrap();
    /// flexray_controller.connect_physical_channel("connection", &physical_channel).unwrap();
    /// for channel in flexray_controller.connected_channels() {
    ///     // ...
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model
    pub fn connected_channels(&self) -> FlexrayCtrlChannelsIterator {
        if let Ok(ecu) = self.ecu_instance().map(|ecuinstance| ecuinstance.element()) {
            FlexrayCtrlChannelsIterator::new(self, &ecu)
        } else {
            FlexrayCtrlChannelsIterator {
                connector_iter: None,
                comm_controller: self.0.clone(),
                model: Err(AutosarDataError::ElementNotFound),
            }
        }
    }

    /// get the `EcuInstance` that contains this `FlexrayCommunicationController`
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
    /// # let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let flexray_controller = ecu_instance.create_flexray_communication_controller("FRCtrl").unwrap();
    /// assert_eq!(ecu_instance, flexray_controller.ecu_instance().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn ecu_instance(&self) -> Result<EcuInstance, AutosarAbstractionError> {
        // unwrapping is safe here - self.0.named_parent() cannot return Ok(None).
        // the FlexrayCommunicationController is always a child of an EcuInstance,
        // or else it is deleted and named_parent() return Err(...), which is handled by the ?
        let ecu: Element = self.0.named_parent()?.unwrap();
        EcuInstance::try_from(ecu)
    }

    /// Connect this [`FlexrayCommunicationController`] inside an [`EcuInstance`] to a [`FlexrayPhysicalChannel`] in the [crate::System]
    ///
    /// Creates a FlexrayCommunicationConnector in the [`EcuInstance`] that contains this [`FlexrayCommunicationController`].
    ///
    /// This function establishes the relationships:
    ///  - [`FlexrayPhysicalChannel`] -> FlexrayCommunicationConnector
    ///  - FlexrayCommunicationConnector -> [`FlexrayCommunicationController`]
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
    /// # let ecu_instance = system.create_ecu_instance("ecu_name", &package).unwrap();
    /// let flexray_controller = ecu_instance.create_flexray_communication_controller("FlxCtrl").unwrap();
    /// # let cluster = system.create_flexray_cluster("Cluster", &package, &FlexrayClusterSettings::default()).unwrap();
    /// # let physical_channel = cluster.create_physical_channel("Channel", FlexrayChannelName::A).unwrap();
    /// flexray_controller.connect_physical_channel("connection", &physical_channel).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model while trying to create the ECU-INSTANCE
    pub fn connect_physical_channel(
        &self,
        connection_name: &str,
        flx_channel: &FlexrayPhysicalChannel,
    ) -> Result<(), AutosarAbstractionError> {
        let ecu = self.0.named_parent()?.unwrap();

        for existing_channel in self.connected_channels() {
            if existing_channel == *flx_channel {
                return Err(AutosarAbstractionError::ItemAlreadyExists);
            }
        }

        // create a new connector
        let connectors = ecu.get_or_create_sub_element(ElementName::Connectors)?;
        let connector =
            connectors.create_named_sub_element(ElementName::FlexrayCommunicationConnector, connection_name)?;
        connector
            .create_sub_element(ElementName::CommControllerRef)
            .and_then(|refelem| refelem.set_reference_target(&self.0))?;

        let channel_connctor_refs = flx_channel
            .element()
            .get_or_create_sub_element(ElementName::CommConnectors)?;
        channel_connctor_refs
            .create_sub_element(ElementName::CommunicationConnectorRefConditional)
            .and_then(|ccrc| ccrc.create_sub_element(ElementName::CommunicationConnectorRef))
            .and_then(|ccr| ccr.set_reference_target(&connector))?;

        Ok(())
    }
}

//##################################################################

#[doc(hidden)]
pub struct FlexrayCtrlChannelsIterator {
    connector_iter: Option<ElementsIterator>,
    comm_controller: Element,
    model: Result<AutosarModel, AutosarDataError>,
}

impl FlexrayCtrlChannelsIterator {
    fn new(controller: &FlexrayCommunicationController, ecu: &Element) -> Self {
        let iter = ecu.get_sub_element(ElementName::Connectors).map(|c| c.sub_elements());
        let comm_controller = controller.element();
        let model = comm_controller.model();
        Self {
            connector_iter: iter,
            comm_controller,
            model,
        }
    }
}

impl Iterator for FlexrayCtrlChannelsIterator {
    type Item = FlexrayPhysicalChannel;

    fn next(&mut self) -> Option<Self::Item> {
        let model = self.model.as_ref().ok()?;
        let connector_iter = self.connector_iter.as_mut()?;
        for connector in connector_iter.by_ref() {
            if connector.element_name() == ElementName::FlexrayCommunicationConnector {
                if let Some(commcontroller_of_connector) = connector
                    .get_sub_element(ElementName::CommControllerRef)
                    .and_then(|ccr| ccr.get_reference_target().ok())
                {
                    if commcontroller_of_connector == self.comm_controller {
                        for ref_origin in model
                            .get_references_to(&connector.path().ok()?)
                            .iter()
                            .filter_map(WeakElement::upgrade)
                            .filter_map(|elem| elem.named_parent().ok().flatten())
                        {
                            // This assumes that each connector will only ever be referenced by at most one
                            // PhysicalChannel, which is true for well-formed files.
                            if ref_origin.element_name() == ElementName::FlexrayPhysicalChannel {
                                return Some(FlexrayPhysicalChannel(ref_origin));
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

//##################################################################

impl FlexrayClusterSettings {
    /// Create a new `FlexrayClusterSettings` object with default values
    #[must_use]
    pub fn new() -> Self {
        Self {
            baudrate: 10_000_000,
            action_point_offset: 4,
            bit: 1e-7,
            cas_rx_low_max: 80,
            cold_start_attempts: 16,
            cycle: 0.005,
            cycle_count_max: 63,
            detect_nit_error: true,
            dynamic_slot_idle_phase: 0,
            ignore_after_tx: 7,
            listen_noise: 2,
            macro_per_cycle: 5000,
            macrotick_duration: 1e-6,
            max_without_clock_correction_fatal: 5,
            max_without_clock_correction_passive: 3,
            minislot_action_point_offset: 3,
            minislot_duration: 10,
            network_idle_time: 46,
            network_management_vector_length: 0,
            number_of_minislots: 185,
            number_of_static_slots: 50,
            offset_correction_start: 4980,
            payload_length_static: 32,
            safety_margin: 1,
            sample_clock_period: Some(1.25e-8),
            static_slot_duration: 62,
            symbol_window: 0,
            symbol_window_action_point_offset: None,
            sync_frame_id_count_max: 8,
            transceiver_standby_delay: None,
            transmission_start_sequence_duration: 9,
            wakeup_rx_idle: 50,
            wakeup_rx_low: 50,
            wakeup_rx_window: 301,
            wakeup_tx_active: 60,
            wakeup_tx_idle: 180,
        }
    }

    /// Check the plausibility of the parameter values
    ///
    /// Returns true if no problem was detected, of false if a problem was found.
    /// The checks performed by this function are not comprehensive. Some problems may remain undetected.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data_abstraction::*;
    /// let mut settings = FlexrayClusterSettings::default();
    /// settings.macro_per_cycle = 1111;
    /// assert!(!settings.verify())
    /// ```
    #[must_use]
    pub fn verify(&self) -> bool {
        // bit time must be the inverse of the baudrate: bit = 1/baudrate
        if 1.0 / (self.baudrate as f64) != self.bit {
            return false;
        }

        // cdCycleMax: 16000µs
        if self.cycle > 0.016 {
            return false;
        }

        // cCycleCountMax: 63
        if self.cycle_count_max != 63 {
            return false;
        }

        // cPayloadLengthMax = 127 two-byte words
        if self.payload_length_static > 127 {
            return false;
        }

        // Duration of a static slot is 4 - 661 MT
        if self.static_slot_duration < 4 || self.static_slot_duration > 661 {
            return false;
        }

        // Duration of a minislot is 2 - 63 MT
        if self.minislot_duration < 2 || self.minislot_duration > 63 {
            return false;
        }

        // The action point offset must be in the range 1 - 63 MT
        if self.action_point_offset < 1
            || self.action_point_offset > 63
            || self.action_point_offset as u16 >= self.static_slot_duration
        {
            return false;
        }

        // minislot action point offset must be in th range 1 - 31 MT
        if self.minislot_action_point_offset < 1 || self.minislot_action_point_offset > 31 {
            return false;
        }

        // the upper limit of the CAS acceptance window is in the range 67 - 99 gdBit
        if self.cas_rx_low_max < 67 || self.cas_rx_low_max > 99 {
            return false;
        }

        // the sample clock period must be one of [0.0125µs, 0.025µs, 0.05µs
        if let Some(sample_clock_period) = self.sample_clock_period {
            if sample_clock_period != 1.25e-8 && sample_clock_period != 2.5e-8 && sample_clock_period != 5e-8 {
                return false;
            }

            if self.bit != (sample_clock_period * 8.0) {
                return false;
            }
        }

        // duration of the symbol window: 0 - 142 MT
        if self.symbol_window > 142 {
            return false;
        }

        // Macroticks per cycle: 10 - 16000 MT
        if self.macro_per_cycle < 10 || self.macro_per_cycle > 16000 {
            return false;
        }

        if self.cycle / (self.macro_per_cycle as f64) != self.macrotick_duration {
            return false;
        }

        // the valid range for the network idle time is 2 - 805MT
        if self.network_idle_time < 2 || self.network_idle_time > 805 {
            return false;
        }

        // idle phase in a dynamic slot is 0 - 2 minislots
        if self.dynamic_slot_idle_phase > 2 {
            return false;
        }

        // number of bits in the transmission start sequence: 3 - 15 bit
        if self.transmission_start_sequence_duration < 3 || self.transmission_start_sequence_duration > 15 {
            return false;
        }

        // cStaticSlotIDMax: 1023; cSlotIDMax: 2047
        if self.number_of_static_slots > 1023 || self.number_of_static_slots + self.number_of_minislots > 2047 {
            return false;
        }

        // check if the configured static and dynamic segments fit into the cycle
        let static_segment_length = self.static_slot_duration as u32 * self.number_of_static_slots as u32;
        let dynamic_segment_length = self.minislot_duration as u32 * self.number_of_minislots as u32;
        if (static_segment_length + dynamic_segment_length + self.network_idle_time as u32)
            > self.macro_per_cycle as u32
        {
            return false;
        }

        // check if the static frame payload fits into the number of macroticks
        // each static frame has a header of 5 bytes + [data] + 3-byte CRC
        let static_frame_size = 5 + 2 * self.payload_length_static + 3;
        let bits_per_macrotick = self.macrotick_duration / self.bit;
        let static_frame_bits =
            (self.static_slot_duration - self.action_point_offset as u16) as f64 * bits_per_macrotick;
        if (static_frame_bits as u32) < (static_frame_size * 8) as u32 {
            return false;
        }

        // offset correction start must fall inside the network idle time
        if (self.offset_correction_start > self.macro_per_cycle)
            || (self.offset_correction_start < self.macro_per_cycle - self.network_idle_time)
        {
            return false;
        }

        // gColdStartAttempts: 2 - 31
        if self.cold_start_attempts < 2 || self.cold_start_attempts > 31 {
            return false;
        }

        // gdWakeupSymbolRxIdle: 14 - 59 gdBit
        if self.wakeup_rx_idle < 14 || self.wakeup_rx_idle > 59 {
            return false;
        }

        // gdWakeupSymbolRxLow 11 - 59 gdBit
        if self.wakeup_rx_low < 11 || self.wakeup_rx_low > 59 {
            return false;
        }

        // gdWakeupSymbolRxWindow: 76 - 301 gdBit
        if self.wakeup_rx_window < 76 || self.wakeup_rx_window > 301 {
            return false;
        }

        // gdWakeupSymbolTxIdle: 45 - 180 gdBit
        if self.wakeup_tx_idle < 45 || self.wakeup_tx_idle > 180 {
            return false;
        }

        // gdWakeupSymbolTxLow: 15 - 60 gdBit
        if self.wakeup_tx_active < 15 || self.wakeup_tx_active > 60 {
            return false;
        }

        // gListenNoise: 2 - 16
        if self.listen_noise < 2 || self.listen_noise > 16 {
            return false;
        }

        // 0 <= gMaxWithoutClockCorrectionPassive <= gMaxWithoutClockCorrectionFatal <= 15
        if (self.max_without_clock_correction_fatal < self.max_without_clock_correction_passive)
            || (self.max_without_clock_correction_fatal > 15)
        {
            return false;
        }

        if self.sync_frame_id_count_max < 2 || self.sync_frame_id_count_max > 15 {
            return false;
        }

        true
    }
}

impl Default for FlexrayClusterSettings {
    fn default() -> Self {
        Self::new()
    }
}

//##################################################################

#[cfg(test)]
mod test {
    use crate::{System, SystemCategory};

    use super::*;
    use autosar_data::AutosarVersion;

    #[test]
    fn cluster() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();

        let pkg2 = ArPackage::get_or_create(&model, "/flexray").unwrap();
        // create the Flexray cluster FlxCluster
        let settings = FlexrayClusterSettings::default();
        let result = system.create_flexray_cluster("FlxCluster", &pkg2, &settings);
        assert!(result.is_ok());
        let cluster = result.unwrap();
        // creating the same cluster again is not possible
        let settings = FlexrayClusterSettings::default();
        let result = system.create_flexray_cluster("FlxCluster", &pkg2, &settings);
        assert!(result.is_err());

        // create channel A
        let result = cluster.create_physical_channel("Channel1", FlexrayChannelName::A);
        assert!(result.is_ok());
        // create channel B
        let result = cluster.create_physical_channel("Channel2", FlexrayChannelName::B);
        assert!(result.is_ok());
        // can't create a third channel
        let result = cluster.create_physical_channel("Channel3", FlexrayChannelName::A);
        assert!(result.is_err());

        let pc = cluster.physical_channels();
        assert!(pc.channel_a.is_some());
        assert!(pc.channel_b.is_some());
    }

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
        let elem_channelname = channel.0.get_sub_element(ElementName::ChannelName).unwrap();
        elem_channelname.remove_character_data().unwrap();
        assert!(channel.channel_name().is_none());

        // now there is no longer a channel A
        let channel2 = cluster.create_physical_channel("channel_name2", FlexrayChannelName::A);
        assert!(channel2.is_ok())
    }

    #[test]
    fn controller() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();
        let ecu = system.create_ecu_instance("ECU", &pkg).unwrap();

        // create a controller
        let result = ecu.create_flexray_communication_controller("Controller");
        let controller = result.unwrap();

        // create some physical channels
        let settings = FlexrayClusterSettings::default();
        let cluster = system.create_flexray_cluster("FlxCluster", &pkg, &settings).unwrap();
        let channel1 = cluster.create_physical_channel("C1", FlexrayChannelName::A).unwrap();

        // connect the controller to channel1
        let result = controller.connect_physical_channel("connection_name1", &channel1);
        assert!(result.is_ok());
        // can't connect to the same channel again
        let result = controller.connect_physical_channel("connection_name2", &channel1);
        assert!(result.is_err());

        let count = controller.connected_channels().count();
        assert_eq!(count, 1);

        // remove the controller and try to list its connected channels again
        let ctrl_parent = controller.0.parent().unwrap().unwrap();
        ctrl_parent.remove_sub_element(controller.0.clone()).unwrap();
        let count = controller.connected_channels().count();
        assert_eq!(count, 0);
    }

    #[test]
    fn flexray_settings() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        let system = System::new("System", &pkg, SystemCategory::SystemDescription).unwrap();
        let settings = FlexrayClusterSettings::default();
        let cluster = system.create_flexray_cluster("FlxCluster", &pkg, &settings).unwrap();

        let mut settings = FlexrayClusterSettings::new();
        assert!(settings.verify());

        // make sure all settings values can be applied and loaded
        settings.sample_clock_period = Some(1.25e-8);
        settings.symbol_window_action_point_offset = Some(settings.action_point_offset);
        settings.transceiver_standby_delay = Some(1.0);
        assert!(settings.verify());
        cluster.update_settings(&settings);
        let settings2 = cluster.get_settings();
        assert_eq!(settings, settings2);

        // test verification of values
        settings.baudrate = 0;
        assert!(!settings.verify());
        settings.baudrate = settings2.baudrate;

        settings.cycle = 0.0161;
        assert!(!settings.verify());
        settings.cycle = settings2.cycle;

        settings.cycle_count_max = 64;
        assert!(!settings.verify());
        settings.cycle_count_max = settings2.cycle_count_max;

        settings.payload_length_static = 128;
        assert!(!settings.verify());
        settings.payload_length_static = settings2.payload_length_static;

        settings.static_slot_duration = 3;
        assert!(!settings.verify());
        settings.static_slot_duration = settings2.static_slot_duration;

        settings.minislot_duration = 64;
        assert!(!settings.verify());
        settings.minislot_duration = settings2.minislot_duration;

        settings.action_point_offset = 64;
        assert!(!settings.verify());
        settings.action_point_offset = settings2.action_point_offset;

        settings.minislot_action_point_offset = 32;
        assert!(!settings.verify());
        settings.minislot_action_point_offset = settings2.minislot_action_point_offset;

        settings.cas_rx_low_max = 66;
        assert!(!settings.verify());
        settings.cas_rx_low_max = settings2.cas_rx_low_max;

        settings.sample_clock_period = Some(1.0);
        assert!(!settings.verify());
        settings.sample_clock_period = settings2.sample_clock_period;

        settings.symbol_window = 143;
        assert!(!settings.verify());
        settings.symbol_window = settings2.symbol_window;

        settings.macro_per_cycle = 9;
        assert!(!settings.verify());
        settings.macro_per_cycle = settings2.macro_per_cycle;

        settings.macro_per_cycle -= 1;
        assert!(!settings.verify());
        settings.macro_per_cycle = settings2.macro_per_cycle;

        settings.network_idle_time = 806;
        assert!(!settings.verify());
        settings.network_idle_time = settings2.network_idle_time;

        settings.dynamic_slot_idle_phase = 3;
        assert!(!settings.verify());
        settings.dynamic_slot_idle_phase = settings2.dynamic_slot_idle_phase;

        settings.transmission_start_sequence_duration = 16;
        assert!(!settings.verify());
        settings.transmission_start_sequence_duration = settings2.transmission_start_sequence_duration;

        settings.number_of_static_slots = 1024;
        assert!(!settings.verify());
        settings.number_of_static_slots = settings2.number_of_static_slots;

        settings.number_of_static_slots += 1;
        assert!(!settings.verify());
        settings.number_of_static_slots = settings2.number_of_static_slots;

        settings.payload_length_static += 1;
        assert!(!settings.verify());
        settings.payload_length_static = settings2.payload_length_static;

        settings.offset_correction_start = settings.macro_per_cycle + 1;
        assert!(!settings.verify());
        settings.offset_correction_start = settings2.offset_correction_start;

        settings.cold_start_attempts = 1;
        assert!(!settings.verify());
        settings.cold_start_attempts = settings2.cold_start_attempts;

        settings.wakeup_rx_idle = 13;
        assert!(!settings.verify());
        settings.wakeup_rx_idle = settings2.wakeup_rx_idle;

        settings.wakeup_rx_low = 10;
        assert!(!settings.verify());
        settings.wakeup_rx_low = settings2.wakeup_rx_low;

        settings.wakeup_rx_window = 75;
        assert!(!settings.verify());
        settings.wakeup_rx_window = settings2.wakeup_rx_window;

        settings.wakeup_tx_idle = 44;
        assert!(!settings.verify());
        settings.wakeup_tx_idle = settings2.wakeup_tx_idle;

        settings.wakeup_tx_active = 14;
        assert!(!settings.verify());
        settings.wakeup_tx_active = settings2.wakeup_tx_active;

        settings.listen_noise = 1;
        assert!(!settings.verify());
        settings.listen_noise = settings2.listen_noise;

        settings.max_without_clock_correction_passive = settings.max_without_clock_correction_fatal + 1;
        assert!(!settings.verify());
        settings.max_without_clock_correction_passive = settings2.max_without_clock_correction_passive;

        settings.sync_frame_id_count_max = 1;
        assert!(!settings.verify());
        settings.sync_frame_id_count_max = settings2.sync_frame_id_count_max;
    }
}
