use crate::communication::{SoAdRoutingGroup, SocketAddress};
use crate::{AbstractionElement, AutosarAbstractionError};
use autosar_data::{Element, ElementName};

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProvidedServiceInstanceV1(Element);

impl AbstractionElement for ProvidedServiceInstanceV1 {
    fn element(&self) -> &Element {
        &self.0
    }
}

impl TryFrom<Element> for ProvidedServiceInstanceV1 {
    type Error = AutosarAbstractionError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        // hierarchy: ApplicationEndpoint -> ProvidedServiceInstances -> ProvidedServiceInstance
        let parent_name = element.parent()?.map(|p| p.element_name());
        if !matches!(parent_name, Some(ElementName::ProvidedServiceInstances)) {
            return Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "ProvidedServiceInstanceV1".to_string(),
            });
        }

        if element.element_name() == ElementName::ProvidedServiceInstance {
            Ok(Self(element))
        } else {
            Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "ProvidedServiceInstanceV1".to_string(),
            })
        }
    }
}

impl ProvidedServiceInstanceV1 {
    pub(crate) fn new(
        name: &str,
        parent: &Element,
        service_identifier: u16,
        instance_identifier: u16,
    ) -> Result<Self, AutosarAbstractionError> {
        let elem = parent.create_named_sub_element(ElementName::ProvidedServiceInstance, name)?;

        elem.create_sub_element(ElementName::ServiceIdentifier)?
            .set_character_data(service_identifier as u64)?;
        elem.create_sub_element(ElementName::InstanceIdentifier)?
            .set_character_data(instance_identifier as u64)?;

        Ok(Self(elem))
    }

    /// get the service identifier of this `ProvidedServiceInstance`
    pub fn service_identifier(&self) -> Option<u32> {
        self.0
            .get_sub_element(ElementName::ServiceIdentifier)
            .and_then(|si| si.character_data())
            .and_then(|cdata| cdata.parse_integer())
    }

    /// get the instance identifier of this `ProvidedServiceInstance`
    pub fn instance_identifier(&self) -> Option<u32> {
        self.0
            .get_sub_element(ElementName::InstanceIdentifier)
            .and_then(|ii| ii.character_data())
            .and_then(|cdata| cdata.parse_integer())
    }

    /// create a new `EventHandlerV1` in this `ProvidedServiceInstance`
    pub fn create_event_handler(&self, name: &str) -> Result<EventHandlerV1, AutosarAbstractionError> {
        let ehs = self.element().get_or_create_sub_element(ElementName::EventHandlers)?;
        EventHandlerV1::new(name, &ehs)
    }

    /// get the `EventHandlerV1`s in this `ProvidedServiceInstance`
    pub fn event_handlers(&self) -> impl Iterator<Item = EventHandlerV1> {
        self.element()
            .get_sub_element(ElementName::EventHandlers)
            .into_iter()
            .flat_map(|ehs| ehs.sub_elements())
            .filter_map(|eh| EventHandlerV1::try_from(eh).ok())
    }

