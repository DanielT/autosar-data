# `autosar-data`

[![Crates.io](https://img.shields.io/crates/v/autosar-data)](https://crates.io/crates/autosar-data)
[![Github Actions](https://github.com/DanielT/autosar-data/workflows/Test/badge.svg)](https://github.com/DanielT/autosar-data/actions)
[![codecov](https://codecov.io/gh/DanielT/autosar-data/branch/main/graph/badge.svg?token=RGKUUJTWZ5)](https://codecov.io/gh/DanielT/autosar-data)

This crate provides functionality to read, modify and write Autosar 4 arxml files,
both separately and in projects consisting of multiple files.

## Features

- read and write arxml files
- fully validate all data when it is loaded
- non-strict mode so that invalid but structurally sound data can be loaded
- various element operations to modify and create sub-elements, data and attributes
- support for Autosar paths and cross references
- all operations are thread safe, e.g. it is possible to load mutliple files on separate threads
- supports Autosar version 4.0.1 and up.

## Example

```rust
use autosar_data::*;

/* load a multi-file data model */
let model = AutosarModel::new();
let (file_1, warnings_1) = model.load_file("some_file.arxml", false)?;
let (file_2, warnings_2) = model.load_file("other_file.arxml", false)?;

/* load a buffer */
let (file_3, _) = model.load_buffer(buffer, "filename.arxml", true)?;

/* write all files of the model */
model.write()?;

/* alternatively: */
for file in model.files() {
    let file_data = file.serialize();
    // do something with file_data
}

/* iterate over all elements in all files */
for (depth, element) in model.elements_dfs() {
    if element.is_identifiable() {
        /* the element is identifiable using an Autosar path */
        println!("{depth}: {}, {}", element.element_name(), element.path()?);
    } else {
        println!("{depth}: {}", element.element_name());
    }
}

/* get an element by its Autosar path */
let pdu_element = model.get_element_by_path("/Package/Mid/PduName").unwrap();

/* work with the content of elements */
if let Some(length) = pdu_element
    .get_sub_element(ElementName::Length)
    .and_then(|elem| elem.character_data())
    .and_then(|cdata| cdata.string_value())
{
    println!("Pdu Length: {length}");
}

/* modify the attributes of an element */
pdu_element.set_attribute_string(AttributeName::Uuid, "12ab34cd-1234-1234-1234-12ab34cd56ef");
pdu_element.remove_attribute(AttributeName::Uuid);
```

## Example Programs

Two complete example programs can be found in the examples directory of the source repostitory. They are:

- businfo, which extracts information about bus settings, frames, pdus and signals from an autosar ECU extract
- generate_files, which for each Autosar version generates an arxml file containing a least one instance of every specified element
