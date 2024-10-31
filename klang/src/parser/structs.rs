use super::ast::Program as AstProgram;
use super::ir::Program as IrProgram;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest/klang.pest"]
pub struct PestParser;

pub struct KlangProgram {
    pub ast_program: AstProgram,
    pub ir_program: IrProgram,
}
