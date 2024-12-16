use crate::*;
use autosar_data::ElementName;
use datatype::AutosarDataType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientServerInterface(pub(crate) Element);
abstraction_element!(ClientServerInterface, ClientServerInterface);

impl ClientServerInterface {
    /// Create a new ClientServerInterface
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let client_server_interface = elements.create_named_sub_element(ElementName::ClientServerInterface, name)?;

        Ok(Self(client_server_interface))
    }

    /// Add a possible error to the client server interface
    pub fn create_possible_error(
        &self,
        name: &str,
        error_code: u64,
    ) -> Result<ApplicationError, AutosarAbstractionError> {
        let possible_errors = self.element().get_or_create_sub_element(ElementName::PossibleErrors)?;
        ApplicationError::new(name, error_code, &possible_errors)
    }

    /// add an operation to the client server interface
    pub fn create_operation(&self, name: &str) -> Result<ClientServerOperation, AutosarAbstractionError> {
        let operations = self.element().get_or_create_sub_element(ElementName::Operations)?;
        ClientServerOperation::new(name, &operations)
    }

    /// iterate over all operations
    pub fn operations(&self) -> impl Iterator<Item = ClientServerOperation> {
        ClientServerOperationIterator::new(self.element().get_sub_element(ElementName::Operations))
    }

    /// iterate over all application errors
    pub fn possible_errors(&self) -> impl Iterator<Item = ApplicationError> {
        ApplicationErrorIterator::new(self.element().get_sub_element(ElementName::PossibleErrors))
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationError(Element);
abstraction_element!(ApplicationError, ApplicationError);

impl ApplicationError {
    /// Create a new ApplicationError
    fn new(name: &str, error_code: u64, parent_element: &Element) -> Result<Self, AutosarAbstractionError> {
        let application_error = parent_element.create_named_sub_element(ElementName::ApplicationError, name)?;
        application_error
            .create_sub_element(ElementName::ErrorCode)?
            .set_character_data(error_code)?;
        Ok(Self(application_error))
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientServerOperation(Element);
abstraction_element!(ClientServerOperation, ClientServerOperation);

impl ClientServerOperation {
    /// Create a new ClientServerOperation
    fn new(name: &str, parent_element: &Element) -> Result<Self, AutosarAbstractionError> {
        let operation = parent_element.create_named_sub_element(ElementName::ClientServerOperation, name)?;
        Ok(Self(operation))
    }

    /// Add an argument to the operation
    pub fn create_argument(
        &self,
        name: &str,
        data_type: &AutosarDataType,
        direction: ArgumentDirection,
    ) -> Result<ArgumentDataPrototype, AutosarAbstractionError> {
        let arguments = self.element().get_or_create_sub_element(ElementName::Arguments)?;
        ArgumentDataPrototype::new(name, &arguments, data_type.element(), direction)
    }

    /// add a reference to possible error to the operation
    pub fn add_possible_error_reference(&self, error: &ApplicationError) -> Result<(), AutosarAbstractionError> {
        if self.element().named_parent()? != error.element().named_parent()? {
            return Err(AutosarAbstractionError::InvalidParameter(
                "Error and operation must be in the same ClientServerInterface".to_string(),
            ));
        }

        let possible_errors = self.element().get_or_create_sub_element(ElementName::PossibleErrors)?;
        possible_errors
            .create_sub_element(ElementName::PossibleErrorRef)?
            .set_reference_target(error.element())?;
        Ok(())
    }

    /// iterate over all arguments
    pub fn arguments(&self) -> impl Iterator<Item = ArgumentDataPrototype> {
        ArgumentDataPrototypeIterator::new(self.element().get_sub_element(ElementName::Arguments))
    }
}

//##################################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgumentDirection {
    In,
    Out,
    InOut,
}

impl TryFrom<EnumItem> for ArgumentDirection {
    type Error = AutosarAbstractionError;

    fn try_from(item: EnumItem) -> Result<Self, Self::Error> {
        match item {
            EnumItem::In => Ok(ArgumentDirection::In),
            EnumItem::Out => Ok(ArgumentDirection::Out),
            EnumItem::Inout => Ok(ArgumentDirection::InOut),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: item.to_string(),
                dest: "ArgumentDirection".to_string(),
            }),
        }
    }
}

impl From<ArgumentDirection> for EnumItem {
    fn from(direction: ArgumentDirection) -> Self {
        match direction {
            ArgumentDirection::In => EnumItem::In,
            ArgumentDirection::Out => EnumItem::Out,
            ArgumentDirection::InOut => EnumItem::Inout,
        }
    }
}

//##################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgumentDataPrototype(Element);
abstraction_element!(ArgumentDataPrototype, ArgumentDataPrototype);

impl ArgumentDataPrototype {
    /// Create a new ArgumentDataPrototype
    fn new(
        name: &str,
        parent_element: &Element,
        data_type: &Element,
        direction: ArgumentDirection,
    ) -> Result<Self, AutosarAbstractionError> {
        let argument = parent_element.create_named_sub_element(ElementName::ArgumentDataPrototype, name)?;
        argument
            .create_sub_element(ElementName::TypeTref)?
            .set_reference_target(data_type)?;
        argument
            .create_sub_element(ElementName::Direction)?
            .set_character_data::<EnumItem>(direction.into())?;
        Ok(Self(argument))
    }
}

//##################################################################

element_iterator!(ClientServerOperationIterator, ClientServerOperation, Some);

//##################################################################

element_iterator!(ApplicationErrorIterator, ApplicationError, Some);

//##################################################################

element_iterator!(ArgumentDataPrototypeIterator, ArgumentDataPrototype, Some);
