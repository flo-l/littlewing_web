extern crate littlewing;

#[cfg(target_arch="wasm32")]
extern crate wasm_bindgen;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use littlewing::game::Game;
use littlewing::clock::Clock;
use littlewing::fen::FEN;
use littlewing::search::Search;

#[cfg(test)]
mod test;

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn get_next_move(fen: &str) -> String {
    get_next_move_inner(fen)
}

fn get_next_move_inner(fen: &str) -> String {
    let mut game = Game::from_fen(fen);
    game.clock = Clock::new(1, 5000); // Search 1 move in 5 seconds

    match game.search(1..15) { // Search from depth 1 to 15
        Some(m) => {
            m.to_coordinate_notation()
        },
        None => {
            panic!("Engine could not find a move to play");
        }
    }
}
