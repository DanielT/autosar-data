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

    // create a project and load the file
    let project = AutosarProject::new();
    let arxmlfile = match project.load_arxml_file(filename, false) {
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

    arxmlfile.sort();

    // write the sorted file
    let filename_prefix = filename.strip_suffix(".arxml").or(Some(filename)).unwrap();
    let new_filename = format!("{filename_prefix}_sorted.arxml");
    arxmlfile.set_filename(&new_filename);
    project.write().unwrap();
    println!("Sorted output has been written to \"{new_filename}\"");
}
