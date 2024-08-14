pub mod ast;
pub mod tests;

use ast::parse_file_to_ast;

fn main() {
    // Throw an error if incorrect number of arguments.
    if std::env::args().count() != 2 {
        eprintln!(
            "Incorrect number of arguments (got {}, expected 1)",
            std::env::args().count() - 1
        );
        std::process::exit(1);
    }

    let filename = std::env::args()
        .nth(1)
        .expect("Missing filename argument! Usage: klang <filename>");
    let ast = parse_file_to_ast(filename);

    match ast {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => eprintln!("{}", e),
    }
}
