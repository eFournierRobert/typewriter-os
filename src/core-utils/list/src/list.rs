use std::{env, path::Path};

/// Lists all non-hidden entries in the given directory (or current dir if empty).
pub fn list(dir: &str) -> Result<(), String> {
    let dir_path = if dir.is_empty() {
        env::current_dir().map_err(|e| e.to_string())?
    } else {
        Path::new(dir).to_path_buf()
    };

    if !is_target_valid(&dir_path) {
        return Err(format!("{} doesn't exist", dir));
    }

    for entry in dir_path.read_dir().map_err(|e| e.to_string())? {
        let entry_path = entry.map_err(|e| e.to_string())?.path();
        let entry_name = match entry_path.file_name() {
            Some(entry_name) => entry_name.to_string_lossy(),
            None => continue,
        };

        if entry_name.starts_with(".") || entry_name == "lost+found" {
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

fn is_target_valid(p: &Path) -> bool {
    let full_path = env::current_dir().unwrap().join(p);
    if let Ok(resolved_path) = full_path.canonicalize() {
        return resolved_path.starts_with("/home/writer")
    }

    false
}