use std::{
    env, fs::File, io::{BufRead, BufReader}, path::Path
};

/// Reads and prints the contents of a file line by line.
pub fn show(filename: &str) -> Result<(), String> {
    let file_path = Path::new(filename);

    if !is_target_valid(file_path) {
        return Err(format!("doesn't exist"));
    }

    if file_path.is_dir() {
        return Err(format!("is a folder"));
    }

    let file = File::open(filename).map_err(|e| e.to_string())?;
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let l = line.map_err(|e| e.to_string())?;
        println!("{}", l);
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