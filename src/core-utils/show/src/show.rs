use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn show(filename: &str) -> Result<(), String> {
    let file_path = Path::new(filename);

    if !file_path.exists() {
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
