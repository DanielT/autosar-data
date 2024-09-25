use crate::*;
use autosar_data::ElementName;
use communication::SystemSignal;
use software_component::{
    AbstractSwComponentType, ComponentPrototype, PortInterface, PortPrototype, RootSwCompositionPrototype,
    SwComponentPrototype, VariableDataPrototype,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemMapping(Element);
abstraction_element!(SystemMapping, SystemMapping);

impl SystemMapping {
    pub(crate) fn new(name: &str, system: &System) -> Result<Self, AutosarAbstractionError> {
        let element = system
            .element()
            .get_or_create_sub_element(ElementName::Mappings)?
            .create_named_sub_element(ElementName::SystemMapping, name)?;

        Ok(Self(element))
    }

    /// get the system that contains this mapping
    pub fn system(&self) -> Result<System, AutosarAbstractionError> {
        let sys_elem = self.element().named_parent()?.unwrap();
        System::try_from(sys_elem)
    }

    /// create a new mapping between a SWC and an ECU
    pub fn map_swc_to_ecu(
        &self,
        name: &str,
        component_prototype: &SwComponentPrototype,
        ecu: &EcuInstance,
    ) -> Result<SwcToEcuMapping, AutosarAbstractionError> {
        let root_composition_prototype =
            self.system()?
                .root_composition()
                .ok_or(AutosarAbstractionError::InvalidParameter(
                    "The root compositon must be set before mapping any swc".to_string(),
                ))?;
        let root_composition_type =
            root_composition_prototype
                .composition()
                .ok_or(AutosarAbstractionError::InvalidParameter(
                    "Incomplete root composition prototype".to_string(),
                ))?;

        let mut context_composition_prototypes = vec![];
        let mut current_composition = component_prototype.parent_composition()?;

        // check if the composition is a child of the root composition; this is needed to ensure that the loop can terminate
        if root_composition_type != current_composition && !root_composition_type.is_parent_of(&current_composition) {
            return Err(AutosarAbstractionError::InvalidParameter(
                "The composition is not a child of the root composition".to_string(),
            ));
        }

        // find all compositions between the root composition and the current composition
        while current_composition != root_composition_type {
            // typical case is that each component is only in one composition, so the for loop should only run once
            for comp_proto in current_composition.instances() {
                // this condition should never fail - it only returns none if comp_proto is the root
                // composition, which we already know is not true
                if let Ok(Some(comp_type)) = comp_proto.parent_composition() {
                    if root_composition_type == comp_type || root_composition_type.is_parent_of(&comp_type) {
                        context_composition_prototypes.push(comp_proto.clone());
                        current_composition = comp_type;
                        break;
                    }
                }
            }
        }

        // the items were collected in reverse order, so we need to reverse them again
        context_composition_prototypes.reverse();

        SwcToEcuMapping::new(
            name,
            component_prototype,
            &context_composition_prototypes,
            &root_composition_prototype,
            ecu,
            self,
        )
    }

    /// create a new mapping between a sender/receiver port and a signal
    ///
    /// `signal`: the system signal that the port is mapped to
    ///
    /// `data_element`: the data element that is mapped to the signal
    ///
    /// `port_prototype`: the port prototype that contains the data element
    ///
    /// `context_components`: a list of component prototypes from the root up to the component that directly contains the port.
    /// This list may be empty, or it could only contain the final application component prototype containing the port.
    ///
    /// `root_composition_prototype`: the root composition prototype that contains the swc_prototype.
    /// Rarely required, but may be needed if multiple root compositions use the same composition/component hierarchy.
    pub fn map_sender_receiver_to_signal(
        &self,
        signal: &SystemSignal,
        data_element: &VariableDataPrototype,
        port_prototype: &PortPrototype,
        context_components: &[&SwComponentPrototype],
        root_composition_prototype: Option<&RootSwCompositionPrototype>,
    ) -> Result<(), AutosarAbstractionError> {
        // sanity checks
        // the port must be a sender/receiver port
        let PortInterface::SenderReceiverInterface(interface) = port_prototype.port_interface()? else {
            return Err(AutosarAbstractionError::InvalidParameter(
                "The port prototype must be a sender/receiver port".to_string(),
            ));
        };

        // the data element must be part of the sender/receiver interface
        if &data_element.interface()? != &interface {
            println!("{:?}", &data_element.interface()?);
            println!("{:?}", &interface);
            return Err(AutosarAbstractionError::InvalidParameter(
                "The data element must be part of the sender/receiver interface".to_string(),
            ));
        }

        // the last context component in the list contains the port prototype
        if let Some(swc_prototype) = context_components.last() {
            let swc_type = port_prototype.component_type()?;
            let swc_prototype_type =
                swc_prototype
                    .component_type()
                    .ok_or(AutosarAbstractionError::InvalidParameter(
                        "invalid SWC prototype: component type ref is missing".to_string(),
                    ))?;
            if swc_type != swc_prototype_type {
                return Err(AutosarAbstractionError::InvalidParameter(
                    "The port must be part of the component prototype".to_string(),
                ));
            }
        }

        // create the mapping
        let data_mappings = self.element().get_or_create_sub_element(ElementName::DataMappings)?;
        let sr_mapping = data_mappings.create_sub_element(ElementName::SenderReceiverToSignalMapping)?;

        let iref = sr_mapping.create_sub_element(ElementName::DataElementIref)?;
        iref.create_sub_element(ElementName::ContextPortRef)?
            .set_reference_target(port_prototype.element())?;
        iref.create_sub_element(ElementName::TargetDataPrototypeRef)?
            .set_reference_target(data_element.element())?;

        // the list of context components is ordered, with the root composition prototype at the beginning
        for comp_proto in context_components {
            iref.create_sub_element(ElementName::ContextComponentRef)?
                .set_reference_target(comp_proto.element())?;
        }

        if let Some(root_composition_prototype) = root_composition_prototype {
            iref.create_sub_element(ElementName::ContextCompositionRef)?
                .set_reference_target(root_composition_prototype.element())?;
        }

        sr_mapping
            .create_sub_element(ElementName::SystemSignalRef)?
            .set_reference_target(signal.element())?;

        Ok(())
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwcToEcuMapping(Element);
abstraction_element!(SwcToEcuMapping, SwcToEcuMapping);

impl SwcToEcuMapping {
    pub(crate) fn new(
        name: &str,
        component_prototype: &SwComponentPrototype,
        context_composition_prototypes: &[ComponentPrototype],
        root_composition_prototype: &RootSwCompositionPrototype,
        ecu: &EcuInstance,
        mapping: &SystemMapping,
    ) -> Result<Self, AutosarAbstractionError> {
        let sw_mappings_elem = mapping.element().get_or_create_sub_element(ElementName::SwMappings)?;
        let swc_to_ecu_mapping = sw_mappings_elem.create_named_sub_element(ElementName::SwcToEcuMapping, name)?;

        let iref = swc_to_ecu_mapping
            .create_sub_element(ElementName::ComponentIrefs)?
            .create_sub_element(ElementName::ComponentIref)?;

        // create the references to root composition and context compositions
        iref.create_sub_element(ElementName::ContextCompositionRef)?
            .set_reference_target(root_composition_prototype.element())?;
        for context_comp in context_composition_prototypes {
            iref.create_sub_element(ElementName::ContextComponentRef)?
                .set_reference_target(context_comp.element())?;
        }
        // create the reference to the target component prototype
        iref.create_sub_element(ElementName::TargetComponentRef)?
            .set_reference_target(component_prototype.element())?;

        swc_to_ecu_mapping
            .create_sub_element(ElementName::EcuInstanceRef)?
            .set_reference_target(ecu.element())?;

        Ok(Self(swc_to_ecu_mapping))
    }

    /// get the component prototype that is mapped here
    pub fn target_component(&self) -> Option<SwComponentPrototype> {
        self.element()
            .get_sub_element(ElementName::ComponentIrefs)
            .and_then(|irefs| irefs.get_sub_element(ElementName::ComponentIref))
            .and_then(|iref| iref.get_sub_element(ElementName::TargetComponentRef))
            .and_then(|target| target.get_reference_target().ok())
            .and_then(|target| SwComponentPrototype::try_from(target).ok())
    }

    /// get the ECU instance which is the target of this mapping
    pub fn ecu_instance(&self) -> Option<EcuInstance> {
        self.element()
            .get_sub_element(ElementName::EcuInstanceRef)
            .and_then(|r| r.get_reference_target().ok())
            .and_then(|target| EcuInstance::try_from(target).ok())
    }
}
