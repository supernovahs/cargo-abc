use clap::{arg, Command};
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item};

fn find_cargo_toml_files(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let pattern = format!("{}/**/Cargo.toml", path.display());
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => files.push(path),
            Err(e) => println!("{:?}", e),
        }
    }
    files
}

fn sort_dependencies_in_cargo_toml(path: &PathBuf) {
    let content = fs::read_to_string(path).expect("Unable to read file");
    let mut doc = content.parse::<DocumentMut>().expect("Invalid TOML format");

    if let Some(Item::Table(dep_table)) = doc.get_mut("dependencies") {
        sort_table_alphabetically(dep_table);
    }

    if let Some(Item::Table(dev_dep_table)) = doc.get_mut("dev-dependencies") {
        sort_table_alphabetically(dev_dep_table);
    }

    fs::write(path, doc.to_string()).expect("Unable to write file");
}

fn sort_table_alphabetically(table: &mut toml_edit::Table) {
    // Collect the key-value pairs into a Vec
    let mut sorted: Vec<(String, Item)> = table
        .iter()
        .map(|(key, value)| (key.to_string(), value.clone()))
        .collect();

    // Sort the Vec by the keys
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    table.clear();

    for (key, value) in sorted {
        table.insert(&key, value);
    }
}

fn main() {
    let matches = Command::new("cargo-abc")
        .version("1.0")
        .about("Automatically sorts dependencies in Cargo.toml files alphabetically")
        .arg(arg!(--path <VALUE>).required(false).default_value("."))
        .get_matches();

    let path = matches.get_one::<String>("path").expect("required");

    let path = PathBuf::from(path);

    // Check if the provided path exists and is a directory
    if !path.exists() {
        eprintln!("Error: The path '{}' does not exist.", path.display());
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("Error: The path '{}' is not a directory.", path.display());
        std::process::exit(1);
    }
    let cargo_toml_files = find_cargo_toml_files(&path);
    for path in cargo_toml_files {
        sort_dependencies_in_cargo_toml(&path);
    }
    println!("Dependencies sorted successfully.");
}
