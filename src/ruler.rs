use regex::Regex;

pub struct Rule<'a> {
  rule: Regex,
  element: &'a str,
}

impl<'a> Rule<'a> {
  pub fn new(re: Regex, element: &'a str) -> Rule {
    Rule { rule: re, element }
  }

  pub fn is(&self, line: &str) -> bool {
    self.rule.is_match(line)
  }

  pub fn format(&self) -> &str {
    self.element
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_heading() {
    let wow = Rule::new(Regex::new(r"^#{1,6}\s").unwrap(), "heading");
    assert!(wow.is("# wow"));
  }
}
