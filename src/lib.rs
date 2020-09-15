mod ragui_parser;
mod parser;
mod ruler;
mod tokenizer;
mod normalizer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
  let result = ragui_parser::parse(input.to_string());
  result
}

#[wasm_bindgen]
pub fn ragui(input: &str) -> String {
  let parsed = parser::parse(input);
  parsed.join("\n")
}
