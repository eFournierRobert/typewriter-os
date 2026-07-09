mod mve;

use std::process::exit;
use std::env;
use crate::mve::mve;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3  || args.contains(&String::from("help")){
        print_help();
        exit(1);
    }

    if let Some(source) = args.get(1) &&
        let Some(destination) = args.get(2) {
        match mve(source, destination) {
            Ok(_) => {},
            Err(err) => eprintln!("Error while moving {}: {}", source, err)
        }
    }
}

fn print_help() {
    println!("Usage: move <source> <destination>");
    println!("Move a file or a folder to another folder.");
}