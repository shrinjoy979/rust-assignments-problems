use intermediate::easy::memory_layout::type_info;

#[test]
fn test_u8() {
    assert_eq!(type_info::<u8>(), (1, 1));
}

#[test]
fn test_u32() {
    assert_eq!(type_info::<u32>(), (4, 4));
}

#[test]
fn test_u64() {
    assert_eq!(type_info::<u64>(), (8, 8));
}

#[test]
fn test_bool() {
    assert_eq!(type_info::<bool>(), (1, 1));
}
