use std::time::Instant;
use macroquad::prelude::*;
use macroquad::texture::*;
use super::physics::*;
use super::camera::*;
use std::any::Any;

pub fn draw_image_line(x1: u32, y1: u32, x2: u32, y2: u32, thickness: u8, image: &mut Image, color: Color) {

    let (width, height) = (image.width, image.height);

    let (a, b, c) = line_equation(x1 as f64, y1 as f64, x2 as f64, y2 as f64);

    let min_x = std::cmp::min(x1, x2);
    let max_x = std::cmp::max(x1, x2);
    let min_y = std::cmp::min(y1, y2);
    let max_y = std::cmp::max(y1, y2);

    for w in min_x..=max_x {
        for h in min_y..=max_y {

            let mut numerator = (a * w as f64 + b * h as f64 + c).abs();
            let denominator = ((a * a) + (b * b)).sqrt();

            let d = numerator / denominator;

            if d <= thickness as f64 {
                image.set_pixel(w as u32, h as u32, color);
            }
        }
    }
}


fn line_equation(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64, f64) {
    let a = y2 - y1;
    let b = x1 - x2;
    let c = x2*y1 - x1*y2;
    
    (a, b, c)
}
