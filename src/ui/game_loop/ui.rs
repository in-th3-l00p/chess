use macroquad::ui::{root_ui, widgets};
use macroquad::hash;
use macroquad::math::vec2;
use crate::board::piece::PieceType;
use crate::ui::{constants, perft, GameState};
use crate::ui::perft::PerftState;

pub fn promote_window(state: &mut GameState) {
    // promote window
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

pub fn menu(state: &mut GameState) {
    // menu
    widgets::Window::new(
        hash!(),
        vec2(
            (constants::window::WINDOW_WIDTH - constants::ui::MENU_WIDTH) as f32,
            0.
        ),
        vec2(
            constants::ui::MENU_WIDTH as f32,
            constants::window::WINDOW_HEIGHT as f32
        )
    )
        .label("Menu")
        .titlebar(true)
        .movable(false)
        .ui(&mut *root_ui(), |ui| {
            widgets::Label::new("Perft:").ui(ui);
            ui.same_line(0.);
            if widgets::Button::new("Run").ui(ui) {
                state.perft_state = Option::from(PerftState::new());
            }
        });
}

pub async fn execute(state: &mut GameState) {
    perft::ui::execute(state);
    menu(state);
    promote_window(state);
}