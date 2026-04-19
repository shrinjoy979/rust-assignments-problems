use fundamentals::medium::lifetime_longest::longest;

#[test]
fn test_first_longer() {
    assert_eq!(longest("hello", "hi"), "hello");
}

#[test]
fn test_second_longer() {
    assert_eq!(longest("hi", "hello"), "hello");
}

#[test]
fn test_equal_length() {
    assert_eq!(longest("abc", "def"), "abc");
}

#[test]
fn test_empty_and_nonempty() {
    assert_eq!(longest("", "x"), "x");
}
