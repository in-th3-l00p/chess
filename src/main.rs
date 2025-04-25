mod board;
mod ui;
mod move_generation;
mod ai;

use crate::ui::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    ui::run().await;
}
