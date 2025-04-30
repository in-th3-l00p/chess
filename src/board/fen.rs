use crate::board::Board;
use crate::board::pieces::color::Color;
use crate::board::pieces::{Piece, PieceType};

impl Board {
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
                            piece_type: PieceType::King { has_moved: true }
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

        // castling
        if let Some(castling) = fen.next() {
            for c in castling.chars() {
                match c {
                    'k' => {
                        board.set_piece((4, 0), &Piece {
                            color: Color::Black,
                            piece_type: PieceType::King { has_moved: false }
                        });
                        board.set_piece((7, 0), &Piece {
                            color: Color::Black,
                            piece_type: PieceType::Rook { has_moved: false }
                        })
                    },
                    'q' => {
                        board.set_piece((4, 0), &Piece {
                            color: Color::Black,
                            piece_type: PieceType::King { has_moved: false }
                        });
                        board.set_piece((0, 0), &Piece {
                            color: Color::Black,
                            piece_type: PieceType::Rook { has_moved: false }
                        })
                    },
                    'K' => {
                        board.set_piece((4, 7), &Piece {
                            color: Color::White,
                            piece_type: PieceType::King { has_moved: false }
                        });
                        board.set_piece((7, 7), &Piece {
                            color: Color::White,
                            piece_type: PieceType::Rook { has_moved: false }
                        })
                    },
                    'Q' => {
                        board.set_piece((4, 7), &Piece {
                            color: Color::White,
                            piece_type: PieceType::King { has_moved: false }
                        });
                        board.set_piece((0, 7), &Piece {
                            color: Color::White,
                            piece_type: PieceType::Rook { has_moved: false }
                        })
                    },
                    _ => {}
                }
            }
        }

        Ok(board)
    }
}