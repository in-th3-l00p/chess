use crate::board::color::Color;
use crate::board::piece::{Piece, PieceType};

const RANK_8: [u8; 10] = [
    255u8,
    0b10000100u8,
    0b10000010u8,
    0b10000011u8,
    0b10000101u8,
    0b10000110u8,
    0b10000011u8,
    0b10000010u8,
    0b10000100u8,
    255u8,
];

const RANK_7: [u8; 10] = [
    255u8,
    0b10000001u8,
    0b10000001u8,
    0b10000001u8,
    0b10000001u8,
    0b10000001u8,
    0b10000001u8,
    0b10000001u8,
    0b10000001u8,
    255u8,
];

const EMPTY_RANK: [u8; 10] = [
    255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8
];


const RANK_2: [u8; 10] = [
    255u8,
    0b00000001u8,
    0b00000001u8,
    0b00000001u8,
    0b00000001u8,
    0b00000001u8,
    0b00000001u8,
    0b00000001u8,
    0b00000001u8,
    255u8,
];

const RANK_1: [u8; 10] = [
    255u8,
    0b00000100u8,
    0b00000010u8,
    0b00000011u8,
    0b00000101u8,
    0b00000110u8,
    0b00000011u8,
    0b00000010u8,
    0b00000100u8,
    255u8
];

pub const INITIAL_BOARD: [[u8; 10]; 12] = [
    [255u8; 10],
    [255u8; 10],
    RANK_8,
    RANK_7,
    EMPTY_RANK,
    EMPTY_RANK,
    EMPTY_RANK,
    EMPTY_RANK,
    RANK_2,
    RANK_1,
    [255u8; 10],
    [255u8; 10]
];

pub const _PIECES: [(Piece, u8); 26] = [
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn {
                double_push: false,
                has_moved: false,
            }
        },
        0b00000001u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn {
                double_push: false,
                has_moved: true,
            }
        },
        0b00001001u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn {
                double_push: true,
                has_moved: false,
            }
        },
        0b00010001u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::Pawn {
                double_push: true,
                has_moved: true,
            }
        },
        0b00011001u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::Pawn {
                double_push: false,
                has_moved: false,
            }
        },
        0b10000001u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::Pawn {
                double_push: false,
                has_moved: true,
            }
        },
        0b10001001u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::Pawn {
                double_push: true,
                has_moved: false,
            }
        },
        0b10010001u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::Pawn {
                double_push: true,
                has_moved: true,
            }
        },
        0b10011001u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::Knight,
        },
        0b00000010u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::Knight,
        },
        0b10000010u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::Bishop,
        },
        0b00000011u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::Bishop,
        },
        0b10000011u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::Rook {
                has_moved: false,
            }
        },
        0b00000100u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::Rook {
                has_moved: true,
            }
        },
        0b00001100u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::Rook {
                has_moved: false,
            },
        },
        0b10000100u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::Rook {
                has_moved: true,
            },
        },
        0b10001100u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::Queen,
        },
        0b00000101u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::Queen
        },
        0b10000101u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::King {
                has_moved: false,
                castling: false,
            }
        },
        0b00000110u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::King {
                has_moved: true,
                castling: false,
            }
        },
        0b00001110u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::King {
                has_moved: false,
                castling: true,
            }
        },
        0b00010110u8,
    ),
    (
        Piece {
            color: Color::White,
            piece_type: PieceType::King {
                has_moved: true,
                castling: true,
            }
        },
        0b00011110u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::King {
                has_moved: false,
                castling: false,
            }
        },
        0b10000110u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::King {
                has_moved: true,
                castling: false,
            }
        },
        0b10001110u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::King {
                has_moved: false,
                castling: true,
            },
        },
        0b10010110u8,
    ),
    (
        Piece {
            color: Color::Black,
            piece_type: PieceType::King {
                has_moved: true,
                castling: true,
            },
        },
        0b10011110u8,
    ),
];
