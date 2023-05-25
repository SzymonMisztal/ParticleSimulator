mod config;

use macroquad::prelude::*;
use config::{window, panel, button};

fn window_conf() -> Conf {
    Conf {
        window_title: window::TITLE.to_string(),
        window_width: window::WIDTH,
        window_height: window::HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let panel = Rect::new(
        0.0,
        0.0,
        panel::WIDTH,
        panel::HEIGHT
    );

    loop {
        clear_background(window::BACKGROUND);

        draw_rectangle_
        draw_rectangle_rec(panel, panel::BACKGROUND);

        next_frame().await
    }
}
