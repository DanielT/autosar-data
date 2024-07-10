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

            for system in refs
                .iter()
                .filter_map(|weak| weak.upgrade())
                .filter(|elem| elem.element_name() == ElementName::FibexElementRef)
                .filter_map(|elem| elem.named_parent().ok().flatten())
                .filter_map(|parent| System::try_from(parent).ok())
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
