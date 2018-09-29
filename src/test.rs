use ::get_next_move_inner;

#[test]
fn returns_a_move() {
    let initial_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    get_next_move_inner(initial_fen);
}
