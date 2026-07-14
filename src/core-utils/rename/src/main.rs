mod rename;

use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.contains(&String::from("help")) {
        print_help();
        exit(1);
    }

    if let Some(path) = args.get(1)
        && let Some(new_name) = args.get(2)
    {
        if let Err(err) = rename::rename(path, new_name) {
            eprintln!("Error while renaming {}: {}", path, err);
            exit(2);
        }
    }
}

fn print_help() {
    println!("Usage: rename <file or folder> <new name>");
    println!("Rename a file or a folder")
}
