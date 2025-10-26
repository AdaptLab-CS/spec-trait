mod utils;

mod test_build_tree {
    // use pretty_assertions::assert_eq;
    use crate::utils::run_with_cargo_bin_and_snippet;

    const FOLDER: &str = "tests/snippets/build-tree";

    fn run_test(name: &str) -> Result<(), String> {
        let snippet = std::fs::read_to_string(format!("{FOLDER}/{name}.rs")).unwrap();
        let (output, _) = run_with_cargo_bin_and_snippet(&snippet, &["--print-nested-bodies"])?;
        let expected = std::fs::read_to_string(format!("{FOLDER}/{name}.expected.txt")).unwrap();

        println!("Output:\n{}", output);
        let output = output.split_whitespace().collect::<String>();
        let expected = expected.split_whitespace().collect::<String>();

        assert!(output.contains(&expected));

        Ok(())
    }

    #[test]
    fn test_var_def_use() -> Result<(), String> {
        run_test("var_def_use")
    }

    #[test]
    fn test_if_else() -> Result<(), String> {
        run_test("if_else")
    }
}
