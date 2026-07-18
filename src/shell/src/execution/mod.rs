//! Command parsing, built-in dispatch, and external binary execution.

#[cfg(test)]
mod test;

use std::env;
use std::path::Path;
use std::process::Command;

/// Parse input and execute a command. Built-ins (`cd`, `help`) are handled inline;
/// everything else spawns a child process.
pub fn execute(input: &str) -> Result<(), String> {
    let full_command = parse_input(input);

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
            let mut child = Command::new(&full_command[0])
                .args(full_command[1..].iter())
                .spawn()
                .map_err(|e| format!("failed to execute {}: {}", full_command[0], e))?;

            if let Ok(e) = child.wait()
                && !e.success()
            {
                return Err(format!(
                    "{} exited with status {}",
                    full_command[0],
                    e.code().unwrap_or(-1)
                ));
            }
        }
    }

    Ok(())
}

/// Tokenize a shell command line.
/// Splits on whitespace while preserving spaces inside single or double quotes.
/// Quote characters themselves are stripped from the output.
fn parse_input(input: &str) -> Vec<String> {
    enum State {
        Normal,
        SingleQuote,
        DoubleQuote,
    }
    let mut input_vec = Vec::new();
    let mut current = String::new();
    let mut state = State::Normal;

    for c in input.chars() {
        match state {
            State::Normal => {
                if c.is_whitespace() {
                    if !current.is_empty() {
                        input_vec.push(std::mem::take(&mut current));
                    }
                    continue;
                }

                if c == '\'' {
                    state = State::SingleQuote;
                    continue;
                } else if c == '"' {
                    state = State::DoubleQuote;
                    continue;
                }

                current.push(c);
            }
            State::SingleQuote => {
                if c == '\'' {
                    state = State::Normal;
                    continue;
                }
                current.push(c);
            }
            State::DoubleQuote => {
                if c == '\"' {
                    state = State::Normal;
                    continue;
                }
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        input_vec.push(current);
    }

    input_vec
}

fn change_dir(dest: &str) -> Result<(), String> {
    env::set_current_dir(Path::new(dest)).map_err(|e| e.to_string())
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
        ("cd", "Change folder"),
        ("vim", "Text editor"),
    ];

    println!("Shell help screen\n");
    println!("List of utilities: ");
    for util in utilities {
        println!("\t{}: {}", util.0, util.1);
    }
}
