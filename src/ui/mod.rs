use macroquad::color::BLACK;
use macroquad::window::{clear_background, next_frame, Conf};

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Crab chess".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

pub async fn update() {
    clear_background(BLACK);

    next_frame().await;
}