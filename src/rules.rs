use regex::{Regex, Captures, Error};

#[derive(Clone)]
pub enum MDTypes {
    Heading,
    Paragraph,
    OrderedList,
    UnorderedList,
    Hr,
    Blockquote,
    Strong,
    Em,
    Strike,
    Link,
    Img,
}

pub trait Parser {
    fn new(name: MDTypes, rule: Regex) -> Self;
    fn parse(&self, line: &str,  expr: &str) -> String;
    fn get_rule(&self) -> &Regex;
}

#[derive(Clone)]
pub struct Rule {
    name: MDTypes,
    rule: Regex,
}

impl Parser for Rule  {
    fn new(name: MDTypes, rule: Regex) -> Self {
        Rule { name, rule }
    }

    fn parse(&self, line: &str, expr: &str) -> String {
        self.rule.replace(line, expr).to_string()
    }

    fn get_rule(&self) -> &Regex {
        &self.rule
    }
}
