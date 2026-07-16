use std::{
    fs,
    io::{self, Write},
    path::Path,
};

/// Deletes a file or folder after interactive confirmation from the user.
pub fn delete(source: &str) -> Result<(), String> {
    let source_path = Path::new(source);

    if !source_path.exists() {
        return Err(format!("{} doesn't exist", source));
    }

    if !ask_confirmation(source) {
        return Err(String::from("Aborted deletion"));
    }

    if source_path.is_dir() {
        fs::remove_dir_all(source_path).map_err(|e| e.to_string())?;
    } else {
        fs::remove_file(source_path).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Prompts the user for confirmation before deletion. Returns `true` if confirmed.
fn ask_confirmation(source: &str) -> bool {
    print!("Are you sure you want to delete {} ? [y|N]: ", source);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let user_answer = input.trim();

    user_answer == "y" || user_answer == "Y" || user_answer == "yes"
}
