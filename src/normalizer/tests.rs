use super::*;

#[test]
fn test_normalize() {
    let docs = String::from("안녕하세요\r\n오늘은 참 맑네요\n이럴수가\r");
    let normalizer = Normalizer::new(docs);
    assert_eq!(normalizer.get(), ["안녕하세요", "오늘은 참 맑네요", "이럴수가", ""]);
}
