pub mod bot_human;
pub mod random;

use quarto_core::{Game, Piece};

pub trait Player {
    fn give_piece(&mut self, game: &Game) -> Piece;
    fn play_piece(&mut self, game: &Game, given_piece: Piece) -> (usize, usize);
}