    /// set the SD server configuration for this `ProvidedServiceInstance`
    pub fn set_sd_server_config(&self, sd_server_config: &SdConfig) -> Result<(), AutosarAbstractionError> {
        // remove any existing SdServerConfig, so that we can start fresh
        if let Some(config_elem) = self.element().get_sub_element(ElementName::SdServerConfig) {
            self.element().remove_sub_element(config_elem)?;
        }

        let config_elem = self.element().get_or_create_sub_element(ElementName::SdServerConfig)?;

        config_elem
            .create_sub_element(ElementName::ServerServiceMajorVersion)?
            .set_character_data(sd_server_config.service_major_version as u64)?;
        config_elem
            .create_sub_element(ElementName::ServerServiceMinorVersion)?
            .set_character_data(sd_server_config.service_minor_version as u64)?;
        config_elem
            .create_sub_element(ElementName::Ttl)?
            .set_character_data(sd_server_config.ttl as u64)?;
        if let Some(offer_cyclic_delay) = sd_server_config.offer_cyclic_delay {
            config_elem
                .create_sub_element(ElementName::OfferCyclicDelay)?
                .set_character_data(offer_cyclic_delay)?;
        }

        let initial_offer_elem = config_elem.create_sub_element(ElementName::InitialOfferBehavior)?;
        initial_offer_elem
            .create_sub_element(ElementName::InitialDelayMaxValue)?
            .set_character_data(sd_server_config.initial_delay_max_value)?;
        initial_offer_elem
            .create_sub_element(ElementName::InitialDelayMinValue)?
            .set_character_data(sd_server_config.initial_delay_min_value)?;
        initial_offer_elem
            .create_sub_element(ElementName::InitialRepetitionsMax)?
            .set_character_data(sd_server_config.initial_repetitions_max as u64)?;
        if let Some(initial_repetitions_base_delay) = sd_server_config.initial_repetitions_base_delay {
            initial_offer_elem
                .create_sub_element(ElementName::InitialRepetitionsBaseDelay)?
                .set_character_data(initial_repetitions_base_delay)?;
        }

        let req_resp_delay_elem = config_elem.create_sub_element(ElementName::RequestResponseDelay)?;
        req_resp_delay_elem
            .create_sub_element(ElementName::MaxValue)?
            .set_character_data(sd_server_config.request_response_delay_max_value)?;
        req_resp_delay_elem
            .create_sub_element(ElementName::MinValue)?
            .set_character_data(sd_server_config.request_response_delay_min_value)?;

        Ok(())
    }

    /// get the SD server configuration for this `ProvidedServiceInstance`
    pub fn sd_server_config(&self) -> Option<SdConfig> {
        let config_elem = self.element().get_sub_element(ElementName::SdServerConfig)?;
        let service_major_version = config_elem
            .get_sub_element(ElementName::ServerServiceMajorVersion)?
            .character_data()?
            .parse_integer()?;
        let service_minor_version = config_elem
            .get_sub_element(ElementName::ServerServiceMinorVersion)?
            .character_data()?
            .parse_integer()?;

        let initial_offer_elem = config_elem.get_sub_element(ElementName::InitialOfferBehavior)?;
        let initial_delay_max_value = initial_offer_elem
            .get_sub_element(ElementName::InitialDelayMaxValue)?
            .character_data()?
            .parse_float()?;
        let initial_delay_min_value = initial_offer_elem
            .get_sub_element(ElementName::InitialDelayMinValue)?
            .character_data()?
            .parse_float()?;
        let initial_repetitions_max = initial_offer_elem
            .get_sub_element(ElementName::InitialRepetitionsMax)?
            .character_data()?
            .parse_integer()?;
        let initial_repetitions_base_delay = initial_offer_elem
            .get_sub_element(ElementName::InitialRepetitionsBaseDelay)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_float());
        let offer_cyclic_delay = initial_offer_elem
            .get_sub_element(ElementName::OfferCyclicDelay)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_float());

        let req_resp_delay_elem = config_elem.get_sub_element(ElementName::RequestResponseDelay)?;
        let request_response_delay_max_value = req_resp_delay_elem
            .get_sub_element(ElementName::MaxValue)?
            .character_data()?
            .parse_float()?;
        let request_response_delay_min_value = req_resp_delay_elem
            .get_sub_element(ElementName::MinValue)?
            .character_data()?
            .parse_float()?;

        let ttl = config_elem
            .get_sub_element(ElementName::Ttl)?
            .character_data()?
            .parse_integer()?;

        Some(SdConfig {
            service_major_version,
            service_minor_version,
            ttl,
            initial_delay_max_value,
            initial_delay_min_value,
            initial_repetitions_max,
            initial_repetitions_base_delay,
            offer_cyclic_delay,
            request_response_delay_max_value,
            request_response_delay_min_value,
        })
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventHandlerV1(Element);

impl AbstractionElement for EventHandlerV1 {
    fn element(&self) -> &Element {
        &self.0
    }
}

impl TryFrom<Element> for EventHandlerV1 {
    type Error = AutosarAbstractionError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        // hierarchy: ApplicationEndpoint(named) -> ProvidedServiceInstances -> ProvidedServiceInstance(named) -> EventHandlers -> EventHandler(named)
        let parent_name = element
            .named_parent()?
            .and_then(|p| p.named_parent().ok().flatten())
            .map(|p| p.element_name());
        if !matches!(parent_name, Some(ElementName::ApplicationEndpoint)) {
            return Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "EventHandlerV1".to_string(),
            });
        }

        if element.element_name() == ElementName::EventHandler {
            Ok(Self(element))
        } else {
            Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "EventHandlerV1".to_string(),
            })
        }
    }
}

