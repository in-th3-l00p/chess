use macroquad::prelude::Color;
use macroquad::shapes::draw_rectangle;
use crate::ui::constants;

pub fn draw_board_square((x, y): (i32, i32), color: Color) {
    draw_rectangle(
        (x * constants::board::SQUARE_SIZE) as f32,
        (y * constants::board::SQUARE_SIZE) as f32,
        constants::board::SQUARE_SIZE as f32,
        constants::board::SQUARE_SIZE as f32,
        color
    )
}