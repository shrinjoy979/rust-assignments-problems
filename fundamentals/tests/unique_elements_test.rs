use fundamentals::medium::unique_elements::unique_elements;

#[test]
fn test_duplicates() {
    assert_eq!(unique_elements(vec![1, 2, 2, 3, 1, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn test_no_duplicates() {
    assert_eq!(unique_elements(vec![5, 4, 3]), vec![5, 4, 3]);
}

#[test]
fn test_empty() {
    assert_eq!(unique_elements(vec![]), vec![]);
}

#[test]
fn test_all_same() {
    assert_eq!(unique_elements(vec![7, 7, 7, 7]), vec![7]);
}
