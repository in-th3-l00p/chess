use crate::board::Board;
use crate::board::pieces::color::Color;
use crate::board::pieces::{Piece, PieceType};

#[derive(Clone, Eq, PartialEq)]
pub struct BoardMove {
    pub from: (i32, i32),
    pub to: (i32, i32),
    pub promote: Option<PieceType>,
}

impl BoardMove {
    pub fn new(
        from: (i32, i32),
        to: (i32, i32),
        promote: Option<PieceType>
    ) -> BoardMove {
        BoardMove { from, to, promote }
    }
}

impl Board {
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
                        piece_type: board_move.promote.clone().unwrap_or(
                            PieceType::Pawn {
                                has_moved: true
                            }
                        )
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
                PieceType::King { has_moved } => {
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
