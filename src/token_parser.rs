enum MDTypes {
  Heading,
  Paragraph,
  Bold,
  Italics 
}

enum MDLayout {
  Block,
  Inline
}

struct Token {
  type: MDTypes,
  layout: MDLayout,
  values: vec!<Token>
}

struct Parser {
  cursor: usize,
  docs: String,
  tokens: vec!<Token>
}

impl Parser {
  fn new(cursor: usize, docs: String) -> Parser {
    Parser { cursor, docs, tokens: Vec::new() }
  }
}