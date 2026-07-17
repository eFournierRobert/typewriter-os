use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const HISTORY_FILE_PATH: &str = "/home/writer/.history";

pub struct History {
    commands: Vec<String>,
    position_index: usize,
}

impl History {
    pub fn append(&mut self, command: &str) {
        if self.commands.len() == 500 {
            self.commands.remove(0);
        }

        self.commands.push(command.to_string());
    }

    pub fn save(self) -> Result<(), String> {
        let mut file = File::create(HISTORY_FILE_PATH)
            .map_err(|e| format!("Error while saving history: {}", e))?;

        for command in self.commands {
            writeln!(file, "{}", command)
                .map_err(|e| format!("Error while saving history: {}", e))?;
        }

        Ok(())
    }

    pub fn reset_index_position(&mut self) {
        if self.commands.len() > 0 {
            self.position_index = self.commands.len();
        } else {
            self.position_index = 0;
        }
    }

    pub fn get_previous_command(&mut self) -> Option<&String> {
        if self.position_index == 0 {
            return None;
        }

        self.position_index -= 1;
        let c = self.commands.get(self.position_index);

        c
    }

    pub fn get_next_command(&mut self) -> Option<&String> {
        if self.position_index == self.commands.len() {
            return None;
        }

        self.position_index += 1;
        let c = self.commands.get(self.position_index);

        c
    }
}

pub fn new() -> History {
    let mut hist = History {
        commands: Vec::new(),
        position_index: 0,
    };

    if let Some(commands_history) = read_history() {
        hist.commands = commands_history;

        hist.reset_index_position();
    }

    hist
}

fn read_history() -> Option<Vec<String>> {
    let file_path = Path::new(HISTORY_FILE_PATH);
    if !file_path.exists() {
        eprintln!("shell: can't load history. .history file does not exist");
        return None;
    }

    match fs::read_to_string(file_path) {
        Ok(content) => Some(content.lines().map(String::from).collect()),
        Err(_) => None,
    }
}
