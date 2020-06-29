mod ragui_parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
  let result = ragui_parser::parse(input.to_string());
  result
}
