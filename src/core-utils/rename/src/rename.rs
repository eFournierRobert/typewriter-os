use std::{char, env, fs, path::Path};

/// Renames a file or folder to a new name within the same parent directory.
pub fn rename(source: &str, new_name: &str) -> Result<(), String> {
    let source_path = Path::new(source);

    if !is_target_valid(source_path) {
        return Err(format!("{} doesn't exist", source));
    }
    if new_name.len() > 255 {
        return Err(String::from("new name is too long"));
    }
    if !new_name.chars().all(char::is_alphanumeric) || new_name.starts_with(".") || new_name == "help"
    {
        return Err(String::from("new name is an invalid file name"));
    }

    let parent_path = source_path.parent().unwrap();
    let renamed_path = parent_path.join(new_name);

    if renamed_path.exists() {
        return Err(format!("{} already exist", renamed_path.display()));
    }

    if let Err(err) = fs::rename(source_path, renamed_path) {
        return Err(err.to_string());
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