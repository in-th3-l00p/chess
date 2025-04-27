use macroquad::hash;
use macroquad::math::vec2;
use macroquad::ui::{root_ui, widgets};
use crate::ui::{constants, GameState};

pub fn execute(game_state: &mut GameState) {
    let mut cancel = false;

    if let Some(ref mut perft_state) = &mut game_state.perft_state {
        if !perft_state.in_dialog {
            widgets::Window::new(
                hash!(),
                vec2(
                    constants::window::WINDOW_WIDTH as f32 / 2. -
                        constants::ui::PERFT_WINDOW_WIDTH / 2.,
                    constants::window::WINDOW_HEIGHT as f32 / 2. -
                        constants::ui::PERFT_WINDOW_HEIGHT / 2.
                ),
                vec2(
                    constants::ui::PERFT_WINDOW_WIDTH,
                    constants::ui::PERFT_WINDOW_HEIGHT
                )
            )
                .label("Perft")
                .movable(false)
                .ui(&mut *root_ui(), |ui| {
                    cancel = widgets::Button::new("Close").ui(ui);
                });
        }
        if perft_state.in_dialog {
            widgets::Window::new(
                hash!(),
                vec2(
                    constants::window::WINDOW_WIDTH as f32 / 2. -
                        constants::ui::PERFT_INPUT_WINDOW_WIDTH / 2.,
                    constants::window::WINDOW_HEIGHT as f32 / 2. -
                        constants::ui::PERFT_INPUT_WINDOW_HEIGHT / 2.
                ),
                vec2(
                    constants::ui::PERFT_INPUT_WINDOW_WIDTH,
                    constants::ui::PERFT_INPUT_WINDOW_HEIGHT
                )
            )
                .label("Perft")
                .movable(false)
                .ui(&mut *root_ui(), |ui| {
                    widgets::Label::new("FEN:")
                        .ui(ui);
                    widgets::InputText::new(hash!())
                        .ui(ui, &mut perft_state.fen);

                    widgets::Label::new("Depth:")
                        .ui(ui);
                    widgets::InputText::new(hash!())
                        .filter_numbers()
                        .ui(ui, &mut perft_state.depth_string);

                    // margins
                    for _ in 0..2 {
                        ui.separator();
                    }

                    if widgets::Button::new("Start").ui(ui) {
                        perft_state.start_testing()
                    }
                    ui.same_line(0.);
                    cancel = widgets::Button::new("Close").ui(ui);
                });
        }
    }

    if cancel {
        game_state.perft_state = None;
    }
}