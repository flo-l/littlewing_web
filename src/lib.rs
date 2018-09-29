extern crate littlewing;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use littlewing::game::Game;
use littlewing::clock::Clock;
use littlewing::fen::FEN;
use littlewing::search::Search;
use littlewing::piece_move_notation::PieceMoveNotation;

#[wasm_bindgen]
pub fn get_next_move(fen: &str) -> String {
    let mut game = Game::from_fen(fen);
    game.clock = Clock::new(1, 5000); // Search 1 move in 5 seconds

    match game.search(1..15) { // Search from depth 1 to 15
        Some(m) => {
            game.move_to_san(m)
        },
        None => {
            panic!("Engine could not find a move to play");
        }
    }
}
