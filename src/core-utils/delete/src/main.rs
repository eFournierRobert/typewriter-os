mod delete;

use std::{env, process::exit};

use crate::delete::delete;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&String::from("help")) {
        print_help();
        exit(1);
    }

    if let Some(path) = args.get(1) {
        match delete(path) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Couldn't delete {}: {}", path, err);
                exit(2);
            }
        }
    }
}

fn print_help() {
    println!("Usage: delete <file or folder>");
    println!("Delete a file or a folder");
}
