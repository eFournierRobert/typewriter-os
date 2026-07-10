mod mve;

use crate::mve::mve;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.contains(&String::from("help")) {
        print_help();
        exit(1);
    }

    if let Some(source) = args.get(1)
        && let Some(destination) = args.get(2)
    {
        if let Err(err) = mve(source, destination) {
            eprintln!("Error while moving {}: {}", source, err);
            exit(2);
        }
    }
}

fn print_help() {
    println!("Usage: move <source> <destination>");
    println!("Move a file or a folder to another folder.");
}
