use std::{fs, path::Path};

pub fn find(name: &String, folder_name: &String) -> Result<(), String> {
    let folder_path = Path::new(folder_name);

    if !folder_path.exists() {
        return Err(format!("{} does not exist", folder_name));
    } else if folder_path.is_file() {
        return Err(format!("{} is a file", folder_name));
    }

    let mut dir_entries: Vec<String> = Vec::new();

    for entry in fs::read_dir(folder_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let entry_name = entry
            .file_name()
            .into_string()
            .map_err(|e| e.into_string().unwrap())?;

        if entry_name.starts_with(".") {
            continue
        }

        if entry_name == *name {
            println!("{}", entry.path().display());
            return Ok(());
        }

        let path = entry.path();

        if path.is_dir() {
            dir_entries.push(path.to_string_lossy().to_string());
        }
    }

    for entry in dir_entries {
        find(name, &entry)?;
    }

    Ok(())
}
