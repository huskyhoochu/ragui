use super::*;
use regex::Regex;
use crate::normalizer::Normalizer;
use crate::rules::{Rule, MDTypes};
use crate::tokenizer::Tokenizer;

#[test]
fn test_tree() {
    let docs = String::from("안녕하세요\r\n**오늘은 참 맑네요**\n이럴수가\r");
    let normalizer = Normalizer::new(docs);
    let normalized = normalizer.get();

    let paragraph = Rule::new(MDTypes::Paragraph, Regex::new(r"(?P<first>.*)").unwrap(), String::from("<p>$first</p>"));
    let mut rules_block = Vec::new();
    rules_block.push(paragraph);

    let strong = Rule::new(MDTypes::Strong, Regex::new(r"(\*\*|__)(?P<first>.+)(\*\*|__)").unwrap(), String::from("<strong>$first</strong>"));
    let mut rules_inline = Vec::new();
    rules_inline.push(strong);

    let mut tokenizer = Tokenizer::new(rules_block, rules_inline);
    let mut tokens = tokenizer.tokenize(normalized);

    let paragraph = Rule::new(MDTypes::Paragraph, Regex::new(r"(?P<first>.*)").unwrap(), String::from("<p>$first</p>"));
    let root = Token::new(paragraph, String::new());

    let mut tree = Tree::new(root);

    tree.put(&mut tokens);

    assert_eq!(tree.show(), String::from("<p>안녕하세요</p><p><strong>오늘은 참 맑네요</strong></p><p>이럴수가</p><p></p>"));


}
