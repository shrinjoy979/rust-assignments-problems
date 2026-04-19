use intermediate::medium::pack_u16::{pack_u16, unpack_u32};

#[test]
fn test_pack() {
    assert_eq!(pack_u16(0x00FF, 0xFF00), 0x00FFFF00);
}

#[test]
fn test_unpack() {
    assert_eq!(unpack_u32(0x00FFFF00), (0x00FF, 0xFF00));
}

#[test]
fn test_roundtrip() {
    let packed = pack_u16(1234, 5678);
    assert_eq!(unpack_u32(packed), (1234, 5678));
}

#[test]
fn test_zeros() {
    assert_eq!(pack_u16(0, 0), 0);
    assert_eq!(unpack_u32(0), (0, 0));
}

#[test]
fn test_max() {
    assert_eq!(pack_u16(u16::MAX, u16::MAX), u32::MAX);
}