impl EventHandlerV1 {
    pub(crate) fn new(name: &str, parent: &Element) -> Result<Self, AutosarAbstractionError> {
        let elem = parent.create_named_sub_element(ElementName::EventHandler, name)?;
        Ok(Self(elem))
    }

    /// add a reference to a `SoAdRoutingGroup` to this `EventHandler`
    pub fn add_routing_group(&self, routing_group: &SoAdRoutingGroup) -> Result<(), AutosarAbstractionError> {
        let elem = self
            .element()
            .get_or_create_sub_element(ElementName::RoutingGroupRefs)?;
        elem.create_sub_element(ElementName::RoutingGroupRef)?
            .set_reference_target(routing_group.element())?;
        Ok(())
    }

    /// get the routing groups referenced by this `EventHandler`
    pub fn routing_groups(&self) -> impl Iterator<Item = SoAdRoutingGroup> {
        self.element()
            .get_sub_element(ElementName::RoutingGroupRefs)
            .into_iter()
            .flat_map(|rgs| rgs.sub_elements())
            .filter_map(|rgref| rgref.get_reference_target().ok())
            .filter_map(|rg| SoAdRoutingGroup::try_from(rg).ok())
    }

    /// set the SD server configuration for this `EventHandler`
    pub fn set_sd_server_config(
        &self,
        request_response_delay_min: f64,
        request_response_delay_max: f64,
        ttl: u32,
    ) -> Result<(), AutosarAbstractionError> {
        // remove any existing SdServerConfig, so that we can start fresh
        if let Some(config_elem) = self.element().get_sub_element(ElementName::SdServerConfig) {
            self.element().remove_sub_element(config_elem)?;
        }

        let sd_config_elem = self.element().create_sub_element(ElementName::SdServerConfig)?;
        sd_config_elem
            .create_sub_element(ElementName::Ttl)?
            .set_character_data(ttl as u64)?;

        let req_resp_delay_elem = sd_config_elem.create_sub_element(ElementName::RequestResponseDelay)?;
        req_resp_delay_elem
            .create_sub_element(ElementName::MinValue)?
            .set_character_data(request_response_delay_min)?;
        req_resp_delay_elem
            .create_sub_element(ElementName::MaxValue)?
            .set_character_data(request_response_delay_max)?;

        Ok(())
    }

    /// get the SD server configuration for this `EventHandler`
    pub fn sd_server_config(&self) -> Option<SdEventConfig> {
        let config_elem = self.element().get_sub_element(ElementName::SdServerConfig)?;
        let ttl = config_elem
            .get_sub_element(ElementName::Ttl)?
            .character_data()?
            .parse_integer()?;

        let req_resp_delay_elem = config_elem.get_sub_element(ElementName::RequestResponseDelay)?;
        let request_response_delay_min_value = req_resp_delay_elem
            .get_sub_element(ElementName::MinValue)?
            .character_data()?
            .parse_float()?;
        let request_response_delay_max_value = req_resp_delay_elem
            .get_sub_element(ElementName::MaxValue)?
            .character_data()?
            .parse_float()?;

        Some(SdEventConfig {
            request_response_delay_max_value,
            request_response_delay_min_value,
            ttl,
        })
    }

    /// get the consumed event groups referenced by this `EventHandler`
    pub fn consumed_event_groups(&self) -> impl Iterator<Item = ConsumedEventGroupV1> {
        self.element()
            .get_sub_element(ElementName::ConsumedEventGroupRefs)
            .into_iter()
            .flat_map(|cegs| cegs.sub_elements())
            .filter_map(|cegref| cegref.get_reference_target().ok())
            .filter_map(|ceg| ConsumedEventGroupV1::try_from(ceg).ok())
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConsumedServiceInstanceV1(Element);

impl AbstractionElement for ConsumedServiceInstanceV1 {
    fn element(&self) -> &Element {
        &self.0
    }
}

impl TryFrom<Element> for ConsumedServiceInstanceV1 {
    type Error = AutosarAbstractionError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        // hierarchy: ApplicationEndpoint -> ConsumedServiceInstances -> ConsumedServiceInstance
        let parent_name = element.parent()?.map(|p| p.element_name());
        if !matches!(parent_name, Some(ElementName::ConsumedServiceInstances)) {
            return Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "ConsumedServiceInstanceV1".to_string(),
            });
        }

