use std::collections::HashSet;

use autosar_data::*;
use autosar_data_specification::CharacterDataSpec;

static VERSIONS: [AutosarVersion; 18] = [
    AutosarVersion::Autosar_4_0_1,
    AutosarVersion::Autosar_4_0_2,
    AutosarVersion::Autosar_4_0_3,
    AutosarVersion::Autosar_4_1_1,
    AutosarVersion::Autosar_4_1_2,
    AutosarVersion::Autosar_4_1_3,
    AutosarVersion::Autosar_4_2_1,
    AutosarVersion::Autosar_4_2_2,
    AutosarVersion::Autosar_4_3_0,
    AutosarVersion::Autosar_00042,
    AutosarVersion::Autosar_00043,
    AutosarVersion::Autosar_00044,
    AutosarVersion::Autosar_00045,
    AutosarVersion::Autosar_00046,
    AutosarVersion::Autosar_00047,
    AutosarVersion::Autosar_00048,
    AutosarVersion::Autosar_00049,
    AutosarVersion::Autosar_00050,
];

fn main() {
    for version in VERSIONS {
        let filename = format!("{}.arxml", version.filename());
        let mut completed: HashSet<(ElementName, ElementName)> = HashSet::new();

        println!("Generating {filename} for \'{}\'", version.describe());

        let project = AutosarProject::new();
        let arxml_file = project.create_file(&filename, version).unwrap();
        let autosar_element = arxml_file.root_element();

        let mut counter = 1;
        create_sub_elements(&autosar_element, &mut counter, &mut completed, version);

        let _ = autosar_element.set_attribute_string(AttributeName::xmlns, "http://autosar.org/schema/r4.0");
        let _ =
            autosar_element.set_attribute_string(AttributeName::xmlnsXsi, "http://www.w3.org/2001/XMLSchema-instance");
        let _ = autosar_element.set_attribute_string(
            AttributeName::xsiSchemalocation,
            &format!("http://autosar.org/schema/r4.0 {}", version.filename()),
        );

        let text = arxml_file.serialize();
        std::fs::write(&filename, text).unwrap();
    }
}

