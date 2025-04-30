pub mod pieces;
mod constants;
mod fen;
pub mod moving;
pub mod generation;

use crate::board::constants::{EMPTY_BOARD, INITIAL_BOARD};
use crate::board::pieces::color::Color;
use crate::board::pieces::Piece;
use moving::BoardMove;

#[derive(Clone, Eq, PartialEq)]
pub struct Board {
    data: [[u8; 12]; 12],
    last_move: Option<BoardMove>,
    turn: Color
}

// private stuff
impl Board {
    fn get_data(&self, coords: (i32, i32)) -> u8 {
        self.data[(coords.1 + 2) as usize][(coords.0 + 2) as usize]
    }

    fn set_data(&mut self, coords: (i32, i32),  piece: u8) {
        self.data[(coords.1 + 2) as usize][(coords.0 + 2) as usize] = piece;
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            data: EMPTY_BOARD,
            last_move: None,
            turn: Color::White
        }
    }

    pub fn new_game() -> Board {
        Board {
            data: INITIAL_BOARD,
            last_move: None,
            turn: Color::White
        }
    }

    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn change_turn(&mut self) {
        self.turn = self.turn.inverse();
    }

    pub fn get_piece(&self, coords: (i32, i32)) -> Option<Piece> {
        Piece::from_u8(self.data[(coords.1 + 2) as usize][(coords.0 + 2) as usize])
    }

    pub fn set_piece(&mut self, coords: (i32, i32), piece: &Piece) {
        self.set_data(coords, piece.to_u8());
    }

    pub fn get_last_move(&self) -> &Option<BoardMove> {
        &self.last_move
    }

    pub fn is_in_bounds(&self, coords: (i32, i32)) -> bool {
        self.get_data(coords) != 255u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let board = Board::new_game();
        assert_eq!(board.data, INITIAL_BOARD);
    }
}