        if element.element_name() == ElementName::ConsumedServiceInstance {
            Ok(Self(element))
        } else {
            Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "ConsumedServiceInstanceV1".to_string(),
            })
        }
    }
}

impl ConsumedServiceInstanceV1 {
    pub(crate) fn new(
        name: &str,
        parent: &Element,
        provided_service_instance: &ProvidedServiceInstanceV1,
    ) -> Result<Self, AutosarAbstractionError> {
        let elem = parent.create_named_sub_element(ElementName::ConsumedServiceInstance, name)?;

        elem.create_sub_element(ElementName::ProvidedServiceInstanceRef)?
            .set_reference_target(provided_service_instance.element())?;

        Ok(Self(elem))
    }

    /// get the `ProvidedServiceInstance` referenced by this `ConsumedServiceInstance`
    pub fn provided_service_instance(&self) -> Option<ProvidedServiceInstanceV1> {
        self.element()
            .get_sub_element(ElementName::ProvidedServiceInstanceRef)
            .and_then(|psiref| psiref.get_reference_target().ok())
            .and_then(|psielem| ProvidedServiceInstanceV1::try_from(psielem).ok())
    }

    /// create a new `ConsumedEventGrup` in this `ConsumedServiceInstance`
    pub fn create_consumed_event_group(
        &self,
        name: &str,
        event_group_identifier: u32,
        event_handler: &EventHandlerV1,
    ) -> Result<ConsumedEventGroupV1, AutosarAbstractionError> {
        let cegs = self
            .element()
            .get_or_create_sub_element(ElementName::ConsumedEventGroups)?;
        ConsumedEventGroupV1::new(name, &cegs, event_group_identifier, event_handler)
    }

    /// get the `ConsumedEventGroup`s in this `ConsumedServiceInstance`
    pub fn consumed_event_groups(&self) -> impl Iterator<Item = ConsumedEventGroupV1> {
        self.element()
            .get_sub_element(ElementName::ConsumedEventGroups)
            .into_iter()
            .flat_map(|cegs| cegs.sub_elements())
            .filter_map(|ceg| ConsumedEventGroupV1::try_from(ceg).ok())
    }

    /// set the SD client configuration for this `ProvidedServiceInstance`
    pub fn set_sd_client_config(&self, sd_client_config: &SdConfig) -> Result<(), AutosarAbstractionError> {
        // remove any existing SdClientConfig, so that we can start fresh
        if let Some(config_elem) = self.element().get_sub_element(ElementName::SdClientConfig) {
            self.element().remove_sub_element(config_elem)?;
        }

        let config_elem = self.element().get_or_create_sub_element(ElementName::SdClientConfig)?;

        config_elem
            .create_sub_element(ElementName::ClientServiceMajorVersion)?
            .set_character_data(sd_client_config.service_major_version as u64)?;
        config_elem
            .create_sub_element(ElementName::ClientServiceMinorVersion)?
            .set_character_data(sd_client_config.service_minor_version as u64)?;
        config_elem
            .create_sub_element(ElementName::Ttl)?
            .set_character_data(sd_client_config.ttl as u64)?;

        let initial_find_elem = config_elem.create_sub_element(ElementName::InitialFindBehavior)?;
        initial_find_elem
            .create_sub_element(ElementName::InitialDelayMaxValue)?
            .set_character_data(sd_client_config.initial_delay_max_value)?;
        initial_find_elem
            .create_sub_element(ElementName::InitialDelayMinValue)?
            .set_character_data(sd_client_config.initial_delay_min_value)?;
        initial_find_elem
            .create_sub_element(ElementName::InitialRepetitionsMax)?
            .set_character_data(sd_client_config.initial_repetitions_max as u64)?;
        if let Some(initial_repetitions_base_delay) = sd_client_config.initial_repetitions_base_delay {
            initial_find_elem
                .create_sub_element(ElementName::InitialRepetitionsBaseDelay)?
                .set_character_data(initial_repetitions_base_delay)?;
        }

        Ok(())
    }

