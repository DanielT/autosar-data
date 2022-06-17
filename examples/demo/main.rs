use std::{env, ffi::{OsString, OsStr}, fs::File, io::Read};

use autosar_data::AutosarData;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <arxml filename>", args[0]);
        return;
    }

    let data = AutosarData::new();
    let filename = OsString::from(&args[1]);
    let buffer = match load_file_data(&filename) {
        Ok(buffer) => buffer,
        Err(error) => {
            println!("IO error: {error}");
            return;
        }
    };
    let now = std::time::Instant::now();
    let result = data.load_named_arxml_buffer(&buffer, &filename, true, Some(|err| println!("logged error: {err}")));
    match result {
        Ok(_) => println!("parsing succeeded in {}ms", now.elapsed().as_micros() as f64 / 1000.0),
        Err(err) => println!("parsing failed: {err}"),
    }

    for file in data.files() {
        println!("loaded arxml file: {}", file.filename().to_string_lossy());
    }
}


fn load_file_data(filename: &OsStr) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(filename)?;

    let filesize = file.metadata().unwrap().len();
    let mut buffer = Vec::with_capacity(filesize as usize);
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

