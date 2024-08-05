use crate::*;
use autosar_data::ElementName;
use datatype::{CompuMethod, DataConstr, Unit};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationArrayDataType(Element);
abstraction_element!(ApplicationArrayDataType, ApplicationArrayDataType);

impl ApplicationArrayDataType {
    pub fn new(
        name: &str,
        package: &ArPackage,
        element_type: &ApplicationDataType,
        max_num_elements: u64,
    ) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let application_array_data_type =
            elements.create_named_sub_element(ElementName::ApplicationArrayDataType, name)?;

        application_array_data_type
            .create_sub_element(ElementName::Category)?
            .set_character_data("ARRAY")?;

        let application_array_data_type = Self(application_array_data_type);
        ApplicationArrayElement::new("Element", &application_array_data_type, element_type, max_num_elements)?;

        Ok(application_array_data_type)
    }

    /// set the array element type of the array data type
    pub fn set_array_element(
        &self,
        element_type: &ApplicationDataType,
    ) -> Result<ApplicationArrayElement, AutosarAbstractionError> {
        let mut max_num_elements = 0;
        if let Some(element) = self.element().get_sub_element(ElementName::Element) {
            max_num_elements = element
                .get_sub_element(ElementName::MaxNumberOfElements)
                .and_then(|mnoe| mnoe.character_data())
                .and_then(|cdata| cdata.parse_integer())
                .unwrap_or(0);
            let _ = self.element().remove_sub_element(element);
        }

        ApplicationArrayElement::new("Element", self, element_type, max_num_elements)
    }

    /// get the array element of the array data type
    pub fn array_element(&self) -> Option<ApplicationArrayElement> {
        self.element().get_sub_element(ElementName::Element)?.try_into().ok()
    }

    /// get the max number of elements of the array data type
    pub fn max_number_of_elements(&self) -> Option<u64> {
        self.element()
            .get_sub_element(ElementName::Element)?
            .get_sub_element(ElementName::MaxNumberOfElements)?
            .character_data()?
            .parse_integer()
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationArrayElement(Element);
abstraction_element!(ApplicationArrayElement, Element);

impl ApplicationArrayElement {
    fn new(
        name: &str,
        parent: &ApplicationArrayDataType,
        data_type: &ApplicationDataType,
        max_num_elements: u64,
    ) -> Result<Self, AutosarAbstractionError> {
        let application_array_element = parent.element().create_named_sub_element(ElementName::Element, name)?;

        let category = data_type.category().ok_or(AutosarAbstractionError::InvalidParameter(
            "array data_type has no category".to_string(),
        ))?;
        application_array_element
            .create_sub_element(ElementName::Category)?
            .set_character_data(category)?;

        application_array_element
            .create_sub_element(ElementName::TypeTref)?
            .set_reference_target(data_type.element())?;

        application_array_element
            .create_sub_element(ElementName::MaxNumberOfElements)?
            .set_character_data(max_num_elements)?;

        Ok(Self(application_array_element))
    }

    /// get the data type of the array element
    pub fn data_type(&self) -> Option<ApplicationDataType> {
        self.element()
            .get_sub_element(ElementName::TypeTref)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationRecordDataType(Element);
abstraction_element!(ApplicationRecordDataType, ApplicationRecordDataType);

impl ApplicationRecordDataType {
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let application_record_data_type =
            elements.create_named_sub_element(ElementName::ApplicationRecordDataType, name)?;

        application_record_data_type
            .create_sub_element(ElementName::Category)?
            .set_character_data("STRUCTURE")?;

        Ok(Self(application_record_data_type))
    }

    /// add a new element to the record data type
    pub fn add_record_element(
        &self,
        name: &str,
        data_type: &ApplicationDataType,
    ) -> Result<ApplicationRecordElement, AutosarAbstractionError> {
        ApplicationRecordElement::new(name, self, data_type)
    }

    ///get an iterator over the record elements of the record data type
    pub fn record_elements(&self) -> impl Iterator<Item = ApplicationRecordElement> {
        ApplicationRecordElementIterator::new(self.element().get_sub_element(ElementName::Elements))
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationRecordElement(Element);
abstraction_element!(ApplicationRecordElement, ApplicationRecordElement);

impl ApplicationRecordElement {
    fn new(
        name: &str,
        parent: &ApplicationRecordDataType,
        data_type: &ApplicationDataType,
    ) -> Result<Self, AutosarAbstractionError> {
        let application_record_element = parent
            .element()
            .get_or_create_sub_element(ElementName::Elements)?
            .create_named_sub_element(ElementName::ApplicationRecordElement, name)?;

        let category = data_type.category().ok_or(AutosarAbstractionError::InvalidParameter(
            "record element data_type has no category".to_string(),
        ))?;
        application_record_element
            .create_sub_element(ElementName::Category)?
            .set_character_data(category)?;

        application_record_element
            .create_sub_element(ElementName::TypeTref)?
            .set_reference_target(data_type.element())?;

        Ok(Self(application_record_element))
    }

    /// get the data type of the record element
    pub fn data_type(&self) -> Option<ApplicationDataType> {
        self.element()
            .get_sub_element(ElementName::TypeTref)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationPrimitiveDataType(Element);
abstraction_element!(ApplicationPrimitiveDataType, ApplicationPrimitiveDataType);

impl ApplicationPrimitiveDataType {
    pub fn new(
        name: &str,
        package: &ArPackage,
        category: ApplicationPrimitiveCategory,
        compu_method: Option<&CompuMethod>,
        unit: Option<&Unit>,
        data_constraint: Option<&DataConstr>,
    ) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let application_primitive_data_type =
            elements.create_named_sub_element(ElementName::ApplicationPrimitiveDataType, name)?;

        application_primitive_data_type
            .create_sub_element(ElementName::Category)?
            .set_character_data(category.to_string())?;

        let application_primitive_data_type = Self(application_primitive_data_type);

        if let Some(compu_method) = compu_method {
            application_primitive_data_type.set_compu_method(compu_method)?;
        }
        if let Some(unit) = unit {
            application_primitive_data_type.set_unit(unit)?;
        }
        if let Some(data_constraint) = data_constraint {
            application_primitive_data_type.set_data_constraint(data_constraint)?;
        }

        Ok(application_primitive_data_type)
    }

    /// set the compu method of the primitive data type
    pub fn set_compu_method(&self, compu_method: &CompuMethod) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::SwDataDefProps)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_or_create_sub_element(ElementName::CompuMethodRef)?
            .set_reference_target(compu_method.element())?;

        Ok(())
    }

    /// get the compu method of the primitive data type
    pub fn get_compu_method(&self) -> Option<CompuMethod> {
        self.element()
            .get_sub_element(ElementName::SwDataDefProps)?
            .get_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_sub_element(ElementName::CompuMethodRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }

    /// set the unit of the primitive data type
    pub fn set_unit(&self, unit: &Unit) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::SwDataDefProps)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_or_create_sub_element(ElementName::UnitRef)?
            .set_reference_target(unit.element())?;

        Ok(())
    }

    /// get the unit of the primitive data type
    pub fn get_unit(&self) -> Option<Unit> {
        self.element()
            .get_sub_element(ElementName::SwDataDefProps)?
            .get_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_sub_element(ElementName::UnitRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }

    /// set the data constraint of the primitive data type
    pub fn set_data_constraint(&self, data_constraint: &DataConstr) -> Result<(), AutosarAbstractionError> {
        self.element()
            .get_or_create_sub_element(ElementName::SwDataDefProps)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_or_create_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_or_create_sub_element(ElementName::DataConstrRef)?
            .set_reference_target(data_constraint.element())?;

        Ok(())
    }

    /// get the data constraint of the primitive data type
    pub fn get_data_constraint(&self) -> Option<DataConstr> {
        self.element()
            .get_sub_element(ElementName::SwDataDefProps)?
            .get_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_sub_element(ElementName::DataConstrRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }
}

//#########################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationPrimitiveCategory {
    Value,
    ValBlk,
    String,
    Boolean,
    ComAxis,
    ResAxis,
    Curve,
    Map,
    Cuboid,
    Cube4,
    Cube5,
}

impl std::fmt::Display for ApplicationPrimitiveCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApplicationPrimitiveCategory::Value => f.write_str("VALUE"),
            ApplicationPrimitiveCategory::ValBlk => f.write_str("VAL_BLK"),
            ApplicationPrimitiveCategory::String => f.write_str("STRING"),
            ApplicationPrimitiveCategory::Boolean => f.write_str("BOOLEAN"),
            ApplicationPrimitiveCategory::ComAxis => f.write_str("COM_AXIS"),
            ApplicationPrimitiveCategory::ResAxis => f.write_str("RES_AXIS"),
            ApplicationPrimitiveCategory::Curve => f.write_str("CURVE"),
            ApplicationPrimitiveCategory::Map => f.write_str("MAP"),
            ApplicationPrimitiveCategory::Cuboid => f.write_str("CUBOID"),
            ApplicationPrimitiveCategory::Cube4 => f.write_str("CUBE_4"),
            ApplicationPrimitiveCategory::Cube5 => f.write_str("CUBE_5"),
        }
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApplicationDataType {
    Array(ApplicationArrayDataType),
    Record(ApplicationRecordDataType),
    Primitive(ApplicationPrimitiveDataType),
}

impl AbstractionElement for ApplicationDataType {
    fn element(&self) -> &Element {
        match self {
            ApplicationDataType::Array(e) => e.element(),
            ApplicationDataType::Record(e) => e.element(),
            ApplicationDataType::Primitive(e) => e.element(),
        }
    }
}

impl TryFrom<Element> for ApplicationDataType {
    type Error = AutosarAbstractionError;

    fn try_from(element: Element) -> Result<Self, Self::Error> {
        match element.element_name() {
            ElementName::ApplicationArrayDataType => Ok(ApplicationDataType::Array(element.try_into()?)),
            ElementName::ApplicationRecordDataType => Ok(ApplicationDataType::Record(element.try_into()?)),
            ElementName::ApplicationPrimitiveDataType => Ok(ApplicationDataType::Primitive(element.try_into()?)),
            _ => Err(AutosarAbstractionError::ConversionError {
                element,
                dest: "ApplicationDataType".to_string(),
            }),
        }
    }
}

impl From<ApplicationPrimitiveDataType> for ApplicationDataType {
    fn from(val: ApplicationPrimitiveDataType) -> Self {
        ApplicationDataType::Primitive(val)
    }
}

impl From<ApplicationRecordDataType> for ApplicationDataType {
    fn from(val: ApplicationRecordDataType) -> Self {
        ApplicationDataType::Record(val)
    }
}

impl From<ApplicationArrayDataType> for ApplicationDataType {
    fn from(val: ApplicationArrayDataType) -> Self {
        ApplicationDataType::Array(val)
    }
}

impl ApplicationDataType {
    pub fn category(&self) -> Option<String> {
        self.element()
            .get_sub_element(ElementName::Category)?
            .character_data()?
            .string_value()
    }
}

//#########################################################

element_iterator!(ApplicationRecordElementIterator, ApplicationRecordElement, Some);

//#########################################################

#[cfg(test)]
mod tests {
    use super::*;
    use autosar_data::AutosarVersion;

    #[test]
    fn test_application_array_data_type() {
        let model = AutosarModel::new();
        let _file = model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/DataTypes").unwrap();
        let element_type = ApplicationPrimitiveDataType::new(
            "Element",
            &package,
            ApplicationPrimitiveCategory::Value,
            None,
            None,
            None,
        )
        .unwrap();
        let array_data_type =
            ApplicationArrayDataType::new("Array", &package, &element_type.clone().into(), 10).unwrap();

        assert_eq!(
            array_data_type.array_element().unwrap().data_type().unwrap(),
            ApplicationDataType::Primitive(element_type)
        );
        assert_eq!(array_data_type.max_number_of_elements().unwrap(), 10);
    }

    #[test]
    fn test_application_record_data_type() {
        let model = AutosarModel::new();
        let _file = model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/DataTypes").unwrap();
        let record_data_type = ApplicationRecordDataType::new("Record", &package).unwrap();
        let element_type = ApplicationPrimitiveDataType::new(
            "Element",
            &package,
            ApplicationPrimitiveCategory::Value,
            None,
            None,
            None,
        )
        .unwrap();
        let record_element = record_data_type
            .add_record_element("Element", &element_type.clone().into())
            .unwrap();

        assert_eq!(
            record_element.data_type().unwrap(),
            ApplicationDataType::Primitive(element_type)
        );
        assert_eq!(record_data_type.record_elements().next().unwrap(), record_element);
    }

    #[test]
    fn test_application_primitive_data_type() {
        let model = AutosarModel::new();
        let _file = model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/DataTypes").unwrap();
        let compu_method = CompuMethod::new("CompuMethod", &package).unwrap();
        let unit = Unit::new("Unit", &package).unwrap();
        let data_constraint = DataConstr::new("DataConstraint", &package).unwrap();
        let primitive_data_type = ApplicationPrimitiveDataType::new(
            "Primitive",
            &package,
            ApplicationPrimitiveCategory::Value,
            Some(&compu_method),
            Some(&unit),
            Some(&data_constraint),
        )
        .unwrap();

        assert_eq!(primitive_data_type.get_compu_method().unwrap(), compu_method);
        assert_eq!(primitive_data_type.get_unit().unwrap(), unit);
        assert_eq!(primitive_data_type.get_data_constraint().unwrap(), data_constraint);
    }
}
