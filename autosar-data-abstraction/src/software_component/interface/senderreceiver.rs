use crate::*;
use autosar_data::ElementName;
use datatype::AutosarDataType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SenderReceiverInterface(pub(crate) Element);
abstraction_element!(SenderReceiverInterface, SenderReceiverInterface);

impl SenderReceiverInterface {
    /// Create a new SenderReceiverInterface
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let sender_receiver_interface =
            elements.create_named_sub_element(ElementName::SenderReceiverInterface, name)?;

        Ok(Self(sender_receiver_interface))
    }

    /// Add a new data element to the sender receiver interface
    pub fn create_data_element<T: Into<AutosarDataType> + AbstractionElement>(
        &self,
        name: &str,
        data_type: &T,
    ) -> Result<VariableDataPrototype, AutosarAbstractionError> {
        let data_elements = self.element().get_or_create_sub_element(ElementName::DataElements)?;
        VariableDataPrototype::new(name, &data_elements, data_type.element())
    }

    /// iterate over all data elements
    pub fn data_elements(&self) -> impl Iterator<Item = VariableDataPrototype> {
        DataElementIterator::new(self.element().get_sub_element(ElementName::DataElements))
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableDataPrototype(Element);
abstraction_element!(VariableDataPrototype, VariableDataPrototype);

impl VariableDataPrototype {
    /// Create a new VariableDataPrototype
    fn new(name: &str, parent_element: &Element, data_type: &Element) -> Result<Self, AutosarAbstractionError> {
        let vdp = parent_element.create_named_sub_element(ElementName::VariableDataPrototype, name)?;
        vdp.create_sub_element(ElementName::TypeTref)?
            .set_reference_target(data_type)?;

        Ok(Self(vdp))
    }

    /// Get the interface containing the data element
    pub fn interface(&self) -> Result<SenderReceiverInterface, AutosarAbstractionError> {
        let named_parent = self.element().named_parent()?.unwrap();
        SenderReceiverInterface::try_from(named_parent)
    }
}

//##################################################################

element_iterator!(DataElementIterator, VariableDataPrototype, Some);
