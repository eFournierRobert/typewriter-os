use std::fs;
use std::path::{Path, PathBuf};

pub fn copy(source: &str, destination: &str) -> Result<(), String> {
    let dest_path = Path::new(destination);
    let src_path = Path::new(source);

    if !src_path.exists() {
        return Err("File or folder not found".to_string());
    }

    if !is_destination_valid(dest_path) {
        return Err("destination is a file that already exist".to_string());
    }

    if src_path.is_file() {
        let real_dest_str = build_dest_path(src_path, dest_path);
        let real_dest_path = Path::new(&real_dest_str);

        if real_dest_path.exists() {
            let error_message = format!("{} already exist", real_dest_path.display());
            return Err(error_message);
        }

        return match fs::copy(src_path, real_dest_path) {
            Ok(_) => Ok(()),
            Err(_) => Err("Couldn't copy file".to_string()),
        }
    }

    let new_dir = build_dest_path(src_path, dest_path);
    match fs::create_dir(&new_dir) {
        Ok(_) => {},
        Err(e) => return Err(format!("Couldn't create destination folder: {}", e)),
    }

    for entry in fs::read_dir(src_path).map_err(|e| format!("Couldn't read source folder: {}", e))? {
        let entry = entry.map_err(|e| format!("Couldn't read entry: {}", e))?;
        let path = entry.path();

        let temp_src = path.to_string_lossy().to_string();
        let temp_dest = build_dest_path(path.as_path(), new_dir.as_path())
            .to_string_lossy()
            .to_string();

        copy(&temp_src, &temp_dest)?;
    }

    Ok(())
}

fn build_dest_path(source: &Path, destination: &Path) -> PathBuf {
    if destination.is_dir() {
        let filename = source.file_name().expect("source has no filename");
        return destination.join(filename);
    }

    destination.to_path_buf()
}

fn is_destination_valid(destination: &Path) -> bool {
    if destination.exists() {
        return destination.is_dir();
    }

    true
}
