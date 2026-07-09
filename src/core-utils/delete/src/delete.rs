use std::{
    fs,
    io::{self, Write},
    path::Path,
};

pub fn delete(source: &String) -> Result<(), String> {
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

fn ask_confirmation(source: &String) -> bool {
    print!("Are you sure you want to delete {} ? [y|N]: ", source);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim() == "y" || input.trim() == "Y"
}