    /// get the SD client configuration for this `ProvidedServiceInstance`
    pub fn sd_client_config(&self) -> Option<SdConfig> {
        let config_elem = self.element().get_sub_element(ElementName::SdClientConfig)?;
        let service_major_version = config_elem
            .get_sub_element(ElementName::ClientServiceMajorVersion)?
            .character_data()?
            .parse_integer()?;
        let service_minor_version = config_elem
            .get_sub_element(ElementName::ClientServiceMinorVersion)?
            .character_data()?
            .parse_integer()?;
        let ttl = config_elem
            .get_sub_element(ElementName::Ttl)?
            .character_data()?
            .parse_integer()?;

        let initial_find_elem = config_elem.get_sub_element(ElementName::InitialFindBehavior)?;
        let initial_delay_max_value = initial_find_elem
            .get_sub_element(ElementName::InitialDelayMaxValue)?
            .character_data()?
            .parse_float()?;
        let initial_delay_min_value = initial_find_elem
            .get_sub_element(ElementName::InitialDelayMinValue)?
            .character_data()?
            .parse_float()?;
        let initial_repetitions_max = initial_find_elem
            .get_sub_element(ElementName::InitialRepetitionsMax)?
            .character_data()?
            .parse_integer()?;
        let initial_repetitions_base_delay = initial_find_elem
            .get_sub_element(ElementName::InitialRepetitionsBaseDelay)
            .and_then(|elem| elem.character_data())
            .and_then(|cdata| cdata.parse_float());

        Some(SdConfig {
            service_major_version,
            service_minor_version,
            ttl,
            initial_delay_max_value,
            initial_delay_min_value,
            initial_repetitions_max,
            initial_repetitions_base_delay,
            offer_cyclic_delay: None,
            request_response_delay_max_value: 0.0,
            request_response_delay_min_value: 0.0,
        })
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConsumedEventGroupV1(Element);

impl AbstractionElement for ConsumedEventGroupV1 {
    fn element(&self) -> &Element {
        &self.0
    }
}

impl TryFrom<Element> for ConsumedEventGroupV1 {
    type Error = AutosarAbstractionError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        // hierarchy: ApplicationEndpoint(named) -> ConsumedServiceInstances -> ConsumedServiceInstance(named) -> ConsumedEventGroups -> ConsumedEventGroup(named)
        let parent_name = element
            .named_parent()?
            .and_then(|p| p.named_parent().ok().flatten())
            .map(|p| p.element_name());
        if !matches!(parent_name, Some(ElementName::ApplicationEndpoint)) {
            return Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "ConsumedEventGroupV1".to_string(),
            });
        }

        if element.element_name() == ElementName::ConsumedEventGroup {
            Ok(Self(element))
        } else {
            Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "ConsumedEventGroupV1".to_string(),
            })
        }
    }
}

impl ConsumedEventGroupV1 {
    pub(crate) fn new(
        name: &str,
        parent: &Element,
        event_group_identifier: u32,
        event_handler: &EventHandlerV1,
    ) -> Result<Self, AutosarAbstractionError> {
        let elem = parent.create_named_sub_element(ElementName::ConsumedEventGroup, name)?;
        elem.create_sub_element(ElementName::EventGroupIdentifier)?
            .set_character_data(event_group_identifier as u64)?;
        let ae = parent.named_parent()?.unwrap().named_parent()?.unwrap();
        elem.create_sub_element(ElementName::ApplicationEndpointRef)?
            .set_reference_target(&ae)?;
        event_handler
            .element()
            .get_or_create_sub_element(ElementName::ConsumedEventGroupRefs)?
            .create_sub_element(ElementName::ConsumedEventGroupRef)?
            .set_reference_target(&elem)?;

        Ok(Self(elem))
    }

