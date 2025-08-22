use std::path::{ Path, PathBuf };
use syn::Item;
use std::fs::{ self, DirEntry };

// TODO: use Cargo.toml to understand which sub-directories to explore
pub fn get_rs_files(dir: &Path) -> Vec<PathBuf> {
    let dir = fs::read_dir(dir).expect("read_dir error");

    dir.filter_map(|entry| entry.ok())
        .flat_map(handle_dir_entry)
        .collect()
}

fn handle_dir_entry(entry: DirEntry) -> Vec<PathBuf> {
    let path = entry.path();
    let file_name = path.file_name().and_then(|s| s.to_str());
    let extension = path.extension().and_then(|s| s.to_str());

    // skipping target directory as it contains build artifacts
    if path.is_dir() && file_name != Some("target") {
        return get_rs_files(&path);
    }

    // skipping root build.rs as it is the file we are running this from
    if path.is_file() && extension == Some("rs") && path.display().to_string() != "./build.rs" {
        return vec![path];
    }

    vec![]
}

pub fn parse(path: &Path) -> Vec<Item> {
    let content = fs::read_to_string(path).expect("failed to read file");
    let file = syn::parse_file(&content).expect("failed to parse content");
    file.items // TODO: parse each item properly
}
