mod rendering;
mod logics;
mod constants;

use crate::board::Board;
use macroquad::window::Conf;
use crate::ui::rendering::pieces::Textures;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from(constants::window::WINDOW_TITLE),
        window_width: constants::window::WINDOW_WIDTH,
        window_height: constants::window::WINDOW_HEIGHT,
        window_resizable: constants::window::WINDOW_RESIZABLE,
        ..Default::default()
    }
}

pub async fn run() {
    let mut board = Board::new();
    let textures = Textures::new().await.unwrap();
    loop {
        // logics::update(&mut board).await;
        logics::render(&textures, &board).await;
    }
}