mod utils;

mod test_possible_spec {
    use crate::utils::run_with_cargo_bin_and_snippet;

    const FOLDER: &str = "tests/snippets/possible-spec";

    fn pair_reg(a: &str, b: &str) -> regex::Regex {
        let re = format!(
            r"\s*{}\s*<-+\s*\d+(.\d+)?\s*-+>\s*{}\s*",
            regex::escape(a),
            regex::escape(b)
        );
        regex::Regex::new(&re).unwrap()
    }

    fn run_test(
        name: &str,
        spec_fns: &[(&str, &str)],
        spec_traits: &[(&str, &str)],
    ) -> Result<(), String> {
        let snippet = std::fs::read_to_string(format!("{FOLDER}/{name}.rs")).unwrap();
        let (output, _) = run_with_cargo_bin_and_snippet(&snippet, &["--print-spec-targets"])?;

        assert!(output.contains(&format!("Specializable fn pairs: {}", spec_fns.len())));
        assert!(output.contains(&format!("Specializable trait pairs: {}", spec_traits.len())));

        for (fn_a, fn_b) in spec_fns {
            assert!(pair_reg(fn_a, fn_b).is_match(&output));
        }

        for (trait_a, trait_b) in spec_traits {
            assert!(pair_reg(trait_a, trait_b).is_match(&output));
        }

        Ok(())
    }

    macro_rules! build_test {
        ($fn_name:ident, $spec_fns:expr, $spec_traits:expr) => {
            #[test]
            fn $fn_name() -> Result<(), String> {
                run_test(stringify!($fn_name), $spec_fns, $spec_traits)
            }
        };
    }

    build_test!(
        manual_dispatch,
        &[("foo_i32", "foo_str")],
        &[("Foo", "Foo")]
    );
    build_test!(
        monomorphized_traits,
        &[("foo", "foo")],
        &[("FooI32", "FooStr")]
    );
    build_test!(standalone_fns, &[("foo_i32", "foo_str")], &[]);
    build_test!(type_level_dispatch, &[("foo", "foo")], &[("Foo", "Foo")]);
}
