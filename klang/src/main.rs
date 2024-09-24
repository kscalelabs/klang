use klang::read_and_parse_file;
use std::env;
use std::path::Path; // Import from the library

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let parsed_file = match read_and_parse_file(Path::new(file_path)) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    println!("{}", parsed_file);
}
