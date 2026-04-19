use fundamentals::hard::fibonacci_iter::Fibonacci;

#[test]
fn test_first_ten() {
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
}

#[test]
fn test_first() {
    let mut fib = Fibonacci::new();
    assert_eq!(fib.next(), Some(0));
}

#[test]
fn test_second() {
    let mut fib = Fibonacci::new();
    fib.next();
    assert_eq!(fib.next(), Some(1));
}

#[test]
fn test_sum_first_20() {
    let sum: u64 = Fibonacci::new().take(20).sum();
    assert_eq!(sum, 10945);
}
