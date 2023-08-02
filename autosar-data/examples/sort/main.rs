use autosar_data::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <arxml filename>", args[0]);
        println!("    Output will be written to <input>_sorted.arxml");
        return;
    }
    let filename = &args[1];

    // create the data model and load the file
    let model = AutosarModel::new();
    let arxmlfile = match model.load_arxml_file(filename, false) {
        Ok((arxmlfile, warnings)) => {
            for warn_msg in warnings {
                println!("Warning: {warn_msg}");
            }
            arxmlfile
        }
        Err(err) => {
            println!("Error: {err}");
            return;
        }
    };

    model.sort();

    // write the sorted file
    let filename_prefix = filename.strip_suffix(".arxml").unwrap_or(filename);
    let new_filename = format!("{filename_prefix}_sorted.arxml");
    arxmlfile.set_filename(&new_filename);
    model.write().unwrap();
    println!("Sorted output has been written to \"{new_filename}\"");
}
