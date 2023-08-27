use std::{
    env,
    ffi::{OsStr, OsString},
    fs::File,
    io::Read,
};

use autosar_data::AutosarModel;
use autosar_data_specification::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <arxml filename> ...", args[0]);
        return;
    }

    let model = AutosarModel::new();
    for arg in args.iter().skip(1) {
        let filename = OsString::from(arg);
        let buffer = match load_file_data(&filename) {
            Ok(buffer) => buffer,
            Err(error) => {
                println!("IO error: {error}");
                return;
            }
        };

        let now = std::time::Instant::now();
        let result = model.load_buffer(&buffer, &filename, false);
        match result {
            Ok((_, warnings)) => {
                println!("parsing succeeded in {}ms", now.elapsed().as_micros() as f64 / 1000.0);
                if !warnings.is_empty() {
                    for w in warnings {
                        println!("   Warning: {w}");
                    }
                }
            }
            Err(err) => println!("parsing failed: {err}"),
        }
        println!("loaded arxml file: {}", arg);
    }

    for (_, elem) in model.elements_dfs() {
        if elem.is_reference() && elem.element_name() != ElementName::DefinitionRef {
            let target_path = elem.character_data().and_then(|cdata| cdata.string_value()).unwrap();
            if model.get_element_by_path(&target_path).is_none() {
                println!("Invalid reference from {} to {target_path}", elem.element_name());
            }
        }
    }

    // let the OS clean up the model when the process exits
    std::mem::forget(model);
}

fn load_file_data(filename: &OsStr) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(filename)?;

    let filesize = file.metadata().unwrap().len();
    let mut buffer = Vec::with_capacity(filesize as usize);
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}
