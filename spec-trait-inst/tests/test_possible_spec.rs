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
        &[("Foo::foo_i32 for S", "Foo::foo_str for S")],
        &[("Foo for S", "Foo for S")]
    );
    build_test!(
        monomorphized_traits,
        &[("FooI32::foo for S", "FooStr::foo for S")],
        &[("FooI32 for S", "FooStr for S")]
    );
    build_test!(standalone_fns, &[("foo_i32", "foo_str")], &[]);
    build_test!(inherent_impls, &[("foo_i32 for S", "foo_str for S")], &[]);
    build_test!(
        type_level_dispatch,
        &[("Foo::foo for SWithV<i32>", "Foo::foo for SWithV<&str>")],
        &[("Foo for SWithV<i32>", "Foo for SWithV<&str>")]
    );
}
