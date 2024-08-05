#[cfg(test)]
mod test {
    use autosar_data::{AutosarModel, AutosarVersion, ElementName};
    use autosar_data_abstraction::{
        communication::{
            CanAddressingMode, CanClusterSettings, CanFrameType, CommunicationDirection, EthernetVlanInfo,
            FlexrayChannelName, FlexrayClusterSettings, FlexrayCommunicationCycle, IPv4AddressSource,
            NetworkEndpointAddress, SocketAddressType, SocketConnectionIpduIdentifierSet, SystemSignal, TpConfig,
            TransferProperty,
        },
        datatype::{BaseTypeEncoding, SwBaseType},
        software_component::CompositionSwComponentType,
        AbstractionElement, ArPackage, ByteOrder, System, SystemCategory,
    };

    #[test]
    fn create_can_system() {
        let model = AutosarModel::new();
        model.create_file("can.arxml", AutosarVersion::LATEST).unwrap();
        let system_package = ArPackage::get_or_create(&model, "/System").unwrap();
        let system = System::new("System", &system_package, SystemCategory::SystemExtract).unwrap();
        let cluster_package = ArPackage::get_or_create(&model, "/Network/Clusters").unwrap();

        let mut settings = CanClusterSettings::default();
        settings.can_fd_baudrate = Some(2000000);
        let can_cluster = system
            .create_can_cluster("CanCluster", &cluster_package, &settings)
            .unwrap();
        assert_eq!(can_cluster.element().element_name(), ElementName::CanCluster);
        let can_channel = can_cluster.create_physical_channel("CanChannel").unwrap();

        let ecu_package = ArPackage::get_or_create(&model, "/Ecus").unwrap();

        // create ECU A and connect it to the CAN channel
        let ecu_instance_a = system.create_ecu_instance("Ecu_A", &ecu_package).unwrap();
        let canctrl_a = ecu_instance_a
            .create_can_communication_controller("CanController")
            .unwrap();
        let channels_iter = canctrl_a.connected_channels();
        assert_eq!(channels_iter.count(), 0);
        canctrl_a
            .connect_physical_channel("Ecu_A_connector", &can_channel)
            .unwrap();
        let channels_iter = canctrl_a.connected_channels();
        assert_eq!(channels_iter.count(), 1);

        // create ECU B and connect it to the CAN channel
        let ecu_instance_b = system.create_ecu_instance("Ecu_B", &ecu_package).unwrap();
        let canctrl_b = ecu_instance_b
            .create_can_communication_controller("CanController")
            .unwrap();
        canctrl_b
            .connect_physical_channel("Ecu_B_connector", &can_channel)
            .unwrap();

        let frame_package = ArPackage::get_or_create(&model, "/Network/Frames").unwrap();
        let pdu_package = ArPackage::get_or_create(&model, "/Network/Pdus").unwrap();
        let isignal_package = ArPackage::get_or_create(&model, "/Network/Signals").unwrap();
        let syssignal_package = ArPackage::get_or_create(&model, "/System/Signals").unwrap();

        // create a base type for the CAN signals
        let base_type_package = ArPackage::get_or_create(&model, "/BaseTypes").unwrap();
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

        // create Frame_1 which contains Pdu_1: Id 0x100, length 8
        let frame1 = system.create_can_frame("Frame_1", 8, &frame_package).unwrap();
        let pdu1 = system.create_isignal_ipdu("Pdu_1", &pdu_package, 8).unwrap();
        frame1
            .map_pdu(
                &pdu1,
                0,
                autosar_data_abstraction::ByteOrder::MostSignificantByteLast,
                None,
            )
            .unwrap();
        let ft_1 = can_channel
            .trigger_frame(&frame1, 0x100, CanAddressingMode::Standard, CanFrameType::Can20)
            .unwrap();
        assert_eq!(frame1.frame_triggerings().count(), 1);
        assert_eq!(ft_1.pdu_triggerings().count(), 1);

        // create Frame_2 which contains Pdu_2: Id 0x101, length 8
        let frame2 = system.create_can_frame("Frame_2", 8, &frame_package).unwrap();
        let pdu2 = system.create_isignal_ipdu("Pdu_2", &pdu_package, 8).unwrap();
        let ss_pdu2signal1 = SystemSignal::new("P2S1", &isignal_package).unwrap();
        let pdu2signal1 = system
            .create_isignal("P2S1", 4, &ss_pdu2signal1, Some(&base_type_u8), &syssignal_package)
            .unwrap();
        let ss_pdu2signal2 = SystemSignal::new("P2S2", &syssignal_package).unwrap();
        let pdu2signal2 = system
            .create_isignal("P2S2", 4, &ss_pdu2signal2, Some(&base_type_u8), &isignal_package)
            .unwrap();
        pdu2.map_signal(
            &pdu2signal1,
            0,
            autosar_data_abstraction::ByteOrder::MostSignificantByteFirst,
            None,
            TransferProperty::Triggered,
        )
        .unwrap();
        frame2
            .map_pdu(
                &pdu2,
                0,
                autosar_data_abstraction::ByteOrder::MostSignificantByteLast,
                None,
            )
            .unwrap();
        let ft_2 = can_channel
            .trigger_frame(&frame2, 0x101, CanAddressingMode::Standard, CanFrameType::Can20)
            .unwrap();
        pdu2.map_signal(
            &pdu2signal2,
            8,
            autosar_data_abstraction::ByteOrder::MostSignificantByteFirst,
            None,
            TransferProperty::Triggered,
        )
        .unwrap();

        // frame 1: Ecu_A -> Ecu_B
        ft_1.connect_to_ecu(&ecu_instance_a, CommunicationDirection::Out)
            .unwrap();
        ft_1.connect_to_ecu(&ecu_instance_b, CommunicationDirection::In)
            .unwrap();
        // frame 2: Ecu_B -> Ecu_A
        ft_2.connect_to_ecu(&ecu_instance_a, CommunicationDirection::In)
            .unwrap();
        ft_2.connect_to_ecu(&ecu_instance_b, CommunicationDirection::Out)
            .unwrap();

        // software component modeling
        let swc_package = ArPackage::get_or_create(&model, "/SoftwareComponents").unwrap();
        let root_composition = CompositionSwComponentType::new("RootComposition", &swc_package).unwrap();

        // ... Todo: create other swc elements ...

        // add the root composition to the system
        system
            .set_root_sw_composition("CanTestComposition", &root_composition)
            .unwrap();

        println!("{}", model.files().next().unwrap().serialize().unwrap());
        model.write().unwrap();
    }

