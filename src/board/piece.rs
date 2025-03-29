pub enum Color {
    White,
    Black
}

impl Color {
    pub fn from_u8(val: u8) -> Color {
        if val & 0b10000000u8 > 0u8 { Color::Black }
        else { Color::White }
    }

    pub fn get_bitmask(&self) -> u8 {
        match self {
            Color::White => 0u8,
            Color::Black => 0b10000000u8
        }
    }
}

pub enum Piece {
    Pawn { color: Color, double_push: bool, has_moved: bool },
    Knight { color: Color },
    Bishop { color: Color },
    Rook { color: Color, has_moved: bool },
    Queen { color: Color },
    King { color: Color, castling: bool, has_moved: bool },
}

// utility functions
impl Piece {
    fn get_bitmask(&self) -> u8 {
        match self {
            Piece::Pawn { color, double_push, has_moved } =>
                (if *has_moved { 0b00001000u8 } else { 0u8 }) |
                    (if *double_push { 0b00010000u8 } else { 0u8 }) |
                    color.get_bitmask(),
            Piece::Knight { color } =>
                color.get_bitmask(),
            Piece::Bishop { color } =>
                color.get_bitmask(),
            Piece::Rook { color, has_moved } =>
                (if *has_moved { 0b00001000u8 } else { 0u8 }) |
                    color.get_bitmask(),
            Piece::Queen { color } =>
                color.get_bitmask(),
            Piece::King { color, castling, has_moved } =>
                (if *has_moved { 0b00001000u8 } else { 0u8 }) |
                    (if *castling { 0b00010000u8 } else { 0u8 }) |
                    color.get_bitmask(),
        }
    }

}

// bit manipulation functions
impl Piece {
    pub fn to_u8(&self) -> u8 {
        let piece_bitmask = match self {
            Piece::Pawn { .. } => 0u8,
            Piece::Knight { .. } => 0b00000001u8,
            Piece::Bishop { .. } => 0b00000010u8,
            Piece::Rook { .. } => 0b00000011u8,
            Piece::Queen { .. } => 0b00000100u8,
            Piece::King { .. } => 0b00000101u8,
        };
        piece_bitmask | self.get_bitmask()
    }

    pub fn from_u8(val: u8) -> Option<Piece> {
        let color = Color::from_u8(val);
        let has_moved = if val & 0b00001000u8 > 0 { true } else { false };
        let double_push = if val & 0b00010000u8 > 0 { true } else { false };

        match val & 0b00000111u8 {
            0u8 => Some(Piece::Pawn { color, double_push, has_moved }),
            1u8 => Some(Piece::Knight { color }),
            2u8 => Some(Piece::Bishop { color }),
            3u8 => Some(Piece::Rook { color, has_moved }),
            4u8 => Some(Piece::Queen { color }),
            5u8 => Some(Piece::King { color, castling: true, has_moved }),
            _ => None
        }
    }
}
