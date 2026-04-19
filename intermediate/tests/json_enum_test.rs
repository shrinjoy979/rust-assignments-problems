use intermediate::hard::json_enum::Value;
use std::collections::HashMap;

#[test]
fn test_null() {
    assert_eq!(Value::Null.to_json_string(), "null");
}

#[test]
fn test_bool() {
    assert_eq!(Value::Bool(true).to_json_string(), "true");
}

#[test]
fn test_number() {
    assert_eq!(Value::Number(42.5).to_json_string(), "42.5");
}

#[test]
fn test_array() {
    let arr = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    assert_eq!(arr.to_json_string(), "[1, 2]");
}

#[test]
fn test_complex_object() {
    let mut map = HashMap::new();
    map.insert("active".to_string(), Value::Bool(true));
    let obj = Value::Object(map);
    assert_eq!(obj.to_json_string(), "{\"active\": true}");
}
