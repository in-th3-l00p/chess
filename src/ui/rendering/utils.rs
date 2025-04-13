use macroquad::prelude::Color;
use macroquad::shapes::draw_rectangle;
use crate::ui::constants;

pub fn draw_board_square((x, y): (usize, usize), color: Color) {
    draw_rectangle(
        (x as i32 * constants::board::SQUARE_SIZE) as f32,
        (y as i32 * constants::board::SQUARE_SIZE) as f32,
        constants::board::SQUARE_SIZE as f32,
        constants::board::SQUARE_SIZE as f32,
        color
    )
}