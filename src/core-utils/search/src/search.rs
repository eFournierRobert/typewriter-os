use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn search(padding: i8, substring: &String, filename: &String) -> Result<(), String> {
    if padding < 0 {
        return Err(String::from("padding cannot be negative"));
    }

    let file_path = Path::new(filename);

    if !file_path.exists() || file_path.is_dir() {
        return Err(String::from("file doesn't exist or is a folder"));
    }

    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let mut content = Vec::new();
    let mut line_indexes = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let l = line.map_err(|e| e.to_string())?;
        if l.contains(substring) {
            line_indexes.push(i);
        }

        content.push(l);
    }

    if line_indexes.len() == 0 {
        return Ok(());
    }

    for i in line_indexes {
        if padding == 0 {
            println!("{}", content.get(i).unwrap());
            continue;
        }

        if i != 0 {
            let mut top = i as i8 - padding;
            if top < 0 {
                top = 0;
            }

            for line in content[top as usize..i].iter() {
                println!("{}", line);
            }
        }

        let mut bottom = i + padding as usize;
        if bottom > content.len() - 1 {
            bottom = content.len() - 1;
        }

        for line in content[i..bottom].iter() {
            println!("{}", line);
        }

        println!();
    }

    Ok(())
}
