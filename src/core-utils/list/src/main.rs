mod list;

use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("help")) {
        print_help();
        exit(1);
    }

    if let Some(folder) = args.get(1) {
        if let Err(err) = list::list(folder) {
            println!("Error listing folder {}: {}", folder, err);
            exit(2);
        }
    } else {
        if let Err(err) = list::list("") {
            println!("Error listing current folder: {}", err);
            exit(2);
        }
    }
}

fn print_help() {
    println!("Usage: list <folder>");
    println!("List all the entries in a folder. If no folder is given, the current one is taken");
}
