#[cfg(test)]
mod test {
    use autosar_data::{AutosarModel, AutosarVersion, ElementName};
    use autosar_data_abstraction::{
        communication::{
            CommunicationDirection, FlexrayChannelName, FlexrayClusterSettings, FlexrayCommunicationCycle,
        },
        software_component::CompositionSwComponentType,
        AbstractionElement, ArPackage, System, SystemCategory,
    };

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
