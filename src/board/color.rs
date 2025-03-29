#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_converts_from_u8() {
        let black_piece: u8 = 0b10000101u8;
        let white_piece: u8 = 0b00001000u8;

        assert!(match Color::from_u8(black_piece)
            { Color::Black => true, _ => false });
        assert!(match Color::from_u8(white_piece)
            { Color::White => true, _ => false });
    }

    #[test]
    pub fn it_gets_bitmask() {
        assert_eq!((Color::White {}).get_bitmask(), 0u8);
        assert_eq!((Color::Black).get_bitmask(), 1u8 << 7u8);
    }
}