    #[test]
    fn create_ethernet_system() {
        let model = AutosarModel::new();
        model
            .create_file("ethernet.arxml", AutosarVersion::Autosar_00046)
            .unwrap();
        let package_1 = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        let system = System::new("System", &package_1, SystemCategory::SystemExtract).unwrap();
        let package_2 = ArPackage::get_or_create(&model, "/Clusters").unwrap();

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

        let package_3 = ArPackage::get_or_create(&model, "/Ecus").unwrap();
        let ecu_instance = system.create_ecu_instance("Ecu_A", &package_3).unwrap();
        let ethctrl = ecu_instance
            .create_ethernet_communication_controller("EthernetController", Some("ab:cd:ef:01:02:03".to_string()))
            .unwrap();
        let channels_iter = ethctrl.connected_channels();
        assert_eq!(channels_iter.count(), 0);
        ethctrl
            .connect_physical_channel("Ecu_A_connector", &eth_channel)
            .unwrap();
        let channels_iter = ethctrl.connected_channels();
        assert_eq!(channels_iter.count(), 1);

        // local socket which belongs to Ecu_A
        let address_1 = NetworkEndpointAddress::IPv4 {
            address: Some("192.168.0.1".to_string()),
            address_source: Some(IPv4AddressSource::Fixed),
            default_gateway: Some("192.168.0.200".to_string()),
            network_mask: Some("255.255.255.0".to_string()),
        };
        let network_endpoint_1 = eth_channel
            .create_network_endpoint("local_endpoint", address_1, None)
            .unwrap();
        let tcp_port_1 = TpConfig::TcpTp {
            port_number: Some(1234),
            port_dynamically_assigned: None,
        };
        let socket_type_1 = SocketAddressType::Unicast(Some(ecu_instance.clone()));
        let socket_address_1 = eth_channel
            .create_socket_address("ServerSocket", &network_endpoint_1, &tcp_port_1, socket_type_1)
            .unwrap();
        assert_eq!(tcp_port_1, socket_address_1.get_tp_config().unwrap());
        let socket_type = socket_address_1.get_type().unwrap();
        assert!(matches!(socket_type, SocketAddressType::Unicast(Some(_))));
        if let SocketAddressType::Unicast(Some(ecu)) = socket_type {
            assert_eq!(ecu, ecu_instance);
        }

        // remote socket
        let address_2 = NetworkEndpointAddress::IPv4 {
            address: Some("192.168.0.2".to_string()),
            address_source: Some(IPv4AddressSource::Fixed),
            default_gateway: Some("192.168.0.200".to_string()),
            network_mask: Some("255.255.255.0".to_string()),
        };
        let network_endpoint_2 = eth_channel
            .create_network_endpoint("remote_endpoint", address_2, None)
            .unwrap();
        let tcp_port_2 = TpConfig::TcpTp {
            port_number: Some(5678),
            port_dynamically_assigned: None,
        };
        let socket_type_2 = SocketAddressType::Unicast(None);
        let socket_address_2 = eth_channel
            .create_socket_address("ClientSocket", &network_endpoint_2, &tcp_port_2, socket_type_2)
            .unwrap();

        // create a connection (V1)
        let socket_connection_bundle = eth_channel
            .create_socket_connection_bundle("ConnectionBundle", &socket_address_1)
            .unwrap();
        let connection = socket_connection_bundle
            .create_bundled_connection(&socket_address_2)
            .unwrap();

        // create a pdu and add it to the connection
        let pdu_package = ArPackage::get_or_create(&model, "/Network/Pdus").unwrap();
        let isignal_package = ArPackage::get_or_create(&model, "/Network/Signals").unwrap();
        let syssignal_package = ArPackage::get_or_create(&model, "/System/Signals").unwrap();
        let pdu = system.create_isignal_ipdu("Pdu_1", &pdu_package, 8).unwrap();
        let system_signal = SystemSignal::new("Signal_1", &syssignal_package).unwrap();
        let isignal = system
            .create_isignal("Signal_1", 8, &system_signal, None, &isignal_package)
            .unwrap();

        let pdu_triggering = connection.add_pdu(&pdu.clone().into(), 0x1000, None, None).unwrap();
        pdu.map_signal(
            &isignal,
            0,
            ByteOrder::MostSignificantByteLast,
            None,
            TransferProperty::Triggered,
        )
        .unwrap();
        assert_eq!(pdu_triggering, connection.pdu_triggerings().next().unwrap());

        pdu_triggering
            .create_pdu_port(&ecu_instance, CommunicationDirection::Out)
            .unwrap();

        // software component modeling
        let swc_package = ArPackage::get_or_create(&model, "/SoftwareComponents").unwrap();
        let root_composition = CompositionSwComponentType::new("RootComposition", &swc_package).unwrap();

        // ... Todo: create other swc elements ...

        // add the root composition to the system
        system
            .set_root_sw_composition("EthernetTestComposition", &root_composition)
            .unwrap();

        println!("{}", model.files().next().unwrap().serialize().unwrap());
        model.write().unwrap();
    }

