use fundamentals::easy::coin_value::{coin_value, Coin};

#[test]
fn test_penny() {
    assert_eq!(coin_value(Coin::Penny), 1);
}

#[test]
fn test_nickel() {
    assert_eq!(coin_value(Coin::Nickel), 5);
}

#[test]
fn test_dime() {
    assert_eq!(coin_value(Coin::Dime), 10);
}

#[test]
fn test_quarter() {
    assert_eq!(coin_value(Coin::Quarter), 25);
}
