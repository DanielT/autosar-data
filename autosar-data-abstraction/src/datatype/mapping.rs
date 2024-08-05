use crate::*;
use autosar_data::ElementName;
use datatype::{ApplicationDataType, ImplementationDataType};

/// A [`DataTypeMappingSet`] contains `DataTypeMap`s
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataTypeMappingSet(Element);
abstraction_element!(DataTypeMappingSet, DataTypeMappingSet);

impl DataTypeMappingSet {
    /// Create a new DataTypeMappingSet
    pub fn new(name: &str, package: &ArPackage) -> Result<Self, AutosarAbstractionError> {
        let elements = package.element().get_or_create_sub_element(ElementName::Elements)?;
        let mapping_set = elements.create_named_sub_element(ElementName::DataTypeMappingSet, name)?;

        Ok(Self(mapping_set))
    }

    /// Add a new `DataTypeMap` to the `DataTypeMappingSet`
    pub fn add_data_type_map(
        &self,
        implementation_data_type: &ImplementationDataType,
        application_data_type: &ApplicationDataType,
    ) -> Result<DataTypeMap, AutosarAbstractionError> {
        let data_type_map = DataTypeMap::new(self.element(), implementation_data_type, application_data_type)?;
        Ok(data_type_map)
    }

    pub fn data_type_maps(&self) -> DataTypeMapIterator {
        DataTypeMapIterator::new(self.element().get_sub_element(ElementName::DataTypeMaps))
    }
}

//#########################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataTypeMap(Element);
abstraction_element!(DataTypeMap, DataTypeMap);

impl DataTypeMap {
    /// Create a new DataTypeMap
    fn new(
        parent: &Element,
        implementation_data_type: &ImplementationDataType,
        application_data_type: &ApplicationDataType,
    ) -> Result<Self, AutosarAbstractionError> {
        let maps = parent.get_or_create_sub_element(ElementName::DataTypeMaps)?;
        let data_type_map = maps.create_sub_element(ElementName::DataTypeMap)?;

        data_type_map
            .create_sub_element(ElementName::ApplicationDataTypeRef)?
            .set_reference_target(application_data_type.element())?;
        data_type_map
            .create_sub_element(ElementName::ImplementationDataTypeRef)?
            .set_reference_target(implementation_data_type.element())?;

        Ok(Self(data_type_map))
    }

    /// Get the `ImplementationDataType` of the `DataTypeMap`
    pub fn implementation_data_type(&self) -> Option<ImplementationDataType> {
        self.element()
            .get_sub_element(ElementName::ImplementationDataTypeRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }

    /// Get the `ApplicationDataType` of the `DataTypeMap`
    pub fn application_data_type(&self) -> Option<ApplicationDataType> {
        self.element()
            .get_sub_element(ElementName::ApplicationDataTypeRef)?
            .get_reference_target()
            .ok()?
            .try_into()
            .ok()
    }
}

//#########################################################

element_iterator!(DataTypeMapIterator, DataTypeMap, Some);

//#########################################################

#[cfg(test)]
mod tests {
    use super::*;
    use autosar_data::AutosarVersion;
    use datatype::{
        ApplicationPrimitiveCategory, ApplicationPrimitiveDataType, BaseTypeEncoding, ImplementationDataTypeSettings,
        SwBaseType,
    };

    #[test]
    fn test_data_type_mapping_set() {
        let model = AutosarModel::new();
        let _file = model.create_file("test.arxml", AutosarVersion::LATEST).unwrap();
        let package = ArPackage::get_or_create(&model, "/DataTypeMappingSets").unwrap();
        let mapping_set = DataTypeMappingSet::new("MappingSet", &package).unwrap();

        // create an implementation data type
        let base_type =
            SwBaseType::new("uint8", &package, 8, BaseTypeEncoding::None, None, None, Some("uint8")).unwrap();
        let impl_data_type = ImplementationDataType::new(
            &package,
            ImplementationDataTypeSettings::Value {
                name: "ImplDataType".to_string(),
                base_type: base_type.clone(),
                compu_method: None,
                data_constraint: None,
            },
        )
        .unwrap();
        // create an application data type
        let app_data_type = ApplicationPrimitiveDataType::new(
            "AppDataType",
            &package,
            ApplicationPrimitiveCategory::Value,
            None,
            None,
            None,
        )
        .unwrap()
        .into();

        let data_type_map = mapping_set.add_data_type_map(&impl_data_type, &app_data_type).unwrap();

        assert_eq!(data_type_map.implementation_data_type().unwrap(), impl_data_type);
        assert_eq!(data_type_map.application_data_type().unwrap(), app_data_type);

        assert_eq!(mapping_set.data_type_maps().count(), 1);
    }
}
