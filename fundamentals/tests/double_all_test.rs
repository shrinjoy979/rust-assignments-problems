use fundamentals::easy::double_all::double_all;

#[test]
fn test_basic() {
    let mut v = vec![1, 2, 3];
    double_all(&mut v);
    assert_eq!(v, vec![2, 4, 6]);
}

#[test]
fn test_empty() {
    let mut v: Vec<i32> = vec![];
    double_all(&mut v);
    assert_eq!(v, vec![]);
}

#[test]
fn test_negative() {
    let mut v = vec![-1, -2, -3];
    double_all(&mut v);
    assert_eq!(v, vec![-2, -4, -6]);
}

#[test]
fn test_zeros() {
    let mut v = vec![0, 0, 0];
    double_all(&mut v);
    assert_eq!(v, vec![0, 0, 0]);
}
