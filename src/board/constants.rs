use crate::board::color::Color;
use crate::board::piece::Piece;

pub const PIECES: [(Piece, u8); 26] = [
    (
        Piece::Pawn {
            color: Color::White,
            double_push: false,
            has_moved: false,
        },
        0u8,
    ),
    (
        Piece::Pawn {
            color: Color::White,
            double_push: false,
            has_moved: true,
        },
        0b00001000u8,
    ),
    (
        Piece::Pawn {
            color: Color::White,
            double_push: true,
            has_moved: false,
        },
        0b00010000u8,
    ),
    (
        Piece::Pawn {
            color: Color::White,
            double_push: true,
            has_moved: true,
        },
        0b00011000u8,
    ),
    (
        Piece::Pawn {
            color: Color::Black,
            double_push: false,
            has_moved: false,
        },
        0b10000000u8,
    ),
    (
        Piece::Pawn {
            color: Color::Black,
            double_push: false,
            has_moved: true,
        },
        0b10001000u8,
    ),
    (
        Piece::Pawn {
            color: Color::Black,
            double_push: true,
            has_moved: false,
        },
        0b10010000u8,
    ),
    (
        Piece::Pawn {
            color: Color::Black,
            double_push: true,
            has_moved: true,
        },
        0b10011000u8,
    ),
    (
        Piece::Knight {
            color: Color::White,
        },
        0b00000001u8,
    ),
    (
        Piece::Knight {
            color: Color::Black,
        },
        0b10000001u8,
    ),
    (
        Piece::Bishop {
            color: Color::White,
        },
        0b00000010u8,
    ),
    (
        Piece::Bishop {
            color: Color::Black,
        },
        0b10000010u8,
    ),
    (
        Piece::Rook {
            color: Color::White,
            has_moved: false,
        },
        0b00000011u8,
    ),
    (
        Piece::Rook {
            color: Color::White,
            has_moved: true,
        },
        0b00001011u8,
    ),
    (
        Piece::Rook {
            color: Color::Black,
            has_moved: false,
        },
        0b10000011u8,
    ),
    (
        Piece::Rook {
            color: Color::Black,
            has_moved: true,
        },
        0b10001011u8,
    ),
    (
        Piece::Queen {
            color: Color::White,
        },
        0b00000100u8,
    ),
    (
        Piece::Queen {
            color: Color::Black,
        },
        0b10000100u8,
    ),
    (
        Piece::King {
            color: Color::White,
            has_moved: false,
            castling: false,
        },
        0b00000101u8,
    ),
    (
        Piece::King {
            color: Color::White,
            has_moved: true,
            castling: false,
        },
        0b00001101u8,
    ),
    (
        Piece::King {
            color: Color::White,
            has_moved: false,
            castling: true,
        },
        0b00010101u8,
    ),
    (
        Piece::King {
            color: Color::White,
            has_moved: true,
            castling: true,
        },
        0b00011101u8,
    ),
    (
        Piece::King {
            color: Color::Black,
            has_moved: false,
            castling: false,
        },
        0b10000101u8,
    ),
    (
        Piece::King {
            color: Color::Black,
            has_moved: true,
            castling: false,
        },
        0b10001101u8,
    ),
    (
        Piece::King {
            color: Color::Black,
            has_moved: false,
            castling: true,
        },
        0b10010101u8,
    ),
    (
        Piece::King {
            color: Color::Black,
            has_moved: true,
            castling: true,
        },
        0b10011101u8,
    ),
];
