use crate::ui::{constants, rendering, GameState};
use macroquad::color::BLACK;
use macroquad::input::{is_mouse_button_down, is_mouse_button_released, mouse_position, MouseButton};
use macroquad::prelude::{clear_background, draw_rectangle, next_frame, Color};
use crate::board::piece::Piece;

fn get_selected_square() -> (usize, usize) {
    (
        (mouse_position().0 / constants::board::SQUARE_SIZE as f32).floor() as usize,
        (mouse_position().1 / constants::board::SQUARE_SIZE as f32).floor() as usize
    )
}

/// used for updating the game logics
/// gets called in the game loop
pub async fn update(state: &mut GameState) {
    if is_mouse_button_down(MouseButton::Left) {
        state.selected_piece = None;
        let selected_square = get_selected_square();
        if state.board.get_piece(selected_square.0, selected_square.1).is_some() {
            state.preview_piece = Some(selected_square);
        }
    }

    if is_mouse_button_released(MouseButton::Left) {
        let selected_square = get_selected_square();
        if state.board.get_piece(selected_square.0, selected_square.1).is_some() {
            state.selected_piece = Some(selected_square);
        }
    }
}

/// used for rendering the current state of the game
/// gets called in the game loop
pub async fn render(state: &GameState) {
    clear_background(BLACK);

    rendering::draw_board(&state.textures, &state.board);
    if state.selected_piece.is_some() || state.preview_piece.is_some() {
        let coords = state
            .selected_piece
            .unwrap_or_else(|| state.preview_piece.unwrap());
        draw_rectangle(
            ((coords.0 as i32) * constants::board::SQUARE_SIZE) as f32,
            ((coords.1 as i32) * constants::board::SQUARE_SIZE) as f32,
            constants::board::SQUARE_SIZE as f32,
            constants::board::SQUARE_SIZE as f32,
            Color::from_rgba(0, 0, 0, 50),
        );

        let piece = state.board.get_piece(coords.0, coords.1).unwrap();
        match piece {
            Piece::Pawn { .. } => {
                let moves = crate::move_generation::generate(&state.board, coords);
                for possible_move in moves {
                    draw_rectangle(
                        ((possible_move.0 as i32) * constants::board::SQUARE_SIZE) as f32,
                        ((possible_move.1 as i32) * constants::board::SQUARE_SIZE) as f32,
                        constants::board::SQUARE_SIZE as f32,
                        constants::board::SQUARE_SIZE as f32,
                        Color::from_rgba(0, 0, 0, 50),
                    );
                }
            },
            _ => {}
        };
    }

    next_frame().await;
}
