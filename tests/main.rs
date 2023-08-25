use hq::query;

#[test]
fn test_default() {
    let input = query::InputParam::default();
    let result = query::query(&input).unwrap();
    assert_eq!(result, "Canada");
}

