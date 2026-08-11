use std::path::Path;
use std::{env, fs};

/// Moves a file or folder from source to a destination directory.
pub fn mve(source: &str, destination: &str) -> Result<(), String> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    if !is_target_valid(source_path) {
        return Err(format!("{} does not exist", source));
    } else if !is_target_valid(destination_path) && destination_path.is_dir() {
        return Err(format!("{} is not a valid destination", destination));
    }

    let real_destination = destination_path.join(match source_path.file_name() {
        Some(name) => name,
        None => return Err(format!("Could not create full path for {}", source)),
    });

    if real_destination.exists() {
        return Err(format!("{} already exists", real_destination.display()));
    }

    if source_path.is_dir() {
        if let Err(err) = fs::create_dir(&real_destination) {
            return Err(err.to_string());
        }
    }

    if let Err(err) = fs::rename(source_path, &real_destination) {
        return Err(err.to_string());
    }

    Ok(())
}

fn is_target_valid(p: &Path) -> bool {
    let full_path = env::current_dir().unwrap().join(p);
    if let Ok(resolved_path) = full_path.canonicalize() {
        return resolved_path.starts_with("/home/writer");
    }

    false
}
