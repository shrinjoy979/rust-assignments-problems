use fundamentals::easy::char_classifier::classify_char;

#[test]
fn test_alpha() {
    assert_eq!(classify_char('a'), "alphabetic");
}

#[test]
fn test_digit() {
    assert_eq!(classify_char('5'), "numeric");
}

#[test]
fn test_space() {
    assert_eq!(classify_char(' '), "whitespace");
}

#[test]
fn test_special() {
    assert_eq!(classify_char('@'), "other");
}

#[test]
fn test_uppercase() {
    assert_eq!(classify_char('Z'), "alphabetic");
}
