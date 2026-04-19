use std::collections::HashMap;

#[test]
fn test_basic() {
    use intermediate::hash_map;
    let m: HashMap<&str, i32> = hash_map! { "a" => 1, "b" => 2 };
    assert_eq!(m.get("a"), Some(&1));
    assert_eq!(m.get("b"), Some(&2));
}

#[test]
fn test_empty() {
    use intermediate::hash_map;
    let m: HashMap<&str, i32> = hash_map! {};
    assert!(m.is_empty());
}

#[test]
fn test_single() {
    use intermediate::hash_map;
    let m: HashMap<&str, &str> = hash_map! { "key" => "value" };
    assert_eq!(m.get("key"), Some(&"value"));
}

#[test]
fn test_integer_keys() {
    use intermediate::hash_map;
    let m: HashMap<i32, &str> = hash_map! { 1 => "one", 2 => "two", 3 => "three" };
    assert_eq!(m.len(), 3);
    assert_eq!(m.get(&2), Some(&"two"));
}
