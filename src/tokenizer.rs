use regex::Regex;
use std::clone::Clone;
use crate::rules::*;

struct Token<T> where T: Clone + Parser {
  rule: T,
  value: String,
  html: String,
  pub children: Vec<Token<T>>,
}

impl<T> Token<T> where T: Clone + Parser {
  fn new(rule: T, value: String) -> Token<T> {
    let mut token = Token { rule, value, html: String::new(), children: Vec::new() };
    token.set_html(token.rule.parse(token.value.as_str(), token.rule.get_parse_expr()));
    token
  }

  fn get_rule(&self) -> &T {
    &self.rule
  }

  fn get_value(&self) -> &String {
    &self.value
  }

  fn get_html(&self) -> &String {
    &self.html
  }

  fn set_html(&mut self, html: String) {
    self.html = html;
  }

  fn size(&self) -> usize {
    self.children.len()
  }

  fn set_child(&mut self, token: Token<T>, index: usize) {
    self.children.insert(index, token);
  }

  fn get_child(&self, index: usize) -> &Token<T> {
    &self.children[index]
  }
}

struct Tokenizer<T> where T: Clone + Parser {
  rules_block: Vec<T>,
  rules_inline: Vec<T>,
}


impl<T> Tokenizer<T> where T: Clone + Parser {
  fn new(rules_block: Vec<T>, rules_inline: Vec<T>) -> Tokenizer<T> {
    Tokenizer { rules_block, rules_inline }
  }

  fn parse_block(&self, line: &str) -> Token<T> {
    let paragraph = T::new(MDTypes::Paragraph, Regex::new(r"(?P<first>.*)").unwrap(), String::from("<p>$first</p>"));
    let mut result = Token::new(paragraph, String::from(line));
    for rule in self.rules_block.iter() {
      if rule.get_rule().is_match(line) {
        result = Token::new(rule.clone(),String::from(line));
      }
    }

    result

  }

  fn tokenize(&mut self, lines: Vec<&str>) -> Vec<Token<T>> {
    let lines_iter = lines.iter();
    let mut result = Vec::new();

    for line in lines_iter {
      let token = self.parse_block(line);
      result.push(token);
    }

    result
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use regex::Regex;
  use crate::normalizer::Normalizer;
  use crate::rules::{Rule};

  #[test]
  fn test_parse_block() {
    let docs = String::from("안녕하세요\r\n오늘은 참 맑네요\n이럴수가\r");
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
  }
}
