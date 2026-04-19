use fundamentals::easy::append_world::append_world;

#[test]
fn test_hello() {
    assert_eq!(append_world(String::from("hello")), "hello world");
}

#[test]
fn test_empty() {
    assert_eq!(append_world(String::from("")), " world");
}

#[test]
fn test_existing() {
    assert_eq!(append_world(String::from("brave new")), "brave new world");
}
