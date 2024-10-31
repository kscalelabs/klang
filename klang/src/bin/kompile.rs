use klang::{compile_file, compile_file_inplace};
use std::env;
use std::path::Path; // Import from the library

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match compile_file_inplace(Path::new(&args[1]), false) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        },
        3 => match compile_file(Path::new(&args[1]), Path::new(&args[2]), false) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Usage: {} <file_path> [output_path]", args[0]);
            std::process::exit(1);
        }
    }
}