    #[test]
    fn create_ethernet_v2_system() {
        let model = AutosarModel::new();
        model
            .create_file("ethernet_new.arxml", AutosarVersion::Autosar_00048)
            .unwrap();
        let package_1 = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        let system = System::new("System", &package_1, SystemCategory::SystemExtract).unwrap();
        let package_2 = ArPackage::get_or_create(&model, "/Clusters").unwrap();

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

        let package_3 = ArPackage::get_or_create(&model, "/Ecus").unwrap();
        let ecu_instance = system.create_ecu_instance("Ecu_A", &package_3).unwrap();
        let ethctrl = ecu_instance
            .create_ethernet_communication_controller("EthernetController", Some("ab:cd:ef:01:02:03".to_string()))
            .unwrap();
        let channels_iter = ethctrl.connected_channels();
        assert_eq!(channels_iter.count(), 0);
        ethctrl
            .connect_physical_channel("Ecu_A_connector", &eth_channel)
            .unwrap();
        let channels_iter = ethctrl.connected_channels();
        assert_eq!(channels_iter.count(), 1);

        // local socket which belongs to Ecu_A
        let address_1 = NetworkEndpointAddress::IPv4 {
            address: Some("192.168.0.1".to_string()),
            address_source: Some(IPv4AddressSource::Fixed),
            default_gateway: Some("192.168.0.200".to_string()),
            network_mask: Some("255.255.255.0".to_string()),
        };
        let network_endpoint_1 = eth_channel
            .create_network_endpoint("local_endpoint", address_1, None)
            .unwrap();
        let tcp_port_1 = TpConfig::TcpTp {
            port_number: Some(1234),
            port_dynamically_assigned: None,
        };
        let socket_type_1 = SocketAddressType::Unicast(Some(ecu_instance.clone()));
        let socket_address_1 = eth_channel
            .create_socket_address("ServerSocket", &network_endpoint_1, &tcp_port_1, socket_type_1)
            .unwrap();
        assert_eq!(tcp_port_1, socket_address_1.get_tp_config().unwrap());
        let socket_type = socket_address_1.get_type().unwrap();
        assert!(matches!(socket_type, SocketAddressType::Unicast(Some(_))));
        if let SocketAddressType::Unicast(Some(ecu)) = socket_type {
            assert_eq!(ecu, ecu_instance);
        }

        // remote socket
        let address_2 = NetworkEndpointAddress::IPv4 {
            address: Some("192.168.0.2".to_string()),
            address_source: Some(IPv4AddressSource::Fixed),
            default_gateway: Some("192.168.0.200".to_string()),
            network_mask: Some("255.255.255.0".to_string()),
        };
        let network_endpoint_2 = eth_channel
            .create_network_endpoint("remote_endpoint", address_2, None)
            .unwrap();
        let tcp_port_2 = TpConfig::TcpTp {
            port_number: Some(5678),
            port_dynamically_assigned: None,
        };
        let socket_type_2 = SocketAddressType::Unicast(None);
        let socket_address_2 = eth_channel
            .create_socket_address("ClientSocket", &network_endpoint_2, &tcp_port_2, socket_type_2)
            .unwrap();

        // create a connection (V2)
        let (static_socket_connection_a, static_socket_connection_b) = eth_channel
            .create_static_socket_connection("StaticSocketConnection", &socket_address_1, &socket_address_2, None)
            .unwrap();

        // create a pdu and add it to the connection
        let pdu_package = ArPackage::get_or_create(&model, "/Network/Pdus").unwrap();
        let isignal_package = ArPackage::get_or_create(&model, "/Network/Signals").unwrap();
        let syssignal_package = ArPackage::get_or_create(&model, "/System/Signals").unwrap();
        let pdu = system.create_isignal_ipdu("Pdu_1", &pdu_package, 8).unwrap();
        let system_signal = SystemSignal::new("Signal_1", &syssignal_package).unwrap();
        let isignal = system
            .create_isignal("Signal_1", 8, &system_signal, None, &isignal_package)
            .unwrap();

        let ipdu_identifier_set_package = ArPackage::get_or_create(&model, "/Network/IpduIdentifierSets").unwrap();
        let socon_ipdu_identifier_set =
            SocketConnectionIpduIdentifierSet::new("IpduIdentifierSet", &ipdu_identifier_set_package).unwrap();
        let ipdu_identifier = socon_ipdu_identifier_set
            .create_socon_ipdu_identifier(
                "IpduIdentifier",
                &pdu.clone().into(),
                &eth_channel,
                Some(0x1000),
                None,
                None,
            )
            .unwrap();

        static_socket_connection_a
            .add_ipdu_identifier(&ipdu_identifier)
            .unwrap();
        static_socket_connection_b
            .add_ipdu_identifier(&ipdu_identifier)
            .unwrap();

        pdu.map_signal(
            &isignal,
            0,
            ByteOrder::MostSignificantByteLast,
            None,
            TransferProperty::Triggered,
        )
        .unwrap();

        let pdu_triggering = ipdu_identifier.pdu_triggering().unwrap();
        pdu_triggering
            .create_pdu_port(&ecu_instance, CommunicationDirection::Out)
            .unwrap();

        // software component modeling
        let swc_package = ArPackage::get_or_create(&model, "/SoftwareComponents").unwrap();
        let root_composition = CompositionSwComponentType::new("RootComposition", &swc_package).unwrap();

        // ... Todo: create other swc elements ...

        // add the root composition to the system
        system
            .set_root_sw_composition("EthernetTestComposition", &root_composition)
            .unwrap();

        println!("{}", model.files().next().unwrap().serialize().unwrap());
        model.write().unwrap();
    }

