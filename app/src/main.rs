use quarto_core::{Board, Game, Stack};
use quarto_players::Player;

const PLAYER_COUNT: usize = 2;

pub enum Outcome {
    Win(usize),
    Draw,
    Illegal(usize),
}

pub fn game_iter(
    game: &mut Game,
    player_idx: usize,
    players: &mut [Box<dyn Player>; PLAYER_COUNT],
) -> Option<Outcome> {
    let current_player = players.get_mut(player_idx).unwrap();
    let piece_idx = current_player.give_piece(game);

    if !game.stack.0.iter().any(|p| p.is_some()) {
        return Some(Outcome::Draw);
    }

    let piece = match game.stack.pick(piece_idx) {
        Some(piece) => piece,
        None => return Some(Outcome::Illegal(player_idx)),
    };

    let next_idx = (player_idx + 1) % PLAYER_COUNT;
    let next_player = players.get_mut(next_idx).unwrap();
    let (x, y) = next_player.play_piece(game, piece);

    if game.board.get_piece(x, y).is_some() {
        return Some(Outcome::Illegal(next_idx));
    }
    game.board.set_piece(x, y, Some(piece));

    if game.board.is_win(x, y) {
        return Some(Outcome::Win(next_idx));
    }

    None
}

pub fn game_loop(players: &mut [Box<dyn Player>; 2]) -> Outcome {
    let player_count = players.len();

    let mut game = Game {
        board: Board::new(),
        stack: Stack::new(),
    };

    loop {
        for i in 0..player_count {
            if let Some(outcome) = game_iter(&mut game, i, players) {
                return outcome;
            }
        }
    }
}

pub fn main() {
    println!("hello, world!");

    let stack = Stack::new();

    for piece in stack.0 {
        let piece = piece.unwrap();
        println!(
            "{}: bright = {}, square = {}, tall = {}, hollow = {}",
            piece,
            piece.is_bright(),
            piece.is_square(),
            piece.is_tall(),
            piece.is_hollow()
        );
    }
}
