use fundamentals::easy::if_let_config::get_config;

#[test]
fn test_some() {
    assert_eq!(get_config(Some("custom".to_string())), "custom");
}

#[test]
fn test_none() {
    assert_eq!(get_config(None), "default");
}

#[test]
fn test_empty_string() {
    assert_eq!(get_config(Some("".to_string())), "");
}
