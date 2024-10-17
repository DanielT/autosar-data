#[cfg(test)]
mod test {
    use autosar_data::{AutosarModel, AutosarVersion, ElementName};
    use autosar_data_abstraction::{
        communication::{
            CommonServiceDiscoveryConfig, CommunicationDirection, CyclicTiming, DataTransformationSet, E2EProfile,
            E2EProfileBehavior, E2ETransformationTechnologyConfig, EthernetVlanInfo, EventControlledTiming,
            EventGroupControlType, GeneralPurposePduCategory, IPv4AddressSource, InitialSdDelayConfig, IpduTiming,
            NetworkEndpointAddress, RequestResponseDelay, SocketAddressType, SomeIpMessageType,
            SomeIpTransformationISignalPropsConfig, SomeIpTransformationTechnologyConfig,
            SomeipSdClientEventGroupTimingConfig, SomeipSdClientServiceInstanceConfig,
            SomeipSdServerEventGroupTimingConfig, SomeipSdServerServiceInstanceConfig, SystemSignal, TpConfig,
            TransferProperty, TransformationISignalPropsConfig, TransformationTechnologyConfig, TransmissionModeTiming,
        },
        datatype::{
            ApplicationArrayDataType, ApplicationPrimitiveCategory, ApplicationPrimitiveDataType, BaseTypeEncoding,
            DataTypeMappingSet, ImplementationDataType, ImplementationDataTypeSettings, SwBaseType,
        },
        software_component::{
            AbstractSwComponentType, ApplicationSwComponentType, CompositionSwComponentType, SenderReceiverInterface,
        },
        AbstractionElement, ArPackage, ByteOrder, System, SystemCategory,
    };

