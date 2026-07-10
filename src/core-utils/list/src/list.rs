use std::{env, path::Path};

pub fn list(dir: &str) -> Result<(), String> {
    let dir_path = if dir.is_empty() {
        env::current_dir().map_err(|e| e.to_string())?
    } else {
        Path::new(dir).to_path_buf()
    };

    if !dir_path.exists() {
        return Err(format!("{} doesn't exist", dir));
    }

    for entry in dir_path.read_dir().map_err(|e| e.to_string())? {
        let entry_path = entry.map_err(|e| e.to_string())?.path();
        let entry_name = match entry_path.file_name() {
            Some(entry_name) => entry_name.to_string_lossy(),
            None => continue,
        };

        if entry_name.starts_with(".") {
            continue;
        }

        println!("Name: {}", entry_name);

        if entry_path.is_dir() {
            println!("\tType: Folder");
        } else {
            println!("\tType: File");
        }
    }

    Ok(())
}
