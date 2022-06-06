use std::{env, ffi::OsString};

use autosar_data::AutosarData;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <arxml filename>", args[0]);
        return;
    }

    let data = AutosarData::new();
    let now = std::time::Instant::now();
    let filename = OsString::from(&args[1]);
    let result = data.load_arxml_file(&filename);
    match result {
        Ok(_) => println!("parsing succeeded in {}ms", now.elapsed().as_micros() as f64 / 1000.0),
        Err(err) => println!("parsing failed: {err}"),
    }

    for file in data.files() {
        println!("loaded arxml file: {}", file.filename().to_string_lossy());
    }
}



