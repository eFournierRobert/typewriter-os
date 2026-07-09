use crate::history::History;
use std::{
    env,
    io::{Write, stdin, stdout},
};
use termion::raw::IntoRawMode;
use termion::{cursor::DetectCursorPos, event::Key, input::TermRead};

fn get_cursor() -> Result<String, String> {
    let current_dir = env::current_dir().map_err(|_| "$ ")?;
    let current_dir_string = current_dir.to_string_lossy().replace("/home/writer", "~");

    Ok(format!("[{}]-> ", current_dir_string))
}

pub fn get_user_input(history: &mut History) -> Result<String, String> {
    let cursor = get_cursor()?;
    let mut stdout = stdout().into_raw_mode().unwrap();
    let prompt_cols = cursor.chars().count() as u16;

    write!(stdout, "{}", cursor).unwrap();
    stdout.flush().unwrap();

    let mut user_input = Vec::new();
    let mut input_cursor_position = 0;

    for k in stdin().keys() {
        match k.as_ref().unwrap() {
            Key::Ctrl('c') => {
                write!(stdout, "{}\r\n", "^C").unwrap();
                stdout.flush().unwrap();
                return Ok(String::new());
            }
            Key::Ctrl('l') => {
                write!(
                    stdout,
                    "{}{}\r\n",
                    termion::clear::All,
                    termion::cursor::Goto(1, 0)
                )
                .unwrap();
                stdout.flush().unwrap();
                return Ok(String::new());
            }
            Key::Ctrl('h') => {
                write!(stdout, "{}\r\n", "help").unwrap();
                return Ok(String::from("help"));
            }
            Key::Char('\n') => {
                write!(stdout, "\r\n").unwrap();
                stdout.flush().unwrap();
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
                if let Some(c) = history.peak_prev_command() {
                    user_input = c.chars().collect();
                    input_cursor_position = user_input.len();
                }
            }
            Key::Down => {
                if let Some(c) = history.peak_next_command() {
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
            _ => {}
        }

        let input_str: String = user_input.iter().collect();
        let terminal_col = prompt_cols + input_cursor_position as u16 + 1;
        let row = stdout.cursor_pos().unwrap().1;

        write!(
            stdout,
            "\r{}{}{}{}",
            termion::clear::CurrentLine,
            cursor,
            input_str,
            termion::cursor::Goto(terminal_col, row)
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    Ok(user_input.iter().collect())
}
