mod utils;

mod test_spec_data {
    use crate::utils::run_with_cargo_bin_and_snippet;
    use itertools::Itertools;

    const FOLDER: &str = "tests/snippets/possible-spec";

    fn run_test(name: &str, data: Vec<usize>) -> Result<(), String> {
        let snippet = std::fs::read_to_string(format!("{FOLDER}/{name}.rs")).unwrap();
        let (output, _) = run_with_cargo_bin_and_snippet(&snippet, &["--print-spec-data"])?;

        let expected = format!("Specialization analysis data: {}", data.iter().join(","));

        println!("Output:\n{}", output);
        assert!(output.starts_with(&expected));

        Ok(())
    }

    macro_rules! build_test {
        ($fn_name:ident, $data:expr) => {
            #[test]
            fn $fn_name() -> Result<(), String> {
                run_test(stringify!($fn_name), $data)
            }
        };
    }

    build_test!(manual_dispatch, vec![2, 1, 0, 0, 2, 0, 0, 1, 1, 2, 1]);
    build_test!(monomorphized_traits, vec![2, 2, 0, 0, 2, 0, 0, 1, 1, 2, 2]);
    build_test!(standalone_fns, vec![2, 0, 2, 0, 0, 0, 0, 1, 0, 2, 0]);
    build_test!(inherent_impls, vec![2, 0, 0, 0, 0, 2, 0, 1, 0, 2, 0]);
    build_test!(type_level_dispatch, vec![2, 1, 0, 0, 2, 0, 0, 1, 1, 2, 1]);
}
