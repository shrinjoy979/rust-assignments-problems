use fundamentals::medium::enum_calculator::{calculate, Operation};

#[test]
fn test_add() {
    assert_eq!(calculate(Operation::Add(2.0, 3.0)), Ok(5.0));
}

#[test]
fn test_sub() {
    assert_eq!(calculate(Operation::Sub(10.0, 4.0)), Ok(6.0));
}

#[test]
fn test_mul() {
    assert_eq!(calculate(Operation::Mul(3.0, 7.0)), Ok(21.0));
}

#[test]
fn test_div_ok() {
    assert_eq!(calculate(Operation::Div(10.0, 2.0)), Ok(5.0));
}

#[test]
fn test_div_by_zero() {
    assert!(calculate(Operation::Div(5.0, 0.0)).is_err());
}
