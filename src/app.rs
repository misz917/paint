use minifb::{MouseButton, MouseMode, Window, WindowOptions};

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

    fn setup(&mut self) {}

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
            let mouse_pos_usize: (usize, usize) = (mouse_pos.0 as usize, mouse_pos.1 as usize);
            self.canvas[mouse_pos_usize] = brush_color;
        }
    }

    fn update_screen(&mut self) {
        self.window
            .update_with_buffer(
                self.canvas.get_buffer(),
                self.canvas.get_x(),
                self.canvas.get_y(),
            )
            .unwrap();
    }
}
