use regex::{Regex};

#[derive(Clone, Copy)]
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
    fn new(name: MDTypes, rule: Regex, parse_expr: String) -> Self;
    fn parse(&self, line: &str,  expr: &String) -> String;
    fn get_rule(&self) -> &Regex;
    fn get_parse_expr(&self) -> &String;
}

#[derive(Clone)]
pub struct Rule {
    name: MDTypes,
    regex: Regex,
    parse_expr: String,
}

impl Parser for Rule {
    fn new(name: MDTypes, regex: Regex, parse_expr: String) -> Self {
        Rule { name, regex, parse_expr }
    }

    fn parse(&self, line: &str, expr: &String) -> String {
        self.regex.replace(line, expr.as_str()).to_string()
    }

    fn get_rule(&self) -> &Regex {
        &self.regex
    }

    fn get_parse_expr(&self) -> &String {
        &self.parse_expr
    }
}
