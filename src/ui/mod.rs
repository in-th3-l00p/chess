mod rendering;
mod game_loop;
mod constants;

use crate::board::Board;
use macroquad::window::Conf;
use crate::move_generation::Move;
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

    selected_piece: Option<(i32, i32)>,
    preview_piece: Option<(i32, i32)>,
    possible_moves: Option<Vec<Move>>,
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
        game_loop::update(&mut state).await;
        game_loop::ui(&mut state).await;
        game_loop::render(&state).await;
    }
}