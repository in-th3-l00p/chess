use crate::move_generation::generate_possible_moves;
use crate::ui::rendering::draw_board;
use crate::ui::rendering::utils::draw_board_square;
use crate::ui::{constants, GameState};
use macroquad::color::BLACK;
use macroquad::hash;
use macroquad::input::{is_mouse_button_down, is_mouse_button_released, mouse_position, MouseButton};
use macroquad::math::vec2;
use macroquad::prelude::{clear_background, next_frame, Color};
use macroquad::ui::{root_ui, widgets};

fn get_selected_square() -> (i32, i32) {
    (
        (mouse_position().0 / constants::board::SQUARE_SIZE as f32).floor() as i32,
        (mouse_position().1 / constants::board::SQUARE_SIZE as f32).floor() as i32
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
                if state.board.get_piece(selected_square).is_some() {
                    state.preview_piece = Some(selected_square);
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            let selected_square = get_selected_square();
            state.possible_moves = Some(generate_possible_moves(&state.board, selected_square));
            if state.board.get_piece(selected_square).is_some() {
                state.selected_piece = Some(selected_square);
            }
        }
    }
}

const PROMOTE_WINDOW_WIDTH: f32 = 178.;
const PROMOTE_WINDOW_HEIGHT: f32 = 40.;

pub async fn ui(state: &mut GameState) {
    if state.promote_pos.is_some() {
        widgets::Window::new(
            hash!(),
            vec2(
                constants::window::WINDOW_WIDTH as f32 / 2. - PROMOTE_WINDOW_WIDTH / 2.,
                constants::window::WINDOW_HEIGHT as f32 / 2. - PROMOTE_WINDOW_HEIGHT / 2.
            ),
            vec2(PROMOTE_WINDOW_WIDTH, PROMOTE_WINDOW_HEIGHT)
        )
            .label("Promote")
            .titlebar(true)
            .movable(false)
            .ui(&mut *root_ui(), |ui| {
                widgets::Button::new("Queen")
                    .ui(ui);
                ui.same_line(0.);
                widgets::Button::new("Rook")
                    .ui(ui);
                ui.same_line(0.);
                widgets::Button::new("Bishop")
                    .ui(ui);
                ui.same_line(0.);
                widgets::Button::new("Knight")
                    .ui(ui);
            });
    }
}

/// used for rendering the current state of the game
pub async fn render(state: &GameState) {
    clear_background(BLACK);

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
