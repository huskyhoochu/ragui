#[derive(PartialEq)]
#[derive(Debug)]
enum MDTypes {
  Heading(usize),
  Paragraph,
  Bold,
  Italics,
  Text,
}

#[derive(Debug)]
enum MDLayout {
  Block,
  Inline,
}

#[derive(Debug)]
struct Atom {
  types: MDTypes,
  layout: MDLayout,
  value: String
}

struct Group {
  types: MDTypes,
  layout: MDLayout,
  values: Vec<Atom>
}

struct Parser {
  cursor: usize,
}

/// TODO:
/// docs 순회 [0]
/// 줄바꿈 단위로 문장 저장 [0]
/// 현재 글자 추출 [0]
/// 현재 글자 및 다음 글자를 받아 어떤 타입인지 파악 [0]
/// 블록 타입 ? 줄바꿈이 나올 때까지 글자를 모은다
/// 인라인 타입 ? 다음 타입이 나올 때까지 글자를 모은다 
/// 토큰으로 저장
/// 토큰 트리에 저장


impl Parser {
  fn new() -> Parser {
    Parser { cursor: 0 }
  }

  /// 일차 구문 트리 생성
  fn parse_first_tree(&mut self, docs: String) -> Vec<Atom> {
    let line_list = self.parse_line(docs);
    let mut result = vec![];

    for line in line_list {
      let atoms = match self.current_char(&line) {
        '#' => self.parse_heading(line),
        _ => self.parse_heading(line),
      };

      result.push(atoms);
    }

    result
  }

  /// 제목을 파싱한다
  fn parse_heading(&mut self, line: String) -> Atom { 
    let headings = self.collect_while(&line, |c| c == '#');
    self.drop_whitespace(&line);

    let text = self.collect_line(&line);
    self.reset_cursor();

    Atom { types: MDTypes::Heading(headings.len()), layout: MDLayout::Block, value: text }
  }

  /// 문단을 파싱한다
  fn parse_paragraph(&mut self, line: String) -> Atom {
    let text = self.collect_line(&line);
    self.reset_cursor();

    Atom { types: MDTypes::Paragraph, layout: MDLayout::Block, value: text }
  }


  /// 전체 문서를 줄바꿈 단위로 분할 저장한다
  fn parse_line(&mut self, docs: String) -> Vec<String> {
    let mut result = vec![];
    while !self.end_of(&docs) {
      let mut raw_line = self.collect_line(&docs);
      raw_line.push('\n');
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
    let mut parser = Parser::new();

    let docs = "동해물과\n백두산이\n마르고 닳도록\n하느님이\n".to_string();
    assert_eq!(parser.parse_line(docs).len(), 4);
  }

  #[test]
  fn parse_heading() {
    let mut parser = Parser::new();

    let docs = "# 안녕하세요\n".to_string();
    assert_eq!(parser.parse_heading(docs).types, MDTypes::Heading(1));
  }

  #[test]
  fn parse_first_tree() {
    let mut parser = Parser::new();
    let docs = "# 애국가\n## 작 안익태\n동해물과 백두산이 마르고 닳도록\n하느님이 보우하사 우리나라 만세\n".to_string();

    let first_tree = parser.parse_first_tree(docs);
    assert_eq!(first_tree.len(), 4);

  }
}
