use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub fn show(filename: &String) -> Result<(), String> {
    let file_path = Path::new(filename);

    if !file_path.exists() {
        return Err(format!("{} doesn't exist", filename));
    }

    if file_path.is_dir() {
        return Err(format!("{} is a folder", filename));
    }

    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();

    if let Err(err) = buf_reader.read_to_string(&mut content) {
        return Err(err.to_string());
    }

    println!("{}", content);

    Ok(())
}
