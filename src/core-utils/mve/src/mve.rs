use std::fs;
use std::path::Path;

pub fn mve(source: &String, destination: &String) -> Result<(), String> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    if !source_path.exists() {
        return Err(format!("{} does not exist", source));
    } else if !is_destination_valid(destination_path) {
        return Err(format!("{} is not a valid destination", destination))
    }

    let real_destination = destination_path.join(source_path.file_name().unwrap());

    if real_destination.exists() {
        return Err(format!("{} already exist", real_destination.display()))
    }

    if source_path.is_dir() {
        if let Err(err) = fs::create_dir(&real_destination) {
            return Err(err.to_string())
        }
    }

    if let Err(err) = fs::rename(source_path, &real_destination) {
        return Err(err.to_string())
    }

    Ok(())
}

fn is_destination_valid(destination: &Path) -> bool {
    destination.exists() && destination.is_dir()
}