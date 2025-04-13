use crate::ui::{constants, rendering, GameState};
use macroquad::color::BLACK;
use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};
use macroquad::prelude::{clear_background, draw_rectangle, next_frame, Color};

/// used for updating the game logics
/// gets called in the game loop
pub async fn update(state: &mut GameState) {
    if is_mouse_button_down(MouseButton::Left) {
        state.selected_piece = Some((
            (mouse_position().0 / constants::board::SQUARE_SIZE as f32).floor() as usize,
            (mouse_position().1 / constants::board::SQUARE_SIZE as f32).floor() as usize
        ));
    } else {
        state.selected_piece = None;
    }
}

/// used for rendering the current state of the game
/// gets called in the game loop
pub async fn render(state: &GameState) {
    clear_background(BLACK);

    rendering::draw_board(&state.textures, &state.board);
    if let Some(coords) = state.selected_piece {
        draw_rectangle(
            ((coords.0 as i32) * constants::board::SQUARE_SIZE) as f32,
            ((coords.1 as i32) * constants::board::SQUARE_SIZE) as f32,
            constants::board::SQUARE_SIZE as f32,
            constants::board::SQUARE_SIZE as f32,
            Color::from_rgba(255, 0, 0, 255),
        );
    }

    next_frame().await;
}
