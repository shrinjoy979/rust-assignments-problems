use fundamentals::medium::trait_default::{Greet, Person};

#[test]
fn test_name() {
    let p = Person { name: "Alice".to_string() };
    assert_eq!(p.name(), "Alice");
}

#[test]
fn test_greeting() {
    let p = Person { name: "Bob".to_string() };
    assert_eq!(p.greeting(), "Hello, Bob!");
}

#[test]
fn test_empty_name() {
    let p = Person { name: "".to_string() };
    assert_eq!(p.greeting(), "Hello, !");
}
