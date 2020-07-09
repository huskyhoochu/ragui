use regex::Regex;

struct Rule<'a> {
  rule: Regex,
  element: &'a str,
}

trait Checkerable {
  fn is_a(&self, line: &str) -> Option<String>;
}

impl<'a> Checkerable for Rule<'a> {
  fn is_a(&self, line: &str) -> Option<String> {
    if self.rule.is_match(line) {
      Some(format!("{}", self.element))
    } else {
      None
    }
  }
}

impl<'a> Rule<'a> {
  fn new(re: Regex, element: &'a str) -> Rule {
    Rule { rule: re, element }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_heading() {
    let wow = Rule::new(Regex::new(r"^#{1,6}\s").unwrap(), "heading");
    assert_eq!(wow.is_a("# wow").unwrap(), "heading".to_string());
  }
}
