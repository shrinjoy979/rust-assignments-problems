#[test]
fn test_basic() {
    use intermediate::measure_time;
    let result: i32 = measure_time!(2 + 2);
    assert_eq!(result, 4);
}

#[test]
fn test_loop() {
    use intermediate::measure_time;
    let result: i32 = measure_time!({
        let mut sum = 0;
        for i in 0..100 { sum += i; }
        sum
    });
    assert_eq!(result, 4950);
}

#[test]
fn test_string() {
    use intermediate::measure_time;
    let s: String = measure_time!("hello".to_string());
    assert_eq!(s, "hello");
}
