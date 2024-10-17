#[cfg(test)]
mod test {
    use autosar_data::{AutosarModel, AutosarVersion, ElementName};
    use autosar_data_abstraction::{
        communication::{
            CanAddressingMode, CanClusterSettings, CanFrameType, CommunicationDirection, SystemSignal, TransferProperty,
        },
        datatype::{BaseTypeEncoding, SwBaseType},
        software_component::CompositionSwComponentType,
        AbstractionElement, ArPackage, System, SystemCategory,
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
}
