use std::fs;
use std::path::Path;

mod parser;

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

    // Parse the file contents using our new parser
    let ast = match PestParser::parse(&unparsed_file) {
        Ok(ast) => ast,
        Err(e) => {
            return Err(format!(
                "Error parsing file '{}': {}",
                file_path.display(),
                e
            ))
        }
    };

    let program = parse_program(ast.next().unwrap());

    Ok(format!("{:#?}", program))
}
