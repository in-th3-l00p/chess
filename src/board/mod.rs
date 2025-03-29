use crate::board::constants::INITIAL_BOARD;

pub mod piece;
pub mod color;
mod constants;

pub struct Board {
    data: [[u8; 10]; 12]
}

impl Board {
    pub fn new() -> Board {
        Board {
            data: INITIAL_BOARD,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let board = Board::new();
        assert_eq!(board.data, INITIAL_BOARD);
    }
}