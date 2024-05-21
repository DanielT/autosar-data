use autosar_data::{AutosarDataError, Element};
use thiserror::Error;

mod arpackage;
mod can;
mod ecuinstance;
mod ethernet;
mod flexray;
mod system;

pub use arpackage::ArPackage;
pub use can::*;
pub use ecuinstance::*;
pub use ethernet::*;
pub use flexray::*;
pub use system::*;

/// The error type `AutosarAbstractionError` wraps all errors from the crate
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AutosarAbstractionError {
    /// converting an autosar-data element to a class in the abstract model failed
    #[error("conversion error: could not convert {} to {}", .element.element_name(), dest)]
    ConversionError { element: Element, dest: String },

    /// `ModelError` wraps [`AutosarDataError`] errors from autosar-data operations, e.g.
    /// [`AutosarDataError::ItemDeleted`], [`AutosarDataError::IncorrectContentType`], ...
    #[error("model error: {}", .0)]
    ModelError(AutosarDataError),

    /// an invalid Autosar path was passed as a parameter
    #[error("invalid path: {}", .0)]
    InvalidPath(String),

    /// an item could not be created because another item already fulfills its role in the model
    #[error("the item already exists")]
    ItemAlreadyExists,

    /// the function parameter has an invalid value
    #[error("invalid parameter: {}", .0)]
    InvalidParameter(String),
}

impl From<AutosarDataError> for AutosarAbstractionError {
    fn from(err: AutosarDataError) -> Self {
        AutosarAbstractionError::ModelError(err)
    }
}

pub trait AbstractionElement {
    #[must_use]
    fn element(&self) -> &Element;

    #[must_use]
    fn name(&self) -> String {
        self.element().item_name().unwrap()
    }
}

macro_rules! abstraction_element {
    ($name: ident, $base_elem: ident) => {
        impl TryFrom<Element> for $name {
            type Error = AutosarAbstractionError;

            fn try_from(element: Element) -> Result<Self, Self::Error> {
                if element.element_name() == ElementName::$base_elem {
                    Ok($name(element))
                } else {
                    Err(AutosarAbstractionError::ConversionError {
                        element,
                        dest: stringify!($name).to_string(),
                    })
                }
            }
        }

        impl AbstractionElement for $name {
            fn element(&self) -> &Element {
                &self.0
            }
        }

        impl From<$name> for Element {
            fn from(val: $name) -> Self {
                val.0
            }
        }
    };
}

pub(crate) use abstraction_element;

#[cfg(test)]
mod test {
    use autosar_data::AutosarModel;

    use super::*;

    #[test]
    fn errors() {
        let model = AutosarModel::new();

        let err = AutosarAbstractionError::ConversionError {
            element: model.root_element(),
            dest: "TEST".to_string(),
        };
        let string = format!("{err}");
        assert!(!string.is_empty());

        let err = AutosarAbstractionError::InvalidPath("lorem ipsum".to_string());
        let string = format!("{err}");
        assert!(!string.is_empty());

        let err = AutosarAbstractionError::ItemAlreadyExists;
        let string = format!("{err}");
        assert!(!string.is_empty());
    }
}
