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

    pub fn set_piece(&mut self, x: usize, y: usize, piece: &Piece) {
        self.set_data(x, y, piece.to_u8());
    }

    fn get_data(&self, x: usize, y: usize) -> u8 {
        self.data[y + 2usize][x + 1usize]
    }

    fn set_data(&mut self, x: usize, y: usize, piece: u8) {
        self.data[y + 2usize][x + 1usize] = piece;
    }

    pub fn move_piece(
        &mut self,
        from: (usize, usize),
        to: (usize, usize)
    ) {
        // marking has_moved
        if let Some(piece) = self.get_piece(from.0, from.1) {
            match piece {
                Piece::Pawn { color, double_push, .. } => {
                    self.data[from.1 + 2usize][from.0 + 1usize] = Piece::Pawn {
                        color,
                        double_push,
                        has_moved: true
                    }.to_u8();
                },
                _ => {}
            }
        }

        self.set_data(to.0, to.1, self.get_data(from.0, from.1));
        self.set_data(from.0, from.1, 0u8);
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