    /// set the SocketAddress that receives events from this `ConsumedEventGroup`
    /// This may be a different SocketAddress than the one that is used to send requests.
    pub fn set_application_endpoint(&self, socket_address: &SocketAddress) -> Result<(), AutosarAbstractionError> {
        let Some(ae_elem) = socket_address
            .element()
            .get_sub_element(ElementName::ApplicationEndpoint)
        else {
            return Err(AutosarAbstractionError::InvalidParameter(
                "SocketAddress does not have an ApplicationEndpoint".to_string(),
            ));
        };
        self.element()
            .get_or_create_sub_element(ElementName::ApplicationEndpointRef)?
            .set_reference_target(&ae_elem)?;
        Ok(())
    }

    /// get the Socket that receives events from this `ConsumedEventGroup`
    /// This may be a different Socket than the one that is used to send requests.
    pub fn application_endpoint(&self) -> Option<SocketAddress> {
        self.element()
            .get_sub_element(ElementName::ApplicationEndpointRef)
            .and_then(|aeref| aeref.get_reference_target().ok())
            .and_then(|ae: Element| ae.parent().ok().flatten())
            .and_then(|sa| SocketAddress::try_from(sa).ok())
    }

    /// get the event group identifier of this `ConsumedEventGroup`
    pub fn event_group_identifier(&self) -> Option<u32> {
        self.0
            .get_sub_element(ElementName::EventGroupIdentifier)
            .and_then(|egi| egi.character_data())
            .and_then(|cdata| cdata.parse_integer())
    }

    /// add a reference to a `SoAdRoutingGroup` to this `ConsumedEventGroup`
    pub fn add_routing_group(&self, routing_group: &SoAdRoutingGroup) -> Result<(), AutosarAbstractionError> {
        let elem = self
            .element()
            .get_or_create_sub_element(ElementName::RoutingGroupRefs)?;
        elem.create_sub_element(ElementName::RoutingGroupRef)?
            .set_reference_target(routing_group.element())?;
        Ok(())
    }

    /// get the routing groups referenced by this `ConsumedEventGroup`
    pub fn routing_groups(&self) -> impl Iterator<Item = SoAdRoutingGroup> {
        self.element()
            .get_sub_element(ElementName::RoutingGroupRefs)
            .into_iter()
            .flat_map(|rgs| rgs.sub_elements())
            .filter_map(|rgref| rgref.get_reference_target().ok())
            .filter_map(|rg| SoAdRoutingGroup::try_from(rg).ok())
    }

    /// set the SD client configuration for this `ConsumedEventGroup`
    pub fn set_sd_client_config(
        &self,
        request_response_delay_min: f64,
        request_response_delay_max: f64,
        ttl: u32,
    ) -> Result<(), AutosarAbstractionError> {
        // remove any existing SdClientConfig, so that we can start fresh
        if let Some(sd_config_elem) = self.element().get_sub_element(ElementName::SdClientConfig) {
            self.element().remove_sub_element(sd_config_elem)?;
        }

        let sd_config_elem = self.element().create_sub_element(ElementName::SdClientConfig)?;
        sd_config_elem
            .create_sub_element(ElementName::Ttl)?
            .set_character_data(ttl as u64)?;

        let req_resp_delay_elem = sd_config_elem.create_sub_element(ElementName::RequestResponseDelay)?;
        req_resp_delay_elem
            .create_sub_element(ElementName::MinValue)?
            .set_character_data(request_response_delay_min)?;
        req_resp_delay_elem
            .create_sub_element(ElementName::MaxValue)?
            .set_character_data(request_response_delay_max)?;

        Ok(())
    }

    /// get the SD client configuration for this `ConsumedEventGroup`
    pub fn sd_client_config(&self) -> Option<SdEventConfig> {
        let config_elem = self.element().get_sub_element(ElementName::SdClientConfig)?;
        let ttl = config_elem
            .get_sub_element(ElementName::Ttl)?
            .character_data()?
            .parse_integer()?;

        let req_resp_delay_elem = config_elem.get_sub_element(ElementName::RequestResponseDelay)?;
        let request_response_delay_min_value = req_resp_delay_elem
            .get_sub_element(ElementName::MinValue)?
            .character_data()?
            .parse_float()?;
        let request_response_delay_max_value = req_resp_delay_elem
            .get_sub_element(ElementName::MaxValue)?
            .character_data()?
            .parse_float()?;

        Some(SdEventConfig {
            request_response_delay_max_value,
            request_response_delay_min_value,
            ttl,
        })
    }
}

