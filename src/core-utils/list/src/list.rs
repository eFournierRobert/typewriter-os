use std::{env, path::Path};

pub fn list(dir: &String) -> Result<(), String> {
    let dir_path = if dir.is_empty() {
        env::current_dir().unwrap()
    } else {
        Path::new(dir).to_path_buf()
    };

    if !dir_path.exists() {
        return Err(format!("{} doesn't exist", dir));
    }

    for entry in dir_path.read_dir().map_err(|e| e.to_string())? {
        let entry_path = entry.unwrap().path();
        let entry_name = entry_path.file_name();

        println!("Name: {}", entry_name.unwrap().to_string_lossy());

        if entry_path.is_dir() {
            println!("\tType: Folder");
        } else {
            println!("\tType: File");
        }
    }

    Ok(())
}
