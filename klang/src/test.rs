#[cfg(test)]
mod tests {
    use klang::read_and_parse_file;
    use std::path::Path;

    #[test]
    fn test_parse_clean_up_cans() {
        let file_path = Path::new("../examples/simple.k");
        let parsed_file = read_and_parse_file(&file_path);
        assert!(parsed_file.is_ok());
    }
}
