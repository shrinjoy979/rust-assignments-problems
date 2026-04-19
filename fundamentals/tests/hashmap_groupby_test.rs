use fundamentals::medium::hashmap_groupby::group_by_first_letter;

#[test]
fn test_basic() {
    let words = vec!["apple", "avocado", "banana", "blueberry"]
        .into_iter().map(String::from).collect();
    let result = group_by_first_letter(words);
    assert_eq!(result.get(&'a').unwrap().len(), 2);
    assert_eq!(result.get(&'b').unwrap().len(), 2);
}

#[test]
fn test_empty() {
    let result = group_by_first_letter(vec![]);
    assert!(result.is_empty());
}

#[test]
fn test_single_word() {
    let words = vec![String::from("rust")];
    let result = group_by_first_letter(words);
    assert_eq!(result.get(&'r').unwrap(), &vec!["rust".to_string()]);
}

#[test]
fn test_ignores_empty_strings() {
    let words = vec![String::from(""), String::from("hello")];
    let result = group_by_first_letter(words);
    assert_eq!(result.len(), 1);
}
