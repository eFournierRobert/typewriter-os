use std::fs;
use std::path::{Path, PathBuf};

/// Copies a file or folder recursively from source to destination.
pub fn copy<S: AsRef<Path>, D: AsRef<Path>>(source: S, destination: D) -> Result<(), String> {
    let dest_path = destination.as_ref();
    let src_path = source.as_ref();

    if !src_path.exists() {
        return Err("File or folder not found".to_string());
    }

    if !is_destination_valid(dest_path) {
        return Err("destination is a file that already exists".to_string());
    }

    if src_path.is_file() {
        let real_dest_path = build_dest_path(src_path, dest_path);

        if real_dest_path.exists() {
            let error_message = format!("{} already exists", real_dest_path.display());
            return Err(error_message);
        }

        return match fs::copy(src_path, real_dest_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Couldn't copy file: {}", e)),
        };
    }

    let new_dir = build_dest_path(src_path, dest_path);
    if let Err(e) = fs::create_dir(&new_dir) {
        return Err(format!("Couldn't create destination folder: {}", e));
    }

    for entry in
        fs::read_dir(src_path).map_err(|e| format!("Couldn't read source folder: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Couldn't read entry: {}", e))?;
        let path = entry.path();
        let temp_dest = build_dest_path(path.as_path(), new_dir.as_path());

        copy(&path, &temp_dest)?;
    }

    Ok(())
}

/// Builds the actual destination path. If destination is a directory, appends the source's filename.
fn build_dest_path(source: &Path, destination: &Path) -> PathBuf {
    if destination.is_dir() {
        let filename = source.file_name().expect("source has no filename");
        return destination.join(filename);
    }

    destination.to_path_buf()
}

/// Checks whether the destination is valid: it must not be an existing file.
fn is_destination_valid(destination: &Path) -> bool {
    if destination.exists() {
        return destination.is_dir();
    }

    true
}
