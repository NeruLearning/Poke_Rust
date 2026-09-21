
#[test]
fn test_clean_input() {
    let input = "Hello world!";
    let expected = vec!["hello".to_string(), "world!".to_string()];
    assert_eq!(super::clean_input(input), expected);
}
