use crate::communication::{CommunicationDirection, PhysicalChannel};
use crate::{
    abstraction_element, communication::ISignalToIPduMapping, element_iterator, make_unique_name, reflist_iterator,
    AbstractionElement, ArPackage, AutosarAbstractionError, EcuInstance,
};
use autosar_data::{AutosarDataError, Element, ElementName, EnumItem, WeakElement};

///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISignal(Element);
abstraction_element!(ISignal, ISignal);

impl ISignal {
    pub(crate) fn new(
        name: &str,
        bit_length: u64,
        syssignal: &SystemSignal,
        package: &ArPackage,
    ) -> Result<Self, AutosarAbstractionError> {
        if bit_length > (u32::MAX as u64) * 8 {
            // max bit_length is 2^32 bytes
            return Err(AutosarAbstractionError::InvalidParameter(format!(
                "isignal {name}: bit length {bit_length} is too big"
            )));
        }

        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_isignal = pkg_elements.create_named_sub_element(ElementName::ISignal, name)?;

        elem_isignal
            .create_sub_element(ElementName::Length)?
            .set_character_data(bit_length.to_string())?;
        elem_isignal
            .create_sub_element(ElementName::SystemSignalRef)?
            .set_reference_target(syssignal.element())?;
        elem_isignal
            .create_sub_element(ElementName::DataTypePolicy)?
            .set_character_data(EnumItem::Override)?;

        Ok(Self(elem_isignal))
    }

    pub fn set_datatype(&self, _datatype: ()) -> Result<(), AutosarAbstractionError> {
        todo!()
    }

    pub fn set_transformation(&self) -> Result<(), AutosarAbstractionError> {
        todo!()
    }

    pub fn length(&self) -> Option<u64> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    pub fn system_signal(&self) -> Option<SystemSignal> {
        self.element()
            .get_sub_element(ElementName::SystemSignalRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }

    /// an iterator over all ISignalToIPduMapping for this signal
    ///
    /// Usually a signal should only be mapped to a single PDU,
    /// so this iterator is expected to return either zero or one item in ordinary cases.
    pub fn mappings(&self) -> ISignalToIPduMappingsIterator {
        let model_result = self.element().model();
        let path_result = self.element().path();
        if let (Ok(model), Ok(path)) = (model_result, path_result) {
            let reflist = model.get_references_to(&path);
            ISignalToIPduMappingsIterator::new(reflist)
        } else {
            ISignalToIPduMappingsIterator::new(vec![])
        }
    }

    pub fn signal_group(&self) -> Option<ISignalGroup> {
        let path = self.element().path().ok()?;
        let referrers = self.element().model().ok()?.get_references_to(&path);
        for elem in referrers.iter().filter_map(WeakElement::upgrade) {
            if let Ok(grp) = ISignalGroup::try_from(elem) {
                return Some(grp);
            }
        }
        None
    }
}

//##################################################################

///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemSignal(Element);
abstraction_element!(SystemSignal, SystemSignal);

impl SystemSignal {
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let package_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_syssignal = package_elements.create_named_sub_element(ElementName::SystemSignal, name)?;

        Ok(Self(elem_syssignal))
    }

    pub fn signal_group(&self) -> Option<SystemSignalGroup> {
        let path = self.element().path().ok()?;
        let referrers = self.element().model().ok()?.get_references_to(&path);
        for elem in referrers.iter().filter_map(WeakElement::upgrade) {
            if let Ok(grp) = SystemSignalGroup::try_from(elem) {
                return Some(grp);
            }
        }
        None
    }
}

//##################################################################

///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISignalGroup(Element);
abstraction_element!(ISignalGroup, ISignalGroup);

impl ISignalGroup {
    pub(crate) fn new(
        name: &str,
        system_signal_group: &SystemSignalGroup,
        package: &ArPackage,
    ) -> Result<Self, AutosarAbstractionError> {
        let sig_pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_isiggrp = sig_pkg_elements.create_named_sub_element(ElementName::ISignalGroup, name)?;

        elem_isiggrp
            .create_sub_element(ElementName::SystemSignalGroupRef)?
            .set_reference_target(system_signal_group.element())?;

        Ok(Self(elem_isiggrp))
    }

