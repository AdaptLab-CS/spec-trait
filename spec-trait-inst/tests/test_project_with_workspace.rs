#![feature(rustc_private)]

mod utils;

mod test_workspaces_simple_project_with_workspace {

    use crate::utils::run_with_cargo_bin;
    // use pretty_assertions::assert_eq;

    #[test]
    fn test_workspaces_simple_project_with_workspace_dot_file() -> Result<(), String> {
        const FOLDER: &str = "tests/workspaces/simple_project_with_workspace";
        run_with_cargo_bin(FOLDER, None, &["-V"])?;
        Ok(())
    }
}
