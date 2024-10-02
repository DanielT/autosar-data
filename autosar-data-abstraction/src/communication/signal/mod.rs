use crate::communication::{CommunicationDirection, PhysicalChannel};
use crate::datatype::{CompuMethod, DataConstr, SwBaseType, Unit};
use crate::{
    abstraction_element, communication::ISignalToIPduMapping, element_iterator, make_unique_name, reflist_iterator,
    AbstractionElement, ArPackage, AutosarAbstractionError, EcuInstance,
};
use autosar_data::{AutosarDataError, Element, ElementName, EnumItem, WeakElement};

/// Signal of the Interaction Layer
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISignal(Element);
abstraction_element!(ISignal, ISignal);

impl ISignal {
    pub(crate) fn new(
        name: &str,
        bit_length: u64,
        syssignal: &SystemSignal,
        datatype: Option<&SwBaseType>,
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

        let isignal = Self(elem_isignal);

        if let Some(datatype) = datatype {
            isignal.set_datatype(datatype)?;
        }

        Ok(isignal)
    }

    /// set the data type for this signal
    pub fn set_datatype(&self, datatype: &SwBaseType) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::NetworkRepresentationProps)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_or_create_sub_element(ElementName::BaseTypeRef)?
            .set_reference_target(datatype.element())?;
        Ok(())
    }

    pub fn set_transformation(&self) -> Result<(), AutosarAbstractionError> {
        todo!()
    }

    /// get the length of this signal in bits
    pub fn length(&self) -> Option<u64> {
        self.element()
            .get_sub_element(ElementName::Length)?
            .character_data()?
            .parse_integer()
    }

    /// get the system signal that corresponds to this isignal
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
    pub fn mappings(&self) -> impl Iterator<Item = ISignalToIPduMapping> {
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

/// The system signal represents the communication system's view of data exchanged between SW components which reside on different ECUs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemSignal(Element);
abstraction_element!(SystemSignal, SystemSignal);

impl SystemSignal {
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let package_elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let elem_syssignal = package_elements.create_named_sub_element(ElementName::SystemSignal, name)?;

        Ok(Self(elem_syssignal))
    }

    /// get the signal group that contains this signal
    pub fn signal_group(&self) -> Option<SystemSignalGroup> {
        let path = self.element().path().ok()?;
        let referrers = self.element().model().ok()?.get_references_to(&path);
        for elem in referrers
            .iter()
            .filter_map(WeakElement::upgrade)
            .filter_map(|refelem| refelem.named_parent().ok().flatten())
        {
            if let Ok(grp) = SystemSignalGroup::try_from(elem) {
                return Some(grp);
            }
        }
        None
    }

    /// set the unit for this signal
    pub fn set_unit(&self, unit: &Unit) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::PhysicalProps)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_or_create_sub_element(ElementName::UnitRef)?
            .set_reference_target(unit.element())?;
        Ok(())
    }

    /// get the unit for this signal
    pub fn unit(&self) -> Option<Unit> {
        self.element()
            .get_sub_element(ElementName::PhysicalProps)?
            .get_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_sub_element(ElementName::UnitRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }

    /// set the compu method for this signal
    pub fn set_compu_method(&self, compu_method: &CompuMethod) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::PhysicalProps)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_or_create_sub_element(ElementName::CompuMethodRef)?
            .set_reference_target(compu_method.element())?;
        Ok(())
    }

    /// get the compu method for this signal
    pub fn compu_method(&self) -> Option<CompuMethod> {
        self.element()
            .get_sub_element(ElementName::PhysicalProps)?
            .get_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_sub_element(ElementName::CompuMethodRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }

    /// set the data constraint for this signal
    pub fn set_data_constr(&self, data_constr: &DataConstr) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::PhysicalProps)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_or_create_sub_element(ElementName::DataConstrRef)?
            .set_reference_target(data_constr.element())?;
        Ok(())
    }

    /// get the data constraint for this signal
    pub fn data_constr(&self) -> Option<DataConstr> {
        self.element()
            .get_sub_element(ElementName::PhysicalProps)?
            .get_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_sub_element(ElementName::DataConstrRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }
}

//##################################################################

