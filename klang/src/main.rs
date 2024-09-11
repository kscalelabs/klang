pub mod ast;
pub mod tests;

use crate::ast::{KlangParser, Rule};
use pest::Parser;
use std::env;
use std::fs;

fn main() {
    // Get the file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // Read the file contents
    let unparsed_file = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };

    // Parse the file
    match KlangParser::parse(Rule::program, &unparsed_file) {
        Ok(pairs) => {
            println!("Successfully parsed the file:");
            for pair in pairs {
                if pair.as_rule() == Rule::expression {
                    let expr = KlangParser::parse_expression(pair.into_inner());
                    println!("{:#?}", expr);
                } else {
                    println!("{:#?}", pair);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            std::process::exit(1);
        }
    }
}
