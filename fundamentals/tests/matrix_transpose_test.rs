use fundamentals::hard::matrix_transpose::transpose;

#[test]
fn test_2x3() {
    let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
    assert_eq!(transpose(m), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
}

#[test]
fn test_square() {
    let m = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(transpose(m), vec![vec![1, 3], vec![2, 4]]);
}

#[test]
fn test_single_row() {
    let m = vec![vec![1, 2, 3]];
    assert_eq!(transpose(m), vec![vec![1], vec![2], vec![3]]);
}

#[test]
fn test_empty() {
    let m: Vec<Vec<i32>> = vec![];
    assert_eq!(transpose(m), Vec::<Vec<i32>>::new());
}
