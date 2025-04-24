use crate::move_generation::{generate_piece_moves};
use crate::ui::rendering::{draw_board, draw_eval_bar};
use crate::ui::rendering::utils::draw_board_square;
use crate::ui::{constants, GameState};
use macroquad::color::BLACK;
use macroquad::hash;
use macroquad::input::{
    is_mouse_button_down,
    is_mouse_button_released,
    mouse_position,
    MouseButton
};
use macroquad::math::vec2;
use macroquad::prelude::{clear_background, next_frame, Color};
use macroquad::ui::{root_ui, widgets};
use crate::board::piece::PieceType;

fn get_selected_square() -> (i32, i32) {
    (
        (mouse_position().0 / constants::ui::SQUARE_SIZE as f32).floor() as i32,
        (mouse_position().1 / constants::ui::SQUARE_SIZE as f32).floor() as i32
    )
}

/// used for updating the game logics
pub async fn update(state: &mut GameState) {
    if state.promote_pos.is_some() {
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
            state.board.move_piece(possible_move);
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


pub async fn ui(state: &mut GameState) {
    if let Some(promote_pos) = state.promote_pos {
        widgets::Window::new(
            hash!(),
            vec2(
                constants::window::WINDOW_WIDTH as f32 / 2. -
                    constants::ui::PROMOTE_WINDOW_WIDTH / 2.,
                constants::window::WINDOW_HEIGHT as f32 / 2. -
                    constants::ui::PROMOTE_WINDOW_HEIGHT / 2.
            ),
            vec2(
                constants::ui::PROMOTE_WINDOW_WIDTH,
                constants::ui::PROMOTE_WINDOW_HEIGHT
            )
        )
            .label("Promote")
            .titlebar(true)
            .movable(false)
            .ui(&mut *root_ui(), |ui| {
                let mut piece = state
                    .board
                    .get_piece(promote_pos)
                    .unwrap();
                if widgets::Button::new("Queen").ui(ui) {
                    piece.piece_type = PieceType::Queen;
                    state.board.set_piece(promote_pos, &piece);
                    state.promote_pos = None;
                }
                ui.same_line(0.);
                if widgets::Button::new("Rook").ui(ui) {
                    piece.piece_type = PieceType::Rook { has_moved: true };
                    state.board.set_piece(promote_pos, &piece);
                    state.promote_pos = None;
                }
                ui.same_line(0.);
                if widgets::Button::new("Bishop").ui(ui) {
                    piece.piece_type = PieceType::Bishop;
                    state.board.set_piece(promote_pos, &piece);
                    state.promote_pos = None;
                }
                ui.same_line(0.);
                if widgets::Button::new("Knight").ui(ui) {
                    piece.piece_type = PieceType::Knight;
                    state.board.set_piece(promote_pos, &piece);
                    state.promote_pos = None;
                }
            });
    }
}

/// used for rendering the current state of the game
pub async fn render(state: &GameState) {
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
