pub mod examples;
pub mod data;

#[cfg(test)]
mod tests {
    use super::examples::ex01_spec;

    #[test]
    fn test_spec_dispatch_ex01() {
        let s = ex01_spec::Summator;
        let data = vec![1i32, 2, 3, 4, 5];
        let result = ex01_spec::sum_i32(&s, &data);
        assert_eq!(result, 15, "spec! should dispatch to i32 specialization, not generic zeroed()");
    }
}
