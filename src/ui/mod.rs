mod rendering;
mod game_loop;
mod constants;
mod perft;

use crate::board::Board;
use crate::move_generation::BoardMove;
use crate::ui::rendering::pieces::Textures;
use macroquad::window::Conf;
use perft::PerftState;
use crate::board::color::Color;

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
    turn: Color,
    textures: Textures,

    promote_pos: Option<(i32, i32)>,
    selected_piece: Option<(i32, i32)>,
    preview_piece: Option<(i32, i32)>,
    possible_moves: Option<Vec<BoardMove>>,

    perft_state: Option<PerftState>,
}

pub async fn run() {
    let mut state = GameState {
        board: Board::new(),
        turn: Color::White,
        textures: Textures::new().await.unwrap(),
        promote_pos: None,
        selected_piece: None,
        preview_piece: None,
        possible_moves: None,
        perft_state: None,
    };
    loop {
        game_loop::update::execute(&mut state).await;
        game_loop::ui::execute(&mut state).await;
        game_loop::render::execute(&state).await;
    }
}