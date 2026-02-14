use quarto_core::Piece;
use rand::RngExt;

use crate::Player;

pub struct RandomBot {
    rng: rand::rngs::ThreadRng,
}

impl Default for RandomBot {
    fn default() -> Self {
        Self { rng: rand::rng() }
    }
}

impl Player for RandomBot {
    fn give_piece(&mut self, game: &quarto_core::Game) -> Piece {
        let count = game.stack.0.count_ones();
        let mut target = self.rng.random_range(0..count);

        for i in 0..16 {
            if (game.stack.0 >> i) & 1 == 1 {
                if target == 0 {
                    return Piece(i);
                }
                target -= 1;
            }
        }

        unreachable!();
    }

    fn play_piece(&mut self, game: &quarto_core::Game, given_piece: Piece) -> (usize, usize) {
        _ = given_piece;

        let count = (0..16)
            .map(|i| (i / 4, i % 4))
            .filter(|(x, y)| game.board.get_piece(*x, *y).is_none())
            .count();
        let mut target = self.rng.random_range(0..count);

        for i in 0..16 {
            let (x, y) = (i / 4, i % 4);
            if game.board.get_piece(x, y).is_none() {
                if target == 0 {
                    return (x, y);
                }
                target -= 1;
            }
        }

        unreachable!();
    }
}
