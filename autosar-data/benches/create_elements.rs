use autosar_data::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fmt::Write;

fn create_elements_a() {
    let model = AutosarModel::new();
    let _file = model.create_file("file", AutosarVersion::LATEST);
    let packages = model
        .root_element()
        .create_sub_element(ElementName::ArPackages)
        .unwrap();
    let package = packages
        .create_named_sub_element(ElementName::ArPackage, "package")
        .unwrap();
    let elements = package.create_sub_element(ElementName::Elements).unwrap();

    let sub_elem_info = elements.list_valid_sub_elements();

    let mut name = String::with_capacity(20);
    let mut seq_id = 0;
    for ValidSubElementInfo {
        element_name,
        is_named,
        is_allowed,
    } in sub_elem_info
    {
        for _ in 0..4 {
            name.clear();
            seq_id += 1;
            write!(name, "element_{seq_id}").unwrap();
            if is_allowed {
                if is_named {
                    elements.create_named_sub_element(element_name, &name).unwrap();
                } else {
                    elements.create_sub_element(element_name).unwrap();
                }
            }
        }
    }
}

fn create_elements_b() {
    let model = AutosarModel::new();
    let _file = model.create_file("file", AutosarVersion::LATEST);
    let packages = model
        .root_element()
        .create_sub_element(ElementName::ArPackages)
        .unwrap();
    let package = packages
        .create_named_sub_element(ElementName::ArPackage, "package")
        .unwrap();
    let elements = package.create_sub_element(ElementName::Elements).unwrap();

    let sub_elem_info = elements.list_valid_sub_elements();

    let mut name = String::with_capacity(20);
    let mut seq_id = 0;
    for _ in 0..4 {
        for ValidSubElementInfo {
            element_name,
            is_named,
            is_allowed,
        } in &sub_elem_info
        {
            name.clear();
            seq_id += 1;
            write!(name, "element_{seq_id}").unwrap();
            if *is_allowed {
                if *is_named {
                    elements.create_named_sub_element(*element_name, &name).unwrap();
                } else {
                    elements.create_sub_element(*element_name).unwrap();
                }
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create_elements (a)", |b| b.iter(|| create_elements_a()));
    c.bench_function("create_elements (b)", |b| b.iter(|| create_elements_b()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
