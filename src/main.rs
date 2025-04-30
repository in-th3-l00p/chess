mod board;
mod ui;
mod ai;

use crate::ui::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    ui::run().await;
}
