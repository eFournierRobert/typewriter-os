mod execution;
mod history;
mod input;

fn main() {
    let mut history = history::new();

    loop {
        match input::get_user_input(&mut history) {
            Ok(user_input) => {
                if !user_input.is_empty() {
                    history.append(&user_input);
                    history.reset_index_position();

                    if user_input.eq("exit") || user_input.eq("poweroff") {
                        break;
                    }

                    if let Err(e) = execution::execute(&user_input) {
                        println!("shell: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    history.save().expect("error occurred while saving history");
}