fn create_sub_elements(
    elem: &Element,
    counter: &mut usize,
    completed: &mut HashSet<(ElementName, ElementName)>,
    version: AutosarVersion,
) -> (bool, bool) {
    create_value(elem, version);
    create_attributes(elem, version);
    let elem_name = elem.element_name();
    let mut any_created = false;
    let mut element_complete = true;
    for (se_name, named, _) in elem.list_valid_sub_elements() {
        if completed.get(&(elem_name, se_name)).is_none() {
            match create_sub_element_helper(&elem, se_name, named, counter) {
                Ok(sub_elem) => {
                    any_created = true;
                    if named {
                        completed.insert((se_name, ElementName::ShortName));
                    }
                    completed.insert((elem_name, se_name));

                    let (se_complete, _) = create_sub_elements(&sub_elem, counter, completed, version);
                    if !se_complete {
                        completed.remove(&(elem_name, se_name));
                        loop {
                            match create_sub_element_helper(&elem, se_name, named, counter) {
                                Ok(sub_elem) => {
                                    let (se_complete, se_any_created) =
                                        create_sub_elements(&sub_elem, counter, completed, version);
                                    if se_complete {
                                        break;
                                    }
                                    if !se_any_created {
                                        element_complete = false;
                                        let _ = elem.remove_sub_element(sub_elem);
                                        break;
                                    }
                                }
                                Err(_) => {
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    element_complete = false;
                }
            }
        }
    }

    (element_complete, any_created)
}

fn create_sub_element_helper(
    elem: &Element,
    se_name: ElementName,
    named: bool,
    counter: &mut usize,
) -> Result<Element, AutosarDataError> {
    if named {
        let item_name_raw = format!("{}_{counter}", se_name.to_str().to_ascii_lowercase());
        let item_name: String = item_name_raw.chars().map(|c| if c == '-' { '_' } else { c }).collect();
        *counter += 1;
        elem.create_named_sub_element(se_name, &item_name)
    } else {
        elem.create_sub_element(se_name)
    }
}

fn create_value(elem: &Element, version: AutosarVersion) {
    if elem.content_type() == ContentType::CharacterData {
        let spec = elem.element_type().chardata_spec().unwrap();
        let cdata = make_cdata(spec, version);
        elem.set_character_data(cdata.clone())
            .unwrap_or_else(|err| panic!("error {err} while setting {cdata} with spec {spec:?}",));
    } else if elem.content_type() == ContentType::Mixed {
        let _ = elem.insert_character_content_item("xXxXx", 0);
    }
}

fn create_attributes(elem: &Element, version: AutosarVersion) {
    for (attr_name, spec, required) in elem.element_type().attribute_spec_iter() {
        if required {
            let _ = elem.set_attribute(attr_name, make_cdata(spec, version));
        }
    }
}

fn make_cdata(spec: &CharacterDataSpec, version: AutosarVersion) -> CharacterData {
    match spec {
        autosar_data_specification::CharacterDataSpec::Enum { items } => {
            let valid_item = items
                .iter()
                .find(|(_, ver_mask)| version.compatible(*ver_mask))
                .unwrap_or_else(|| panic!("found no valid enum value from {items:?} in version {version}"));
            CharacterData::Enum(valid_item.0)
        }
        autosar_data_specification::CharacterDataSpec::Pattern { regex, .. } => match *regex {
            r"0x[0-9a-z]*" => CharacterData::String("0xdeadbeef".to_string()),
            r"[1-9][0-9]*|0[xX][0-9a-fA-F]*|0[bB][0-1]+|0[0-7]*|UNSPECIFIED|UNKNOWN|BOOLEAN|PTR" => {
                CharacterData::String("UNSPECIFIED".to_string())
            }
            r"[1-9][0-9]*|0[xX][0-9a-fA-F]+|0[0-7]*|0[bB][0-1]+|ANY|ALL" => CharacterData::String("ALL".to_string()),
            r"[0-9]+|ANY" => CharacterData::String("000".to_string()),
            r"[0-9]+|STRING|ARRAY" => CharacterData::String("ARRAY".to_string()),
            r"0|1|true|false" => CharacterData::String("false".to_string()),
            r"[a-zA-Z_][a-zA-Z0-9_]*" => CharacterData::String("_identifier".to_string()),
            r"[a-zA-Z][a-zA-Z0-9_]*" => CharacterData::String("identifier".to_string()),
            r"([0-9]{4}-[0-9]{2}-[0-9]{2})(T[0-9]{2}:[0-9]{2}:[0-9]{2}(Z|([+\-][0-9]{2}:[0-9]{2})))?" => {
                CharacterData::String("2022-01-01T12:00:00Z".to_string())
            }
            r"[a-zA-Z][a-zA-Z0-9-]*" => CharacterData::String("identifier-".to_string()),
            r"[0-9a-zA-Z_\-]+" => CharacterData::String("09AZ_-".to_string()),
            r"%[ \-+#]?[0-9]*(\.[0-9]+)?[diouxXfeEgGcs]" => CharacterData::String("%23.456d".to_string()),
            r"0|[\+\-]?[1-9][0-9]*|0[xX][0-9a-fA-F]+|0[bB][0-1]+|0[0-7]+" => {
                CharacterData::String("0b1010101".to_string())
            }
            r"(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)|ANY" => {
                CharacterData::String("192.168.0.1".to_string())
            }
            r"[0-9A-Fa-f]{1,4}(:[0-9A-Fa-f]{1,4}){7,7}|ANY" => {
                CharacterData::String("fe80:0:abcd:1234:0:0:0:1".to_string())
            }
            r"(0[xX][0-9a-fA-F]+)|(0[0-7]+)|(0[bB][0-1]+)|(([+\-]?[1-9][0-9]+(\.[0-9]+)?|[+\-]?[0-9](\.[0-9]+)?)([eE]([+\-]?)[0-9]+)?)|\.0|INF|-INF|NaN" => {
                CharacterData::String("-INF".to_string())
            }
            r"([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}" => CharacterData::String("00:00:00:00:00:00".to_string()),
            r"[a-zA-Z_][a-zA-Z0-9_]*(\[([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+)\])*(\.[a-zA-Z_][a-zA-Z0-9_]*(\[([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+)\])*)*" => {
                CharacterData::String("aabb9_cd[x][y].cde".to_string())
            }
            r"[A-Z][a-zA-Z0-9_]*" => CharacterData::String("Q_9".to_string()),
            r"[1-9][0-9]*" => CharacterData::String("1234567890".to_string()),
            r"0|[\+]?[1-9][0-9]*|0[xX][0-9a-fA-F]+|0[bB][0-1]+|0[0-7]+" => CharacterData::String("123".to_string()),
            r"-?([0-9]+|MAX-TEXT-SIZE|ARRAY-SIZE)" => CharacterData::String("MAX-TEXT-SIZE".to_string()),
            r"/?[a-zA-Z][a-zA-Z0-9_]{0,127}(/[a-zA-Z][a-zA-Z0-9_]{0,127})*" => {
                CharacterData::String("/invalid".to_string())
            }
            r"[0-9]+\.[0-9]+\.[0-9]+([\._;].*)?" => CharacterData::String("0.1.2_something".to_string()),
            r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(-((0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(\.(0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(\+([0-9a-zA-Z-]+(\.[0-9a-zA-Z-]+)*))?" => {
                CharacterData::String("0.0.0-ab-c.0.0+zz-Z".to_string())
            }
            _ => {
                panic!("unknown regex: {regex}");
            }
        },
        autosar_data_specification::CharacterDataSpec::String { .. } => {
            CharacterData::String("lorem ipsum".to_string())
        }
        autosar_data_specification::CharacterDataSpec::UnsignedInteger => CharacterData::UnsignedInteger(42),
        autosar_data_specification::CharacterDataSpec::Double => CharacterData::Double(3.1415),
    }
}
