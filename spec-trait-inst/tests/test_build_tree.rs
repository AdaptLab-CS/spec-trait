mod utils;

mod test_build_tree {
    // use pretty_assertions::assert_eq;
    use crate::utils::run_with_cargo_bin_and_snippet;

    const FOLDER: &str = "tests/snippets/build-tree";

    fn run_test(name: &str) -> Result<(), String> {
        let snippet = std::fs::read_to_string(format!("{FOLDER}/{name}.rs")).unwrap();
        let (output, _) = run_with_cargo_bin_and_snippet(&snippet, &["--print-nested-bodies"])?;
        let expected = std::fs::read_to_string(format!("{FOLDER}/{name}.expected.txt")).unwrap();

        println!("Formatted Output:\n{}", output); // TODO: remove
        let output = output.split_whitespace().collect::<String>();
        let expected = expected.split_whitespace().collect::<String>();

        println!("Output:\n{}\nExpected:\n{}", output, expected);
        assert!(output.contains(&expected));

        Ok(())
    }

    macro_rules! build_test {
        ($fn_name:ident) => {
            #[test]
            fn $fn_name() -> Result<(), String> {
                run_test(stringify!($fn_name))
            }
        };
    }

    build_test!(var_def_use);
    build_test!(if_else);
    build_test!(fn_def_call);
    build_test!(matches);
}
