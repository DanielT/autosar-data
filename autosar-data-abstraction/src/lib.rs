use autosar_data::{AutosarDataError, AutosarModel, Element, EnumItem};
use thiserror::Error;

pub mod communication;

mod arpackage;
mod ecuinstance;
mod system;

pub use arpackage::ArPackage;
pub use ecuinstance::*;
pub use system::*;

/// The error type `AutosarAbstractionError` wraps all errors from the crate
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AutosarAbstractionError {
    /// converting an autosar-data element to a class in the abstract model failed
    #[error("conversion error: could not convert {} to {}", .element.element_name(), dest)]
    ConversionError { element: Element, dest: String },

    /// converting an autosar-data element to a class in the abstract model failed
    #[error("value conversion error: could not convert {} to {}", .value, .dest)]
    ValueConversionError { value: String, dest: String },

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

//#########################################################

pub trait AbstractionElement: Clone + PartialEq {
    #[must_use]
    fn element(&self) -> &Element;

    #[must_use]
    fn name(&self) -> Option<String> {
        self.element().item_name()
    }

    fn set_timestamp(&self) {
        todo!()
    }
}

macro_rules! abstraction_element {
    ($name: ident, $base_elem: ident) => {
        impl TryFrom<autosar_data::Element> for $name {
            type Error = AutosarAbstractionError;

            fn try_from(element: autosar_data::Element) -> Result<Self, Self::Error> {
                if element.element_name() == autosar_data::ElementName::$base_elem {
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
            fn element(&self) -> &autosar_data::Element {
                &self.0
            }
        }

        impl From<$name> for autosar_data::Element {
            fn from(val: $name) -> Self {
                val.0
            }
        }
    };
}

pub(crate) use abstraction_element;

//#########################################################

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    /// Most significant byte at the lowest address = big endian
    MostSignificantByteFirst,
    /// Most significant byte at the highest address = little endian
    MostSignificantByteLast,
    Opaque,
}

impl TryFrom<EnumItem> for ByteOrder {
    type Error = AutosarAbstractionError;

    fn try_from(value: EnumItem) -> Result<Self, Self::Error> {
        match value {
            EnumItem::MostSignificantByteFirst => Ok(ByteOrder::MostSignificantByteFirst),
            EnumItem::MostSignificantByteLast => Ok(ByteOrder::MostSignificantByteLast),
            EnumItem::Opaque => Ok(ByteOrder::Opaque),
            _ => Err(AutosarAbstractionError::ValueConversionError {
                value: value.to_string(),
                dest: "ByteOrder".to_string(),
            }),
        }
    }
}

impl From<ByteOrder> for EnumItem {
    fn from(value: ByteOrder) -> Self {
        match value {
            ByteOrder::MostSignificantByteFirst => EnumItem::MostSignificantByteFirst,
            ByteOrder::MostSignificantByteLast => EnumItem::MostSignificantByteLast,
            ByteOrder::Opaque => EnumItem::Opaque,
        }
    }
}

//#########################################################

macro_rules! element_iterator {
    ($name: ident, $output: ident, $closure: tt) => {
        #[doc(hidden)]
        pub struct $name {
            iter: Option<autosar_data::ElementsIterator>,
        }

        impl $name {
            pub(crate) fn new(elem_container: Option<Element>) -> Self {
                let iter = elem_container.map(|se| se.sub_elements());
                Self { iter }
            }
        }

        impl Iterator for $name {
            type Item = $output;

            fn next(&mut self) -> Option<Self::Item> {
                let iter = self.iter.as_mut()?;
                for element in iter.by_ref() {
                    if let Some(sub_element) = $closure(element) {
                        if let Ok(output_item) = $output::try_from(sub_element) {
                            return Some(output_item);
                        }
                    }
                }
                self.iter = None;
                None
            }
        }

        impl std::iter::FusedIterator for $name {}
    };
}

pub(crate) use element_iterator;

//#########################################################

macro_rules! reflist_iterator {
    ($name: ident, $output: ident) => {
        #[doc(hidden)]
        pub struct $name {
            items: Vec<autosar_data::WeakElement>,
            position: usize,
        }

        impl $name {
            pub(crate) fn new(items: Vec<autosar_data::WeakElement>) -> Self {
                Self { items, position: 0 }
            }
        }

        impl Iterator for $name {
            type Item = $output;

            fn next(&mut self) -> Option<Self::Item> {
                while self.position < self.items.len() {
                    if let Some(out) = self.items[self.position]
                        .upgrade()
                        .and_then(|ref_elem| ref_elem.named_parent().ok().flatten())
                        .and_then(|elem| $output::try_from(elem).ok())
                    {
                        self.position += 1;
                        return Some(out);
                    }
                    self.position += 1;
                }

                self.position = usize::MAX;
                None
            }
        }

        impl std::iter::FusedIterator for $name {}
    };
}

pub(crate) use reflist_iterator;

//##################################################################

pub(crate) fn make_unique_name(model: &AutosarModel, base_path: String, initial_name: String) -> String {
    let mut full_path = format!("{base_path}/{initial_name}");
    let mut name = initial_name.clone();
    let mut counter = 0;
    while model.get_element_by_path(&full_path).is_some() {
        counter += 1;
        name = format!("{initial_name}_{counter}");
        full_path = format!("{base_path}/{name}");
    }

    name
}

//#########################################################

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
