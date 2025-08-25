use crate::files;
use std::path::{ Path, PathBuf };
use std::fs;
use glob::glob;
use spec_trait_utils::cache::CrateCache;

#[derive(Debug)]
pub struct Crate {
    pub name: String,
    pub path: PathBuf,
    pub files: Vec<PathBuf>,
    pub content: CrateCache,
}

// Get all crates in the given directory, considering both single-package and workspace setups
pub fn get_crates(dir: &Path) -> Vec<Crate> {
    let cargo_toml_path = dir.join("Cargo.toml");
    let cargo_toml_content = fs
        ::read_to_string(cargo_toml_path)
        .expect("Failed to read Cargo.toml");
    let cargo_toml_value = toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");

    let crate_from_package = get_crate_from_package(&cargo_toml_value, dir);
    let crates_from_workspace_members = get_crates_from_workspace_members(&cargo_toml_value, dir);
    crate_from_package.into_iter().chain(crates_from_workspace_members.into_iter()).collect()
}

fn get_crate_from_package(value: &toml::Value, dir: &Path) -> Option<Crate> {
    if let Some(package) = value.get("package") {
        if let Some(name) = package.get("name").and_then(|n| n.as_str()) {
            let files = get_crate_rs_files(dir);
            let content = get_crate_content_from_files(&files);
            return Some(Crate {
                name: name.to_string(),
                path: dir.to_path_buf(),
                files,
                content,
            });
        }
    }
    None
}

fn get_crate_content_from_files(files: &[PathBuf]) -> CrateCache {
    let crate_files_content = files
        .iter()
        .map(|f| files::parse(&f))
        .collect::<Vec<_>>();
    files::flatten_contents(&crate_files_content)
}

fn get_crates_from_workspace_members(value: &toml::Value, dir: &Path) -> Vec<Crate> {
    let mut crates = vec![];
    if let Some(workspace) = value.get("workspace") {
        if let Some(members) = workspace.get("members").and_then(|m| m.as_array()) {
            for member in members {
                if let Some(member_str) = member.as_str() {
                    let member_crates = handle_workspace_member_pattern(member_str, dir);
                    crates.extend(member_crates);
                }
            }
        }
    }
    crates
}

// member_str can be something like "crates/my_crate", "crates/*", etc.
fn handle_workspace_member_pattern(member_str: &str, dir: &Path) -> Vec<Crate> {
    let member_dir = dir.join(member_str);
    let pattern = member_dir.to_str().expect("Invalid UTF-8 in member path");
    let paths = glob(&pattern).expect("Failed to parse member pattern");

    paths
        .filter_map(Result::ok)
        .flat_map(|path| get_crates(&path))
        .collect()
}

// get all .rs files in the src directory of the crate located at dir
fn get_crate_rs_files(dir: &Path) -> Vec<PathBuf> {
    let src_path = dir.join("src");
    handle_dir(&src_path)
}

// recursively find all .rs files in the given directory and subdirectories
fn handle_dir(dir: &Path) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir).expect("Failed to read directory");
    entries
        .filter_map(Result::ok)
        .flat_map(|entry| {
            let path = entry.path();
            let extension = path.extension().and_then(|s| s.to_str());
            let is_rs = extension == Some("rs");

            if path.is_dir() {
                handle_dir(&path)
            } else if path.is_file() && is_rs {
                vec![path]
            } else {
                vec![]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs::{ create_dir_all, write };
    use std::path::Path;
    use tempfile::tempdir;
    use super::*;

    fn make_package(dir: &Path, name: &str, src_files: &[(&str, &str)]) {
        create_dir_all(dir.join("src")).expect("create src");
        let cargo = format!(r#"[package]
name = "{}"
version = "0.1.0"
"#, name);
        write(dir.join("Cargo.toml"), cargo).expect("write Cargo.toml");
        for (fname, content) in src_files {
            write(dir.join("src").join(fname), content).expect("write src file");
        }
    }

    fn make_workspace(dir: &Path, members: &[&str]) {
        let members_list = members
            .iter()
            .map(|m| format!(r#""{}""#, m))
            .collect::<Vec<_>>()
            .join(", ");
        let cargo = format!(r#"[workspace]
members = [{}]
"#, members_list);
        write(dir.join("Cargo.toml"), cargo).expect("write Cargo.toml");
    }

    #[test]
    // single package, no workspace
    fn single_package() {
        let td = tempdir().unwrap();
        let root = td.path();

        make_package(
            root,
            "foo",
            &[
                ("lib.rs", "pub fn main(){}"),
                ("foo.rs", "pub fn foo(){}"),
            ]
        );

        let crates = get_crates(root);

        assert_eq!(crates.len(), 1);
        assert_eq!(crates[0].name, "foo");
        assert!(crates[0].path.ends_with(root));
        assert!(crates[0].files.iter().any(|p| p.ends_with("lib.rs")));
        assert!(crates[0].files.iter().any(|p| p.ends_with("foo.rs")));
    }

    #[test]
    // workspace with members listed explicitly
    fn workspace_members_list() {
        let td = tempdir().unwrap();
        let root = td.path();

        make_workspace(root, &["foo", "bar"]);
        make_package(&root.join("foo"), "foo", &[("lib.rs", "pub fn foo(){}")]);
        make_package(&root.join("bar"), "bar", &[("lib.rs", "pub fn bar(){}")]);
        make_package(&root.join("baz"), "baz", &[("lib.rs", "pub fn baz(){}")]);

        let crates = get_crates(root);

        let names = crates
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(crates.len(), 2);
        assert!(names.contains(&"foo"));
        assert!(names.contains(&"bar"));
        assert!(!names.contains(&"baz"));
    }

    #[test]
    // workspace with members listed via glob
    fn workspace_members_glob() {
        let td = tempdir().unwrap();
        let root = td.path();

        make_workspace(root, &["crates/*"]);
        make_package(&root.join("crates").join("foo"), "foo", &[("lib.rs", "pub fn foo(){}")]);
        make_package(&root.join("crates").join("bar"), "bar", &[("lib.rs", "pub fn bar(){}")]);

        let crates = get_crates(root);

        let names = crates
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(crates.len(), 2);
        assert!(names.contains(&"foo"));
        assert!(names.contains(&"bar"));
    }

    #[test]
    // root package with workspace
    fn root_package_with_workspace_members() {
        let td = tempdir().unwrap();
        let root = td.path();

        make_package(&root, "root", &[("lib.rs", "pub fn main(){}")]);
        make_package(&root.join("crates").join("foo"), "foo", &[("lib.rs", "pub fn foo(){}")]);
        make_package(&root.join("crates").join("bar"), "bar", &[("lib.rs", "pub fn bar(){}")]);

        let cargo =
            "[package]
name = \"root\"
version = \"0.1.0\"

[workspace]
members = [\"crates/*\"]
";
        write(root.join("Cargo.toml"), cargo).expect("write Cargo.toml");

        let crates = get_crates(root);

        let names = crates
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(crates.len(), 3);
        assert!(names.contains(&"root"));
        assert!(names.contains(&"foo"));
        assert!(names.contains(&"bar"));
    }
}
