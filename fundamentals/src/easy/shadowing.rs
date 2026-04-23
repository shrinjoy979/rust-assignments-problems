/*
  Problem 2: Shadowing Transformer

  Write a function that takes a u32, doubles it, then converts the result to a String.
  You should shadow the variable at each step within the function body.

  Run the tests for this problem with:
    cargo test --test shadowing_test
*/

pub fn shadow_transform(x: u32) -> String {
    let double_variable: u32 = x * 2;
    let my_string: String = double_variable.to_string();
    return my_string;
}
