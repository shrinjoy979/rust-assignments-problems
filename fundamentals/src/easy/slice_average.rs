/*
  Problem 11: Slice Average

  Write a function that takes a slice of f64 values and returns the arithmetic mean.
  If the slice is empty, return None.

  Run the tests for this problem with:
    cargo test --test slice_average_test
*/

pub fn average(values: &[f64]) -> Option<f64> {
    let mut total: f64 = 0.0;

    if values.len() == 0 {
      return None;
    }
    let total_values = values.len();

    for value in values {
      total = total + *value;
    }

    let result = total / total_values as f64;

    return Some(result);
}
