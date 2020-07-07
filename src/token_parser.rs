// 원하는 거
// Rule: 각 규칙을 고립시킨 것
// Ruler: 규칙을 순회하여 텍스트 라인의 타입을 밝히는 함수
// 먼저 block 규칙을 다 돌고 아무데도 안 걸리면 inline 규칙 돌림
// inline 규칙: block 전체를 paragraph 토큰화한 뒤 전체 텍스트를 순회하며 내부에 inline 토큰 추가
// Parser: 전체 라인을 줄바꿈 단위로 순회하여 토큰을 받아 트리에 저장
// Token: 일정한 단위로 문장을 쪼개 성격을 밝혀 놓은 것
// Tokenizer: 토큰 생성 함수. Ruler로부터 텍스트와 타입을 넘겨받으면 토큰으로 생성
// Tree: 최종적으로 만들어지는 구문 트리
// 제작 순서
// 1. Token과 Token 생성 함수
// 2. 스트링 슬라이스를 판별하여 타입을 리턴하는 Rule
// 3. 전체 Rule을 순회하여 각 라인의 텍스트와 타입을 Tokenizer에게 넘기는 Ruler
// 4. 텍스트와 타입을 받아 토큰을 생성하는 Tokenizer
// 5. 전체 프로세스를 진행시켜 토큰을 트리에 저장하는 파서

mod parser {
  use regex::Regex;

  pub struct Parser {
    pub cursor: usize,
    pub docs: String,
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
    
    pub fn hunting_line(&mut self) -> Vec<String> {
      let mut vec: Vec<String> = Vec::new();

      while self.cursor < self.docs.len() {
        let mut result = String::new();
        let mut iter = self.docs[self.cursor..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
  
        if cur_char != '\n' {
          result.push(cur_char);
        } else {
          vec.push(result);
        }
  
        self.cursor += next_pos;
      }

      vec
    }
  }

  pub fn normalize<'a>(docs: &'a str) -> String {
    let has_carriage = Regex::new(r"\r\n?").unwrap();
    let not_end_newline = Regex::new(r"\n$").unwrap();
    let too_many_newline = Regex::new(r"\n+$").unwrap();
    
    let mut result = has_carriage.replace_all(docs, "\n").into_owned();

    if !not_end_newline.is_match(&result) {
      result.push_str("\n");
    }
    if too_many_newline.is_match(&result) {
      let wow = too_many_newline.replace_all(&result, "\n");
      result = wow.into_owned();
    }
  
    result
  }
}

mod rule {
  pub mod block {
    use regex::Regex;
    use std::fmt::Write;

    pub struct Rule {
      pub rule: regex::Regex,
      pub element: String,
    }

    pub trait Checkerable {
      fn is_a(&self, line: &str) -> Option<String>;
    }

    impl Checkerable for Rule {
      fn is_a(&self, line: &str) -> Option<String> {
        if self.rule.is_match(line) {
          Some(format!("{}", self.element))
        } else {
          None
        }
      }
    }



    pub fn is_heading(line: &str) -> Option<String> {
      let re_heading = Regex::new(r"^#{1,6}\s").unwrap();
      if re_heading.is_match(line) {
        let heading_len = re_heading
          .captures(line)
          .unwrap()
          .get(0)
          .unwrap()
          .as_str()
          .chars()
          .count()
          - 1;

        let mut heading = String::new();
        write!(&mut heading, "heading {}", heading_len);

        Some(heading)
      } else {
        None
      }
    }

    pub fn is_hr(line: &str) -> Option<String> {
      let re_hr = Regex::new(r"^-{3}$").unwrap();
      if re_hr.is_match(line) {
        Some("hr".to_string())
      } else {
        None
      }
    }

    pub fn is_ul(line: &str) -> Option<String> {
      let re_ul = Regex::new(r"^[*-]\s").unwrap();
      if re_ul.is_match(line) {
        Some("ul".to_string())
      } else {
        None
      }
    }

    pub fn is_ol(line: &str) -> Option<String> {
      let re_ol = Regex::new(r"^\d+\.\s").unwrap();
      if re_ol.is_match(line) {
        Some("ol".to_string())
      } else {
        None
      }
    }

  }

  pub mod inline {}
}

#[cfg(test)]
mod tests {
  use regex::Regex;
  use super::*;
  use rule::block::Checkerable;

  #[test]
  fn normalize() {
    let docs = "안녕하세요\r\n오늘은 참 맑네요\n이럴수가\r";
    assert_eq!(parser::normalize(docs), "안녕하세요\n오늘은 참 맑네요\n이럴수가\n");

    let docs = "줄바꿈없음";
    assert_eq!(parser::normalize(docs), "줄바꿈없음\n");

    let docs = "줄바꿈 두개\n\n";
    assert_eq!(parser::normalize(docs), "줄바꿈 두개\n");

    let docs = "줄바꿈 네개\n\n\n\n";
    assert_eq!(parser::normalize(docs), "줄바꿈 네개\n");
  }

  #[test]
  fn hunting_line() {
    let docs = "안녕하세요\r\n오늘은 참 맑네요\n이럴수가\r동해물과baekdusan\n마르고 닳도록";
    let normalized = parser::normalize(docs);
    let mut parser = parser::Parser { cursor: 0, docs: normalized };
    let list = parser.hunting_line();

    assert_eq!(list.len(), 5);
  }

  #[test]
  fn is_heading() {
    let wow = rule::block::Rule { rule: Regex::new(r"^#{1,6}\s").unwrap(), element: "heading".to_string() };
    assert_eq!(wow.is_a("# wow").unwrap(), "heading".to_string());

    let result = rule::block::is_heading("## wow!");
    assert_eq!(result.unwrap(), "heading 2".to_string());

    let result = rule::block::is_heading("### wow!");
    assert_eq!(result.unwrap(), "heading 3".to_string());

    let result = rule::block::is_heading("#### wow!");
    assert_eq!(result.unwrap(), "heading 4".to_string());

    let result = rule::block::is_heading("##### wow!");
    assert_eq!(result.unwrap(), "heading 5".to_string());

    let result = rule::block::is_heading("###### wow!");
    assert_eq!(result.unwrap(), "heading 6".to_string());

    let result = rule::block::is_heading("####### wow!");
    assert_eq!(result, None);

    let result = rule::block::is_heading("#wow!");
    assert_eq!(result, None);
  }

  #[test]
  fn is_hr() {
    let result = rule::block::is_hr("---");
    assert_eq!(result.unwrap(), "hr".to_string());

    let result = rule::block::is_hr("----");
    assert_eq!(result, None);
  }

  #[test]
  fn is_ul() {
    let result = rule::block::is_ul("- first");
    assert_eq!(result.unwrap(), "ul".to_string());

    let result = rule::block::is_ul("* first");
    assert_eq!(result.unwrap(), "ul".to_string());

    let result = rule::block::is_ul("-- first");
    assert_eq!(result, None);

    let result = rule::block::is_ul("** first");
    assert_eq!(result, None);
  }

  #[test]
  fn is_ol() {
    let result = rule::block::is_ol("1. hello");
    assert_eq!(result.unwrap(), "ol".to_string());

    let result = rule::block::is_ol("10. hello");
    assert_eq!(result.unwrap(), "ol".to_string());

    let result = rule::block::is_ol("1hello");
    assert_eq!(result, None);
  }
}
