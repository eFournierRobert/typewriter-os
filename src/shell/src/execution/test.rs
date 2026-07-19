use super::parse_input;

#[test]
fn test_basic_words() {
    let result = parse_input("echo hello world");
    assert_eq!(
        result,
        vec!["echo".to_string(), "hello".to_string(), "world".to_string()]
    );
}

#[test]
fn test_single_quotes_preserve_spaces() {
    let result = parse_input("show 'my file.txt'");
    assert_eq!(result, vec!["show".to_string(), "my file.txt".to_string()]);
}

#[test]
fn test_double_quotes_preserve_spaces() {
    let result = parse_input(r#"copy "hello world""#);
    assert_eq!(result, vec!["copy".to_string(), "hello world".to_string()]);
}

#[test]
fn test_mixed_quoted_and_unquoted() {
    let result = parse_input(r#"list -l "dir name""#);
    assert_eq!(
        result,
        vec!["list".to_string(), "-l".to_string(), "dir name".to_string()]
    );
}

#[test]
fn test_multiple_spaces_as_single_delimiter() {
    let result = parse_input("cmd   arg1    arg2");
    assert_eq!(
        result,
        vec!["cmd".to_string(), "arg1".to_string(), "arg2".to_string()]
    );
}
