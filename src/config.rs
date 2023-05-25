pub mod button {
    pub static WIDTH: f32 = 100.0;
    pub static HEIGHT: f32 = 50.0;
}
pub mod panel {
    use macroquad::color::GRAY;
    use crate::config::window;

    pub static WIDTH: f32 = 300.0;
    pub static HEIGHT: f32 = window::HEIGHT as f32;
    pub static BACKGROUND: macroquad::color::Color = GRAY;
}
pub mod window {
    use macroquad::color::BLACK;

    pub static TITLE: &str = "Title";
    pub static WIDTH: i32 = 1200;
    pub static HEIGHT: i32 = 800;
    pub static BACKGROUND: macroquad::color::Color = BLACK;
}
