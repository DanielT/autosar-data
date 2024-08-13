use crate::*;
use autosar_data::ElementName;
use datatype::{CompuMethod, DataConstr, SwBaseType};
use std::fmt::Display;

pub trait AbstractImplementationDataType: AbstractionElement {
    /// get the category of this implementation data type
    fn category(&self) -> Option<ImplementationDataCategory> {
        self.element()
            .get_sub_element(ElementName::Category)?
            .character_data()?
            .string_value()?
            .as_str()
            .try_into()
            .ok()
    }

    /// create an iterator over the sub-elements of this implementation data type
    fn sub_elements(&self) -> ImplementationDataTypeElementIterator {
        ImplementationDataTypeElementIterator::new(self.element().get_sub_element(ElementName::SubElements))
    }

    /// get the SwBaseType of this implementation data type [category: VALUE]
    fn base_type(&self) -> Option<SwBaseType> {
        let category = self.category()?;
        if category != ImplementationDataCategory::Value {
            return None;
        }
        self.element()
            .get_sub_element(ElementName::SwDataDefProps)?
            .get_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_sub_element(ElementName::BaseTypeRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }

    /// get the CompuMethod of this implementation data type [category: VALUE, TYPE_REFERENCE]
    fn compu_method(&self) -> Option<CompuMethod> {
        let category = self.category()?;
        if category != ImplementationDataCategory::Value && category != ImplementationDataCategory::TypeReference {
            return None;
        }
        self.element()
            .get_sub_element(ElementName::SwDataDefProps)?
            .get_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_sub_element(ElementName::CompuMethodRef)
            .and_then(|cmref| cmref.get_reference_target().ok())
            .and_then(|refelem| refelem.try_into().ok())
    }

    /// get the DataConstr of this implementation data type [category: VALUE, TYPE_REFERENCE]
    fn data_constraint(&self) -> Option<DataConstr> {
        let category = self.category()?;
        if category != ImplementationDataCategory::Value && category != ImplementationDataCategory::TypeReference {
            return None;
        }
        self.element()
            .get_sub_element(ElementName::SwDataDefProps)?
            .get_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_sub_element(ElementName::DataConstrRef)
            .and_then(|dcref| dcref.get_reference_target().ok())
            .and_then(|refelem| refelem.try_into().ok())
    }

    /// get the referenced implementation data type [category: TYPE_REFERENCE]
    fn referenced_type(&self) -> Option<ImplementationDataType> {
        let category = self.category()?;
        if category != ImplementationDataCategory::TypeReference {
            return None;
        }
        self.element()
            .get_sub_element(ElementName::SwDataDefProps)?
            .get_sub_element(ElementName::SwDataDefPropsVariants)?
            .get_sub_element(ElementName::SwDataDefPropsConditional)?
            .get_sub_element(ElementName::ImplementationDataTypeRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }

    /// get the array size of this implementation data type [category: ARRAY]
    fn array_size(&self) -> Option<u32> {
        let category = self.category()?;
        if category != ImplementationDataCategory::Array {
            return None;
        }
        self.sub_elements()
            .next()?
            .element()
            .get_sub_element(ElementName::ArraySize)?
            .character_data()?
            .parse_integer()
    }

    /// get the settings of this implementation data type
    fn settings(&self) -> Option<ImplementationDataTypeSettings> {
        let category = self.category()?;
        match category {
            ImplementationDataCategory::Value => Some(ImplementationDataTypeSettings::Value {
                name: self.name()?,
                base_type: self.base_type()?,
                compu_method: self.compu_method(),
                data_constraint: self.data_constraint(),
            }),
            ImplementationDataCategory::Array => {
                let element_settings = self.sub_elements().next()?.settings()?;
                Some(ImplementationDataTypeSettings::Array {
                    name: self.name()?,
                    length: self.array_size()?,
                    element_type: Box::new(element_settings),
                })
            }
            ImplementationDataCategory::Structure => {
                let elements = self
                    .sub_elements()
                    .map(|elem| elem.settings())
                    .collect::<Option<Vec<_>>>()?;
                Some(ImplementationDataTypeSettings::Structure {
                    name: self.name()?,
                    elements,
                })
            }
            ImplementationDataCategory::Union => {
                let elements = self
                    .sub_elements()
                    .map(|elem| elem.settings())
                    .collect::<Option<Vec<_>>>()?;
                Some(ImplementationDataTypeSettings::Union {
                    name: self.name()?,
                    elements,
                })
            }
            ImplementationDataCategory::DataReference => {
                Some(ImplementationDataTypeSettings::DataReference { name: self.name()? })
            }
            ImplementationDataCategory::FunctionReference => {
                Some(ImplementationDataTypeSettings::FunctionReference { name: self.name()? })
            }
            ImplementationDataCategory::TypeReference => Some(ImplementationDataTypeSettings::TypeReference {
                name: self.name()?,
                reftype: self.referenced_type()?,
                compu_method: self.compu_method(),
                data_constraint: self.data_constraint(),
            }),
        }
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationDataType(Element);
abstraction_element!(ImplementationDataType, ImplementationDataType);

impl AbstractImplementationDataType for ImplementationDataType {}

impl ImplementationDataType {
    pub fn new(package: &ArPackage, settings: ImplementationDataTypeSettings) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let implementation_data_type =
            elements.create_named_sub_element(ElementName::ImplementationDataType, settings.name())?;

        apply_impl_data_settings(&implementation_data_type, &settings)?;

        Ok(Self(implementation_data_type))
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationDataTypeElement(Element);
abstraction_element!(ImplementationDataTypeElement, ImplementationDataTypeElement);

impl AbstractImplementationDataType for ImplementationDataTypeElement {}

impl ImplementationDataTypeElement {
    pub fn new(parent: &Element, settings: &ImplementationDataTypeSettings) -> Result<Self, AutosarAbstractionError> {
        let implementation_data_type_element =
            parent.create_named_sub_element(ElementName::ImplementationDataTypeElement, settings.name())?;

        apply_impl_data_settings(&implementation_data_type_element, settings)?;

        Ok(Self(implementation_data_type_element))
    }
}

//#########################################################

fn apply_impl_data_settings(
    element: &Element,
    settings: &ImplementationDataTypeSettings,
) -> Result<(), AutosarAbstractionError> {
    if let Some(cat_elem) = element.get_sub_element(ElementName::Category) {
        element.remove_sub_element(cat_elem)?;
    }
    if let Some(sub_elements_elem) = element.get_sub_element(ElementName::SubElements) {
        element.remove_sub_element(sub_elements_elem)?;
    }
    if let Some(sw_data_def_props_elem) = element.get_sub_element(ElementName::SwDataDefProps) {
        element.remove_sub_element(sw_data_def_props_elem)?;
    }

    match settings {
        ImplementationDataTypeSettings::Value {
            base_type,
            compu_method,
            data_constraint,
            ..
        } => {
            element
                .create_sub_element(ElementName::Category)?
                .set_character_data("VALUE")?;
            let sw_data_def_props = element
                .create_sub_element(ElementName::SwDataDefProps)?
                .create_sub_element(ElementName::SwDataDefPropsVariants)?
                .create_sub_element(ElementName::SwDataDefPropsConditional)?;
            sw_data_def_props
                .create_sub_element(ElementName::BaseTypeRef)?
                .set_reference_target(base_type.element())?;
            if let Some(compu_method) = compu_method {
                sw_data_def_props
                    .create_sub_element(ElementName::CompuMethodRef)?
                    .set_reference_target(compu_method.element())?;
            }
            if let Some(data_constraint) = data_constraint {
                sw_data_def_props
                    .create_sub_element(ElementName::DataConstrRef)?
                    .set_reference_target(data_constraint.element())?;
            }
        }
        ImplementationDataTypeSettings::Array {
            length, element_type, ..
        } => {
            element
                .create_sub_element(ElementName::Category)?
                .set_character_data("ARRAY")?;
            let sub_elements = element.get_or_create_sub_element(ElementName::SubElements)?;
            let array_element = ImplementationDataTypeElement::new(&sub_elements, element_type)?;
            array_element
                .element()
                .create_sub_element(ElementName::ArraySize)?
                .set_character_data(*length as u64)?;
            array_element
                .element()
                .create_sub_element(ElementName::ArraySizeSemantics)?
                .set_character_data(EnumItem::FixedSize)?;
        }
        ImplementationDataTypeSettings::Structure { elements, .. } => {
            element
                .create_sub_element(ElementName::Category)?
                .set_character_data("STRUCTURE")?;
            let sub_elements = element.get_or_create_sub_element(ElementName::SubElements)?;
            for sub_element in elements {
                ImplementationDataTypeElement::new(&sub_elements, sub_element)?;
            }
        }
        ImplementationDataTypeSettings::Union { elements, .. } => {
            element
                .create_sub_element(ElementName::Category)?
                .set_character_data("UNION")?;
            let sub_elements = element.get_or_create_sub_element(ElementName::SubElements)?;
            for sub_element in elements {
                ImplementationDataTypeElement::new(&sub_elements, sub_element)?;
            }
        }
        ImplementationDataTypeSettings::DataReference { .. } => {
            element
                .create_sub_element(ElementName::Category)?
                .set_character_data("DATA_REFERENCE")?;
        }
        ImplementationDataTypeSettings::FunctionReference { .. } => {
            element
                .create_sub_element(ElementName::Category)?
                .set_character_data("FUNCTION_REFERENCE")?;
        }
        ImplementationDataTypeSettings::TypeReference {
            reftype,
            compu_method,
            data_constraint,
            ..
        } => {
            element
                .create_sub_element(ElementName::Category)?
                .set_character_data("TYPE_REFERENCE")?;
            let sw_data_def_props = element
                .create_sub_element(ElementName::SwDataDefProps)?
                .create_sub_element(ElementName::SwDataDefPropsVariants)?
                .create_sub_element(ElementName::SwDataDefPropsConditional)?;
            sw_data_def_props
                .create_sub_element(ElementName::ImplementationDataTypeRef)?
                .set_reference_target(reftype.element())?;
            if let Some(compu_method) = compu_method {
                sw_data_def_props
                    .create_sub_element(ElementName::CompuMethodRef)?
                    .set_reference_target(compu_method.element())?;
            }
            if let Some(data_constraint) = data_constraint {
                sw_data_def_props
                    .create_sub_element(ElementName::DataConstrRef)?
                    .set_reference_target(data_constraint.element())?;
            }
        }
    }

    Ok(())
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImplementationDataCategory {
    Value,
    DataReference,
    FunctionReference,
    TypeReference,
    Structure,
    Union,
    Array,
}

impl Display for ImplementationDataCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImplementationDataCategory::Value => f.write_str("VALUE"),
            ImplementationDataCategory::DataReference => f.write_str("DATA_REFERENCE"),
            ImplementationDataCategory::FunctionReference => f.write_str("FUNCTION_REFERENCE"),
            ImplementationDataCategory::TypeReference => f.write_str("TYPE_REFERENCE"),
            ImplementationDataCategory::Structure => f.write_str("STRUCTURE"),
            ImplementationDataCategory::Union => f.write_str("UNION"),
            ImplementationDataCategory::Array => f.write_str("ARRAY"),
        }
    }
}

impl TryFrom<&str> for ImplementationDataCategory {
    type Error = AutosarAbstractionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "VALUE" => Ok(ImplementationDataCategory::Value),
            "DATA_REFERENCE" => Ok(ImplementationDataCategory::DataReference),
            "FUNCTION_REFERENCE" => Ok(ImplementationDataCategory::FunctionReference),
            "TYPE_REFERENCE" => Ok(ImplementationDataCategory::TypeReference),
            "STRUCTURE" => Ok(ImplementationDataCategory::Structure),
            "UNION" => Ok(ImplementationDataCategory::Union),
            "ARRAY" => Ok(ImplementationDataCategory::Array),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "ImplementationDataCategory".to_string(),
            }),
        }
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImplementationDataTypeSettings {
    Value {
        name: String,
        base_type: SwBaseType,
        compu_method: Option<CompuMethod>,
        data_constraint: Option<DataConstr>,
    },
    Array {
        name: String,
        length: u32,
        element_type: Box<ImplementationDataTypeSettings>,
    },
    Structure {
        name: String,
        elements: Vec<ImplementationDataTypeSettings>,
    },
    Union {
        name: String,
        elements: Vec<ImplementationDataTypeSettings>,
    },
    DataReference {
        name: String,
        // TODO: Add reference to the referenced data type
    },
    FunctionReference {
        name: String,
        // TODO: Add reference to the referenced function type
    },
    TypeReference {
        name: String,
        reftype: ImplementationDataType,
        compu_method: Option<CompuMethod>,
        data_constraint: Option<DataConstr>,
    },
}

impl ImplementationDataTypeSettings {
    pub fn name(&self) -> &str {
        match self {
            ImplementationDataTypeSettings::Value { name, .. } => name,
            ImplementationDataTypeSettings::Array { name, .. } => name,
            ImplementationDataTypeSettings::Structure { name, .. } => name,
            ImplementationDataTypeSettings::Union { name, .. } => name,
            ImplementationDataTypeSettings::DataReference { name, .. } => name,
            ImplementationDataTypeSettings::FunctionReference { name, .. } => name,
            ImplementationDataTypeSettings::TypeReference { name, .. } => name,
        }
    }
}

//#########################################################

element_iterator!(
    ImplementationDataTypeElementIterator,
    ImplementationDataTypeElement,
    Some
);

//#########################################################

#[cfg(test)]
mod tests {
    use super::*;
    use autosar_data::AutosarVersion;
    use datatype::{BaseTypeEncoding, CompuMethodCategory};

