use fundamentals::medium::error_propagation::parse_and_add;

#[test]
fn test_valid() {
    assert_eq!(parse_and_add("10", "20"), Ok(30));
}

#[test]
fn test_first_invalid() {
    assert!(parse_and_add("abc", "5").is_err());
}

#[test]
fn test_second_invalid() {
    assert!(parse_and_add("5", "xyz").is_err());
}

#[test]
fn test_both_invalid() {
    assert!(parse_and_add("a", "b").is_err());
}

#[test]
fn test_negative() {
    assert_eq!(parse_and_add("-3", "7"), Ok(4));
}