    #[test]
    fn create_flexray_system() {
        let model = AutosarModel::new();
        model.create_file("flexray.arxml", AutosarVersion::LATEST).unwrap();
        let system_package = ArPackage::get_or_create(&model, "/System").unwrap();
        let system = System::new("System", &system_package, SystemCategory::SystemExtract).unwrap();
        let cluster_package = ArPackage::get_or_create(&model, "/Network/Clusters").unwrap();

        let settings = FlexrayClusterSettings::default();
        let flx_cluster = system
            .create_flexray_cluster("FlxCluster", &cluster_package, &settings)
            .unwrap();
        assert_eq!(flx_cluster.element().element_name(), ElementName::FlexrayCluster);
        let flx_channel = flx_cluster
            .create_physical_channel("FlxChannel", FlexrayChannelName::A)
            .unwrap();

        let ecu_package = ArPackage::get_or_create(&model, "/Ecus").unwrap();

        // create ECU A and connect it to the Flexray channel
        let ecu_instance_a = system.create_ecu_instance("Ecu_A", &ecu_package).unwrap();
        let flxctrl_a = ecu_instance_a
            .create_flexray_communication_controller("FlexrayController")
            .unwrap();
        let channels_iter = flxctrl_a.connected_channels();
        assert_eq!(channels_iter.count(), 0);
        flxctrl_a
            .connect_physical_channel("Ecu_A_connector", &flx_channel)
            .unwrap();
        let channels_iter = flxctrl_a.connected_channels();
        assert_eq!(channels_iter.count(), 1);

        // create ECU B and connect it to the Flexray channel
        let ecu_instance_b = system.create_ecu_instance("Ecu_B", &ecu_package).unwrap();
        let flxctrl_b = ecu_instance_b
            .create_flexray_communication_controller("FlexrayController")
            .unwrap();
        flxctrl_b
            .connect_physical_channel("Ecu_B_connector", &flx_channel)
            .unwrap();

        let frame_package = ArPackage::get_or_create(&model, "/Network/Frames").unwrap();
        let pdu_package = ArPackage::get_or_create(&model, "/Network/Pdus").unwrap();

        // create Frame_1 which contains Pdu_1: Id 0x100, length 8
        let frame1 = system.create_flexray_frame("Frame_1", 32, &frame_package).unwrap();
        let pdu1 = system.create_isignal_ipdu("Pdu_1", &pdu_package, 8).unwrap();
        frame1
            .map_pdu(
                &pdu1,
                0,
                autosar_data_abstraction::ByteOrder::MostSignificantByteLast,
                None,
            )
            .unwrap();
        let frame_timing_1 = FlexrayCommunicationCycle::Repetition {
            base_cycle: 0,
            cycle_repetition: autosar_data_abstraction::communication::CycleRepetition::C1,
        };
        let ft_1 = flx_channel.trigger_frame(&frame1, 1, &frame_timing_1).unwrap();
        assert_eq!(frame1.frame_triggerings().count(), 1);
        assert_eq!(ft_1.pdu_triggerings().count(), 1);

        // create Frame_2 which contains Pdu_2: Id 0x101, length 8
        let frame2 = system.create_flexray_frame("Frame_2", 64, &frame_package).unwrap();
        let pdu2 = system.create_isignal_ipdu("Pdu_2", &pdu_package, 8).unwrap();
        frame2
            .map_pdu(
                &pdu2,
                0,
                autosar_data_abstraction::ByteOrder::MostSignificantByteLast,
                None,
            )
            .unwrap();
        let frame_timing_2 = FlexrayCommunicationCycle::Repetition {
            base_cycle: 0,
            cycle_repetition: autosar_data_abstraction::communication::CycleRepetition::C1,
        };
        let ft_2 = flx_channel.trigger_frame(&frame2, 2, &frame_timing_2).unwrap();

        // frame 1: Ecu_A -> Ecu_B
        ft_1.connect_to_ecu(&ecu_instance_a, CommunicationDirection::Out)
            .unwrap();
        ft_1.connect_to_ecu(&ecu_instance_b, CommunicationDirection::In)
            .unwrap();
        // frame 2: Ecu_B -> Ecu_A
        ft_2.connect_to_ecu(&ecu_instance_a, CommunicationDirection::In)
            .unwrap();
        ft_2.connect_to_ecu(&ecu_instance_b, CommunicationDirection::Out)
            .unwrap();

        // software component modeling
        let swc_package = ArPackage::get_or_create(&model, "/SoftwareComponents").unwrap();
        let root_composition = CompositionSwComponentType::new("RootComposition", &swc_package).unwrap();

        // ... Todo: create other swc elements ...

        // add the root composition to the system
        system
            .set_root_sw_composition("FlexrayTestComposition", &root_composition)
            .unwrap();

        println!("{}", model.files().next().unwrap().serialize().unwrap());
        // model.write().unwrap();
    }
}
