use intermediate::medium::fixed_arrays_rotate::rotate_left;

#[test]
fn test_rotate_1() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 1), [2, 3, 4, 5, 6, 7, 8, 1]);
}

#[test]
fn test_rotate_0() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 0), [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_rotate_8() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 8), [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_rotate_3() {
    assert_eq!(rotate_left([10, 20, 30, 40, 50, 60, 70, 80], 3), [40, 50, 60, 70, 80, 10, 20, 30]);
}

#[test]
fn test_rotate_large() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 10), [3, 4, 5, 6, 7, 8, 1, 2]);
}
