use regex::Regex;

pub fn parse<'a>(input: &'a str) -> Vec<String> {
  let mut parser = Parser::new(input);
  parser.hunting_line()
}

struct Parser<'a> {
  docs: &'a str,
}

impl<'a> Parser<'a> {
  fn new(input: &'a str) -> Parser {
    Parser { docs: input }
  }

  fn normalize(&mut self) -> String {
    let has_carriage = Regex::new(r"\r\n?").unwrap();
    let not_end_newline = Regex::new(r"\n$").unwrap();
    let too_many_newline = Regex::new(r"\n+$").unwrap();
  
    let mut result = has_carriage.replace_all(self.docs, "\n").into_owned();
  
    if !not_end_newline.is_match(&result) {
      result.push_str("\n");
    } 
    
    if too_many_newline.is_match(&result) {
      result = too_many_newline.replace_all(&result, "\n").into_owned();    
    }
  
    result
  }

  fn hunting_line(&mut self) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let normalized = self.normalize();

    let mut one_line = String::new();
      for (_, char) in normalized.chars().enumerate() {
        if char != '\n' {
          one_line.push(char);
        } else {
          vec.push(one_line);
          one_line = "".to_string();
        }
      }

    vec
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_normalize() {
    let mut docs = Parser::new("안녕하세요\r\n오늘은 참 맑네요\n이럴수가\r");
    assert_eq!(docs.normalize(), "안녕하세요\n오늘은 참 맑네요\n이럴수가\n".to_string());

    let mut docs = Parser::new("줄바꿈없음");
    assert_eq!(docs.normalize(), "줄바꿈없음\n".to_string());

    let mut docs = Parser::new("줄바꿈 두개\n\n");
    assert_eq!(docs.normalize(), "줄바꿈 두개\n".to_string());

    let mut docs = Parser::new("줄바꿈 네개\n\n\n\n");
    assert_eq!(docs.normalize(), "줄바꿈 네개\n".to_string());
  }

  #[test]
  fn hunting_line() {
    let docs = "안녕하세요\r\n오늘은 참 맑네요\n이럴수가\r동해물과baekdusan\n마르고 닳도록";
    let mut parser = Parser::new(docs);
    let list = parser.hunting_line();
    let joined = list.join("\n");

    println!("{}", joined);
    assert_eq!(list.len(), 5);
  }
}
