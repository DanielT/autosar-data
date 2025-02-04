# Changelog

## Version 0.18.0

Released 2025-02-04

### API

- Element::set_attribute_value now behaves like Element::set_character_data, in that it accepts any parameter that can be converted to a CharacterData object

### Enhancements

- Debug diplay of multiple types has been improved
- Large arrays in the specification are now declared as static instead of const, which noticeable improves compilation speed

## Version 0.17.0

Released 2024-12-16

### API

- Added `CharacterData::parse_bool` and `impl From<bool> for CharacterData`
- Added `Element::remove_sub_element_kind()`, which equivalent to `get_sub_element(ElementName)` followed by `remove_sub_element()`
- Added extra info to the errors `InvalidSubElement` and `ElementInsertionConflict`

### Enhancements

- Removed the dependency on num_derive, which should make the build a bit faster - Thanks @shahn
- autosar-data now uses `#![warn(missing_docs)]`, and all items have been documented

## Version 0.16.0

Released 2024-11-30

### Feature

- Support Autosar version 24-11

## Version 0.15.1

Released 2024-10-28

### Fixes

- Fix a regression in the attribute parsing code, introduced in 0.15

## Version 0.15

Released 2024-10-24

### Api

- Allow specifying a max depth for elements_dfs - Thanks @shahn
- Add additional info the the errors `ElementInsertionConflict` and `ElementNotFound`

### Fixes

- when loading two or more files, the model merging code could hang in an endless loop
- XML allows attributes in single quotes, but the parser did not accept this

## Version 0.14

Released 2024-08-16

### API

- The usability of `Element::set_character_data` has been improved: it is no longer required to manually wrap the value in a CharacterData object.
- `CharacterData::decode_interger` has been renamed to `CharacterData::parse_integer`
- `CharacterData::Double` renamed to `CharacterData::Float`
- added `CharacterData::parse_float`
- The errors `ItemNameRequired`, `IncorrectContentType`, `InvalidSubElement`, and `DuplicateItemName` contain additional information
- implement PartialOrd / Ord for elements

### Enhancements

- sorting of elements is now more complete, and takes the CharacterData and sub-elements into account
- criterion has been added for benchmarking

### Fixes

- Add a fast path that dramatically speeds up a common case in element insertion

## Version 0.13

Released 2024-05-19.

### API

- add `AutosarModel::duplicate()`
    This creates a full copy of the model. It is useful because `clone()` is overridden to only create a new reference to the existing model.
- add `Element::named_parent()`
    Unlike `parent()` which only returns the direct parent of the element, `named_parent()` performs several steps until a named (identifiable) parent is found.
- add `CharacterData::decode_integer()`
    Many numbers in an arxml file are declared as strings, which allows them to be represented in hexadecimal (0x...) or binary (0b...) form in the serialized arxml file. This function parses these strings into integer values.
- add `Element::get_or_create_sub_element()` and `Element::get_or_create_named_sub_element()`
    Often it is only required that some sub element should exist, but it doesn't matter if an existing element is used, or if a new one is ceated. These functions simplify that use case.
- make `Element::min_version()` public
    It returns the `AutosarVersion` of file containing the element. The `Element` might be part of multiple arxml files in the merged model, and if this is the case, then the lowest version of all contining files is returned.
- modified `AutoarModel::identifiable_elements()` to return an iterator instead of a list.
    If a list is desired, then the user of the function can `collect()` the iterator.
- add `ElementType::verify_reference_dest()`
    It checks if the value of the DEST attribute in a reference is valid acoording to the specification.

### Fixes

- fix a parsing failure if a comment contained an embedded `>` character. Additionally, `Element::set_comment()` now checks if the new string contains `--` and substitiutes with `__` if it does: The XML spec prohibits `--` in XML comments.
- fix `AutosarModel::check_references()`, which reported some valid references as invalid.
- fix `ArxmlFile::elements_dfs()`, which would panic if it was called on a deleted file or model.

### Other

- mark several functions as `#[must_use]`
- Fix the formatting of many doc comments. For example the section "Possible errors" was renamed to "Errors", and many code items are now enclosed in backticks.
- Spelling fixes in verious comments - Thanks @b4skyx

---

## Version 0.12.1

Released 2024-02-01.

### Fixes

- fix `check_file()` and `check_buffer()`, which were broken if the arxml header contained a comment.

---

## Version 0.12

Released 2023-12-13.

### Features

- Support Autosar version R23-11
- Support comments in the model
    Comments are no longer discarded during parsing. One comment can be stored with each element.
- The data model of autosar-data-specification was improved. It now provides more information.
- autosar-data-specification has a create feature `docstrings`. If this is set, the create can provide the original doc strings from the Autosar meta model for each element type.

### API

- Comments can be modified with `Element::set_comment()` and retrieved with `Element::comment()`
- added `ElementType::std_restriction()`, which informs if an element is restricted to `AdaptivePlatform` or `ClassicPlatform`
- mark `AutosarDataError`  and several other enums as `#[non_exhaustive]`

### Fixes

- The information returned by `ElementType::is_ordered()` and `ElementType::is_splittable()` was previously inaccurate in some cases. This was fixed by improving the representation of this information in the data tables.

---

## Version 0.11.1

Released 2023-09-27.

### Fixes

- `Element::sort()` reversed the order of equally-ranked sub-elements.
- `Element::sort()` takes the `<INDEX>` of its sub-elements into account, if it exists.

---

## Version 0.11

Released 2023-09-16.
