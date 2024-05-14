#[cfg(test)]
mod test {
    use autosar_data::{AutosarModel, AutosarVersion, ElementName};
    use autosar_data_abstraction::{
        AbstactionElement, ArPackage, CanClusterSettings, EthernetVlanInfo, FlexrayChannelName, FlexrayClusterSettings,
        System, SystemCategory,
    };

    #[test]
    fn create_can_system() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package_1 = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        let system = System::new("System", &package_1, SystemCategory::SystemExtract).unwrap();
        let package_2 = ArPackage::get_or_create(&model, "/Clusters").unwrap();

        let mut settings = CanClusterSettings::default();
        settings.can_fd_baudrate = Some(2000000);
        let can_cluster = system.create_can_cluster("CanCluster", &package_2, &settings).unwrap();
        assert_eq!(can_cluster.element().element_name(), ElementName::CanCluster);
        let can_channel = can_cluster.create_physical_channel("CanChannel").unwrap();

        let package_3 = ArPackage::get_or_create(&model, "/Ecus").unwrap();
        let ecu_instance = system.create_ecu_instance("Ecu_A", &package_3).unwrap();
        let canctrl = ecu_instance
            .create_can_communication_controller("CanController")
            .unwrap();
        let channels_iter = canctrl.connected_channels();
        assert_eq!(channels_iter.count(), 0);
        canctrl
            .connect_physical_channel("Ecu_A_connector", &can_channel)
            .unwrap();
        let channels_iter = canctrl.connected_channels();
        assert_eq!(channels_iter.count(), 1);

        println!("{}", model.files().next().unwrap().serialize().unwrap());
    }

    #[test]
    fn create_ethernet_system() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
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

        println!("{}", model.files().next().unwrap().serialize().unwrap());
    }

    #[test]
    fn create_flexray_system() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package_1 = ArPackage::get_or_create(&model, "/SYSTEM").unwrap();
        let system = System::new("System", &package_1, SystemCategory::SystemExtract).unwrap();
        let package_2 = ArPackage::get_or_create(&model, "/Clusters").unwrap();

        let settings = FlexrayClusterSettings::default();
        let flx_cluster = system
            .create_flexray_cluster("FlxCluster", &package_2, &settings)
            .unwrap();
        assert_eq!(flx_cluster.element().element_name(), ElementName::FlexrayCluster);
        let flx_channel = flx_cluster
            .create_physical_channel("FlxChannel", FlexrayChannelName::A)
            .unwrap();

        let package_3 = ArPackage::get_or_create(&model, "/Ecus").unwrap();
        let ecu_instance = system.create_ecu_instance("Ecu_A", &package_3).unwrap();
        let flxctrl = ecu_instance
            .create_flexray_communication_controller("FlexRayController")
            .unwrap();
        flxctrl
            .connect_physical_channel("connection_name", &flx_channel)
            .unwrap();

        println!("{}", model.files().next().unwrap().serialize().unwrap());
    }
}
