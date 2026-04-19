use r#async::hard::parallel_word_count::parallel_word_count;

#[test]
fn test_basic() {
    let lines = vec![
        "hello world".to_string(),
        "hello rust".to_string(),
        "parallel rust".to_string(),
    ];
    let counts = parallel_word_count(lines);
    assert_eq!(counts.get("hello"), Some(&2));
    assert_eq!(counts.get("rust"), Some(&2));
    assert_eq!(counts.get("world"), Some(&1));
    assert_eq!(counts.get("parallel"), Some(&1));
}

#[test]
fn test_empty() {
    let counts = parallel_word_count(vec![]);
    assert!(counts.is_empty());
}

#[test]
fn test_single_line() {
    let lines = vec!["one two three".to_string()];
    let counts = parallel_word_count(lines);
    assert_eq!(counts.len(), 3);
}
