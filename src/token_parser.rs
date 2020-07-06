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

mod rule {
  pub mod block {
    use regex::Regex;
    use std::fmt::Write;

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
  use super::*;

  #[test]
  fn is_heading() {
    let result = rule::block::is_heading("# wow!");
    assert_eq!(result.unwrap(), "heading 1".to_string());

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
