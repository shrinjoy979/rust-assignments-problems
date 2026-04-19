use r#async::medium::parallel_map::parallel_map;

#[test]
fn test_double() {
    let result = parallel_map(vec![1, 2, 3], |x| x * 2);
    assert_eq!(result, vec![2, 4, 6]);
}

#[test]
fn test_square() {
    let result = parallel_map(vec![1, 2, 3, 4, 5], |x| x * x);
    assert_eq!(result, vec![1, 4, 9, 16, 25]);
}

#[test]
fn test_empty() {
    let result = parallel_map(vec![], |x| x);
    assert!(result.is_empty());
}
