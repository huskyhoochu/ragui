// use crate::tokenizer::Token;
// use crate::rules::Parser;
//
// struct Tree<T: Parser> {
//     root: Token<T>,
// }
//
// impl<T> Tree<T> where T: Parser {
//     fn new(token: Token<T>) -> Tree<T> {
//         Tree { root: token }
//     }
//
//     fn size(&self) -> usize {
//         self.root.size()
//     }
//
//     fn _put(token: Token<T>, mut parent: &Token<T>, index: usize) {
//         parent.set_child(token, index);
//     }
//
//     pub fn put(&self, token: Token<T>, index: number) {
//         Tree::_put(token, &self.root, index);
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_tree() {
//
//     }
// }