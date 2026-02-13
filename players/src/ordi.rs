use quarto_core::{Board, Piece, Stack};

pub trait Player {
    fn give_piece_to_other_player(&mut self, board: &Board, stack: &Stack) -> Piece;
    fn play_piece(&mut self, board: &Board, given_piece: &Piece) -> (usize, usize);
}
