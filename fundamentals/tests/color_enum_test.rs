use fundamentals::easy::color_enum::{color_to_rgb, Color};

#[test]
fn test_red() {
    assert_eq!(color_to_rgb(Color::Red), (255, 0, 0));
}

#[test]
fn test_green() {
    assert_eq!(color_to_rgb(Color::Green), (0, 255, 0));
}

#[test]
fn test_blue() {
    assert_eq!(color_to_rgb(Color::Blue), (0, 0, 255));
}

#[test]
fn test_custom() {
    assert_eq!(color_to_rgb(Color::Custom(128, 64, 32)), (128, 64, 32));
}
