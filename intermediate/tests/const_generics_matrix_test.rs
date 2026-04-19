use intermediate::hard::const_generics_matrix::Matrix;

#[test]
fn test_2x2_matrix() {
    let m = Matrix::new([[1, 2], [3, 4]]);
    assert_eq!(m.get(0, 0), Some(&1));
    assert_eq!(m.get(1, 1), Some(&4));
}

#[test]
fn test_out_of_bounds() {
    let m = Matrix::new([[1, 2], [3, 4]]);
    assert_eq!(m.get(2, 0), None);
    assert_eq!(m.get(0, 2), None);
}

#[test]
fn test_1x3_matrix() {
    let m = Matrix::new([[10, 20, 30]]);
    assert_eq!(m.get(0, 2), Some(&30));
}

#[test]
fn test_copy_types() {
    let m = Matrix::new([[1.5, 2.5]]);
    assert_eq!(m.get(0, 0), Some(&1.5));
}
