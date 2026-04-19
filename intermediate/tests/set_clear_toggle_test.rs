use intermediate::medium::set_clear_toggle::{clear_bit, set_bit, toggle_bit};

#[test]
fn test_set_bit() {
    assert_eq!(set_bit(0b0000, 2), 0b0100);
}

#[test]
fn test_set_already_set() {
    assert_eq!(set_bit(0b0100, 2), 0b0100);
}

#[test]
fn test_clear_bit() {
    assert_eq!(clear_bit(0b1111, 1), 0b1101);
}

#[test]
fn test_toggle_on() {
    assert_eq!(toggle_bit(0b0000, 3), 0b1000);
}

#[test]
fn test_toggle_off() {
    assert_eq!(toggle_bit(0b1000, 3), 0b0000);
}