    /// Add a signal to the signal group
    pub fn add_signal(&self, signal: &ISignal) -> Result<(), AutosarAbstractionError> {
        // make sure the relation of signal to signal group is maintained for the referenced system signal
        let syssig_grp_of_signal = signal.system_signal().and_then(|ss| ss.signal_group());
        let syssig_grp = self.system_signal_group();
        if syssig_grp != syssig_grp_of_signal {
            return Err(AutosarAbstractionError::InvalidParameter("".to_string()));
        }

        let isrefs = self.element().get_or_create_sub_element(ElementName::ISignalRefs)?;

        // check if the signal already exists in isrefs?

        isrefs
            .create_sub_element(ElementName::ISignalRef)?
            .set_reference_target(signal.element())?;

        Ok(())
    }

    pub fn system_signal_group(&self) -> Option<SystemSignalGroup> {
        self.element()
            .get_sub_element(ElementName::SystemSignalGroupRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }

    /// Iterator over all [`Signal`]s in this group
    ///
    /// # Example
    pub fn signals(&self) -> ISignalsIterator {
        ISignalsIterator::new(self.element().get_sub_element(ElementName::Signals))
    }
}

//##################################################################

///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemSignalGroup(Element);
abstraction_element!(SystemSignalGroup, SystemSignalGroup);

impl SystemSignalGroup {
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let pkg_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let signalgroup = pkg_elements.create_named_sub_element(ElementName::SystemSignalGroup, name)?;

        Ok(Self(signalgroup))
    }

    /// Add a signal to the signal group
    pub fn add_signal(&self, signal: &SystemSignal) -> Result<(), AutosarAbstractionError> {
        let ssrefs = self
            .element()
            .get_or_create_sub_element(ElementName::SystemSignalRefs)?;

        // check if the signal already exists in ssrefs?

        ssrefs
            .create_sub_element(ElementName::SystemSignalRef)?
            .set_reference_target(signal.element())?;

        Ok(())
    }
}

//##################################################################

/// an ISignalTriggering triggers a signal in a PDU
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISignalTriggering(Element);
abstraction_element!(ISignalTriggering, ISignalTriggering);

impl ISignalTriggering {
    pub(crate) fn new(signal: &ISignal, channel: &PhysicalChannel) -> Result<Self, AutosarAbstractionError> {
        let model = channel.element().model()?;
        let base_path = channel.element().path()?;
        let signal_name = signal
            .name()
            .ok_or(AutosarAbstractionError::InvalidParameter("invalid signal".to_string()))?;
        let pt_name = format!("ST_{signal_name}");
        let pt_name = make_unique_name(&model, base_path, pt_name);

        let triggerings = channel
            .element()
            .get_or_create_sub_element(ElementName::ISignalTriggerings)?;
        let st_elem = triggerings.create_named_sub_element(ElementName::ISignalTriggering, &pt_name)?;
        st_elem
            .create_sub_element(ElementName::ISignalRef)?
            .set_reference_target(signal.element())?;

        let pt = Self(st_elem);

        Ok(pt)
    }

    pub(crate) fn new_group(
        signal_group: &ISignalGroup,
        channel: &PhysicalChannel,
    ) -> Result<Self, AutosarAbstractionError> {
        let model = channel.element().model()?;
        let base_path = channel.element().path()?;
        let signal_name = signal_group.name().ok_or(AutosarAbstractionError::InvalidParameter(
            "invalid signal group".to_string(),
        ))?;
        let pt_name = format!("ST_{signal_name}");
        let pt_name = make_unique_name(&model, base_path, pt_name);

        let triggerings = channel
            .element()
            .get_or_create_sub_element(ElementName::ISignalTriggerings)?;
        let st_elem = triggerings.create_named_sub_element(ElementName::ISignalTriggering, &pt_name)?;
        st_elem
            .create_sub_element(ElementName::ISignalGroupRef)?
            .set_reference_target(signal_group.element())?;

        let pt = Self(st_elem);

        Ok(pt)
    }

    /// get the physical channel that contains this signal triggering
    pub fn physical_channel(&self) -> Result<PhysicalChannel, AutosarAbstractionError> {
        let channel_elem = self.element().named_parent()?.ok_or(AutosarDataError::ItemDeleted)?;
        PhysicalChannel::try_from(channel_elem)
    }

