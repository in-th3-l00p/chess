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

    pub fn get_piece(&self, coords: (usize, usize)) -> Option<Piece> {
        if coords.0 >= 8 || coords.1 >= 8 {
            None
        } else {
            Piece::from_u8(self.data[coords.1 + 2usize][coords.0 + 1usize])
        }
    }

    pub fn set_piece(&mut self, coords: (usize, usize), piece: &Piece) {
        self.set_data(coords, piece.to_u8());
    }

    fn get_data(&self, coords: (usize, usize)) -> u8 {
        self.data[coords.1 + 2usize][coords.0 + 1usize]
    }

    fn set_data(&mut self, coords: (usize, usize),  piece: u8) {
        self.data[coords.1 + 2usize][coords.0 + 1usize] = piece;
    }

    pub fn move_piece(
        &mut self,
        from: (usize, usize),
        to: (usize, usize)
    ) {
        // marking has_moved
        if let Some(piece) = self.get_piece(from) {
            match piece {
                Piece::Pawn { color, double_push, .. } => {
                    self.set_piece(from, &Piece::Pawn {
                        color,
                        double_push,
                        has_moved: true
                    });
                },
                Piece::Rook { color, .. } => {
                    self.set_piece(from, &Piece::Rook {
                        color,
                        has_moved: true
                    });
                },
                Piece::King { color, .. } => {
                    self.set_piece(from, &Piece::King {
                        color,
                        castling: false,
                        has_moved: true
                    });
                }
                _ => {}
            }
        }

        self.set_data(to, self.get_data(from));
        self.set_data(from, 0u8);
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