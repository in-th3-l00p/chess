use crate::board::constants::INITIAL_BOARD;
use crate::board::piece::{Piece, PieceType};

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

    pub fn get_piece(&self, coords: (i32, i32)) -> Option<Piece> {
        Piece::from_u8(self.data[(coords.1 + 2) as usize][(coords.0 + 1) as usize])
    }

    pub fn set_piece(&mut self, coords: (i32, i32), piece: &Piece) {
        self.set_data(coords, piece.to_u8());
    }

    fn get_data(&self, coords: (i32, i32)) -> u8 {
        self.data[(coords.1 + 2) as usize][(coords.0 + 1) as usize]
    }

    fn set_data(&mut self, coords: (i32, i32),  piece: u8) {
        self.data[(coords.1 + 2) as usize][(coords.0 + 1) as usize] = piece;
    }

    pub fn move_piece(
        &mut self,
        from: (i32, i32),
        to: (i32, i32)
    ) {
        // marking has_moved
        if let Some(piece) = self.get_piece(from) {
            match piece.piece_type {
                PieceType::Pawn { double_push, .. } => {
                    self.set_piece(from, &Piece {
                        color: piece.color,
                        piece_type: PieceType::Pawn {
                            double_push,
                            has_moved: true
                        },
                    });
                },
                PieceType::Rook { .. } => {
                    self.set_piece(from, &Piece {
                        color: piece.color,
                        piece_type: PieceType::Rook {
                            has_moved: true
                        },
                    });
                },
                PieceType::King { castling, .. } => {
                    self.set_piece(from, &Piece {
                        color: piece.color,
                        piece_type: PieceType::King {
                            castling,
                            has_moved: true
                        }
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