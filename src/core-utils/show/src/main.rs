mod show;

use std::{env, process::exit};

use crate::show::show;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&String::from("help")) {
        print_help();
        exit(1);
    }

    if let Some(filename) = args.get(1) {
        if let Err(err) = show(filename) {
            eprintln!("couldn't read {}: {}", filename, err);
            exit(2);
        }
    }
}

fn print_help() {
    println!("Usage: show <file name>");
    println!("Prints the content of the given file");
}
