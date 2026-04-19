use intermediate::easy::extract_nibbles::extract_nibbles;

#[test]
fn test_ab() {
    assert_eq!(extract_nibbles(0xAB), (0x0A, 0x0B));
}

#[test]
fn test_zero() {
    assert_eq!(extract_nibbles(0x00), (0x00, 0x00));
}

#[test]
fn test_ff() {
    assert_eq!(extract_nibbles(0xFF), (0x0F, 0x0F));
}

#[test]
fn test_f0() {
    assert_eq!(extract_nibbles(0xF0), (0x0F, 0x00));
}
