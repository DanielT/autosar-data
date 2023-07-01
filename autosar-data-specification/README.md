# autosar-data-specification

[![Crates.io](https://img.shields.io/crates/v/autosar-data-specification)](https://crates.io/crates/autosar-data-specification)
[![Github Actions](https://github.com/DanielT/autosar-data/workflows/Test/badge.svg)](https://github.com/DanielT/autosar-data/actions)
[![codecov](https://codecov.io/gh/DanielT/autosar-data/branch/main/graph/badge.svg?token=RGKUUJTWZ5)](https://codecov.io/gh/DanielT/autosar-data)

This crate exists to support the autosar-data crate.

The Autosar data model is originally specified as .xsd files - one for each version of the standard.
All these separate xsd files were parsed into data structures and combined; this crate contains the
combined specification data of all 19 Autosar 4 standard revisions.

## Supported standards

| xsd filename      | description               |
|-------------------|---------------------------|
| AUTOSAR_4-0-1.xsd | AUTOSAR 4.0.1             |
| AUTOSAR_4-0-2.xsd | AUTOSAR 4.0.2             |
| AUTOSAR_4-0-3.xsd | AUTOSAR 4.0.3             |
| AUTOSAR_4-1-1.xsd | AUTOSAR 4.1.1             |
| AUTOSAR_4-1-2.xsd | AUTOSAR 4.1.2             |
| AUTOSAR_4-1-3.xsd | AUTOSAR 4.1.3             |
| AUTOSAR_4-2-1.xsd | AUTOSAR 4.2.1             |
| AUTOSAR_4-2-2.xsd | AUTOSAR 4.2.2             |
| AUTOSAR_4-3-0.xsd | AUTOSAR 4.3.0             |
| AUTOSAR_00042.xsd | AUTOSAR Adaptive 17-03    |
| AUTOSAR_00043.xsd | AUTOSAR Adaptive 17-10    |
| AUTOSAR_00044.xsd | AUTOSAR Classic 4.3.1     |
| AUTOSAR_00045.xsd | AUTOSAR Adaptive 18-03    |
| AUTOSAR_00046.xsd | AUTOSAR Classic 4.4.0 / Adaptive 18-10 |
| AUTOSAR_00047.xsd | AUTOSAR Adaptive 19-03    |
| AUTOSAR_00048.xsd | AUTOSAR 4.5.0             |
| AUTOSAR_00049.xsd | AUTOSAR R20-11            |
| AUTOSAR_00050.xsd | AUTOSAR R21-11            |
| AUTOSAR_00051.xsd | AUTOSAR R22-11            |

## Using the crate

The main datatype is the `ElementType`. The type of the `<AUTOSAR>` element at the root of every arxml file is
available as `ElementType::ROOT`.

## Example

```rust
use autosar_data_specification::*;
use std::str::FromStr;

let root_type = ElementType::ROOT;
// parsing an element
let element_name_text = "AR-PACKAGES";
let element_name = ElementName::from_str(element_name_text).unwrap();
assert_eq!(element_name, ElementName::ArPackages);

let version_mask = AutosarVersion::Autosar_4_3_0 as u32;
if let Some((element_type, index_list)) = root_type.find_sub_element(
    element_name,
    version_mask
) {
    // parsing an attribute
    let attribute_name = AttributeName::from_str("UUID").unwrap();
    if let Some(attribute_spec) = element_type.find_attribute_spec(attribute_name) {
        // ...
    }
    // ...
}
```
