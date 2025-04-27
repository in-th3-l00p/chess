use macroquad::input::{is_mouse_button_down, is_mouse_button_released, mouse_position, MouseButton};
use crate::ai;
use crate::move_generation::generate_piece_moves;
use crate::ui::{constants, GameState};

fn get_selected_square() -> (i32, i32) {
    (
        (mouse_position().0 / constants::ui::SQUARE_SIZE as f32).floor() as i32,
        (mouse_position().1 / constants::ui::SQUARE_SIZE as f32).floor() as i32
    )
}

/// used for updating the game logics
pub async fn execute(state: &mut GameState) {
    if state.promote_pos.is_some() {
        return;
    }

    // ai move
    if let crate::board::color::Color::Black = state.turn {
        if let Some(ai_move) = ai::get_move(&state.board, state.turn) {
            state.board.make_move(ai_move);
            state.turn = state.turn.inverse();
        }
        return;
    }

    if
        state.possible_moves.is_some()  &&
        is_mouse_button_released(MouseButton::Left)
    {
        // moving
        let selected_square = get_selected_square();
        if state.possible_moves
            .as_ref()
            .unwrap()
            .iter()
            .any(|possible_move| {
                possible_move.to == selected_square
            })
        {
            let possible_move =
                state.possible_moves
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|possible_move| {
                        possible_move.to == selected_square
                    })
                    .unwrap()
                    .clone();

            if possible_move.promote.is_some() {
                state.promote_pos = Some(possible_move.to);
            }
            state.board.make_move(possible_move);
            state.turn = state.turn.inverse();
            state.selected_piece = None;
            state.possible_moves = None;
            state.preview_piece = None;
        }
    } else {
        if is_mouse_button_down(MouseButton::Left) {
            let selected_square = get_selected_square();
            if !(
                state.possible_moves.is_some() &&
                state.possible_moves
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|possible_move| {
                        possible_move.to == selected_square
                    })
            ) {
                state.selected_piece = None;
                state.possible_moves = None;

                // pressing on other piece
                if let Some(piece) =  state.board.get_piece(selected_square) {
                    if piece.color == state.turn {
                        state.preview_piece = Some(selected_square);
                    }
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            let selected_square = get_selected_square();
            if let Some(piece) =  state.board.get_piece(selected_square) {
                if piece.color == state.turn {
                    state.possible_moves = Some(
                        generate_piece_moves(&state.board, selected_square)
                    );
                    state.selected_piece = Some(selected_square);
                }
            }
        }
    }
}