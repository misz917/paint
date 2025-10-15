use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

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
pub mod ui;

fn main() {
    App::new(NAME, WIDTH, HEIGHT).unwrap().run();
}

// fn main() {
//     // Create a buffer to store pixel data (RGB)
//     let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

//     // Create window
//     let mut window = Window::new(
//         "Simple Drawing Program - Press ESC to exit",
//         WIDTH,
//         HEIGHT,
//         WindowOptions::default(),
//     )
//     .unwrap_or_else(|e| {
//         panic!("{}", e);
//     });

//     // Drawing state
//     let mut is_drawing = false;
//     let mut last_mouse_pos = (0.0, 0.0);
//     let brush_color = 0x00FF00; // Green color
//     let brush_size = 5;

//     println!("Drawing Program Started!");
//     println!("- Left click and drag to draw");
//     println!("- Press ESC to exit");
//     println!("- Press SPACE to clear the canvas");

//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         // Clear screen if space is pressed
//         if window.is_key_down(Key::Space) {
//             buffer = vec![0; WIDTH * HEIGHT];
//         }

//         // Get mouse state
//         let mouse_pos = window
//             .get_mouse_pos(MouseMode::Clamp)
//             .unwrap_or((-1.0, -1.0));

//         // Check if left mouse button is pressed
//         let left_pressed = window.get_mouse_down(MouseButton::Left);

//         if left_pressed {
//             if !is_drawing {
//                 // Just started drawing
//                 is_drawing = true;
//                 last_mouse_pos = mouse_pos;
//             }

//             // Draw at current mouse position
//             draw_brush(
//                 &mut buffer,
//                 mouse_pos.0 as i32,
//                 mouse_pos.1 as i32,
//                 brush_size,
//                 brush_color,
//             );

//             // Draw line between last position and current position for smooth drawing
//             if is_drawing {
//                 draw_line(
//                     &mut buffer,
//                     last_mouse_pos.0 as i32,
//                     last_mouse_pos.1 as i32,
//                     mouse_pos.0 as i32,
//                     mouse_pos.1 as i32,
//                     brush_size,
//                     brush_color,
//                 );
//             }

//             last_mouse_pos = mouse_pos;
//         } else {
//             is_drawing = false;
//         }

//         // Update window with buffer
//         window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
//     }
// }

// fn draw_brush(buffer: &mut [u32], x: i32, y: i32, size: i32, color: u32) {
//     let half_size = size / 2;

//     for dy in -half_size..=half_size {
//         for dx in -half_size..=half_size {
//             let px = x + dx;
//             let py = y + dy;

//             if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
//                 let index = (py as usize) * WIDTH + (px as usize);
//                 buffer[index] = color;
//             }
//         }
//     }
// }

// fn draw_line(buffer: &mut [u32], x0: i32, y0: i32, x1: i32, y1: i32, size: i32, color: u32) {
//     // Simple line drawing using Bresenham's algorithm
//     let dx = (x1 - x0).abs();
//     let dy = -(y1 - y0).abs();
//     let sx = if x0 < x1 { 1 } else { -1 };
//     let sy = if y0 < y1 { 1 } else { -1 };
//     let mut err = dx + dy;

//     let mut x = x0;
//     let mut y = y0;

//     while x != x1 || y != y1 {
//         draw_brush(buffer, x, y, size, color);

//         let e2 = 2 * err;
//         if e2 >= dy {
//             err += dy;
//             x += sx;
//         }
//         if e2 <= dx {
//             err += dx;
//             y += sy;
//         }
//     }

//     draw_brush(buffer, x1, y1, size, color);
// }
