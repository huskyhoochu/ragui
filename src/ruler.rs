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

#[cfg(test)]
mod tests {
  use regex::Regex;
  use super::*;
  use rule::block::Checkerable;

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
