struct Parser {
  pos: usize,
  input: String,
}

pub fn parse(source: String) -> String {
  Parser {
    pos: 0,
    input: source,
  }.parse_lines()
}

impl Parser {
  /// 마크다운을 HTML로 파싱한 결과물을 합친다
  fn parse_lines(&mut self) -> String {
    let mut result = String::new();

    loop {
      self.consume_whitespace();
      if self.end_of_line() {
        break;
      }

      result.push_str(&self.parse_line());
    }

    result
  }

  /// 마크다운을 HTML로 파싱한다
  fn parse_line(&mut self) -> String {
    match self.next_char() {
      '#' => self.parse_title(),
      '-' => {
        if char::is_whitespace(self.input[self.pos + 1..].chars().next().unwrap()) {
          self.parse_list()
        } else {
          self.parse_text()
        }
      },
       _ => self.parse_text(),
    }
  }

  /// 목록 글자를 파싱한다
  fn parse_list(&mut self) -> String {
    self.consume_char();
    self.consume_whitespace();

    let text = self.parse_text();
    create_html_element("li".to_string(), text)
  }

  /// 제목 글자를 파싱한다
  fn parse_title(&mut self) -> String {
    let pound = self.consume_while(|c| c == '#');
    self.consume_whitespace();
    let text = self.parse_text();

    create_html_element(format!("h{}", pound.len()), text)
  }

  /// 줄바꿈이 일어나기 전까지 모든 글자를 취합한다
  fn parse_text(&mut self) -> String {
    self.consume_while(|c| !is_newline(c))
  }

  /// 현재 위치가 라인의 마지막인지 체크한다
  fn end_of_line(&self) -> bool {
    self.pos >= self.input.len()
  }

  /// 주어진 문장의 현재 위치 글자를 출력한다
  fn next_char(&self) -> char {
    self.input[self.pos..].chars().next().unwrap()
  }

  /// 주어진 문장의 현재 위치 글자를 출력하고
  /// 현재 위치를 한 칸 이동한다
  fn consume_char(&mut self) -> char {
    let mut iter = self.input[self.pos..].char_indices();
    let (_, cur_char) = iter.next().unwrap();
    let (next_pos, _) = iter.next().unwrap_or((1, ' '));
    self.pos += next_pos;
    cur_char
  }

  /// 특정 조건을 만족하는 모든 글자를 모아 출력한다
  fn consume_while<F>(&mut self, cond: F) -> String
  where
    F: Fn(char) -> bool,
  {
    let mut result = String::new();
    while !self.end_of_line() && cond(self.next_char()) {
      result.push(self.consume_char())
    }

    result
  }

  /// 띄어쓰기를 출력하고 위치를 한 칸 이동한다
  fn consume_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }
}

/// html 엘리먼트를 출력한다
fn create_html_element(tag_name: String, text: String) -> String {
  format!("<{}>{}</{}>", tag_name, text, tag_name)
}

/// 줄바꿈인지 판단한다
fn is_newline(c: char) -> bool {
  c == '\n'
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn end_of_line() {
    let mut parser = Parser {
      pos: 0,
      input: String::from("hello"),
    };
    assert_eq!(parser.end_of_line(), false);

    parser.pos = 5;
    assert_eq!(parser.end_of_line(), true);
  }

  #[test]
  fn next_char() {
    let mut parser = Parser {
      pos: 0,
      input: String::from("hello"),
    };
    assert_eq!(parser.next_char(), "h".chars().last().unwrap());
    parser.pos = 1;
    assert_eq!(parser.next_char(), "e".chars().last().unwrap());
  }

  #[test]
  fn consume_char() {
    let mut parser = Parser {
      pos: 0,
      input: String::from("hello"),
    };
    assert_eq!(parser.consume_char(), "h".chars().last().unwrap());
    assert_eq!(parser.consume_char(), "e".chars().last().unwrap());
  }

  #[test]
  fn consume_whitespace() {
    let mut parser = Parser {
      pos: 0,
      input: String::from("Hello, I'm Steve."),
    };
    assert_eq!(parser.consume_whitespace(), ());
  }

  #[test]
  fn create_html() {
    assert_eq!(create_html_element(String::from("h1"), String::from("hello")), String::from("<h1>hello</h1>"));
  }
}
