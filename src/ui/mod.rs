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

// keeps all the variables of the game
struct GameState {
    board: Board,
    textures: Textures,

    selected_piece: Option<(usize, usize)>,
    preview_piece: Option<(usize, usize)>,
    possible_moves: Option<Vec<(usize, usize)>>,
}

pub async fn run() {
    let mut state = GameState {
        board: Board::new(),
        textures: Textures::new().await.unwrap(),
        selected_piece: None,
        preview_piece: None,
        possible_moves: None,
    };
    loop {
        logics::update(&mut state).await;
        logics::render(&state).await;
    }
}