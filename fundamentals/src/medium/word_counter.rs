/*
  Problem 19: Word Counter

  Write a function that takes a &str and returns a HashMap<String, usize>
  where each key is a lowercase word and each value is the number of occurrences.
  Split on whitespace and convert to lowercase.

  Run the tests for this problem with:
    cargo test --test word_counter_test
*/

use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
  let mut map = HashMap::new();

  let words = text.split_whitespace();

  for word in words {
    *map.entry(word.to_lowercase()).or_insert(0) += 1;
  }

  return map;
}
