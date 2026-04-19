use intermediate::medium::custom_error_enum::{validate_packet, DataError};

#[test]
fn test_valid_packet() {
    let mut packet = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    packet[9] = packet[..9].iter().fold(0u8, |acc, &b| acc ^ b);
    assert!(validate_packet(&packet).is_ok());
}

#[test]
fn test_invalid_length() {
    let result = validate_packet(&[1, 2, 3]);
    assert!(matches!(result, Err(DataError::InvalidLength { .. })));
}

#[test]
fn test_checksum_mismatch() {
    let packet = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0]; // bad checksum
    assert!(matches!(validate_packet(&packet), Err(DataError::ChecksumMismatch)));
}

#[test]
fn test_display() {
    let err = DataError::InvalidLength { expected: 10, actual: 5 };
    let msg = format!("{}", err);
    assert!(msg.contains("10") && msg.contains("5"));
}
