use crate::*;
use autosar_data::{Element, ElementName};

mod applicationtype;
mod basetype;
mod compu_method;
mod implementationtype;
mod mapping;

pub use applicationtype::*;
pub use basetype::*;
pub use compu_method::*;
pub use implementationtype::*;
pub use mapping::*;

/// AutosarDataType is the abstract base class for all data types in the AUTOSAR metamodel.
///
/// It encapsulates both application data types and implementation data types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutosarDataType {
    ApplicationPrimitiveDataType(ApplicationPrimitiveDataType),
    ApplicationArrayDataType(ApplicationArrayDataType),
    ApplicationRecordDataType(ApplicationRecordDataType),
    ImplementationDataType(ImplementationDataType),
}

impl AbstractionElement for AutosarDataType {
    fn element(&self) -> &Element {
        match self {
            AutosarDataType::ApplicationPrimitiveDataType(data_type) => data_type.element(),
            AutosarDataType::ApplicationArrayDataType(data_type) => data_type.element(),
            AutosarDataType::ApplicationRecordDataType(data_type) => data_type.element(),
            AutosarDataType::ImplementationDataType(data_type) => data_type.element(),
        }
    }
}

impl TryFrom<Element> for AutosarDataType {
    type Error = AutosarAbstractionError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        match element.element_name() {
            ElementName::ApplicationPrimitiveDataType => Ok(ApplicationPrimitiveDataType::try_from(element)?.into()),
            ElementName::ApplicationArrayDataType => Ok(ApplicationArrayDataType::try_from(element)?.into()),
            ElementName::ApplicationRecordDataType => Ok(ApplicationRecordDataType::try_from(element)?.into()),
            ElementName::ImplementationDataType => Ok(ImplementationDataType::try_from(element)?.into()),
            _ => Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "AutosarDataType".to_string(),
            }),
        }
    }
}

impl From<ApplicationPrimitiveDataType> for AutosarDataType {
    fn from(data_type: ApplicationPrimitiveDataType) -> Self {
        AutosarDataType::ApplicationPrimitiveDataType(data_type)
    }
}

impl From<ApplicationArrayDataType> for AutosarDataType {
    fn from(data_type: ApplicationArrayDataType) -> Self {
        AutosarDataType::ApplicationArrayDataType(data_type)
    }
}

impl From<ApplicationRecordDataType> for AutosarDataType {
    fn from(data_type: ApplicationRecordDataType) -> Self {
        AutosarDataType::ApplicationRecordDataType(data_type)
    }
}

impl From<ImplementationDataType> for AutosarDataType {
    fn from(data_type: ImplementationDataType) -> Self {
        AutosarDataType::ImplementationDataType(data_type)
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Unit(Element);
abstraction_element!(Unit, Unit);

impl Unit {
    /// Create a new unit
    pub fn new(name: &str, package: &ArPackage, display_name: Option<&str>) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let unit = elements.create_named_sub_element(ElementName::Unit, name)?;

        if let Some(display_name) = display_name {
            unit.create_sub_element(ElementName::DisplayName)?
                .set_character_data(display_name)?;
        }

        Ok(Self(unit))
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

    /// Create a data constraint rule
    pub fn create_data_constr_rule(
        &self,
        rule_type: DataConstrType,
        lower_limit: Option<f64>,
        upper_limit: Option<f64>,
    ) -> Result<DataConstrRule, AutosarAbstractionError> {
        let data_constr_rules = self.element().get_or_create_sub_element(ElementName::DataConstrRules)?;
        let rule = DataConstrRule::new(&data_constr_rules, rule_type, lower_limit, upper_limit)?;
        Ok(rule)
    }

    /// Get all data constraint rules
    pub fn data_constr_rules(&self) -> impl Iterator<Item = DataConstrRule> {
        DataConstrRulesIterator::new(self.element().get_sub_element(ElementName::DataConstrRules))
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataConstrRule(Element);
abstraction_element!(DataConstrRule, DataConstrRule);

impl DataConstrRule {
    pub(crate) fn new(
        parent: &Element,
        rule_type: DataConstrType,
        lower_limit: Option<f64>,
        upper_limit: Option<f64>,
    ) -> Result<Self, AutosarAbstractionError> {
        let rule = parent.create_sub_element(ElementName::DataConstrRule)?;
        let constrs = match rule_type {
            DataConstrType::Internal => rule.create_sub_element(ElementName::InternalConstrs)?,
            DataConstrType::Physical => rule.create_sub_element(ElementName::PhysConstrs)?,
        };

        if let Some(lower_limit) = lower_limit {
            constrs
                .create_sub_element(ElementName::LowerLimit)?
                .set_character_data(lower_limit)?;
        }

        if let Some(upper_limit) = upper_limit {
            constrs
                .create_sub_element(ElementName::UpperLimit)?
                .set_character_data(upper_limit)?;
        }

        Ok(Self(rule))
    }

    /// get the constraint type
    pub fn rule_type(&self) -> DataConstrType {
        if self.element().get_sub_element(ElementName::InternalConstrs).is_some() {
            DataConstrType::Internal
        } else {
            DataConstrType::Physical
        }
    }

    /// get the lower limit
    pub fn lower_limit(&self) -> Option<f64> {
        let cdata = self
            .element()
            .get_sub_element(ElementName::InternalConstrs)
            .or_else(|| self.element().get_sub_element(ElementName::PhysConstrs))
            .and_then(|constrs| constrs.get_sub_element(ElementName::LowerLimit))
            .and_then(|limit| limit.character_data())?;

        if let Some(value) = cdata.parse_integer::<i64>() {
            Some(value as f64)
        } else {
            cdata.string_value().and_then(|s| s.parse::<f64>().ok())
        }
    }

    /// get the upper limit
    pub fn upper_limit(&self) -> Option<f64> {
        let cdata = self
            .element()
            .get_sub_element(ElementName::InternalConstrs)
            .or_else(|| self.element().get_sub_element(ElementName::PhysConstrs))
            .and_then(|constrs| constrs.get_sub_element(ElementName::UpperLimit))
            .and_then(|limit| limit.character_data())?;

        if let Some(value) = cdata.parse_integer::<i64>() {
            Some(value as f64)
        } else {
            cdata.string_value().and_then(|s| s.parse::<f64>().ok())
        }
    }
}

//#########################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataConstrType {
    Internal,
    Physical,
}

//#########################################################

element_iterator!(DataConstrRulesIterator, DataConstrRule, Some);

//#########################################################

#[cfg(test)]
mod test {
    use super::*;
    use autosar_data::AutosarVersion;

    #[test]
    fn data_constr() {
        let model = AutosarModel::new();
        let _file = model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/DataConstraints").unwrap();

        let data_constr = DataConstr::new("DataConstr", &package).unwrap();

        let rule1 = data_constr
            .create_data_constr_rule(DataConstrType::Internal, Some(1.0), Some(100.0))
            .unwrap();
        assert_eq!(rule1.rule_type(), DataConstrType::Internal);
        assert_eq!(rule1.lower_limit(), Some(1.0));
        assert_eq!(rule1.upper_limit(), Some(100.0));

        let rule2 = data_constr
            .create_data_constr_rule(DataConstrType::Physical, Some(2.0), Some(200.0))
            .unwrap();
        assert_eq!(rule2.rule_type(), DataConstrType::Physical);
        assert_eq!(rule2.lower_limit(), Some(2.0));
        assert_eq!(rule2.upper_limit(), Some(200.0));

        let rules = data_constr.data_constr_rules().collect::<Vec<_>>();
        assert_eq!(rules.len(), 2);
    }
}
