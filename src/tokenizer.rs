use regex::Regex;
use std::clone::Clone;
use crate::rules::*;

#[derive(Clone)]
pub struct Token<T> where T: Clone + Parser {
  rule: T,
  value: String,
  html: String,
  pub children: Vec<Token<T>>,
}

impl<T> Token<T> where T: Clone + Parser {
  pub(crate) fn new(rule: T, value: String) -> Token<T> {
    let mut token = Token { rule, value, html: String::new(), children: Vec::new() };
    token.set_html(token.rule.parse(token.value.as_str(), token.rule.get_parse_expr()));
    token
  }

  pub fn get_html(&self) -> &String {
    &self.html
  }

  fn set_html(&mut self, html: String) {
    self.html = html;
  }

  pub fn set_children(&mut self, tokens: &mut Vec<Token<T>>) {
    self.children = tokens.clone();
  }
}

pub struct Tokenizer<T> where T: Clone + Parser {
  rules_block: Vec<T>,
  rules_inline: Vec<T>,
}


impl<T> Tokenizer<T> where T: Clone + Parser {
  pub(crate) fn new(rules_block: Vec<T>, rules_inline: Vec<T>) -> Tokenizer<T> {
    Tokenizer { rules_block, rules_inline }
  }

  fn parse_block(&self, line: &str) -> Token<T> {
    let paragraph = T::new(MDTypes::Paragraph, Regex::new(r"(?P<first>.*)").unwrap(), String::from("<p>$first</p>"));
    let mut result = Token::new(paragraph, String::from(line));
    for rule in self.rules_block.iter() {
      if rule.get_regex().is_match(line) {
        result = Token::new(rule.clone(),String::from(line));
      }
    }

    result

  }

  fn parse_inline(&self, token: &mut Token<T>) {
    for rule in self.rules_inline.iter() {
      if rule.get_regex().is_match(token.get_html()) {
        let new_html  = rule.parse(token.get_html(), rule.get_parse_expr());
        token.set_html(new_html);
      }
    }
  }

  pub(crate) fn tokenize(&mut self, lines: Vec<&str>) -> Vec<Token<T>> {
    let mut result = Vec::new();

    for line in lines.iter() {
      let token = self.parse_block(line);
      result.push(token);
    }

    for token in result.iter_mut() {
      self.parse_inline(token);
    }

    result
  }
}


#[cfg(test)]
mod tests;
