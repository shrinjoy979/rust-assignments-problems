use fundamentals::easy::reverse_string::reverse_string;

#[test]
fn test_hello() {
    assert_eq!(reverse_string("hello"), "olleh");
}

#[test]
fn test_empty() {
    assert_eq!(reverse_string(""), "");
}

#[test]
fn test_single() {
    assert_eq!(reverse_string("a"), "a");
}

#[test]
fn test_palindrome() {
    assert_eq!(reverse_string("racecar"), "racecar");
}

#[test]
fn test_unicode() {
    assert_eq!(reverse_string("rust 🦀"), "🦀 tsur");
}
