pub mod window {
    pub const WINDOW_TITLE: &str = "Crab chess";
    pub const WINDOW_WIDTH: i32 = 600;
    pub const WINDOW_HEIGHT: i32 = 600;
    pub const WINDOW_RESIZABLE: bool = false;
}

pub mod board {
    use macroquad::color::Color;

    pub const WHITE: Color = Color::new(0.941, 0.851, 0.710, 1.0);
    pub const BLACK: Color = Color::new(0.710, 0.533, 0.388, 1.0);

    pub const SQUARE_SIZE: i32 = super::window::WINDOW_WIDTH / 8;
    pub const PIECE_PADDING: i32 = 5;
}
