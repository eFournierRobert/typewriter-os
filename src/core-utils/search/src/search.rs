use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn search(padding: usize, substring: &str, filename: &str) -> Result<(), String> {
    let file_path = Path::new(filename);

    if !file_path.exists() || file_path.is_dir() {
        return Err(String::from("file doesn't exist or is a folder"));
    }

    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let mut buffer: VecDeque<String> = VecDeque::new();
    let mut last_match_at: Option<usize> = None;

    for (i, line) in reader.lines().enumerate() {
        let l = line.map_err(|e| e.to_string())?;
        match last_match_at {
            Some(j) => {
                let bottom = j + padding;
                if l.contains(substring) {
                    last_match_at = Some(i);
                }

                if bottom > i {
                    println!("{}", l);
                } else if bottom == i {
                    println!("{}", l);
                    last_match_at = None;
                    buffer.clear();
                }
            }
            None => {
                if l.contains(substring) {
                    last_match_at = Some(i);

                    for top_l in &buffer {
                        println!("{}", top_l)
                    }
                    println!("{}", l);
                    continue;
                }

                buffer.push_back(l);
                if buffer.len() > padding {
                    buffer.pop_front();
                }
            }
        }
    }

    Ok(())
}
