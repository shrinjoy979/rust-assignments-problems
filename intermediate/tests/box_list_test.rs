use intermediate::medium::box_list::List;

#[test]
fn test_sum() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    assert_eq!(list.sum(), 6);
}

#[test]
fn test_len() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    assert_eq!(list.len(), 2);
}

#[test]
fn test_empty() {
    assert_eq!(List::Nil.sum(), 0);
    assert_eq!(List::Nil.len(), 0);
}

#[test]
fn test_single() {
    let list = List::Cons(42, Box::new(List::Nil));
    assert_eq!(list.sum(), 42);
    assert_eq!(list.len(), 1);
}
