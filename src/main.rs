mod board;
mod ui;
mod move_generation;

use crate::ui::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    ui::run().await;
}
