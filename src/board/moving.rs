use crate::board::Board;
use crate::board::pieces::color::Color;
use crate::board::pieces::{Piece, PieceType};

#[derive(Clone, Eq, PartialEq)]
pub struct BoardMove {
    pub from: (i32, i32),
    pub to: (i32, i32),
    pub promote: Option<PieceType>,

    pub moved_piece: Piece,
    pub captured_piece: Option<Piece>,
    pub en_passant: bool,
    pub castling_rook_move: Option<((i32,i32),(i32,i32))>,
}

impl BoardMove {
    pub fn new(
        from: (i32, i32),
        to: (i32, i32),
        promote: Option<PieceType>,
    ) -> BoardMove {
        BoardMove {
            from, to, promote,
            moved_piece: Piece::from_u8(1).unwrap(),
            captured_piece: None,
            en_passant: false,
            castling_rook_move: None,
        }
    }
}

impl Board {
    pub fn make_move(
        &mut self,
        mut board_move: BoardMove,
    ) {
        board_move.moved_piece = self.get_piece(board_move.from).unwrap();
        board_move.captured_piece = self.get_piece(board_move.to);

        // marking has_moved
        if let Some(piece) = self.get_piece(board_move.from) {
            match piece.piece_type {
                PieceType::Pawn { .. } => {
                    // check for en passant
                    let delta = if let Color::White = piece.color { -1 } else { 1 };
                    if board_move.to.1 == board_move.from.1 + delta &&
                        (
                            board_move.from.0 + 1 == board_move.to.0 ||
                                board_move.from.0 - 1 == board_move.to.0
                        ) &&
                        self.get_data(board_move.to) == 0u8
                    {
                        let capture = (board_move.to.0, board_move.to.1 - delta);
                        board_move.en_passant = true;
                        board_move.captured_piece = self.get_piece(capture);
                        self.set_empty(capture);
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
                        let mut rook_from = None;
                        let mut rook_to = None;
                        if board_move.to.0 == 6 {
                            rook_from = Some((7, board_move.from.1));
                            rook_to = Some((5, board_move.from.1));

                        } else if board_move.to.0 == 2 {
                            rook_from = Some((0, board_move.from.1));
                            rook_to = Some((3, board_move.from.1));
                        }

                        if let Some(rook_from) = rook_from {
                            if let Some(rook_to) = rook_to {
                                board_move.castling_rook_move = Some((rook_from, rook_to));
                                self.set_data(
                                    rook_to,
                                    self.get_data((7, board_move.from.1)) | 0b00001000u8
                                );
                                self.set_empty(rook_from);
                            }
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
        self.turn = self.turn.inverse();
        self.set_data(board_move.to, self.get_data(board_move.from));
        self.set_empty(board_move.from);
    }

    pub fn unmake_move(&mut self) {
    }
}
