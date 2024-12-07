use std::fs;

use walkdir::{DirEntry, WalkDir};

use crate::types::{
    module_name::ModuleName, openapi_spec::OpenApiSpec3_1, router::Router, structure::Structure,
};

pub fn load_and_parse(root_path: &str) -> OpenApiSpec3_1 {
    let routers: Vec<Router> = vec![];
    let structures: Vec<Structure> = vec![];
    let module_names: Vec<ModuleName> = vec![];

    let walker = WalkDir::new(root_path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    println!("Found file: {}", entry.path().display());

                    let file_contents = fs::read_to_string(entry.path()).unwrap();
                } else if entry.file_type().is_dir() {
                    println!("Found directory: {}", entry.path().display());
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    todo!()
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
