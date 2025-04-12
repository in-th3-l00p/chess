mod board;
mod ui;

use crate::ui::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        ui::update().await;
    }
}
