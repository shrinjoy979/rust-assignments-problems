/*
  Problem 65: Data Processing — JSON-like Enum

  Define an enum Value representing a simplified JSON value:
  Null, Bool(bool), Number(f64), String(String), Array(Vec<Value>),
  and Object(HashMap<String, Value>). Implement a method fn to_json_string(&self) -> String
  that produces a JSON-formatted string.

  Run the tests for this problem with:
    cargo test --test json_enum_test
*/

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn to_json_string(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_json_string()).collect();
                format!("[{}]", items.join(", "))  // ", " not ","
            }
            Value::Object(map) => {
                let mut pairs: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_json_string()))  // ": " not ":"
                    .collect();
                pairs.sort();
                format!("{{{}}}", pairs.join(", "))  // ", " not ","
            }
        }
    }
}
