use super::History;

fn new_test_history(max: usize) -> History {
    History {
        commands: Vec::new(),
        position_index: 0,
        max_history_size: max,
    }
}

#[test]
fn test_max_history_overflow() {
    let mut hist = new_test_history(5);

    for i in 0..7 {
        hist.append(&i.to_string());
    }

    assert_eq!(hist.commands.len(), 5);
    assert_eq!(hist.commands[0], "2");
    assert_eq!(hist.commands[1], "3");
    assert_eq!(hist.commands[2], "4");
    assert_eq!(hist.commands[3], "5");
    assert_eq!(hist.commands[4], "6");
}

#[test]
fn test_previous_navigation() {
    let mut hist = new_test_history(10);

    hist.append("first");
    hist.append("second");
    hist.append("third");
    hist.reset_index_position();

    assert_eq!(hist.get_previous_command().unwrap(), "third");
    assert_eq!(hist.get_previous_command().unwrap(), "second");
    assert_eq!(hist.get_previous_command().unwrap(), "first");
}

#[test]
fn test_previous_at_start_returns_none() {
    let mut hist = new_test_history(10);

    hist.append("cmd");
    hist.reset_index_position();

    assert_eq!(hist.get_previous_command().unwrap(), "cmd");
    assert!(hist.get_previous_command().is_none());
}

#[test]
fn test_next_navigation() {
    let mut hist = new_test_history(10);

    hist.append("first");
    hist.append("second");
    hist.append("third");
    hist.reset_index_position();

    assert_eq!(hist.get_previous_command().unwrap(), "third");
    assert_eq!(hist.get_previous_command().unwrap(), "second");
    assert_eq!(hist.get_next_command().unwrap(), "third");
}

#[test]
fn test_next_at_end_returns_none() {
    let mut hist = new_test_history(10);

    hist.append("cmd");
    hist.reset_index_position();

    assert!(hist.get_next_command().is_none());
}

#[test]
fn test_reset_moves_cursor_to_end() {
    let mut hist = new_test_history(10);

    hist.append("first");
    hist.append("second");
    hist.reset_index_position();

    assert_eq!(hist.get_previous_command().unwrap(), "second");
    hist.reset_index_position();
    assert!(hist.get_next_command().is_none());
}
