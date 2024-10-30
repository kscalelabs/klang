use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest/klang.pest"]
pub struct PestParser;
