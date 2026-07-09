use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&String::from("help")) {
        print_help();
        exit(1);
    }

    if let Some(name) = args.get(1) {
        if let Err(err) = fs::create_dir(name) {
            eprintln!("Couldn't create {}: {}", name, err);
            exit(2);
        }
    }
}

fn print_help() {
    println!("Usage: new <folder name>");
    println!("Creates a new folder with the given name");
}
