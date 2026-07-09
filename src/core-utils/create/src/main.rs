use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&String::from("help")) {
        print_help();
        exit(1);
    }

    if let Some(filename) = args.get(1) {
        if let Err(err) = fs::File::create(filename) {
            eprintln!("Error while creating {}: {}", filename, err);
            exit(2);
        }
    }
}

fn print_help() {
    println!("Usage: create <file name>");
    println!("Creates a file with the given name");
}
