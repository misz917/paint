use std::{thread, time::Duration};

use minifb::{MouseButton, MouseMode, Window, WindowOptions};

use crate::{canvas::Canvas, common::XY, ui::CanvasDrawable};

const RED: u32 = 0xFF0000;
const GREEN: u32 = 0x00FF00;
const BLUE: u32 = 0x0000FF;

pub struct App {
    window: Window,
    canvas: Canvas, // window requires a 1-dimension array but working with it sucks
    ui: Vec<Box<dyn CanvasDrawable>>,
    drawn_objects: Vec<Box<dyn CanvasDrawable>>,
}

impl App {
    pub fn new(
        name: &str,
        width: usize,
        height: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = Window::new(name, width, height, WindowOptions::default())?;
        let canvas = Canvas::new(width, height);

        let app = App {
            window,
            canvas,
            ui: Vec::new(),
            drawn_objects: Vec::new(),
        };

        Ok(app)
    }

    pub fn run(mut self) {
        self.setup();
        while self.window.is_open() {
            self.handle_input();
            self.update_screen();
        }
    }

    fn setup(&mut self) {
        draw_line(
            &mut self.canvas,
            XY::new(200, 200),
            XY::new(800, 400),
            5,
            RED,
        );
    }

    fn handle_input(&mut self) {
        // Get mouse state
        let mouse_pos = self
            .window
            .get_mouse_pos(MouseMode::Clamp)
            .unwrap_or((0.0, 0.0));

        // Check if left mouse button is pressed
        let left_pressed = self.window.get_mouse_down(MouseButton::Left);

        let brush_color = 0x00FF00;
        if left_pressed {
            // let mouse_pos_usize: (usize, usize) = (mouse_pos.0 as usize, mouse_pos.1 as usize);
            // self.canvas[mouse_pos_usize] = brush_color;

            let (x, y) = (mouse_pos.0 as usize, mouse_pos.1 as usize);
            draw_dot(&mut self.canvas, XY { x, y }, 3, brush_color);
        }
    }

    fn update_screen(&mut self) {
        // place ui above drawings

        self.window
            .update_with_buffer(
                self.canvas.get_buffer(),
                self.canvas.get_x(),
                self.canvas.get_y(),
            )
            .unwrap();
    }
}

pub fn draw_dot(canvas: &mut Canvas, point: XY<usize>, radius: usize, color: u32) {
    let point = XY {
        x: point.x as i32,
        y: point.y as i32,
    };
    let radius = radius as i32;

    for x in -radius..=radius {
        for y in -radius..=radius {
            let distance = ((x.pow(2) + y.pow(2)) as f32).sqrt();
            if distance <= radius as f32 {
                let x = point.x - x;
                let y = point.y - y;
                if x > 0 && y > 0 {
                    canvas[(x as usize, y as usize)] = color;
                }
            }
        }
    }
}

pub fn draw_line(canvas: &mut Canvas, p1: XY<usize>, p2: XY<usize>, radius: usize, color: u32) {
    let p1 = XY {
        x: p1.x as i32,
        y: p1.y as i32,
    };
    let p2 = XY {
        x: p2.x as i32,
        y: p2.y as i32,
    };

    let long_vector = XY::new(p2.x - p1.x, p2.y - p1.y);
    let length = ((long_vector.x.pow(2) + long_vector.y.pow(2)) as f32).sqrt();
    let short_vector = XY::new(long_vector.x as f32 / length, long_vector.y as f32 / length);

    for step in 0..=length as i32 {
        let point = XY::new(
            (step as f32 * short_vector.x + p1.x as f32) as usize,
            (step as f32 * short_vector.y + p1.y as f32) as usize,
        );
        draw_dot(canvas, point, radius, color);
    }
}
