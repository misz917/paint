use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use minifb::{Window, WindowOptions};

use crate::{canvas::Canvas, shapes::Shape, ui::UIElement};

pub struct App {
    window: Window,
    canvas: Canvas, // window requires a 1-dimension array but working with it sucks
    ui: Vec<Box<dyn UIElement>>,
    drawn_objects: Vec<Box<dyn Shape>>,
}

impl App {
    pub fn new(
        name: &str,
        width: usize,
        height: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = Window::new(name, width, height, WindowOptions::default())?;
        // let canvas: Vec<u32> = vec![0; width * height];
        // let canvas = Canvas

        todo!()
    }

    pub fn run(mut self) {
        self.setup();
        self.main_loop();
        self.destroy();
    }

    fn setup(&mut self) {}

    fn main_loop(&mut self) {}

    fn destroy(&mut self) {}
}
