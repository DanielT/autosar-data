# Changelog

## Version 0.23.1

### Fixes

- Replace the unmaintained fxhash with rustc_hash

## Version 0.23.0

### API

- remove `AutosarModel::set_version`: the version is a property of the individual files in a model. `ArxmlFile::set_version` should be used instead.
- `ElementType::get_sub_element_container_mode` and `ElementType::find_common_group` in autosar-data-specification now return an `Option`, and return `None` for invalid input instead of panicking

### Fixes

**Parsing**

- Fix a panix on an attribute value consisting only of whitespace (e.g. `UUID="   "`)
- Fix truncated values on unescaped `>` inside an attribute value (e.g. `<AR-PACKAGE S="a > b">`)
- Extra spaces around the `=` in an attribute and around the closing `>` of an end tag (`</ELEMENT   >`) are now accepted
- Duplicate attributes on an element are now detected, which is an error in strict mode and otherwise resolved by keeping the last value
- An empty SHORT-NAME (`<SHORT-NAME></SHORT-NAME>`, or one containing only whitespace or a comment) is now rejected with the new error `ArxmlParserError::EmptyShortName`.

**Concurrency**

- Fix a deadlock when passing an element to its own `remove_sub_element`, `move_element_here` or `move_element_here_at`; doing so now returns an error
- Upward lock acquisitions in `set_reference_target`, `path()` and the cycle checks of copy/move operations no longer deadlock against concurrent downward operations
- `serialize` and `get_references_to` no longer double-lock non-reentrantly
- Concurrent `load_buffer` calls into an empty model are now serialized, so they can no longer both succeed while one of the two loaded element trees is silently discarded

**Panics on invalid input**

- Fix panics on invalid or out-of-range input, which now return an error or `None` instead: `set_relative_reference_target` and `get_reference_target` for a reference element outside any package (e.g. an SDX-REF in the AUTOSAR root's ADMIN-DATA); `ElementType::get_sub_element_version_mask`, `get_sub_element_multiplicity`, `get_sub_element_container_mode` and `find_common_group` for an out-of-range index list; and `create_sub_element` and other insertion functions for a sibling element that a non-strict load retained even though it isn't valid in the file's version
- Fix `PartialEq`/`Ord` for elements, which violated the total ordering requirement and could panic on NaN values during `sort()`; NaN and Inf are now also rejected by the parser and by `set_character_data`, which reports the rejection as the more specific `InvalidCharacterData` instead of `IncorrectContentType`

**Multi-file models**

- Fix `Element::remove_from_file`, which left the model unusable when called for the root element of a single-file model; it now returns the new error `RootElementRemovalForbidden`. It also now returns `ShortNameRemovalForbidden`, instead of silently doing nothing, when asked to remove a SHORT-NAME from its only file
- Fix `remove_file` leaving a valid parent pointer on sub elements that were removed along with the file, which could confuse code that still held a reference to one of them.
- Fix file membership of elements moved across models with `move_element_here`: they no longer hold on to weak references to files in the old model, which previously caused them to be omitted when serializing
- Fix `AutosarModel::duplicate` for a model whose files use different Autosar versions: the copy was filtered by the oldest file version, silently dropping elements only valid in later versions and shifting file membership onto the wrong elements. Duplication is now unfiltered, and each file of the copy keeps the version of the original

**Merging**

- Fix data loss when merging a file: if two sub elements of a splittable element could not be merged and were kept side by side instead, every following sub element of that splittable element was silently skipped rather than merged
- Fix the recovery from a failed merge of two sub elements of a splittable element, which could leave one element in the content of two different parents at once
- Conflicting BSW parameters (same DEFINITION-REF, incompatible content) can no longer be merged side by side; the merge now fails instead
- Fix duplicate REFERENCE-BASE declarations left behind when the same REFERENCE-BASE exists in several files of a split model: the merge keeps only one of the elements, and removing it previously left a stale duplicate that could still resolve references

**Relative references**

- Relative-reference handling was reworked so REFERENCE-BASE declarations are read live from the element tree instead of a separate cache, and relative references are indexed by their resolved absolute target path alongside absolute references. This removes the staleness bugs above at the root, and makes `get_references_to`, `check_references` and `get_reference_target` cheaper for models that use relative references

### Enhancements

- `check_references` now also checks relative references: it reports a reference whose BASE attribute names an out-of-scope reference base, or whose relative path does not lead to an existing element
- Relative references now follow a renamed or moved target, the same way absolute references already did: `set_item_name` and `move_element_here`/`move_element_here_at` rewrite the relative path to keep pointing at the same element, including when the renamed/moved element is only an ancestor of the target, or is the package a REFERENCE-BASE points at. Note that the BASE attribute itself is never rewritten, so a reference cannot follow a target that is moved out of its reference base's subtree, and moving a reference to where a different base is in scope changes what it resolves to — use `check_references` to find references invalidated this way
- Merging multi-file models is much faster: the merge used a linear search per sub element, making it quadratic. Merging a file with 8000 packages into a model previously took ~6 seconds and now takes ~6 milliseconds
- Building a model element by element is about twice as fast. Benchmarking showed that most of the overhead is in locking, so now each element caches a reference to the model, avoiding many upward traversals though the model and taking much fewer locks per operation.
- A loaded model needs ~10% less memory.
- Regex validation of character data now uses hand-written code rather than generated DFA state tables, eliminating 54kB of static data in linked binaries. The hand written code is also faster in micro-benchmarks, but this doesn't show up in real world usage.

## Version 0.22.0

### Features

- Support for relative references using the BASE attribute and a REFERENCE-BASE

## Version 0.21.2

### Fixes

- Allow extra spaces in the XML header. For example, `<?xml  version = "1.0" encoding = "utf-8" ?>` is a valid header.

## Version 0.21.1

### Fixes

- revert automatic element ordering. There are some cases where the ordering of elements with different element nams is semantically meaningful.

## Version 0.21.0

Released 2025-12-05

### Feature

- Support Autosar version 25-11
- Automatically order elements by their element names

### Fixes

- Fix a bug in the model merge when loading multiple files.

## Version 0.20.0

Released 2025-04-16

- Fix AutosarModel::check_references(), which reported valid references as invalid
- fix a new clippy warning

## Version 0.19.0

Released 2025-03-15

- Update to edition 2024
- Stop locking dependency versions - Thanks @shahn

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
- Add additional info for the errors `ElementInsertionConflict` and `ElementNotFound`

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