    #[test]
    fn create_ethernet_v2_system() {
        let model = AutosarModel::new();
        model
            .create_file("ethernet_v2.arxml", AutosarVersion::Autosar_00048)
            .unwrap();
        let package_1 = ArPackage::get_or_create(&model, "/System").unwrap();
        let system = System::new("System", &package_1, SystemCategory::SystemExtract).unwrap();
        let package_2 = ArPackage::get_or_create(&model, "/Clusters").unwrap();

        // create an Ethernet cluster and a physical channel for VLAN 33
        let eth_cluster = system.create_ethernet_cluster("EthCluster", &package_2).unwrap();
        assert_eq!(eth_cluster.element().element_name(), ElementName::EthernetCluster);
        let vlan_info = EthernetVlanInfo {
            vlan_id: 33,
            vlan_name: "VLAN_33".to_string(),
        };
        let eth_channel = eth_cluster
            .create_physical_channel("EthChannel", Some(vlan_info))
            .unwrap();
        let vlan_info_2 = eth_channel.get_vlan_info().unwrap();
        assert_eq!(vlan_info_2.vlan_id, 33);

        // create an ECU instance and connect it to the Ethernet channel
        let package_3 = ArPackage::get_or_create(&model, "/Ecus").unwrap();
        let ecu_instance_a = system.create_ecu_instance("Ecu_A", &package_3).unwrap();
        let ethctrl = ecu_instance_a
            .create_ethernet_communication_controller("EthernetController", Some("ab:cd:ef:01:02:03".to_string()))
            .unwrap();
        let channels_iter = ethctrl.connected_channels();
        assert_eq!(channels_iter.count(), 0);
        ethctrl
            .connect_physical_channel("Ecu_A_connector", &eth_channel)
            .unwrap();
        let channels_iter = ethctrl.connected_channels();
        assert_eq!(channels_iter.count(), 1);

        // create a local socket which belongs to Ecu_A
        let network_address_ecu_a = NetworkEndpointAddress::IPv4 {
            address: Some("192.168.0.1".to_string()),
            address_source: Some(IPv4AddressSource::Fixed),
            default_gateway: Some("192.168.0.200".to_string()),
            network_mask: Some("255.255.255.0".to_string()),
        };
        let network_endpoint_ecu_a = eth_channel
            .create_network_endpoint("local_endpoint", network_address_ecu_a, None)
            .unwrap();
        let tcp_port_ecu_a = TpConfig::TcpTp {
            port_number: Some(1234),
            port_dynamically_assigned: None,
        };
        let socket_type_1 = SocketAddressType::Unicast(Some(ecu_instance_a.clone()));
        let socket_address_tcp_ecu_a = eth_channel
            .create_socket_address("ServerSocket", &network_endpoint_ecu_a, &tcp_port_ecu_a, socket_type_1)
            .unwrap();
        assert_eq!(tcp_port_ecu_a, socket_address_tcp_ecu_a.get_tp_config().unwrap());
        let socket_type = socket_address_tcp_ecu_a.get_type().unwrap();
        assert!(matches!(socket_type, SocketAddressType::Unicast(Some(_))));
        if let SocketAddressType::Unicast(Some(ecu)) = socket_type {
            assert_eq!(ecu, ecu_instance_a);
        }

        // remote socket - not associated with any ECU
        let network_address_remote = NetworkEndpointAddress::IPv4 {
            address: Some("192.168.0.2".to_string()),
            address_source: Some(IPv4AddressSource::Fixed),
            default_gateway: Some("192.168.0.200".to_string()),
            network_mask: Some("255.255.255.0".to_string()),
        };
        let network_endpoint_remote = eth_channel
            .create_network_endpoint("remote_endpoint", network_address_remote, None)
            .unwrap();
        let tcp_port_remote = TpConfig::TcpTp {
            port_number: Some(5678),
            port_dynamically_assigned: None,
        };
        let socket_type_2 = SocketAddressType::Unicast(None);
        let socket_address_tcp_remote = eth_channel
            .create_socket_address(
                "ClientSocket",
                &network_endpoint_remote,
                &tcp_port_remote,
                socket_type_2,
            )
            .unwrap();

        // create a connection (V2)
        let (static_socket_connection_a, static_socket_connection_b) = eth_channel
            .create_static_socket_connection_pair(
                "StaticSocketConnection",
                &socket_address_tcp_ecu_a,
                &socket_address_tcp_remote,
                None,
            )
            .unwrap();

        // create a pdu and add it to the connection
        // PDU-based communication is not typically used in Ethernet, but it is possible. A more conventional choice would be to use SomeIp.
        let pdu_package = ArPackage::get_or_create(&model, "/Network/Pdus").unwrap();
        let isignal_package = ArPackage::get_or_create(&model, "/Network/Signals").unwrap();
        let syssignal_package = ArPackage::get_or_create(&model, "/System/Signals").unwrap();
        let static_pdu = system.create_isignal_ipdu("Pdu_1", &pdu_package, 800).unwrap();
        // create two signals for the PDU
        let system_signal_1 = SystemSignal::new("Signal_1", &syssignal_package).unwrap();
        let system_signal_2 = SystemSignal::new("Signal_2", &syssignal_package).unwrap();
        let static_isignal_1 = system
            .create_isignal("Signal_1", 400, &system_signal_1, None, &isignal_package)
            .unwrap();
        let static_isignal_2 = system
            .create_isignal("Signal_2", 400, &system_signal_2, None, &isignal_package)
            .unwrap();
        static_pdu
            .set_timing(&IpduTiming {
                minimum_delay: None,
                transmission_mode_false_timing: None,
                transmission_mode_true_timing: Some(TransmissionModeTiming {
                    event_controlled_timing: None,
                    cyclic_timing: Some(CyclicTiming {
                        time_period: 0.01,
                        time_offset: None,
                    }),
                }),
            })
            .unwrap();

        // map the signals to the PDU
        static_pdu
            .map_signal(
                &static_isignal_1,
                0,
                ByteOrder::MostSignificantByteLast,
                None,
                TransferProperty::Triggered,
            )
            .unwrap();
        static_pdu
            .map_signal(
                &static_isignal_2,
                400,
                ByteOrder::MostSignificantByteLast,
                None,
                TransferProperty::Triggered,
            )
            .unwrap();

        // create an IPduIdentifier, which is used to map the PDU to both sides of the socket connection
        let ipdu_identifier_set_package = ArPackage::get_or_create(&model, "/Network/IpduIdentifierSets").unwrap();
        let socon_ipdu_identifier_set = system
            .create_socket_connection_ipdu_identifier_set("IpduIdentifierSet", &ipdu_identifier_set_package)
            .unwrap();
        let ipdu_identifier = socon_ipdu_identifier_set
            .create_socon_ipdu_identifier("IpduIdentifier", &static_pdu, &eth_channel, Some(0x1000), None, None)
            .unwrap();

        static_socket_connection_a
            .add_ipdu_identifier(&ipdu_identifier)
            .unwrap();
        static_socket_connection_b
            .add_ipdu_identifier(&ipdu_identifier)
            .unwrap();

        // mapping the PDU to the ECU gives us a PduTriggering, on which a PduPort can be created
        let pdu_triggering = ipdu_identifier.pdu_triggering().unwrap();
        pdu_triggering
            .create_pdu_port(&ecu_instance_a, CommunicationDirection::Out)
            .unwrap();

        let system_mapping = system.get_or_create_mapping("SystemMapping").unwrap();

        // ---------------------------------------------------------
        // software component modeling
        let swc_package = ArPackage::get_or_create(&model, "/SoftwareComponents").unwrap();
        let root_composition = CompositionSwComponentType::new("RootComposition", &swc_package).unwrap();

        // add the root composition to the system
        system
            .set_root_sw_composition("EthernetTestComposition", &root_composition)
            .unwrap();

        // create a composition type and create a composition prototype from it for Ecu_A
        let ecu_a_composition = CompositionSwComponentType::new("Ecu_A_Composition", &swc_package).unwrap();
        let ecu_a_composition_prototype = root_composition
            .create_component("Ecu_A_Composition_Prototype", &ecu_a_composition)
            .unwrap();
        system_mapping
            .map_swc_to_ecu(
                "Ecu_A_Composition_Prototype_Mapping",
                &ecu_a_composition_prototype,
                &ecu_instance_a,
            )
            .unwrap();

        // create an application software component and a prototype from it for Ecu_A
        let application_swc_a = ApplicationSwComponentType::new("ApplicationSwComponent", &swc_package).unwrap();
        let application_swc_a_prototype = ecu_a_composition
            .create_component("ApplicationSwComponent_Prototype", &application_swc_a)
            .unwrap();
        system_mapping
            .map_swc_to_ecu(
                "ApplicationSwComponent_Prototype_Mapping",
                &application_swc_a_prototype,
                &ecu_instance_a,
            )
            .unwrap();

        // create a pair of implementaion and application data types
        let base_type_package = ArPackage::get_or_create(&model, "/BaseTypes").unwrap();
        let data_type_package = ArPackage::get_or_create(&model, "/DataTypes").unwrap();

        let base_type_u8 = SwBaseType::new(
            "uint8",
            &base_type_package,
            8,
            BaseTypeEncoding::None,
            None,
            None,
            Some("uint8"),
        )
        .unwrap();
        let implementation_data_type_u8 = ImplementationDataType::new(
            &data_type_package,
            ImplementationDataTypeSettings::Value {
                name: "ImplDataType_u8".to_string(),
                base_type: base_type_u8.clone(),
                compu_method: None,
                data_constraint: None,
            },
        )
        .unwrap();

        let implementation_data_type_array = ImplementationDataType::new(
            &data_type_package,
            ImplementationDataTypeSettings::Array {
                name: "ImplDataType_array".to_string(),
                element_type: Box::new(ImplementationDataTypeSettings::Value {
                    name: "ImplDataType_u8".to_string(),
                    base_type: base_type_u8,
                    compu_method: None,
                    data_constraint: None,
                }),
                length: 50,
            },
        )
        .unwrap();

        let application_data_type_u8 = ApplicationPrimitiveDataType::new(
            "AppDataType_u8",
            &data_type_package,
            ApplicationPrimitiveCategory::Value,
            None,
            None,
            None,
        )
        .unwrap();
        let application_data_type_array =
            ApplicationArrayDataType::new("AppDataType_array", &data_type_package, &application_data_type_u8, 50)
                .unwrap();

        // create a type mapping
        let type_mapping_package = ArPackage::get_or_create(&model, "/TypeMappings").unwrap();
        let type_mapping_set = DataTypeMappingSet::new("TypeMappingSet", &type_mapping_package).unwrap();
        type_mapping_set
            .create_data_type_map(&implementation_data_type_u8, &application_data_type_u8)
            .unwrap();
        type_mapping_set
            .create_data_type_map(&implementation_data_type_array, &application_data_type_array)
            .unwrap();

        // create a sender-receiver interface
        let sender_receiver_package = ArPackage::get_or_create(&model, "/Interfaces").unwrap();
        let sender_receiver_interface =
            SenderReceiverInterface::new("SenderReceiverInterface", &sender_receiver_package).unwrap();
        let data_element_a = sender_receiver_interface
            .create_data_element("DataElement_a", &application_data_type_array)
            .unwrap();
        let data_element_b = sender_receiver_interface
            .create_data_element("DataElement_b", &application_data_type_array)
            .unwrap();

        // create a port for the sender-receiver interface at every level of the component hierarchy
        let pport_prototype = application_swc_a
            .create_p_port("provide_port", &sender_receiver_interface)
            .unwrap();
        let pport_prototype_2 = ecu_a_composition
            .create_p_port("provide_port", &sender_receiver_interface)
            .unwrap();
        let pport_prototype_3 = root_composition
            .create_p_port("provide_port", &sender_receiver_interface)
            .unwrap();

        // connect the ports to each other; this results in the creation of delegation connectors
        let _delegation_connector_1 = root_composition
            .create_delegation_connector(
                "delegation_connector",
                &pport_prototype_2,
                &ecu_a_composition_prototype,
                &pport_prototype_3,
            )
            .unwrap();
        let _delegation_connector_2 = ecu_a_composition
            .create_delegation_connector(
                "delegation_connector",
                &pport_prototype,
                &application_swc_a_prototype,
                &pport_prototype_2,
            )
            .unwrap();

        // map the sender-receiver interface to the signals
        system_mapping
            .map_sender_receiver_to_signal(&system_signal_1, &data_element_a, &pport_prototype_3, &[], None)
            .unwrap();
        system_mapping
            .map_sender_receiver_to_signal(&system_signal_2, &data_element_b, &pport_prototype_3, &[], None)
            .unwrap();

        // ---------------------------------------------------------

        // SomeIP modeling
        // configure the Ethernet channel to use service discovery
        let unicast_socket = eth_channel
            .create_socket_address(
                "UnicastSocket",
                &network_endpoint_ecu_a,
                &TpConfig::UdpTp {
                    port_number: Some(30490),
                    port_dynamically_assigned: None,
                },
                SocketAddressType::Unicast(Some(ecu_instance_a.clone())),
            )
            .unwrap();
        let multicast_rx_endpoint = eth_channel
            .create_network_endpoint(
                "MulticastEndpoint",
                NetworkEndpointAddress::IPv4 {
                    address: Some("239.0.0.1".to_string()),
                    address_source: Some(IPv4AddressSource::Fixed),
                    default_gateway: None,
                    network_mask: None,
                },
                None,
            )
            .unwrap();
        let multicast_rx_socket = eth_channel
            .create_socket_address(
                "MulticastSocket",
                &multicast_rx_endpoint,
                &TpConfig::UdpTp {
                    port_number: Some(30490),
                    port_dynamically_assigned: None,
                },
                SocketAddressType::Multicast(vec![ecu_instance_a.clone()]),
            )
            .unwrap();
        let remote_anyaddr_endpoint = eth_channel
            .create_network_endpoint(
                "RemoteEndpoint",
                NetworkEndpointAddress::IPv4 {
                    address: Some("ANY".to_string()),
                    address_source: None,
                    default_gateway: None,
                    network_mask: None,
                },
                None,
            )
            .unwrap();
        let remote_anyaddr_socket = eth_channel
            .create_socket_address(
                "RemoteSocket",
                &remote_anyaddr_endpoint,
                &TpConfig::UdpTp {
                    port_number: Some(0),
                    port_dynamically_assigned: None, // this attribute is deprecated
                },
                SocketAddressType::Unicast(None),
            )
            .unwrap();
        let unicast_rx_pdu = system
            .create_general_purpose_pdu("UnicastRxPdu", &pdu_package, 0, GeneralPurposePduCategory::Sd)
            .unwrap();
        let unicast_tx_pdu = system
            .create_general_purpose_pdu("UnicastTxPdu", &pdu_package, 0, GeneralPurposePduCategory::Sd)
            .unwrap();
        let multicast_rx_pdu = system
            .create_general_purpose_pdu("MulticastRxPdu", &pdu_package, 0, GeneralPurposePduCategory::Sd)
            .unwrap();
        let common_config = CommonServiceDiscoveryConfig {
            multicast_rx_socket: &multicast_rx_socket,
            multicast_rx_pdu: &multicast_rx_pdu,
            remote_socket: &remote_anyaddr_socket,
            name_prefix: None,
            prefer_static_socket_connections: true,
            ipdu_identifier_set: Some(&socon_ipdu_identifier_set),
        };

        eth_channel
            .configure_service_discovery_for_ecu(
                &ecu_instance_a,
                &unicast_socket,
                &unicast_rx_pdu,
                &unicast_tx_pdu,
                &common_config,
            )
            .unwrap();

        // create communication elements for SomeIP
        // In SomeIp communication, a pdu only contains a single signal. The data type of the signal is a byte array.
        // Complex application data is transformed into the byte array by a data transformation that contains a SomeIP transformer.
        // the chain can also contain other transformations, in particular an E2E transformation.

        // create a data transformation
        let data_transformer_package = ArPackage::get_or_create(&model, "/DataTransformations").unwrap();
        let transformation_set =
            DataTransformationSet::new("DataTransformationSet", &data_transformer_package).unwrap();
        let someip_config = TransformationTechnologyConfig::SomeIp(SomeIpTransformationTechnologyConfig {
            alignment: 8,
            byte_order: ByteOrder::MostSignificantByteLast,
            interface_version: 1,
        });
        let someip_tranformation_technology = transformation_set
            .create_transformation_technology("SomeIpTransformationTechnology", &someip_config)
            .unwrap();
        let e2e_config = TransformationTechnologyConfig::E2E(E2ETransformationTechnologyConfig {
            profile: E2EProfile::P07,
            zero_header_length: false, // not used in this combination with SomeIP
            transform_in_place: false,
            offset: 64,
            max_delta_counter: 5,
            max_error_state_init: 0,
            max_error_state_invalid: 0,
            max_error_state_valid: 0,
            max_no_new_or_repeated_data: 4,
            min_ok_state_init: 4,
            min_ok_state_invalid: 5,
            min_ok_state_valid: 2,
            window_size: 10,
            window_size_init: None,
            window_size_invalid: None,
            window_size_valid: None,
            profile_behavior: Some(E2EProfileBehavior::R4_2),
            sync_counter_init: None,
            data_id_mode: None,
            data_id_nibble_offset: None,
            crc_offset: None,
            counter_offset: None,
        });
        let e2e_tranformation_technology = transformation_set
            .create_transformation_technology("E2ETransformationTechnology", &e2e_config)
            .unwrap();
        let data_transformation = transformation_set
            .create_data_transformation(
                "DataTransformation",
                &[&someip_tranformation_technology, &e2e_tranformation_technology],
                true,
            )
            .unwrap();

        // create a Signal for SomeIP communication
        let system_signal_3 = SystemSignal::new("SomeIp_Signal_1", &syssignal_package).unwrap();
        let someip_isignal_1 = system
            .create_isignal("Someip_Signal_1", 400, &system_signal_3, None, &isignal_package)
            .unwrap();
        someip_isignal_1.add_data_transformation(&data_transformation).unwrap();
        someip_isignal_1
            .add_transformation_isignal_props(
                &someip_tranformation_technology,
                &TransformationISignalPropsConfig::SomeIp(SomeIpTransformationISignalPropsConfig {
                    legacy_strings: None,
                    dynamic_length: None,
                    message_type: Some(SomeIpMessageType::Notification),
                    size_of_array_length: None,
                    size_of_string_length: None,
                    size_of_struct_length: None,
                    size_of_union_length: None,
                    interface_version: None,
                }),
            )
            .unwrap();

        // create a PDU for SomeIp communication. The PDU is larger than the signal to account for the SomeIP and E2E headers.
        // SomeIp: 64 bits; E2E Profile 7: 160 bits
        let someip_pdu = system.create_isignal_ipdu("Someip_Pdu", &pdu_package, 624).unwrap();
        someip_pdu
            .map_signal(
                &someip_isignal_1,
                0,
                ByteOrder::MostSignificantByteLast,
                None,
                TransferProperty::Triggered,
            )
            .unwrap();
        someip_pdu
            .set_timing(&IpduTiming {
                minimum_delay: None,
                transmission_mode_false_timing: None,
                transmission_mode_true_timing: Some(TransmissionModeTiming {
                    event_controlled_timing: Some(EventControlledTiming {
                        number_of_repetitions: 0,
                        repetition_period: None,
                    }),
                    cyclic_timing: None,
                }),
            })
            .unwrap();

        // SomeIp transport layer: the service instances are now created in a ServiceInstanceCollectionSet instead of directly in the socket
        // The sockets are referenced in various roles from the service instances.
        let service_instance_package = ArPackage::get_or_create(&model, "/ServiceInstances").unwrap();
        let service_instance_collection_set = system
            .create_service_instance_collection_set("ServiceInstances", &service_instance_package)
            .unwrap();

        // socket_address_tcp_ecu_a already exists
        let socket_address_udp_ecu_a = eth_channel
            .create_socket_address(
                "UdpSocket",
                &network_endpoint_ecu_a,
                &TpConfig::UdpTp {
                    port_number: Some(50000), // arbitrary port number, in particular it does not need to match the SD port number
                    port_dynamically_assigned: None,
                },
                SocketAddressType::Unicast(Some(ecu_instance_a.clone())),
            )
            .unwrap();

        let socket_address_udp_remote = eth_channel
            .create_socket_address(
                "UdpRemoteSocket",
                &network_endpoint_remote,
                &TpConfig::UdpTp {
                    port_number: Some(50000),
                    port_dynamically_assigned: None,
                },
                SocketAddressType::Unicast(None),
            )
            .unwrap();

        let service_identifier = 0xbaad;
        let instance_identifier = 0xfeed;
        let psi_ecu_a = service_instance_collection_set
            .create_provided_service_instance("PSI_Ecu_A", service_identifier, instance_identifier, 1, 2)
            .unwrap();
        psi_ecu_a.set_local_unicast_address(&socket_address_udp_ecu_a).unwrap();
        let psi_ecu_a_eh = psi_ecu_a.create_event_handler("PSI_Ecu_A_EH_1", 1).unwrap();
        let psi_ecu_a_eh_pdu_group = psi_ecu_a_eh
            .create_pdu_activation_routing_group("PduGroup", EventGroupControlType::ActivationUnicast)
            .unwrap();

        let csi_remote = service_instance_collection_set
            .create_consumed_service_instance("CSI_Remote", service_identifier, instance_identifier, 1, "2")
            .unwrap();
        csi_remote
            .set_local_unicast_address(&socket_address_udp_remote)
            .unwrap();
        let csi_remote_ceg = csi_remote.create_consumed_event_group("CSI_Remote_CEG_1", 1).unwrap();
        let csi_remote_ceg_pdu_group = csi_remote_ceg
            .create_pdu_activation_routing_group("PduGroup", EventGroupControlType::ActivationUnicast)
            .unwrap();

        // note: for SomeIp the service id forms the upper 16 bits of the PDU header id
        // the lower 16 bits are the method id (0x0 - 0x7fff) or event id (0x8000 - 0xffff)
        let method_id = 0x8001u64;
        let pdu_header_id = (service_identifier as u64) << 16 | method_id;
        let someip_ipdu_identifier = socon_ipdu_identifier_set
            .create_socon_ipdu_identifier(
                "SomeIpIpduIdentifier",
                &someip_pdu,
                &eth_channel,
                Some(pdu_header_id),
                None,
                None,
            )
            .unwrap();

        let pdu_triggering_someip = someip_ipdu_identifier.pdu_triggering().unwrap();
        pdu_triggering_someip
            .create_pdu_port(&ecu_instance_a, CommunicationDirection::Out)
            .unwrap();

        psi_ecu_a_eh_pdu_group
            .add_ipdu_identifier_udp(&someip_ipdu_identifier)
            .unwrap();
        csi_remote_ceg_pdu_group
            .add_ipdu_identifier_udp(&someip_ipdu_identifier)
            .unwrap();

        // set SD-specific parameters for the service events
        // in the new model, this information is stored separately and each service instance/event references it
        let sd_config_package = ArPackage::get_or_create(&model, "/SomeipSdTimingConfigs").unwrap();
        let rrd = RequestResponseDelay {
            min_value: 0.1,
            max_value: 0.2,
        };
        let isdc = InitialSdDelayConfig {
            initial_delay_max_value: 0.1,
            initial_delay_min_value: 0.05,
            initial_repetitions_base_delay: None,
            initial_repetitions_max: None,
        };

        let server_service_instance_config =
            SomeipSdServerServiceInstanceConfig::new("ssssic", &sd_config_package, 10).unwrap();
        server_service_instance_config.set_request_response_delay(&rrd).unwrap();
        server_service_instance_config
            .set_initial_offer_behavior(&isdc)
            .unwrap();
        server_service_instance_config.set_offer_cyclic_delay(2.0).unwrap();
        server_service_instance_config.set_priority(4).unwrap();
        psi_ecu_a
            .set_sd_server_instance_config(&server_service_instance_config)
            .unwrap();

        let client_service_instance_config =
            SomeipSdClientServiceInstanceConfig::new("sscsic", &sd_config_package).unwrap();
        client_service_instance_config.set_initial_find_behavior(&isdc).unwrap();
        client_service_instance_config.set_priority(3).unwrap();
        csi_remote
            .set_sd_client_instance_config(&client_service_instance_config)
            .unwrap();

        let server_event_config = SomeipSdServerEventGroupTimingConfig::new("ssetc", &sd_config_package, &rrd).unwrap();
        psi_ecu_a_eh
            .set_sd_server_event_group_timing_config(&server_event_config)
            .unwrap();

        let client_event_config = SomeipSdClientEventGroupTimingConfig::new("secsc", &sd_config_package, 10).unwrap();
        client_event_config.set_request_response_delay(&rrd).unwrap();
        client_event_config.set_subscribe_eventgroup_retry_delay(0.4).unwrap();
        client_event_config.set_subscribe_eventgroup_retry_max(3).unwrap();
        csi_remote_ceg.set_sd_client_timer_config(&client_event_config).unwrap();

        println!("{}", model.files().next().unwrap().serialize().unwrap());
        model.write().unwrap();
    }
}
