use intermediate::medium::from_vec_payload::Payload;

#[test]
fn test_conversion() {
    let input = vec![0x1234, 0x5678];
    let payload = Payload::from(input);
    assert_eq!(payload.data, vec![0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn test_empty() {
    let payload = Payload::from(vec![]);
    assert!(payload.data.is_empty());
}

#[test]
fn test_single() {
    let payload = Payload::from(vec![0xFFFF]);
    assert_eq!(payload.data, vec![0xFF, 0xFF]);
}
