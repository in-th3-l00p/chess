use crate::board::Board;
use macroquad::shapes::draw_rectangle;

fn draw_background() {
    for x in 0..8 {
        for y in 0..8 {
            draw_rectangle(
                (x * super::constants::board::SQUARE_SIZE) as f32,
                (y * super::constants::board::SQUARE_SIZE) as f32,
               super::constants::board::SQUARE_SIZE as f32,
               super::constants::board::SQUARE_SIZE as f32,
               if (x + y) % 2 == 0 {
                   super::constants::board::WHITE
               } else {
                   super::constants::board::BLACK
               }
            )
        }
    }
}

pub fn draw_board(board: &Board) {
    draw_background();
}