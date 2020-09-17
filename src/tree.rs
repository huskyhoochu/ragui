use crate::tokenizer::Token;
use crate::rules::{Parser};

struct Tree<T> where T: Clone + Parser {
    root: Token<T>,
}

impl<T> Tree<T> where T: Clone + Parser {
    fn new(root: Token<T>) -> Tree<T> {
        Tree { root }
    }

    pub fn put(&mut self, tokens: &mut Vec<Token<T>>) {
       self.root.set_children(tokens);
    }

    pub fn show(&self) -> String {
        let mut result = String::new();

        for child in self.root.children.iter() {
            let html = child.get_html();
            result.push_str(html.as_str());
        }

        result
    }
}


#[cfg(test)]
mod tests;
