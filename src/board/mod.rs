use crate::board::constants::INITIAL_BOARD;
use crate::board::piece::Piece;

pub mod piece;
pub mod color;
mod constants;

pub struct Board {
    data: [[u8; 10]; 12]
}

impl Board {
    pub fn new() -> Board {
        Board {
            data: INITIAL_BOARD,
        }
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        if x >= 8 || y >= 8 {
            None
        } else {
            Piece::from_u8(self.data[y + 2usize][x + 1usize])
        }
    }

    pub fn move_piece(
        &mut self,
        from: (usize, usize),
        to: (usize, usize)
    ) {
        self.data[to.1 + 2usize][to.0 + 1usize] = self.data[from.1 + 2][from.0 + 1usize];
        self.data[from.1 + 2usize][from.0 + 1usize] = 0u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let board = Board::new();
        assert_eq!(board.data, INITIAL_BOARD);
    }
}