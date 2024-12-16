use crate::{abstraction_element, AbstractionElement, AutosarAbstractionError};
use autosar_data::{Element, ElementName};

mod can_tp;
mod doip_tp;
mod flexray_ar_tp;
mod flexray_tp;

pub use can_tp::*;
pub use doip_tp::*;
pub use flexray_ar_tp::*;
pub use flexray_tp::*;

//##################################################################

/// Represents an ECUs transport layer address on the referenced channel
///
/// The TpAddress element is used by FlexrayArTpConfig and FlexrayTpConfig
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TpAddress(Element);
abstraction_element!(TpAddress, TpAddress);

impl TpAddress {
    pub(crate) fn new(name: &str, parent: &Element, address: u32) -> Result<Self, AutosarAbstractionError> {
        let tp_address_elem = parent.create_named_sub_element(ElementName::TpAddress, name)?;
        tp_address_elem
            .create_sub_element(ElementName::TpAddress)?
            .set_character_data(address as u64)?;
        Ok(Self(tp_address_elem))
    }
}
