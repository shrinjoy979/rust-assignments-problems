use intermediate::easy::byte_checksum::compute_checksum;

#[test]
fn test_basic() {
    assert_eq!(compute_checksum(&[1, 2, 3]), 0); // 1 ^ 2 ^ 3 = 0
}

#[test]
fn test_empty() {
    assert_eq!(compute_checksum(&[]), 0);
}

#[test]
fn test_single() {
    assert_eq!(compute_checksum(&[42]), 42);
}

#[test]
fn test_all_same() {
    assert_eq!(compute_checksum(&[255, 255]), 0);
}

#[test]
fn test_complex() {
    assert_eq!(compute_checksum(&[0xaa, 0xbb, 0xcc]), 0xdd); // 0xAA^0xBB^0xCC = 0xDD
}
