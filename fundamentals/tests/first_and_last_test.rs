use fundamentals::easy::first_and_last::first_and_last;

#[test]
fn test_normal() {
    assert_eq!(first_and_last(&[1, 2, 3, 4, 5]), Some((1, 5)));
}

#[test]
fn test_single() {
    assert_eq!(first_and_last(&[42]), Some((42, 42)));
}

#[test]
fn test_empty() {
    assert_eq!(first_and_last(&[]), None);
}

#[test]
fn test_two_elements() {
    assert_eq!(first_and_last(&[10, 20]), Some((10, 20)));
}