    pub fn connect_to_ecu(
        &self,
        ecu: &EcuInstance,
        direction: CommunicationDirection,
    ) -> Result<ISignalPort, AutosarAbstractionError> {
        for signal_port in self.signal_ports() {
            if let (Some(existing_ecu), Some(existing_direction)) =
                (signal_port.ecu(), signal_port.communication_direction())
            {
                if existing_ecu == *ecu && existing_direction == direction {
                    return Ok(signal_port);
                }
            }
        }

        let channel = self.physical_channel()?;
        let connector = channel
            .get_ecu_connector(ecu)
            .ok_or(AutosarAbstractionError::InvalidParameter(
                "The ECU is not connected to the channel".to_string(),
            ))?;

        let name = self.name().ok_or(AutosarDataError::ItemDeleted)?;
        let suffix = match direction {
            CommunicationDirection::In => "Rx",
            CommunicationDirection::Out => "Tx",
        };
        let port_name = format!("{name}_{suffix}",);
        let sp_elem = connector
            .get_or_create_sub_element(ElementName::EcuCommPortInstances)?
            .create_named_sub_element(ElementName::ISignalPort, &port_name)?;
        sp_elem
            .create_sub_element(ElementName::CommunicationDirection)?
            .set_character_data::<EnumItem>(direction.into())?;

        self.element()
            .get_or_create_sub_element(ElementName::ISignalPortRefs)?
            .create_sub_element(ElementName::ISignalPortRef)?
            .set_reference_target(&sp_elem)?;

        Ok(ISignalPort(sp_elem))
    }

    pub fn signal_ports(&self) -> ISignalPortIterator {
        ISignalPortIterator::new(self.element().get_sub_element(ElementName::ISignalPortRefs))
    }
}

//##################################################################

/// The IPduPort allows an ECU to send or receive a PDU
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISignalPort(Element);
abstraction_element!(ISignalPort, ISignalPort);

impl ISignalPort {
    pub fn ecu(&self) -> Option<EcuInstance> {
        let comm_connector_elem = self.element().named_parent().ok()??;
        let ecu_elem = comm_connector_elem.named_parent().ok()??;
        EcuInstance::try_from(ecu_elem).ok()
    }

    pub fn communication_direction(&self) -> Option<CommunicationDirection> {
        self.element()
            .get_sub_element(ElementName::CommunicationDirection)?
            .character_data()?
            .enum_value()?
            .try_into()
            .ok()
    }
}

//##################################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferProperty {
    Pending,
    Triggered,
    TriggeredOnChange,
    TriggeredOnChangeWithoutRepetition,
    TriggeredWithoutRepetition,
}

impl From<TransferProperty> for EnumItem {
    fn from(value: TransferProperty) -> Self {
        match value {
            TransferProperty::Pending => EnumItem::Pending,
            TransferProperty::Triggered => EnumItem::Triggered,
            TransferProperty::TriggeredOnChange => EnumItem::TriggeredOnChange,
            TransferProperty::TriggeredOnChangeWithoutRepetition => EnumItem::TriggeredOnChangeWithoutRepetition,
            TransferProperty::TriggeredWithoutRepetition => EnumItem::TriggeredWithoutRepetition,
        }
    }
}

impl TryFrom<EnumItem> for TransferProperty {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<Self, Self::Error> {
        match value {
            EnumItem::Pending => Ok(TransferProperty::Pending),
            EnumItem::Triggered => Ok(TransferProperty::Triggered),
            EnumItem::TriggeredOnChange => Ok(TransferProperty::TriggeredOnChange),
            EnumItem::TriggeredOnChangeWithoutRepetition => Ok(TransferProperty::TriggeredOnChangeWithoutRepetition),
            EnumItem::TriggeredWithoutRepetition => Ok(TransferProperty::TriggeredWithoutRepetition),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "TransferProperty".to_string(),
            }),
        }
    }
}

//##################################################################

element_iterator!(
    ISignalPortIterator,
    ISignalPort,
    (|element: Element| element.get_reference_target().ok())
);

//##################################################################

element_iterator!(
    ISignalsIterator,
    ISignal,
    (|element: Element| element.get_reference_target().ok())
);

//##################################################################

reflist_iterator!(ISignalToIPduMappingsIterator, ISignalToIPduMapping);
