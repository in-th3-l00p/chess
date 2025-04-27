use crate::board::color::Color;
use crate::board::constants::{EMPTY_BOARD, INITIAL_BOARD};
use crate::board::piece::{Piece, PieceType};
use crate::move_generation::BoardMove;

pub mod piece;
pub mod color;
mod constants;

#[derive(Clone, Eq, PartialEq)]
pub struct Board {
    data: [[u8; 12]; 12],
    last_move: Option<BoardMove>,
    turn: Color
}

// private stuff
impl Board {
    fn get_data(&self, coords: (i32, i32)) -> u8 {
        self.data[(coords.1 + 2) as usize][(coords.0 + 2) as usize]
    }

    fn set_data(&mut self, coords: (i32, i32),  piece: u8) {
        self.data[(coords.1 + 2) as usize][(coords.0 + 2) as usize] = piece;
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            data: EMPTY_BOARD,
            last_move: None,
            turn: Color::White
        }
    }

    pub fn new_game() -> Board {
        Board {
            data: INITIAL_BOARD,
            last_move: None,
            turn: Color::White
        }
    }

    pub fn from_fen(fen: &str) -> Result<Board, ()> {
        let mut board = Self::new();
        let mut fen = fen.split_whitespace();

        // parsing positions
        if let Some(positions) = fen.next() {
            let mut x = 0;
            let mut y = 0;
            for c in positions.chars() {
                if c == '/' {
                    x = 0;
                    y += 1;
                } else if c.is_digit(10) {
                    x += c.to_digit(10).unwrap() as i32;
                } else if c.is_alphabetic() {
                    if c.to_ascii_lowercase() == 'p' {
                        board.set_piece((x, y), &Piece {
                            color: if c.is_uppercase() { Color::White } else { Color::Black },
                            piece_type: PieceType::Pawn {
                                has_moved: if c.is_uppercase() { y != 6 } else { y != 1 }
                            }
                        });
                    } else if c.to_ascii_lowercase() == 'n' {
                        board.set_piece((x, y), &Piece {
                            color: if c.is_uppercase() { Color::White } else { Color::Black },
                            piece_type: PieceType::Knight
                        });
                    } else if c.to_ascii_lowercase() == 'b' {
                        board.set_piece((x, y), &Piece {
                            color: if c.is_uppercase() { Color::White } else { Color::Black },
                            piece_type: PieceType::Bishop
                        });
                    } else if c.to_ascii_lowercase() == 'r' {
                        board.set_piece((x, y), &Piece {
                            color: if c.is_uppercase() { Color::White } else { Color::Black },
                            piece_type: PieceType::Rook { has_moved: true }
                        });
                    } else if c.to_ascii_lowercase() == 'q' {
                        board.set_piece((x, y), &Piece {
                            color: if c.is_uppercase() { Color::White } else { Color::Black },
                            piece_type: PieceType::Queen
                        });
                    } else if c.to_ascii_lowercase() == 'k' {
                        board.set_piece((x, y), &Piece {
                            color: if c.is_uppercase() { Color::White } else { Color::Black },
                            piece_type: PieceType::King { castling: false, has_moved: true }
                        });
                    } else {
                        return Err(());
                    }


                    x += 1;
                }
            }
        }

        // getting current turn
        if let Some(turn) = fen.next() {
            match turn {
                "w" => board.turn = Color::White,
                "b" => board.turn = Color::Black,
                _ => return Err(())
            }
        }

        Ok(board)
    }

    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn change_turn(&mut self) {
        self.turn = self.turn.inverse();
    }

    pub fn get_piece(&self, coords: (i32, i32)) -> Option<Piece> {
        Piece::from_u8(self.data[(coords.1 + 2) as usize][(coords.0 + 2) as usize])
    }

    pub fn set_piece(&mut self, coords: (i32, i32), piece: &Piece) {
        self.set_data(coords, piece.to_u8());
    }

    pub fn get_last_move(&self) -> &Option<BoardMove> {
        &self.last_move
    }

    pub fn is_in_bounds(&self, coords: (i32, i32)) -> bool {
        self.get_data(coords) != 255u8
    }

    pub fn make_move(
        &mut self,
        board_move: BoardMove,
    ) {
        // marking has_moved
        if let Some(piece) = self.get_piece(board_move.from) {
            match piece.piece_type {
                PieceType::Pawn { .. } => {
                    // check for en passant
                    let delta = if let Color::White = piece.color { -1 } else { 1 };
                    if
                        board_move.to.1 == board_move.from.1 + delta &&
                        (
                            board_move.from.0 + 1 == board_move.to.0 ||
                            board_move.from.0 - 1 == board_move.to.0
                        ) &&
                        self.get_data(board_move.to) == 0u8
                    {
                        self.set_data(
                            (board_move.to.0, board_move.to.1 - delta),
                            0u8
                        );
                    }
                    self.set_piece(board_move.from, &Piece {
                        color: piece.color,
                        piece_type: PieceType::Pawn {
                            has_moved: true
                        },
                    });
                },
                PieceType::Rook { .. } => {
                    self.set_piece(board_move.from, &Piece {
                        color: piece.color,
                        piece_type: PieceType::Rook {
                            has_moved: true
                        },
                    });
                },
                PieceType::King { has_moved, castling } => {
                    // check for castling
                    if !has_moved && board_move.to.1 == board_move.from.1 {
                        if board_move.to.0 == 6 {
                            self.set_data(
                                (5, board_move.from.1),
                                self.get_data((7, board_move.from.1)) | 0b00001000u8
                            );
                            self.set_data((7, board_move.from.1), 0u8);
                        } else if board_move.to.0 == 2 {
                            self.set_data(
                                (3, board_move.from.1),
                                self.get_data((0, board_move.from.1)) | 0b00001000u8
                            );
                            self.set_data((0, board_move.from.1), 0u8);
                        }
                    }

                    self.set_piece(board_move.from, &Piece {
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

        self.last_move = Some(board_move.clone());
        self.set_data(board_move.to, self.get_data(board_move.from));
        self.set_data(board_move.from, 0u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let board = Board::new_game();
        assert_eq!(board.data, INITIAL_BOARD);
    }
}