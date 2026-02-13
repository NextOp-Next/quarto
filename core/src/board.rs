use crate::Piece;

pub struct Board(pub [[Option<Piece>; 4]; 4]);

impl Board {
    pub fn new() -> Self {
        Board([[None; 4]; 4])
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        if x >= 4 || y >= 4 {
            return None;
        }

        self.0[x][y]
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Option<Piece>) {
        if x >= 4 || y >= 4 {
            return;
        }

        self.0[x][y] = piece;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_piece() {
        let board = Board::new();

        assert_eq!(None, board.get_piece(1, 1));
        assert_eq!(None, board.get_piece(5, 1));
    }

    #[test]
    fn test_set_piece() {
        let mut board = Board::new();
        let p0 = Piece::new(true, true, false, false);
        let p1 = Piece::new(false, true, true, true);

        assert_eq!(None, board.get_piece(1, 3));
        assert_eq!(None, board.get_piece(2, 0));
        assert_eq!(None, board.get_piece(0, 1));

        board.set_piece(1, 3, Some(p0));
        board.set_piece(2, 0, Some(p1));
        assert_eq!(Some(p0), board.get_piece(1, 3));
        assert_eq!(Some(p1), board.get_piece(2, 0));
        assert_eq!(None, board.get_piece(0, 1));

        board.set_piece(1, 3, None);
        assert_eq!(None, board.get_piece(1, 3));
        assert_eq!(Some(p1), board.get_piece(2, 0));
        assert_eq!(None, board.get_piece(0, 1));
    }
}
