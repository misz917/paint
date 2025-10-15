use crate::{canvas::Canvas, common::XY};

pub struct Pencil;

impl Pencil {
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
            Self::draw_dot(canvas, point, radius, color);
        }
    }
}
