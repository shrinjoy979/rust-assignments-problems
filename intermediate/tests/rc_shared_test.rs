use intermediate::medium::rc_shared::shared_ownership;

#[test]
fn test_ref_count() {
    let (count, val) = shared_ownership("hello".to_string());
    assert_eq!(count, 3); // original + 2 clones
    assert_eq!(val, "hello");
}

#[test]
fn test_empty_string() {
    let (count, val) = shared_ownership("".to_string());
    assert_eq!(count, 3);
    assert_eq!(val, "");
}

#[test]
fn test_long_string() {
    let s = "a".repeat(1000);
    let (count, val) = shared_ownership(s.clone());
    assert_eq!(count, 3);
    assert_eq!(val, s);
}
