pub mod window {
    use crate::ui::constants::ui::{BOARD_HEIGHT, BOARD_WIDTH, EVAL_BAR_WIDTH};

    pub const WINDOW_TITLE: &str = "Crab chess";
    pub const WINDOW_WIDTH: i32 = BOARD_WIDTH + EVAL_BAR_WIDTH;
    pub const WINDOW_HEIGHT: i32 = BOARD_HEIGHT;
    pub const WINDOW_RESIZABLE: bool = false;
}

pub mod ui {
    use macroquad::color::Color;

    pub const WHITE: Color = Color::new(0.941, 0.851, 0.710, 1.0);
    pub const BLACK: Color = Color::new(0.710, 0.533, 0.388, 1.0);

    pub const BOARD_WIDTH: i32 = 600;
    pub const BOARD_HEIGHT: i32 = 600;

    pub const SQUARE_SIZE: i32 = BOARD_WIDTH / 8;
    pub const PIECE_PADDING: i32 = 5;

    pub const PROMOTE_WINDOW_WIDTH: f32 = 178.;
    pub const PROMOTE_WINDOW_HEIGHT: f32 = 40.;

    pub const EVAL_BAR_WIDTH: i32 = 20;
    pub const BOARD_RENDER_OFFSET: (i32, i32) = (EVAL_BAR_WIDTH, 0);
}
