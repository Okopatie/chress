use chess::GameState;
fn main() {
    let mut game = GameState::new();
    game.setup_board();
    game.print_board();
    let pawn_moves = game.pawn_move(2, 2, chess::Side::White);
    println!("{pawn_moves:?}");
}
