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

fn create_file(name: &str) {
    if Path::new(name).exists() {
        eprintln!("file {} already exists", name);
        exit(4);
    }

    if let Err(err) = fs::File::create(name) {
        eprintln!("Error while creating {}: {}", name, err);
        exit(2);
    }

    println!("File {} was created!", name);
}

fn print_help() {
    println!("Usage: create <file name>");
    println!("Creates a file with the given name");
}
