use intermediate::medium::workspace_deps::is_crate_active;

#[test]
fn test_active_crates() {
    assert!(is_crate_active("fundamentals"));
    assert!(is_crate_active("intermediate"));
    assert!(is_crate_active("async"));
}

#[test]
fn test_inactive_crates() {
    assert!(!is_crate_active("unknown-crate"));
    assert!(!is_crate_active(""));
}
