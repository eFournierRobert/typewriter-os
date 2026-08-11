use std::{env, fs, path::Path, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&String::from("help")) {
        print_help();
        exit(1);
    }

    if let Some(filename) = args.get(1) {
        create_file(filename);
    }
}

/// Creates an empty file with the given name. Exits on error.
fn create_file(name: &str) {
    let file_path = Path::new(name);
    if file_path.exists() {
        eprintln!("file {} already exists", name);
        exit(4);
    }

    if let Err(err) = fs::File::create(name) {
        eprintln!("Error while creating {}: {}", name, err);
        exit(2);
    }

    if !is_target_valid(file_path) {
        let _ = fs::remove_file(file_path);
        eprintln!("Cannot create file {}", name);
        exit(3);
    }

    println!("File {} was created!", name);
}

fn print_help() {
    println!("Usage: create <file name>");
    println!("Creates a file with the given name");
}

fn is_target_valid(p: &Path) -> bool {
    let full_path = env::current_dir().unwrap().join(p);
    if let Ok(resolved_path) = full_path.canonicalize() {
        return resolved_path.starts_with("/home/writer")
    }

    false
}
