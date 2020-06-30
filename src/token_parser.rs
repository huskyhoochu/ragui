enum MDTypes {
  Heading(u8),
  Paragraph,
  Bold,
  Italics,
  Text,
}

enum MDLayout {
  Block,
  Inline,
}

enum Token {
  Atom,
  Group,
}

struct Atom {
  types: MDTypes,
  layout: MDLayout,
  value: String
}

struct Group {
  types: MDTypes,
  layout: MDLayout,
  values: Vec<Token>
}

struct Parser {
  cursor: usize,
}

/// TODO:
/// docs 순회 [0]
/// 줄바꿈 단위로 문장 저장 [0]
/// 현재 글자 및 다음 글자 추출
/// 현재 글자 및 다음 글자를 받아 어떤 타입인지 파악
/// 블록 타입 ? 줄바꿈이 나올 때까지 글자를 모은다
/// 인라인 타입 ? 다음 타입이 나올 때까지 글자를 모은다
/// 토큰으로 저장
/// 토큰 트리에 저장


impl Token {
  
}

impl Parser {



  // fn determine_block_type(&mut self) -> Vec<Token> {
  //   let line_list = self.parse_line();
  //   let mut result = vec![];

  //   for line in line_list {
  //     // 여기서 블록 타입을 결정한다
  //     // 타입이 안 나오면 paragraph
  //     match {
  //       '#' =>, // h 타입의 atom 생성
  //       _ => ,// p 타입의 atom 생성
  //     }
  //   }

  //   result
  // }

  // fn parse_heading(&mut self, line: String) -> Token { 
  //   let headings = self.collect_while(line, |c| c == '#');
  //   self.drop_whitespace(line);

  // }


  fn parse_line(&mut self, docs: String) -> Vec<String> {
    let mut result = vec![];
    while !self.end_of(&docs) {
      let raw_line = self.collect_line(&docs);
      result.push(raw_line);
      self.cursor += 1;
    }
    self.reset_cursor();
    result
  }

  /// 커서를 리셋한다
  fn reset_cursor(&mut self) {
    self.cursor = 0;
  }
  
  /// 줄바꿈이 일어나기 전까지 모든 글자를 취합한다
  fn collect_line(&mut self, docs: &String) -> String {
    self.collect_while(&docs, |c| !is_newline(c))
  }

  /// 현재 위치가 문서의 마지막인지 체크한다
  fn end_of(&self, docs: &String) -> bool {
    self.cursor >= docs.len()
  }

  /// 주어진 문장의 현재 위치 글자를 출력한다
  fn current_char(&self, docs: &String) -> char {
    docs[self.cursor..].chars().next().unwrap()
  }

  /// 주어진 문장의 현재 위치 글자를 출력하고
  /// 현재 위치를 한 칸 이동한다
  fn collect_char(&mut self, docs: &String) -> char {
    let mut iter = docs[self.cursor..].char_indices();
    let (_, cur_char) = iter.next().unwrap();
    let (next_cursor, _) = iter.next().unwrap_or((1, ' '));
    self.cursor += next_cursor;
    cur_char
  }

  /// 특정 조건을 만족하는 모든 글자를 모아 출력한다
  fn collect_while<F>(&mut self, docs: &String, cond: F) -> String
  where
    F: Fn(char) -> bool,
  {
    let mut result = String::new();
    while cond(self.current_char(&docs)) {
      result.push(self.collect_char(&docs))
    }
    result
  }

  /// 띄어쓰기를 출력하고 위치를 한 칸 이동한다
  fn drop_whitespace(&mut self, docs: &String) {
    self.collect_while(&docs, char::is_whitespace);
  }
}

/// 줄바꿈인지 판단한다
fn is_newline(c: char) -> bool {
  c == '\n'
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_line() {
    let mut parser = Parser{ cursor: 0 };

    let docs = "동해물과\n백두산이\n마르고 닳도록\n하느님이\n".to_string();
    assert_eq!(parser.parse_line(docs).len(), 4);
  }
}