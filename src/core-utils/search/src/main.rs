mod search;

use std::{env, process::exit};

use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints the help message");
    opts.optopt("p", "padding", "amount of padding in the output", "PADDING");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("failed to parse arguments: {}", err);
            exit(2);
        }
    };

    if matches.opt_present("h") || matches.free.len() < 2 {
        print_help();
        exit(1);
    }

    let mut padding: usize = 0;
    if matches.opt_present("p") {
        padding = matches.opt_get("p").unwrap().unwrap();
    }

    let substring = matches.free.get(0).unwrap();
    let filename = matches.free.get(1).unwrap();

    if let Err(e) = search::search(padding, substring, filename) {
        eprintln!("Error while searching {}: {}", filename, e);
        exit(2);
    }
}

fn print_help() {
    println!("Usage: search [-p --padding] <substring> <file>");
    println!("Finds a substring inside a file");
    println!("\nOptions: ");
    println!("\t-p, --padding\tAmount of padding in the output");
    println!("\t-h, --help\tShows this help menu");
}