//##################################################################

/// SD configuration for a service instance
///
/// This struct is used to configure the SD server and client behavior for a service instance.
/// it is used for the old V1 service definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct SdConfig {
    pub service_major_version: u32,
    pub service_minor_version: u32,
    pub initial_delay_max_value: f64,
    pub initial_delay_min_value: f64,
    pub initial_repetitions_base_delay: Option<f64>,
    pub initial_repetitions_max: u32,
    pub offer_cyclic_delay: Option<f64>,
    pub request_response_delay_max_value: f64,
    pub request_response_delay_min_value: f64,
    pub ttl: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SdEventConfig {
    pub request_response_delay_max_value: f64,
    pub request_response_delay_min_value: f64,
    pub ttl: u32,
}

//##################################################################

#[cfg(test)]
mod test {
    use crate::{
        communication::{
            EthernetVlanInfo, EventGroupControlType, NetworkEndpointAddress, SocketAddress, SocketAddressType, TpConfig,
        },
        ArPackage, System, SystemCategory,
    };

    use super::*;
    use autosar_data::{AutosarModel, AutosarVersion};

    /// helper function to create a test setup with:
    /// - a system
    /// - an ethernet cluster
    ///   - a physical channel
    ///   - a network endpoint
    ///   - a socket address
    fn helper_create_test_objects(model: &AutosarModel) -> SocketAddress {
        let package = ArPackage::get_or_create(&model, "/ethernet").unwrap();
        let system = System::new("system", &package, SystemCategory::EcuExtract).unwrap();
        let cluster = system.create_ethernet_cluster("ethcluster", &package).unwrap();
        let channel = cluster
            .create_physical_channel(
                "channel",
                Some(EthernetVlanInfo {
                    vlan_name: "VLAN_02".to_string(),
                    vlan_id: 2,
                }),
            )
            .unwrap();
        let network_endpoint_address = NetworkEndpointAddress::IPv4 {
            address: Some("192.168.2.222".to_string()),
            address_source: None,
            default_gateway: None,
            network_mask: None,
        };
        let network_endpoint = channel
            .create_network_endpoint("endpoint", network_endpoint_address, None)
            .unwrap();
        let tp_config = TpConfig::UdpTp {
            port_number: Some(1234),
            port_dynamically_assigned: None,
        };
        channel
            .create_socket_address(
                "socket",
                &network_endpoint,
                &tp_config,
                SocketAddressType::Unicast(None),
            )
            .unwrap()
    }

