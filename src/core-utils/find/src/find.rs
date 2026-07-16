use std::{fs, path::Path};

pub fn find(name: &str, folder_name: &str) -> Result<(), String> {
    let folder_path = Path::new(folder_name);

    if !folder_path.exists() {
        return Err(format!("{} does not exist", folder_name));
    } else if folder_path.is_file() {
        return Err(format!("{} is a file", folder_name));
    }

    for entry in fs::read_dir(folder_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let entry_name = entry
            .file_name()
            .into_string()
            .map_err(|e| e.to_string_lossy().to_string())?;

        if entry_name.starts_with(".") {
            continue;
        }

        if entry_name == *name {
            println!("{}", entry.path().display());
            return Ok(());
        }

        if entry.path().is_dir() {
            find(name, &entry.path().to_string_lossy().to_string())?;
        }
    }

    Ok(())
}
