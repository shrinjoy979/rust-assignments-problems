use intermediate::medium::refcell_counter::Counter;

#[test]
fn test_increment() {
    let c = Counter::new(0);
    c.increment();
    c.increment();
    assert_eq!(c.get(), 2);
}

#[test]
fn test_initial_value() {
    let c = Counter::new(10);
    assert_eq!(c.get(), 10);
}

#[test]
fn test_many_increments() {
    let c = Counter::new(0);
    for _ in 0..100 {
        c.increment();
    }
    assert_eq!(c.get(), 100);
}
