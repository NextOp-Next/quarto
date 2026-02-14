use crate::Piece;

pub struct Stack(pub [Option<Piece>; 16]);

impl Stack {
    pub fn new() -> Self {
        let mut arr = [None; 16];

        for (i, item) in arr.iter_mut().enumerate() {
            *item = Some((i as u8).into());
        }

        Stack(arr)
    }

    pub fn find(&self, piece: Piece) -> Option<usize> {
        self.0
            .iter()
            .enumerate()
            .find(|(_, p)| Some(piece) == **p)
            .map(|(i, _)| i)
    }

    pub fn pick(&mut self, i: usize) -> Option<Piece> {
        if i > 16 {
            return None;
        }

        let piece = match self.0[i] {
            Some(piece) => piece,
            None => {
                return None;
            }
        };

        self.0[i] = None;
        Some(piece)
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let mut stack = Stack::new();

        assert_eq!(Some(2), stack.find(Piece(0b0010)));
        assert_eq!(Some(5), stack.find(Piece(0b0101)));
        assert_eq!(None, stack.find(Piece(0b11111111)));

        _ = stack.pick(2);
        assert_eq!(None, stack.find(Piece(0b0010)));
        assert_eq!(Some(5), stack.find(Piece(0b0101)));
        assert_eq!(None, stack.find(Piece(0b11111111)));
    }
}
