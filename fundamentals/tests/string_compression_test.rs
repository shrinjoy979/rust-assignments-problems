use fundamentals::medium::string_compression::compress;

#[test]
fn test_compression() {
    assert_eq!(compress("aabcccccaaa"), "a2b1c5a3");
}

#[test]
fn test_no_compression_needed() {
    assert_eq!(compress("abcdef"), "abcdef");
}

#[test]
fn test_empty() {
    assert_eq!(compress(""), "");
}

#[test]
fn test_single_char() {
    assert_eq!(compress("a"), "a");
}

#[test]
fn test_all_same() {
    assert_eq!(compress("aaaa"), "a4");
}
