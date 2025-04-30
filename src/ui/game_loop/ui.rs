use macroquad::ui::{root_ui, widgets};
use macroquad::hash;
use macroquad::math::vec2;
use crate::board::Board;
use crate::board::pieces::PieceType;
use crate::ui::{constants, perft, GameState};
use crate::ui::perft::PerftState;

pub fn promote_window(state: &mut GameState) {
    // promote window
    if let Some(mut promote_move) = state.promote_move.clone() {
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
                let mut selected = false;
                if widgets::Button::new("Queen").ui(ui) {
                    promote_move.promote = Some(PieceType::Queen);
                    selected = true;
                }
                ui.same_line(0.);
                if widgets::Button::new("Rook").ui(ui) {
                    promote_move.promote = Some(PieceType::Rook { has_moved: true });
                    selected = true;
                }
                ui.same_line(0.);
                if widgets::Button::new("Bishop").ui(ui) {
                    promote_move.promote = Some(PieceType::Bishop);
                    selected = true;
                }
                ui.same_line(0.);
                if widgets::Button::new("Knight").ui(ui) {
                    promote_move.promote = Some(PieceType::Knight);
                    selected = true;
                }

                if selected {
                    state.board.make_move(promote_move);
                    state.board.change_turn();
                    state.selected_piece = None;
                    state.possible_moves = None;
                    state.preview_piece = None;
                    state.promote_move = None;
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

            ui.separator();

            widgets::Label::new("FEN:").ui(ui);
            widgets::InputText::new(hash!())
                .ui(ui, &mut state.fen);
            if widgets::Button::new("Set").ui(ui) {
                if let Ok(new_board) = Board::from_fen(state.fen.as_str()) {
                    state.board = new_board;
                }
            }
        });
}

pub async fn execute(state: &mut GameState) {
    perft::ui::execute(state);
    menu(state);
    promote_window(state);
}