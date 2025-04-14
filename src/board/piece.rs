use crate::board::color::Color;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Piece {
    pub(crate) color: Color,
    pub(crate) piece_type: PieceType
}

#[derive(Debug)]
pub enum PieceType {
    Pawn {
        double_push: bool,
        has_moved: bool,
    },
    Knight,
    Bishop,
    Rook {
        has_moved: bool,
    },
    Queen,
    King {
        castling: bool,
        has_moved: bool,
    },
}

impl PieceType {
    pub fn to_u8(&self) -> u8 {
        match self {
            PieceType::Pawn { .. } => 0b00000001u8,
            PieceType::Knight { .. } => 0b00000010u8,
            PieceType::Bishop { .. } => 0b00000011u8,
            PieceType::Rook { .. } => 0b00000100u8,
            PieceType::Queen { .. } => 0b00000101u8,
            PieceType::King { .. } => 0b00000110u8,
        }
    }
}

// utility functions
impl Piece {
    // gets bitmask of the color and state of the piece, without type
    fn get_data_bitmask(&self) -> u8 {
        self.color.get_bitmask() | match self.piece_type {
            PieceType::Pawn {
                double_push,
                has_moved,
            } => {
                (if has_moved { 0b00001000u8 } else { 0u8 })
                    | (if double_push { 0b00010000u8 } else { 0u8 })
            }
            PieceType::Rook { has_moved } => {
                if has_moved { 0b00001000u8 } else { 0u8 }
            }
            PieceType::King {
                castling,
                has_moved,
            } => {
                (if has_moved { 0b00001000u8 } else { 0u8 })
                    | (if castling { 0b00010000u8 } else { 0u8 })
            },
            _ => 0u8
        }
    }
}

// bit manipulation functions
impl Piece {
    pub fn to_u8(&self) -> u8 {
        self.piece_type.to_u8() | self.get_data_bitmask()
    }

    pub fn from_u8(val: u8) -> Option<Piece> {
        if val == 0u8 || val == 255u8 {
            return None;
        }

        let color = Color::from_u8(val);
        let castling = if val & 0b00010000u8 > 0 { true } else { false };
        let has_moved = if val & 0b00001000u8 > 0 { true } else { false };
        let double_push = if val & 0b00010000u8 > 0 { true } else { false };

        let piece_type = match val & 0b00000111u8 {
            1u8 => Some(PieceType::Pawn {
                double_push,
                has_moved,
            }),
            2u8 => Some(PieceType::Knight),
            3u8 => Some(PieceType::Bishop),
            4u8 => Some(PieceType::Rook { has_moved }),
            5u8 => Some(PieceType::Queen),
            6u8 => Some(PieceType::King {
                castling,
                has_moved,
            }),
            _ => None,
        };

        if let Some(piece_type) = piece_type {
            Some(Piece {
                piece_type, color
            })
        } else {
            None
        }
    }
}

impl PartialEq<Self> for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.to_u8() == other.to_u8()
    }
}

impl Eq for Piece {}

impl Hash for Piece {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*state).write_u8(self.to_u8());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::constants::_PIECES;

    #[test]
    pub fn it_converts_to_u8() {
        for (piece, piece_mask) in _PIECES {
            assert_eq!(piece.to_u8(), piece_mask);
        }
    }

    #[test]
    pub fn it_converts_from_u8() {
        for (piece, piece_mask) in _PIECES {
            let optional_piece = Piece::from_u8(piece_mask);
            assert!(optional_piece.is_some());
            assert_eq!(optional_piece.unwrap(), piece);
        }
        assert_eq!(Piece::from_u8(0u8), None);
    }

    #[test]
    pub fn it_gets_data_bitmask() {
        for (piece, piece_mask) in _PIECES {
            assert_eq!(piece.get_data_bitmask(), piece_mask & !0b00000111u8);
        }
    }

    // mocked hasher implementation so that I can compare
    // hashes directly
    struct TestHasher {
        output: Vec<u8>,
    }

    impl TestHasher {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }

    impl Hasher for TestHasher {
        fn finish(&self) -> u64 {
            let mut bytes = [0u8; 8];
            let len = self.output.len().min(8);
            bytes[..len].copy_from_slice(&self.output[..len]);
            u64::from_ne_bytes(bytes)
        }

        fn write(&mut self, bytes: &[u8]) {
            self.output.extend_from_slice(bytes);
        }
    }

    #[test]
    pub fn it_hashes() {
        for (piece, piece_mask) in _PIECES {
            let mut hasher = TestHasher::new();
            piece.to_u8().hash(&mut hasher);
            assert_eq!(hasher.finish(), piece_mask as u64);
            assert_eq!(hasher.finish(), piece.to_u8() as u64);
        }
    }

    #[test]
    pub fn it_compares() {
        for (piece1, piece1_mask) in _PIECES {
            for (piece2, piece2_mask) in _PIECES {
                if piece1_mask == piece2_mask {
                    assert_eq!(piece1, piece2);
                } else {
                    assert_ne!(piece1, piece2);
                }
            }
        }
    }
}