    #[test]
    fn someip_v1() {
        let model = AutosarModel::new();
        let _file = model.create_file("file.arxml", AutosarVersion::Autosar_00047).unwrap();
        let package = ArPackage::get_or_create(&model, "/ethernet").unwrap();

        let socket_address = helper_create_test_objects(&model);
        let system = System::try_from(model.get_element_by_path("/ethernet/system").unwrap()).unwrap();
        let psi = socket_address
            .create_provided_service_instance("provided_service", 0x1234, 0x5678)
            .unwrap();
        assert_eq!(psi.service_identifier(), Some(0x1234));
        assert_eq!(psi.instance_identifier(), Some(0x5678));

        psi.set_sd_server_config(&SdConfig {
            service_major_version: 1,
            service_minor_version: 2,
            initial_delay_max_value: 0.0,
            initial_delay_min_value: 0.0,
            initial_repetitions_base_delay: None,
            initial_repetitions_max: 0,
            offer_cyclic_delay: None,
            request_response_delay_max_value: 0.0,
            request_response_delay_min_value: 0.0,
            ttl: 22,
        })
        .unwrap();
        assert_eq!(psi.sd_server_config().unwrap().ttl, 22);
        assert_eq!(psi.sd_server_config().unwrap().service_major_version, 1);
        assert_eq!(psi.sd_server_config().unwrap().service_minor_version, 2);

        assert_eq!(psi.event_handlers().count(), 0);
        let eh = psi.create_event_handler("event").unwrap();
        assert_eq!(psi.event_handlers().count(), 1);
        eh.set_sd_server_config(0.0, 0.99, 22).unwrap();
        assert_eq!(eh.sd_server_config().unwrap().ttl, 22);
        assert_eq!(eh.sd_server_config().unwrap().request_response_delay_max_value, 0.99);
        assert_eq!(eh.sd_server_config().unwrap().request_response_delay_min_value, 0.0);

        let rg = system
            .create_so_ad_routing_group(
                "routing_group",
                &package,
                Some(EventGroupControlType::ActivationMulticast),
            )
            .unwrap();
        eh.add_routing_group(&rg).unwrap();
        assert_eq!(eh.routing_groups().count(), 1);
        assert_eq!(eh.routing_groups().next().unwrap(), rg);
        assert_eq!(eh.consumed_event_groups().count(), 0);

        let csi = socket_address
            .create_consumed_service_instance("consumed_service", &psi)
            .unwrap();
        assert_eq!(csi.provided_service_instance().unwrap(), psi);
        csi.set_sd_client_config(&SdConfig {
            service_major_version: 1,
            service_minor_version: 2,
            initial_delay_max_value: 0.0,
            initial_delay_min_value: 0.0,
            initial_repetitions_base_delay: None,
            initial_repetitions_max: 0,
            offer_cyclic_delay: None,
            request_response_delay_max_value: 0.0,
            request_response_delay_min_value: 0.0,
            ttl: 22,
        })
        .unwrap();
        assert_eq!(csi.sd_client_config().unwrap().ttl, 22);
        assert_eq!(csi.sd_client_config().unwrap().service_major_version, 1);
        assert_eq!(csi.sd_client_config().unwrap().service_minor_version, 2);

        assert_eq!(csi.consumed_event_groups().count(), 0);
        let ceg = csi.create_consumed_event_group("consumed_event", 0x1234, &eh).unwrap();
        assert_eq!(csi.consumed_event_groups().count(), 1);
        assert_eq!(csi.consumed_event_groups().next().unwrap(), ceg);
        assert_eq!(ceg.event_group_identifier(), Some(0x1234));
        // when the consumed event group is created, it is automatically added to the event handler
        assert_eq!(eh.consumed_event_groups().count(), 1);

        ceg.set_sd_client_config(0.0, 0.99, 22).unwrap();
        assert_eq!(ceg.sd_client_config().unwrap().ttl, 22);
        assert_eq!(ceg.sd_client_config().unwrap().request_response_delay_max_value, 0.99);
        assert_eq!(ceg.sd_client_config().unwrap().request_response_delay_min_value, 0.0);

        assert_eq!(ceg.application_endpoint().unwrap(), socket_address);
        ceg.set_application_endpoint(&socket_address).unwrap();
        assert_eq!(ceg.application_endpoint().unwrap(), socket_address);

        ceg.add_routing_group(&rg).unwrap();
        assert_eq!(ceg.routing_groups().count(), 1);
        assert_eq!(ceg.routing_groups().next().unwrap(), rg);
    }

    #[test]
    fn element_conversion() {
        let model = AutosarModel::new();
        let _file = model.create_file("file.arxml", AutosarVersion::Autosar_00047).unwrap();

        let socket_address = helper_create_test_objects(&model);
        let psi = socket_address
            .create_provided_service_instance("provided_service", 0x1234, 0x5678)
            .unwrap();
        let eh = psi.create_event_handler("event").unwrap();

        let csi = socket_address
            .create_consumed_service_instance("consumed_service", &psi)
            .unwrap();
        let ceg = csi.create_consumed_event_group("consumed_event", 0x1234, &eh).unwrap();

        let psi2 = ProvidedServiceInstanceV1::try_from(psi.element().clone()).unwrap();
        assert_eq!(psi2, psi);

        let eh2 = EventHandlerV1::try_from(eh.element().clone()).unwrap();
        assert_eq!(eh2, eh);

        let csi2 = ConsumedServiceInstanceV1::try_from(csi.element().clone()).unwrap();
        assert_eq!(csi2, csi);

        let ceg2 = ConsumedEventGroupV1::try_from(ceg.element().clone()).unwrap();
        assert_eq!(ceg2, ceg);
    }
}
