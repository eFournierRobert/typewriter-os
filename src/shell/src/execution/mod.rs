use std::env;
use std::path::Path;
use std::process::Command;

pub fn execute(input: &str) -> Result<(), String> {
    let full_command = parse_input(&input);

    match full_command[0].as_str() {
        "cd" => {
            if full_command.len() > 1 {
                change_dir(&full_command[1])?;
            } else {
                change_dir("/home/writer")?;
            }
        }
        "help" => {
            print_help_screen();
        }
        _ => {
            let mut child = Command::new(full_command.get(0).unwrap())
                .args(full_command[1..].iter())
                .spawn()
                .map_err(|e| format!("failed to execute {}: {}", full_command[0], e))?;

            if let Ok(e) = child.wait()
                && !e.success()
            {
                return Err(format!(
                    "{} exited with status {}",
                    full_command[0],
                    e.code().unwrap()
                ));
            }
        }
    }

    Ok(())
}

fn parse_input(input: &str) -> Vec<String> {
    input.split_whitespace().map(String::from).collect()
}

fn change_dir(dest: &str) -> Result<(), String> {
    Ok(env::set_current_dir(Path::new(dest)).map_err(|e| e.to_string())?)
}

fn print_help_screen() {
    let utilities = [
        ("copy", "Copy files and folders"),
        ("move", "Move files and folders to a folder"),
        ("rename", "Rename a file or a folder"),
        ("delete", "Delete a file or a folder"),
        ("list", "List the elements in a directory"),
        ("create", "Create a new empty file"),
        ("new", "Create a new folder"),
        ("show", "Show the content of a file"),
        ("find", "Find a file or a folder inside a folder"),
        ("search", "Search for a string inside a file"),
        ("vim", "Text editor"),
    ];

    println!("Shell help screen\n");
    println!("List of utilities: ");
    for util in utilities {
        println!("\t{}: {}", util.0, util.1);
    }
}
