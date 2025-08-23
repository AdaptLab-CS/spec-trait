use std::path::{ Path, PathBuf };
use syn::Item;
use std::fs::{ self, DirEntry };
use glob::glob;

#[derive(Debug)]
pub struct Crate {
    pub name: String,
    pub path: PathBuf,
    pub files: Vec<PathBuf>,
}

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
            return Some(Crate {
                name: name.to_string(),
                path: dir.to_path_buf(),
                files: get_rs_files(dir),
            });
        }
    }
    None
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

fn handle_workspace_member_pattern(member_str: &str, dir: &Path) -> Vec<Crate> {
    let member_dir = dir.join(member_str);

    if !has_glob_chars(&member_str) {
        return get_crates(&member_dir);
    }

    let pattern = member_dir.to_str().expect("Invalid UTF-8 in path");
    let paths = glob(&pattern).expect("Failed to read glob pattern");

    paths
        .filter_map(Result::ok)
        .flat_map(|path| get_crates(&path))
        .collect()
}

fn has_glob_chars(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[') || s.contains('{') || s.contains('}')
}

fn get_rs_files(dir: &Path) -> Vec<PathBuf> {
    let src_path = dir.join("src");
    handle_dir(&src_path)
}

fn handle_dir(dir: &Path) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir).expect("Failed to read directory");
    entries.filter_map(Result::ok).flat_map(handle_dir_entry).collect()
}

fn handle_dir_entry(entry: DirEntry) -> Vec<PathBuf> {
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
}

pub fn parse(path: &Path) -> Vec<Item> {
    let content = fs::read_to_string(path).expect("failed to read file");
    let file = syn::parse_file(&content).expect("failed to parse content");
    file.items // TODO: parse each item properly
}
