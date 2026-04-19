use r#async::easy::move_closure::spawn_move_thread;

#[test]
fn test_string_length() {
    let handle = spawn_move_thread("rust".to_string());
    assert_eq!(handle.join().unwrap(), 4);
}

#[test]
fn test_empty_string() {
    let handle = spawn_move_thread("".to_string());
    assert_eq!(handle.join().unwrap(), 0);
}

#[test]
fn test_unicode() {
    let handle = spawn_move_thread("🦀".to_string());
    assert_eq!(handle.join().unwrap(), 4); // bytes, not chars
}
