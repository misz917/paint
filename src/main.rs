use crate::{app::App, colors::BLACK};

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
const NAME: &str = "Adobu";

const DEFAULT_CANVAS_COLOR: u32 = BLACK;

pub mod app;
pub mod canvas;
pub mod colors;
pub mod common;
pub mod pencil;
pub mod shapes;
pub mod task;
pub mod ui;

fn main() {
    App::new(NAME, WIDTH, HEIGHT).unwrap().run();
}
