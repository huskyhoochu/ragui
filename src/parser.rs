use regex::Regex;

struct Parser {
  docs: String,
}

pub fn parse(input: &str) -> String {
  let docs = normalize(input);
  let mut parser = Parser::new(docs);
  let result = parser.hunting_line();
  let joined = result.join("\n");
  joined
}

impl Parser {
  /// 순회를 위해 필요한 것
  /// 커서를 이용해 스트링을 슬라이스로 만들어
  /// 문서가 끝날 때까지 한 칸씩 순회한다
  /// 순회하기 전에 줄바꿈을 정규화하고
  /// 순회가 시작되면 먼저 줄바꿈이 나타날 때까지 글자를 모아다가
  /// 줄바꿈이 나타나면 그때까지 모은 글자를 하나의 원소로
  /// 동적 배열에 넣는다
  /// 이 작업이 끝나면
  /// 배열을 순회 돌리면서
  /// 각 라인을 정규표현식에 대입해 블록 타입을 산출한다
  ///
  /// 
  /// 
  fn new(input: String) -> Parser {
    Parser { docs: input }
  }

  fn hunting_line(&mut self) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    let mut one_line = String::new();
      for (_, char) in self.docs.chars().enumerate() {
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

fn normalize<'a>(docs: &'a str) -> String {
  let has_carriage = Regex::new(r"\r\n?").unwrap();
  let not_end_newline = Regex::new(r"\n$").unwrap();
  let too_many_newline = Regex::new(r"\n+$").unwrap();

  let mut result = has_carriage.replace_all(docs, "\n").into_owned();

  if !not_end_newline.is_match(&result) {
    result.push_str("\n");
  }  if too_many_newline.is_match(&result) {
    let wow = too_many_newline.replace_all(&result, "\n");
    result = wow.into_owned();
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use normalize;

  #[test]
  fn test_normalize() {
    let docs = "안녕하세요\r\n오늘은 참 맑네요\n이럴수가\r";
    assert_eq!(normalize(docs), "안녕하세요\n오늘은 참 맑네요\n이럴수가\n");

    let docs = "줄바꿈없음";
    assert_eq!(normalize(docs), "줄바꿈없음\n");

    let docs = "줄바꿈 두개\n\n";
    assert_eq!(normalize(docs), "줄바꿈 두개\n");

    let docs = "줄바꿈 네개\n\n\n\n";
    assert_eq!(normalize(docs), "줄바꿈 네개\n");
  }

  #[test]
  fn hunting_line() {
    let docs = "안녕하세요\r\n오늘은 참 맑네요\n이럴수가\r동해물과baekdusan\n마르고 닳도록";
    let normalized = normalize(docs);
    let mut parser = Parser::new(normalized);
    let list = parser.hunting_line();
    let joined = list.join("\n");

    println!("{}", joined);
    assert_eq!(list.len(), 5);
  }
}
