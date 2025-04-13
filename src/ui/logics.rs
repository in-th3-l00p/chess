use crate::ui::{constants, rendering, GameState};
use macroquad::color::BLACK;
use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};
use macroquad::prelude::{clear_background, next_frame};

/// used for updating the game logics
/// gets called in the game loop
pub async fn update(state: &mut GameState) {
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
pub async fn render(state: &GameState) {
    clear_background(BLACK);

    rendering::draw_board(&state.textures, &state.board);

    next_frame().await;
}