    #[test]
    fn test_impl_data_type() {
        let model = AutosarModel::new();
        let _file = model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/DataTypes").unwrap();
        let base_type =
            SwBaseType::new("uint8", &package, 8, BaseTypeEncoding::None, None, None, Some("uint8")).unwrap();
        let compu_method = CompuMethod::new_raw("linear", &package, CompuMethodCategory::Linear).unwrap();
        let data_constraint = DataConstr::new("constraint", &package).unwrap();
        let other_impl_data_type = ImplementationDataType::new(
            &package,
            ImplementationDataTypeSettings::Value {
                name: "OtherImplDataType".to_string(),
                base_type: base_type.clone(),
                compu_method: Some(compu_method.clone()),
                data_constraint: None,
            },
        )
        .unwrap();
        let settings = ImplementationDataTypeSettings::Structure {
            name: "Structure".to_string(),
            elements: vec![
                ImplementationDataTypeSettings::Value {
                    name: "MyImplDataType1".to_string(),
                    base_type: base_type.clone(),
                    compu_method: Some(compu_method.clone()),
                    data_constraint: Some(data_constraint.clone()),
                },
                ImplementationDataTypeSettings::Array {
                    name: "MyArray".to_string(),
                    length: 10,
                    element_type: Box::new(ImplementationDataTypeSettings::Value {
                        name: "MyImplDataType2".to_string(),
                        base_type: base_type.clone(),
                        compu_method: Some(compu_method.clone()),
                        data_constraint: None,
                    }),
                },
                ImplementationDataTypeSettings::TypeReference {
                    name: "ReferenceType".to_string(),
                    reftype: other_impl_data_type.clone(),
                    compu_method: Some(compu_method.clone()),
                    data_constraint: Some(data_constraint.clone()),
                },
            ],
        };
        let impl_data_type = ImplementationDataType::new(&package, settings.clone()).unwrap();

        assert_eq!(impl_data_type.category(), Some(ImplementationDataCategory::Structure));

        let sub_elements = impl_data_type.sub_elements().collect::<Vec<_>>();
        assert_eq!(sub_elements.len(), 3);
        assert_eq!(sub_elements[0].category(), Some(ImplementationDataCategory::Value));
        assert_eq!(sub_elements[1].category(), Some(ImplementationDataCategory::Array));
        assert_eq!(
            sub_elements[2].category(),
            Some(ImplementationDataCategory::TypeReference)
        );

        let settings2 = impl_data_type.settings().unwrap();
        assert_eq!(settings, settings2);
    }
}
