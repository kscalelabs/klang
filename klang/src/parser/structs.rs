use super::ast::{Command as AstCommand, Program as AstProgram};
use super::errors::ParseError;
use pest_derive::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[grammar = "pest/klang.pest"]
pub struct PestParser;

pub struct Node {
    pub text: String,
    pub children: Vec<Node>,
}

impl Node {
    pub fn to_ast(&self) -> AstCommand {
        AstCommand {
            text: self.text.clone(),
            children: self.children.iter().map(|child| child.to_ast()).collect(),
        }
    }

    pub fn from_ast(ast: &AstCommand) -> Self {
        Node {
            text: ast.text.clone(),
            children: ast.children.iter().map(Node::from_ast).collect(),
        }
    }

    pub fn to_string(&self, indent: usize) -> String {
        let mut result = format!("{:indent$}{}", " ", self.text, indent = indent);
        if !self.children.is_empty() {
            result.push_str(" {\n");
            for child in &self.children {
                result.push_str(&child.to_string(indent + 2));
            }
            result.push_str(&format!("{:indent$}}}", " ", indent = indent));
        }
        result.push('\n');
        result
    }

    pub fn to_list(&self) -> Vec<Vec<String>> {
        if self.children.is_empty() {
            vec![vec![self.text.clone()]]
        } else {
            self.children
                .iter()
                .flat_map(|child| {
                    child.to_list().into_iter().map(|mut line| {
                        line.push(self.text.clone());
                        line
                    })
                })
                .collect()
        }
    }
}

pub struct KlangProgram {
    pub program: Vec<Node>,
}

impl KlangProgram {
    pub fn to_ast(&self) -> AstProgram {
        AstProgram {
            commands: self.program.iter().map(|node| node.to_ast()).collect(),
        }
    }

    pub fn from_ast(ast: &AstProgram) -> Self {
        KlangProgram {
            program: ast.commands.iter().map(Node::from_ast).collect(),
        }
    }

    pub fn save_binary(&self, path: &Path) -> Result<(), ParseError> {
        let mut buf = Vec::new();
        prost::Message::encode(&self.to_ast(), &mut buf)?;
        fs::write(path, &buf)?;
        Ok(())
    }

    pub fn load_binary(path: &Path) -> Result<Self, ParseError> {
        let buf = fs::read(path)?;
        let program = prost::Message::decode(&*buf)?;
        Ok(KlangProgram::from_ast(&program))
    }

    pub fn save_text(&self, path: &Path) -> Result<(), ParseError> {
        let output = self.to_text();
        fs::write(path, &output)?;
        Ok(())
    }

    pub fn to_text(&self) -> String {
        self.program
            .iter()
            .map(|node| node.to_string(0))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn to_list(&self) -> Vec<Vec<String>> {
        self.program
            .iter()
            .flat_map(|node| node.to_list())
            .collect()
    }
}

impl std::fmt::Display for KlangProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_text())
    }
}
