use macroquad::color::BLACK;
use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};
use macroquad::prelude::{clear_background, next_frame};
use crate::board::Board;
use crate::ui::{constants, rendering};
use crate::ui::rendering::pieces::Textures;

/// used for updating the game logics
/// gets called in the game loop
pub async fn update(board: &mut Board) {
    if is_mouse_button_down(MouseButton::Left) {
        let (x, y) = (
            (mouse_position().0 / constants::board::SQUARE_SIZE as f32).floor() as usize,
            (mouse_position().1 / constants::board::SQUARE_SIZE as f32).floor() as usize
        );

        println!("{} {}", x, y);
    }
}

/// used for rendering the current state of the game
/// gets called in the game loop
pub async fn render(textures: &Textures, board: &Board) {
    clear_background(BLACK);

    rendering::draw_board(textures, board);

    next_frame().await;
}
