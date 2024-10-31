mod ast {
    include!(concat!(env!("OUT_DIR"), "/proto/ast.rs"));
}

mod ir {
    include!(concat!(env!("OUT_DIR"), "/proto/ir.rs"));
}

pub mod errors;
pub mod lang;
pub mod passes;
pub mod structs;

use errors::ParseError;
use lang::parse_program;
use pest::Parser;
use std::fs;
use std::path::Path;
use structs::{KlangProgram, PestParser, Rule};

pub fn parse_string(input: &str) -> Result<KlangProgram, ParseError> {
    match PestParser::parse(Rule::program, input) {
        Ok(mut pairs) => parse_program(pairs.next().unwrap()),
        Err(e) => Err(ParseError::new(format!("Error parsing input: {}", e))),
    }
}

pub fn parse_file(file_path: &Path) -> Result<KlangProgram, ParseError> {
    let unparsed_file = fs::read_to_string(file_path).map_err(|e| {
        ParseError::new(format!(
            "Error reading file '{}': {}",
            file_path.display(),
            e
        ))
    })?;

    parse_string(&unparsed_file)
}

pub fn write_program_to_file(
    program: &KlangProgram,
    file_path: &Path,
    binary: bool,
) -> Result<(), ParseError> {
    if binary {
        let mut buf = Vec::new();
        prost::Message::encode(&program.ast_program, &mut buf).map_err(|e| {
            ParseError::new(format!(
                "Error encoding program to file '{}': {}",
                file_path.display(),
                e
            ))
        })?;

        fs::write(file_path, &buf).map_err(|e| {
            ParseError::new(format!(
                "Error writing program to file '{}': {}",
                file_path.display(),
                e
            ))
        })?;
    } else {
        let mut output = String::new();

        fn render_command(cmd: &ast::Command, indent: usize) -> String {
            let mut result = format!("{:indent$}{}", "", cmd.text, indent = indent);
            if !cmd.children.is_empty() {
                result.push_str(" {\n");
                for child in &cmd.children {
                    result.push_str(&render_command(child, indent + 2));
                }
                result.push_str(&format!("{:indent$}}}\n", "", indent = indent));
            } else {
                result.push('\n');
            }
            result
        }

        for command in &program.ast_program.commands {
            output.push_str(&render_command(command, 0));
        }

        fs::write(file_path, &output).map_err(|e| {
            ParseError::new(format!(
                "Error writing program to file '{}': {}",
                file_path.display(),
                e
            ))
        })?;
    }

    Ok(())
}
