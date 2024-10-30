use super::ast::Program;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest/klang.pest"]
pub struct PestParser;

pub struct KlangProgram {
    pub program: Program,
}
