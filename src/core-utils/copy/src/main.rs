use std::env;
use std::process::exit;

mod copy;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 || args.contains(&String::from("help")) {
        print_help();
        exit(1);
    }

    if let Some(source) = args.get(1) &&
        let Some(destination) = args.get(2) {

        match copy::copy(source, destination) {
            Ok(()) => {},
            Err(msg) => eprintln!("Error: {}", msg),
        }
    }
}

fn print_help() {
    println!("Usage: copy <source> <destination>");
    println!("Copy a file or a folder.")
}
