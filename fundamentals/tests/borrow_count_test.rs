use fundamentals::easy::borrow_count::count_above;

#[test]
fn test_basic() {
    assert_eq!(count_above(&vec![1, 5, 10, 15, 20], 10), 2);
}

#[test]
fn test_none_above() {
    assert_eq!(count_above(&vec![1, 2, 3], 10), 0);
}

#[test]
fn test_all_above() {
    assert_eq!(count_above(&vec![100, 200, 300], 0), 3);
}

#[test]
fn test_empty() {
    assert_eq!(count_above(&vec![], 5), 0);
}
