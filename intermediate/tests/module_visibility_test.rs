use intermediate::easy::module_visibility::Account;

#[test]
fn test_new_account() {
    let acc = Account::new(100.0);
    assert!((acc.balance() - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_deposit() {
    let mut acc = Account::new(50.0);
    acc.deposit(25.0);
    assert!((acc.balance() - 75.0).abs() < f64::EPSILON);
}

#[test]
fn test_withdraw_ok() {
    let mut acc = Account::new(100.0);
    assert!(acc.withdraw(50.0).is_ok());
    assert!((acc.balance() - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_withdraw_insufficient() {
    let mut acc = Account::new(10.0);
    assert!(acc.withdraw(20.0).is_err());
}
