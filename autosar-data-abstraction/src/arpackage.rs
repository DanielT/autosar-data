use autosar_data::{AutosarModel, Element, ElementName};

use crate::{abstraction_element, AbstactionElement, AutosarAbstractionError};

/// An ArPackage is an Autosar package, which can contain other packages or elements
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArPackage(Element);
abstraction_element!(ArPackage, ArPackage);

impl ArPackage {
    /// Get or create an autosar package for the given path
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # use autosar_data_abstraction::*;
    /// # let model = AutosarModel::new();
    /// # model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();
    /// let package = ArPackage::get_or_create(&model, "/pkg1").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`AutosarAbstractionError::InvalidPath`] The value in package_path was not an Autosar path
    /// - [`AutosarAbstractionError::ModelError`] An error occurred in the Autosar model
    pub fn get_or_create(model: &AutosarModel, package_path: &str) -> Result<Self, AutosarAbstractionError> {
        if let Some(pkg_elem) = model.get_element_by_path(package_path) {
            pkg_elem.try_into()
        } else {
            let mut parts_iter = package_path.split('/');
            let first_part = parts_iter.next();
            if first_part.is_none() || first_part.unwrap() != "" {
                return Err(AutosarAbstractionError::InvalidPath(package_path.to_string()));
            }
            let mut pkg_elem = model.root_element();
            for part in parts_iter {
                pkg_elem = pkg_elem
                    .get_or_create_sub_element(ElementName::ArPackages)?
                    .get_or_create_named_sub_element(ElementName::ArPackage, part)?;
            }
            pkg_elem.try_into()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{System, SystemCategory};

    use super::*;
    use autosar_data::AutosarVersion;

    #[test]
    fn package() {
        let model = AutosarModel::new();
        // can't do anything in an incomplete model: it has no files
        let result = ArPackage::get_or_create(&model, "/bad");
        assert!(result.is_err());

        model.create_file("filename", AutosarVersion::Autosar_00048).unwrap();

        // create a new package
        let result = ArPackage::get_or_create(&model, "/pkg1");
        assert!(result.is_ok());
        let package = result.unwrap();
        assert_eq!(package.name(), "pkg1");
        // get the existing package
        let result = ArPackage::get_or_create(&model, "/pkg1");
        assert!(result.is_ok());
        // create multiple levels
        let result = ArPackage::get_or_create(&model, "/level1/level2/level3");
        assert!(result.is_ok());
        let package = result.unwrap();
        assert_eq!(package.name(), "level3");

        // can't create a package due to an element name conflict
        let pkg = ArPackage::get_or_create(&model, "/test").unwrap();
        System::new("system", &pkg, SystemCategory::EcuExtract).unwrap();
        let result = ArPackage::get_or_create(&model, "/test/system");
        assert!(result.is_err());
        let result = ArPackage::get_or_create(&model, "/test/system/sub");
        assert!(result.is_err());

        // invalid path: does not start with '/'
        let result = ArPackage::get_or_create(&model, "hello world");
        assert!(result.is_err());

        // conversions
        let element: Element = pkg.into();
        let result = ArPackage::try_from(element);
        assert!(result.is_ok());
        let result = ArPackage::try_from(model.root_element());
        assert!(result.is_err());
    }
}
