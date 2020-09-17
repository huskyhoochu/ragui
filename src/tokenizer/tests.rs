use super::*;
use regex::Regex;
use crate::normalizer::Normalizer;
use crate::rules::{Rule};

#[test]
fn test_parse_block() {
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
  let tokens = tokenizer.tokenize(normalized);
  assert_eq!(tokens.len(), 4);
  assert_eq!(tokens[0].get_html(), "<p>안녕하세요</p>");
  assert_eq!(tokens[1].get_html(), "<p><strong>오늘은 참 맑네요</strong></p>");
}
