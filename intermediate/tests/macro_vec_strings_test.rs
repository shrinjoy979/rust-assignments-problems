#[test]
fn test_basic() {
    use intermediate::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["hello", "world"];
    assert_eq!(v, vec!["hello".to_string(), "world".to_string()]);
}

#[test]
fn test_single() {
    use intermediate::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["rust"];
    assert_eq!(v, vec!["rust".to_string()]);
}

#[test]
fn test_empty() {
    use intermediate::vec_of_strings;
    let v: Vec<String> = vec_of_strings![];
    assert!(v.is_empty());
}

#[test]
fn test_multiple() {
    use intermediate::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["a", "b", "c", "d"];
    assert_eq!(v.len(), 4);
}
