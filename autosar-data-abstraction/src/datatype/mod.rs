use crate::*;
use autosar_data::{Element, ElementName};

mod applicationtype;
mod basetype;
mod implementationtype;
mod mapping;

pub use applicationtype::*;
pub use basetype::*;
pub use implementationtype::*;
pub use mapping::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Unit(Element);
abstraction_element!(Unit, Unit);

impl Unit {
    /// Create a new unit
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let unit = elements.create_named_sub_element(ElementName::Unit, name)?;

        Ok(Self(unit))
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompuMethod(Element);
abstraction_element!(CompuMethod, CompuMethod);

impl CompuMethod {
    /// Create a new CompuMethod
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let compu_method = elements.create_named_sub_element(ElementName::CompuMethod, name)?;

        Ok(Self(compu_method))
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataConstr(Element);
abstraction_element!(DataConstr, DataConstr);

impl DataConstr {
    /// Create a new data constraint
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let data_constr = elements.create_named_sub_element(ElementName::DataConstr, name)?;

        Ok(Self(data_constr))
    }
}
