use fundamentals::medium::word_counter::word_count;

#[test]
fn test_basic() {
    let result = word_count("hello world hello");
    assert_eq!(result.get("hello"), Some(&2));
    assert_eq!(result.get("world"), Some(&1));
}

#[test]
fn test_case_insensitive() {
    let result = word_count("Hello HELLO hello");
    assert_eq!(result.get("hello"), Some(&3));
}

#[test]
fn test_empty() {
    let result = word_count("");
    assert!(result.is_empty());
}

#[test]
fn test_single_word() {
    let result = word_count("rust");
    assert_eq!(result.get("rust"), Some(&1));
    assert_eq!(result.len(), 1);
}
