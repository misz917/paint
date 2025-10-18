use minifb::{MouseButton, MouseMode, Window, WindowOptions};
use std::collections::VecDeque;

use crate::{
    canvas::Canvas,
    colors::{BLUE, GREEN, RED, YELLOW},
    common::{CanvasDrawable, xy::XY},
    shapes::{circle::Circle, line::Line, square::Square},
    task::Task,
};

pub struct App {
    window: Window,
    canvas: Canvas, // window requires a 1-dimension array but working with it sucks
    canvas_objects: Vec<Box<dyn CanvasDrawable>>,
    task_queue: VecDeque<Task>,
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
            canvas_objects: Vec::new(),
            task_queue: VecDeque::new(),
        };

        Ok(app)
    }

    pub fn run(mut self) {
        self.setup();
        while self.window.is_open() {
            self.handle_input();
            self.handle_task_queue();
            self.update_screen();
        }
    }

    fn handle_task_queue(&mut self) {
        while let Some(task) = self.task_queue.pop_front() {
            match task {
                Task::RedrawCanvasObjects => self.draw_canvas_objects(),
                Task::SummonCanvasObject(canvas_drawable) => {
                    self.canvas_objects.push(canvas_drawable)
                }
                Task::LeftMouse(xy) => todo!(),
            }
        }
    }

    fn setup(&mut self) {
        let square = Square::new(XY::new(100, 100), XY::new(200, 200), 1, YELLOW);
        self.canvas_objects.push(Box::new(square));
        let line = Line::new(XY::new(500, 500), XY::new(800, 100), 4, BLUE);
        self.canvas_objects.push(Box::new(line));
        let circle = Circle::new(XY::new(200, 400), XY::new(500, 500), 2, RED);
        self.canvas_objects.push(Box::new(circle));
        self.task_queue.push_back(Task::RedrawCanvasObjects);
    }

    fn draw_canvas_objects(&mut self) {
        for object in self.canvas_objects.iter() {
            object.draw(&mut self.canvas);
        }
    }

    fn handle_input(&mut self) {
        let mouse_pos = self
            .window
            .get_mouse_pos(MouseMode::Clamp)
            .unwrap_or((0.0, 0.0));

        let left_pressed = self.window.get_mouse_down(MouseButton::Left);

        if left_pressed {
            let (x, y) = (mouse_pos.0 as usize, mouse_pos.1 as usize);
            let p1 = XY::new(x, y);
            let p2 = XY::new(x + 5, y + 5);
            self.task_queue
                .push_back(Task::SummonCanvasObject(Box::new(Circle::new(
                    p1, p2, 2, GREEN,
                ))));
            self.task_queue.push_back(Task::RedrawCanvasObjects);
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
