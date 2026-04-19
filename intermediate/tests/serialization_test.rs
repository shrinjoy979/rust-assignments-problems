use intermediate::medium::serialization::Record;

#[test]
fn test_roundtrip() {
    let r = Record { id: 42, value: 1000 };
    let bytes = r.to_bytes();
    assert_eq!(Record::from_bytes(&bytes), Ok(Record { id: 42, value: 1000 }));
}

#[test]
fn test_byte_length() {
    let r = Record { id: 1, value: 2 };
    assert_eq!(r.to_bytes().len(), 6);
}

#[test]
fn test_from_bytes_too_short() {
    assert!(Record::from_bytes(&[0, 1, 2]).is_err());
}

#[test]
fn test_zero_values() {
    let r = Record { id: 0, value: 0 };
    let bytes = r.to_bytes();
    assert_eq!(bytes, vec![0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_max_values() {
    let r = Record { id: u32::MAX, value: u16::MAX };
    let bytes = r.to_bytes();
    assert_eq!(Record::from_bytes(&bytes), Ok(r));
}
