// For each file in the `examples` directory, test that we can get an AST from it.

#[test]
fn test_examples() {
    use crate::ast::parse_file_to_ast;
    use std::fs;
    use std::path::Path;

    let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("examples");
    for entry in fs::read_dir(examples_dir).unwrap() {
        let ast = parse_file_to_ast(entry.unwrap().path());
        match ast {
            Ok(_ast) => (),
            Err(e) => panic!("{}", e),
        }
    }
}
