use fundamentals::hard::mini_phonebook::Phonebook;

#[test]
fn test_add_and_lookup() {
    let mut pb = Phonebook::new();
    pb.add("Alice", "123-4567");
    assert_eq!(pb.lookup("Alice"), Some(&"123-4567".to_string()));
}

#[test]
fn test_lookup_missing() {
    let pb = Phonebook::new();
    assert_eq!(pb.lookup("Nobody"), None);
}

#[test]
fn test_remove() {
    let mut pb = Phonebook::new();
    pb.add("Bob", "999-0000");
    assert!(pb.remove("Bob"));
    assert_eq!(pb.lookup("Bob"), None);
}

#[test]
fn test_remove_missing() {
    let mut pb = Phonebook::new();
    assert!(!pb.remove("Ghost"));
}

#[test]
fn test_display_sorted() {
    let mut pb = Phonebook::new();
    pb.add("Charlie", "111");
    pb.add("Alice", "222");
    let display = format!("{}", pb);
    assert!(display.starts_with("Alice: 222"));
    assert!(display.contains("Charlie: 111"));
}
