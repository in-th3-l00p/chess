use macroquad::prelude::{clear_background, next_frame};
use macroquad::color::{Color, BLACK};
use crate::ui::GameState;
use crate::ui::rendering::{draw_board, draw_eval_bar};
use crate::ui::rendering::utils::draw_board_square;

/// used for rendering the current state of the game
pub async fn execute(state: &GameState) {
    clear_background(BLACK);

    draw_eval_bar(&state.board);
    draw_board(&state.textures, &state.board);

    if state.selected_piece.is_some() || state.preview_piece.is_some() {
        let coords = state
            .selected_piece
            .unwrap_or_else(|| state.preview_piece.unwrap());
        draw_board_square(
            coords,
            Color::from_rgba(0, 0, 0, 50)
        );
    }

    if state.possible_moves.is_some() {
        for possible_move in state.possible_moves.as_ref().unwrap() {
            draw_board_square(
                possible_move.to,
                Color::from_rgba(0, 0, 0, 50)
            );
        }
    }

    next_frame().await;
}