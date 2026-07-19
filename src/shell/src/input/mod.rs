//! Raw terminal input handling with cursor-based editing and history navigation.

use crate::history::History;
use std::{
    env,
    io::{Write, stdin, stdout},
};
use termion::raw::IntoRawMode;
use termion::{cursor::DetectCursorPos, event::Key, input::TermRead};

const INPUT_ERROR_MESSAGE: &str = "failed to get user input";

/// Build the prompt string from the current working directory.
fn get_cursor() -> Result<String, String> {
    let current_dir = env::current_dir().map_err(|_| "failed to get current directory")?;
    let current_dir_string = current_dir.to_string_lossy().replace("/home/writer", "~");

    Ok(format!("[{}]-> ", current_dir_string))
}

/// Read a line of user input with interactive keyboard editing.
pub fn get_user_input(history: &mut History) -> Result<String, String> {
    let cursor = get_cursor()?;
    let mut stdout = stdout()
        .into_raw_mode()
        .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
    let prompt_cols = cursor.chars().count() as u16;

    write!(stdout, "{}", cursor).map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
    stdout
        .flush()
        .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;

    let mut user_input = Vec::new();
    let mut input_cursor_position = 0;

    for k in stdin().keys() {
        match k
            .as_ref()
            .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?
        {
            Key::Ctrl('c') => {
                write!(stdout, "{}\r\n", "^C")
                    .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
                stdout
                    .flush()
                    .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
                return Ok(String::new());
            }
            Key::Ctrl('l') => {
                write!(
                    stdout,
                    "{}{}\r\n",
                    termion::clear::All,
                    termion::cursor::Goto(1, 0)
                )
                .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
                stdout
                    .flush()
                    .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
                return Ok(String::new());
            }
            Key::Ctrl('h') => {
                write!(stdout, "{}\r\n", "help")
                    .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
                return Ok(String::from("help"));
            }
            Key::Char('\n') => {
                write!(stdout, "\r\n").map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
                stdout
                    .flush()
                    .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
                break;
            }
            Key::Char(c) => {
                user_input.insert(input_cursor_position, c.clone());
                input_cursor_position += 1;
            }
            Key::Left => {
                if input_cursor_position > 0 {
                    input_cursor_position -= 1;
                }
            }
            Key::Right => {
                if input_cursor_position < user_input.len() {
                    input_cursor_position += 1;
                }
            }
            Key::Up => {
                if let Some(c) = history.get_previous_command() {
                    user_input = c.chars().collect();
                    input_cursor_position = user_input.len();
                }
            }
            Key::Down => {
                if let Some(c) = history.get_next_command() {
                    user_input = c.chars().collect();
                    input_cursor_position = user_input.len();
                } else {
                    user_input = "".chars().collect();
                    input_cursor_position = 0;
                }
            }
            Key::Backspace => {
                if input_cursor_position > 0 {
                    user_input.remove(input_cursor_position - 1);
                    input_cursor_position -= 1;
                }
            }
            Key::Delete => {
                if user_input.len() > 0 {
                    user_input.clear();
                    input_cursor_position = 0;
                }
            }
            Key::End => {
                input_cursor_position = user_input.len();
            }
            Key::Home => {
                input_cursor_position = 0;
            }
            _ => {}
        }

        let input_str: String = user_input.iter().collect();
        let terminal_col = prompt_cols + input_cursor_position as u16 + 1;
        let row = stdout
            .cursor_pos()
            .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?
            .1;

        write!(
            stdout,
            "\r{}{}{}{}",
            termion::clear::CurrentLine,
            cursor,
            input_str,
            termion::cursor::Goto(terminal_col, row)
        )
        .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
        stdout
            .flush()
            .map_err(|e| format!("{} {}", INPUT_ERROR_MESSAGE, e))?;
    }

    Ok(user_input.iter().collect())
}
