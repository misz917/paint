use minifb::{MouseButton, MouseMode, Window, WindowOptions};

use crate::{
    canvas::Canvas,
    colors::{BLUE, GREEN, RED, YELLOW},
    common::{CanvasDrawable, xy::XY},
    pencil::Pencil,
    shapes::{circle::Circle, line::Line, square::Square},
    task::Task,
};

pub struct App {
    window: Window,
    canvas: Canvas, // window requires a 1-dimension array but working with it sucks
    ui_objects: Vec<Box<dyn CanvasDrawable>>,
    drawn_objects: Vec<Box<dyn CanvasDrawable>>,
    task_queue: Vec<Task>,
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
            ui_objects: Vec::new(),
            drawn_objects: Vec::new(),
            task_queue: Vec::new(),
        };

        Ok(app)
    }

    pub fn run(mut self) {
        self.setup();
        while self.window.is_open() {
            self.handle_input();
            self.handle_task_queue();
            self.draw_objects();
            self.update_screen();
        }
    }

    fn handle_task_queue(&mut self) {
        while let Some(task) = self.task_queue.pop() {
            match task {
                _ => {}
            }
        }
    }

    fn setup(&mut self) {
        let square = Square::new(XY::new(100, 100), XY::new(200, 200), 1, YELLOW);
        self.drawn_objects.push(Box::new(square));
        let line = Line::new(XY::new(500, 500), XY::new(800, 100), 4, BLUE);
        self.drawn_objects.push(Box::new(line));
        let circle = Circle::new(XY::new(200, 400), XY::new(500, 500), 2, RED);
        self.drawn_objects.push(Box::new(circle));
    }

    fn draw_objects(&mut self) {
        for object in self.drawn_objects.iter() {
            object.draw(&mut self.canvas);
        }

        for ui_element in self.ui_objects.iter() {
            ui_element.draw(&mut self.canvas);
        }
    }

    fn handle_input(&mut self) {
        // Get mouse state
        let mouse_pos = self
            .window
            .get_mouse_pos(MouseMode::Clamp)
            .unwrap_or((0.0, 0.0));

        // Check if left mouse button is pressed
        let left_pressed = self.window.get_mouse_down(MouseButton::Left);

        let brush_color = GREEN;
        if left_pressed {
            // let mouse_pos_usize: (usize, usize) = (mouse_pos.0 as usize, mouse_pos.1 as usize);
            // self.canvas[mouse_pos_usize] = brush_color;

            let (x, y) = (mouse_pos.0 as usize, mouse_pos.1 as usize);
            Pencil::draw_dot(&mut self.canvas, XY { x, y }, 3, brush_color);
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
