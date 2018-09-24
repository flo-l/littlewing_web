extern crate littlewing;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_next_move(board_fen: &str, white: bool) -> String {
    "e2e4".into()
}
