use crate::{AbstractionElement, System};

mod can;
mod ethernet;
mod flexray;

use autosar_data::ElementName;
pub use can::*;
pub use ethernet::*;
pub use flexray::*;

//##################################################################

pub trait AbstractCluster: AbstractionElement {
    fn system(&self) -> Option<System> {
        if let Ok(model) = self.element().model() {
            let path = self.element().path().ok()?;
            let refs = model.get_references_to(&path);

            if let Some(system) = refs
                .iter()
                .filter_map(|weak| weak.upgrade())
                .filter(|elem| elem.element_name() == ElementName::FibexElementRef)
                .filter_map(|elem| elem.named_parent().ok().flatten())
                .filter_map(|parent| System::try_from(parent).ok())
                .next()
            {
                return Some(system);
            }
        }
        None
    }
}

//##################################################################

/// A [`Cluster`] is returned by [`System::clusters`].
/// It can contain any supported communication cluster.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Cluster {
    /// The Cluster is a [`CanCluster`]
    Can(CanCluster),
    /// The Cluster is an [`EthernetCluster`]
    Ethernet(EthernetCluster),
    /// The Cluster is a [`FlexrayCluster`]
    FlexRay(FlexrayCluster),
    // missing: Lin, TTCAN, J1939, CDD (aka user defined)
}

impl AbstractionElement for Cluster {
    fn element(&self) -> &autosar_data::Element {
        match self {
            Cluster::Can(cancluster) => cancluster.element(),
            Cluster::Ethernet(ethcluster) => ethcluster.element(),
            Cluster::FlexRay(flxcluster) => flxcluster.element(),
        }
    }
}

impl AbstractCluster for Cluster {}

impl From<CanCluster> for Cluster {
    fn from(value: CanCluster) -> Self {
        Cluster::Can(value)
    }
}

impl From<EthernetCluster> for Cluster {
    fn from(value: EthernetCluster) -> Self {
        Cluster::Ethernet(value)
    }
}

impl From<FlexrayCluster> for Cluster {
    fn from(value: FlexrayCluster) -> Self {
        Cluster::FlexRay(value)
    }
}

//##################################################################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ArPackage;
    use autosar_data::{AutosarModel, AutosarVersion};

    #[test]
    fn cluster_system() {
        let model = AutosarModel::new();
        let _file = model.create_file("test.arxml", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/Test").unwrap();
        let system = System::new("System", &package, crate::SystemCategory::EcuExtract).unwrap();
        let settings = CanClusterSettings::default();
        let can_cluster = CanCluster::new("CanCluster", &package, &settings).unwrap();

        assert!(can_cluster.system().is_none());
        system.create_fibex_element_ref(can_cluster.element()).unwrap();
        assert_eq!(can_cluster.system().unwrap(), system);
    }

    #[test]
    fn cluster_conversion() {
        let model = AutosarModel::new();
        let _file = model.create_file("test.arxml", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/Test").unwrap();
        let can_settings = CanClusterSettings::default();
        let can_cluster = CanCluster::new("CanCluster", &package, &can_settings).unwrap();
        let ethernet_cluster = EthernetCluster::new("EthernetCluster", &package).unwrap();
        let flexray_settings = FlexrayClusterSettings::default();
        let flexray_cluster = FlexrayCluster::new("FlexrayCluster", &package, &flexray_settings).unwrap();

        let can: Cluster = can_cluster.into();
        let ethernet: Cluster = ethernet_cluster.into();
        let flexray: Cluster = flexray_cluster.into();

        assert_eq!(can.element().item_name().unwrap(), "CanCluster");
        assert_eq!(ethernet.element().item_name().unwrap(), "EthernetCluster");
        assert_eq!(flexray.element().item_name().unwrap(), "FlexrayCluster");
    }
}
