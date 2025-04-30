mod constants;

use crate::board::pieces::color::Color;
use crate::board::Board;
use crate::board::pieces::{Piece, PieceType};

fn is_end_game(board: &Board) -> bool {
    let mut black_queen = false;
    let mut white_queen = false;
    for x in 0..8 {
        for y in 0..8 {
            if let Some(piece) = board.get_piece((x, y)) {
                if let PieceType::Queen = piece.piece_type {
                    if piece.color == Color::Black {
                        black_queen = true;
                    } else {
                        white_queen = true;
                    }
                }
            }
        }
    }

    !white_queen && !black_queen
}

fn get_white_table_value(piece: &Piece, (x, y): (i32, i32), endgame: bool) -> i32 {
    match piece.piece_type {
        PieceType::Pawn { .. } => constants::tables::PAWN_TABLE[y as usize][x as usize],
        PieceType::Knight => constants::tables::KNIGHT_TABLE[y as usize][x as usize],
        PieceType::Bishop => constants::tables::BISHOP_TABLE[y as usize][x as usize],
        PieceType::Rook { .. } => constants::tables::ROOK_TABLE[y as usize][x as usize],
        PieceType::Queen => constants::tables::QUEEN_TABLE[y as usize][x as usize],
        PieceType::King { .. } =>
            if endgame {
                constants::tables::KING_END_GAME_TABLE[y as usize][x as usize]
            } else {
                constants::tables::KING_MIDDLE_GAME_TABLE[y as usize][x as usize]
            }
    }
}

fn get_black_table_value(piece: &Piece, (x, y): (i32, i32), endgame: bool) -> i32 {
    match piece.piece_type {
        PieceType::Pawn { .. } => constants::tables::PAWN_TABLE[7 - y as usize][x as usize],
        PieceType::Knight => constants::tables::KNIGHT_TABLE[7 - y as usize][x as usize],
        PieceType::Bishop => constants::tables::BISHOP_TABLE[7 - y as usize][x as usize],
        PieceType::Rook { .. } => constants::tables::ROOK_TABLE[7 - y as usize][x as usize],
        PieceType::Queen => constants::tables::QUEEN_TABLE[7 - y as usize][x as usize],
        PieceType::King { .. } =>
            if endgame {
                constants::tables::KING_END_GAME_TABLE[7 - y as usize][x as usize]
            } else {
                constants::tables::KING_MIDDLE_GAME_TABLE[7 - y as usize][x as usize]
            }
    }
}

pub fn evaluate(board: &Board) -> i32 {
    let mut white_score = 0;
    let mut black_score = 0;
    let endgame = is_end_game(board);

    for x in 0..8 {
        for y in 0..8 {
            if let Some(piece) = board.get_piece((x, y)) {
                match piece.color {
                    Color::White =>
                        white_score +=
                            piece.get_worth() +
                            get_white_table_value(&piece, (x, y), endgame),
                    Color::Black =>
                        black_score +=
                            piece.get_worth() +
                            get_black_table_value(&piece, (x, y), endgame),
                }
            }
        }
    }

    white_score - black_score
}