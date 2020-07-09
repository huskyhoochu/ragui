use regex::Regex;
use super::ruler;
use super::parser;

/// 얘네가 해야 할 일
/// ruler를 받아서 각종 규칙을 생성한다
/// 벡터를 받아서 돌린다
/// 돌리면서 ruler들을 통과시킨다
/// 일치하는 rule을 발견하면 토큰으로 만들어 트리에 저장한다
/// 트리는 어디에 있나??
/// 
/// 

enum Layout {
  Block,
  Inline,
}

struct Token {
  layout: Layout, // 레이아웃: 블럭, 인라인
  value: String, // 값
  line: usize, // 몇번째 라인
  position: usize, // 몇번째 포지션 <- 정렬 및 검색, 치환에 사용
}

struct Tokenizer {}

impl Tokenizer {
  // 생성자
  fn new() -> Tokenizer {
    Tokenizer {}
  }
  // 순회 및 매칭
  fn traverse(&self, list: Vec<String>) {
    let heading = ruler::Rule::new(Regex::new(r"^#{1,6}\s").unwrap(), "heading");
    let hr = ruler::Rule::new(Regex::new(r"^-{3}").unwrap(), "hr");
    let blockquote = ruler::Rule::new(Regex::new(r"^>\s").unwrap(), "blockquote");
    let ol = ruler::Rule::new(Regex::new(r"^\d\.\s").unwrap(), "ol");
    let ul = ruler::Rule::new(Regex::new(r"^[*-]\s").unwrap(), "ul");
    let codeblock = ruler::Rule::new(Regex::new(r"^`{3}").unwrap(), "codeblock");

    let rules = vec![heading, hr, blockquote, ol, ul, codeblock];

    for line in list {
      for rule in &rules {
        if rule.is(&line) {
          println!("{}, {}", rule.format(), line);
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_traverse() {
    let docs = "## 우리집\n강아지는 복슬강아지\n---\n> 동요임\n";
    let parsed = parser::parse(docs);
    let tokenizer = Tokenizer::new();
    tokenizer.traverse(parsed);
  }

}