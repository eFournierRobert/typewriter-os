mod find;

use std::{env, process::exit};

use crate::find::find;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.contains(&String::from("help")) {
        print_help();
        exit(1);
    }

    if let Some(name) = args.get(1)
        && let Some(folder_name) = args.get(2)
    {
        if let Err(err) = find(name, folder_name) {
            eprintln!("couldn't find {}: {}", name, err);
            exit(2);
        }
    }
}

fn print_help() {
    println!("Usage: find <name> <folder name>");
    println!("Find a file or a folder")
}
