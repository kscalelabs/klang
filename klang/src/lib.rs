use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[grammar = "klang.pest"] // relative path to your Pest grammar file
struct KlangParser;

pub fn read_and_parse_file(file_path: &Path) -> Result<String, String> {
    // Read the file contents
    let unparsed_file = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            return Err(format!(
                "Error reading file '{}': {}",
                file_path.display(),
                e
            ))
        }
    };

    // Parse the file contents using the KlangParser
    let parsed_file = match KlangParser::parse(Rule::program, &unparsed_file) {
        Ok(parsed) => format!("{:#?}", parsed),
        Err(e) => {
            return Err(format!(
                "Error parsing file '{}': {}",
                file_path.display(),
                e
            ))
        }
    };

    Ok(parsed_file)
}
