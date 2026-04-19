use intermediate::hard::packed_flags::PackedFlags;

#[test]
fn test_initially_all_false() {
    let flags = PackedFlags::new();
    for i in 0..8 {
        assert!(!flags.get(i));
    }
}

#[test]
fn test_set_and_get() {
    let mut flags = PackedFlags::new();
    flags.set(3, true);
    assert!(flags.get(3));
    assert!(!flags.get(2));
}

#[test]
fn test_clear_flag() {
    let mut flags = PackedFlags::new();
    flags.set(5, true);
    flags.set(5, false);
    assert!(!flags.get(5));
}

#[test]
fn test_as_byte() {
    let mut flags = PackedFlags::new();
    flags.set(0, true);
    flags.set(7, true);
    assert_eq!(flags.as_byte(), 0b10000001);
}

#[test]
fn test_all_set() {
    let mut flags = PackedFlags::new();
    for i in 0..8 {
        flags.set(i, true);
    }
    assert_eq!(flags.as_byte(), 0xFF);
}
