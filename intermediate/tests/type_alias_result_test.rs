use intermediate::easy::type_alias_result::{find_user, parse_age, AppError};

#[test]
fn test_find_user_ok() {
    assert_eq!(find_user(1), Ok("User_1".to_string()));
}

#[test]
fn test_find_user_not_found() {
    assert_eq!(find_user(0), Err(AppError::NotFound("User 0 not found".to_string())));
}

#[test]
fn test_parse_age_ok() {
    assert_eq!(parse_age("25"), Ok(25));
}

#[test]
fn test_parse_age_fail() {
    assert!(matches!(parse_age("abc"), Err(AppError::ParseFailed(_))));
}
