mod constants;

use crate::board::color::Color;
use crate::board::Board;
use crate::board::piece::{Piece, PieceType};

fn get_white_table_value(piece: &Piece, (x, y): (i32, i32)) -> i32 {
    match piece.piece_type {
        PieceType::Pawn { .. } => constants::tables::PAWN_TABLE[y as usize][x as usize],
        PieceType::Knight => constants::tables::KNIGHT_TABLE[y as usize][x as usize],
        PieceType::Bishop => constants::tables::BISHOP_TABLE[y as usize][x as usize],
        PieceType::Rook { .. } => constants::tables::ROOK_TABLE[y as usize][x as usize],
        PieceType::Queen => constants::tables::QUEEN_TABLE[y as usize][x as usize],
        PieceType::King { .. } =>
            constants::tables::KING_MIDDLE_GAME_TABLE[y as usize][x as usize]
    }
}

fn get_black_table_value(piece: &Piece, (x, y): (i32, i32)) -> i32 {
    match piece.piece_type {
        PieceType::Pawn { .. } => constants::tables::PAWN_TABLE[7 - y as usize][x as usize],
        PieceType::Knight => constants::tables::KNIGHT_TABLE[7 - y as usize][x as usize],
        PieceType::Bishop => constants::tables::BISHOP_TABLE[7 - y as usize][x as usize],
        PieceType::Rook { .. } => constants::tables::ROOK_TABLE[7 - y as usize][x as usize],
        PieceType::Queen => constants::tables::QUEEN_TABLE[7 - y as usize][x as usize],
        PieceType::King { .. } =>
            constants::tables::KING_MIDDLE_GAME_TABLE[7 - y as usize][x as usize]
    }
}

pub fn evaluate(board: &Board) -> i32 {
    let mut white_score = 0;
    let mut black_score = 0;

    for x in 0..8 {
        for y in 0..8 {
            if let Some(piece) = board.get_piece((x, y)) {
                match piece.color {
                    Color::White =>
                        white_score +=
                            piece.get_worth() +
                            get_white_table_value(&piece, (x, y)),
                    Color::Black =>
                        black_score +=
                            piece.get_worth() +
                            get_black_table_value(&piece, (x, y)),
                }
            }
        }
    }

    white_score - black_score
}