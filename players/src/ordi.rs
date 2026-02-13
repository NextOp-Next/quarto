use quarto_core::{Board, Piece};

pub trait Player {
    fn give_piece_to_other_player(&mut self, board: &Board) -> Piece;
    fn play_piece(&mut self, board: &Board, given_piece: &Piece) -> (usize, usize);
}
