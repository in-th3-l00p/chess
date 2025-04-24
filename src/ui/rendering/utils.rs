use macroquad::prelude::Color;
use macroquad::shapes::draw_rectangle;
use crate::ui::constants;

pub fn draw_board_square((x, y): (i32, i32), color: Color) {
    draw_rectangle(
        (constants::ui::BOARD_RENDER_OFFSET.0 + x * constants::ui::SQUARE_SIZE) as f32,
        (constants::ui::BOARD_RENDER_OFFSET.1 + y * constants::ui::SQUARE_SIZE) as f32,
        constants::ui::SQUARE_SIZE as f32,
        constants::ui::SQUARE_SIZE as f32,
        color
    )
}