/// An ISignalGroup groups signals that should always be kept together
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
            return Err(AutosarAbstractionError::InvalidParameter(
                "The isignal and the system signal must both be part of corresponding signal groups".to_string(),
            ));
        }

        let isrefs = self.element().get_or_create_sub_element(ElementName::ISignalRefs)?;

        // check if the signal already exists in isrefs?

        isrefs
            .create_sub_element(ElementName::ISignalRef)?
            .set_reference_target(signal.element())?;

        Ok(())
    }

    /// get the system signal group that is associated with this signal group
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
    pub fn signals(&self) -> impl Iterator<Item = ISignal> {
        ISignalsIterator::new(self.element().get_sub_element(ElementName::ISignalRefs))
    }
}

//##################################################################

/// A signal group refers to a set of signals that shall always be kept together. A signal group is used to
/// guarantee the atomic transfer of AUTOSAR composite data types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemSignalGroup(Element);
abstraction_element!(SystemSignalGroup, SystemSignalGroup);

impl SystemSignalGroup {
    /// Create a new system signal group
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

    /// connect this signal triggering to an ECU
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

    /// create an iterator over all signal ports that are connected to this signal triggering
    pub fn signal_ports(&self) -> impl Iterator<Item = ISignalPort> {
        ISignalPortIterator::new(self.element().get_sub_element(ElementName::ISignalPortRefs))
    }
}

//##################################################################

/// The IPduPort allows an ECU to send or receive a PDU
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISignalPort(Element);
abstraction_element!(ISignalPort, ISignalPort);

impl ISignalPort {
    /// get the ECU that is connected to this signal port
    pub fn ecu(&self) -> Option<EcuInstance> {
        let comm_connector_elem = self.element().named_parent().ok()??;
        let ecu_elem = comm_connector_elem.named_parent().ok()??;
        EcuInstance::try_from(ecu_elem).ok()
    }

    /// get the communication direction of this port
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

//##################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datatype::{BaseTypeEncoding, CompuMethodContent, SwBaseType, Unit};
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn test_signal() {
        let model = AutosarModel::new();
        let _file = model.create_file("test.arxml", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/test").unwrap();
        let unit = Unit::new("unit", &package, Some("Unit Name")).unwrap();
        let compu_method = CompuMethod::new("compu_method", &package, CompuMethodContent::Identical).unwrap();
        let data_constr = DataConstr::new("data_constr", &package).unwrap();
        let sw_base_type =
            SwBaseType::new("sw_base_type", &package, 8, BaseTypeEncoding::None, None, None, None).unwrap();

        let sys_signal = SystemSignal::new("sys_signal", &package).unwrap();
        let signal = ISignal::new("signal", 8, &sys_signal, Some(&sw_base_type), &package).unwrap();

        sys_signal.set_unit(&unit).unwrap();
        sys_signal.set_compu_method(&compu_method).unwrap();
        sys_signal.set_data_constr(&data_constr).unwrap();

        assert_eq!(signal.length(), Some(8));
        assert_eq!(signal.system_signal(), Some(sys_signal.clone()));
        assert_eq!(sys_signal.unit(), Some(unit));
        assert_eq!(sys_signal.compu_method(), Some(compu_method));
        assert_eq!(sys_signal.data_constr(), Some(data_constr));
    }

    #[test]
    fn test_signal_group() {
        let model = AutosarModel::new();
        let _file = model.create_file("test.arxml", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/test").unwrap();
        let sys_signal_group = SystemSignalGroup::new("sys_signal_group", &package).unwrap();
        let signal_group = ISignalGroup::new("signal_group", &sys_signal_group, &package).unwrap();
        assert_eq!(signal_group.system_signal_group(), Some(sys_signal_group.clone()));

        let sys_signal = SystemSignal::new("sys_signal", &package).unwrap();
        let signal = ISignal::new("signal", 8, &sys_signal, None, &package).unwrap();
        assert_eq!(signal.system_signal(), Some(sys_signal.clone()));

        sys_signal_group.add_signal(&sys_signal).unwrap();
        assert_eq!(sys_signal.signal_group(), Some(sys_signal_group.clone()));

        signal_group.add_signal(&signal).unwrap();
        assert_eq!(signal_group.signals().count(), 1);
    }
}
