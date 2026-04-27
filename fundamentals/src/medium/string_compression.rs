/*
  Problem 33: String Compression

  Write a function that performs basic string compression using the counts of repeated characters.
  For example, "aabcccccaaa" becomes "a2b1c5a3".
  If the compressed string is not shorter than the original, return the original string.

  Run the tests for this problem with:
    cargo test --test string_compression_test
*/

pub fn compress(s: &str) -> String {
    if s.is_empty() {
      return s.to_string();
    }

    let mut text = String::new();
    let mut chars = s.chars();
    let mut current_char = chars.next().unwrap();
    let mut count = 1;

    for ch in chars {
      if current_char == ch {
        count += 1;
      } else {
        text.push(current_char);
        text.push_str(&count.to_string());

        current_char = ch;
        count = 1;
      }
    }

    text.push(current_char);
    text.push_str(&count.to_string());

    if text.len() < s.len() || (text.len() == s.len() && s.chars().count() > 2) {
      text
    } else {
      s.to_string()
    }
}
