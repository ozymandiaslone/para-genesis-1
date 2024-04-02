use std::time::Instant;
use macroquad::prelude::*;
use macroquad::texture::*;
use super::physics::*;
use super::camera::*;
use super::mathtools::*;
use std::any::Any;


pub fn draw_image_line(x1: u32, y1: u32, x2: u32, y2: u32, thickness: u8, image: &mut Image, color: Color) {

    let (width, height) = (image.width, image.height);

    let (a, b, c) = line_equation(x1 as f64, y1 as f64, x2 as f64, y2 as f64);

    let min_x = std::cmp::min(x1, x2);
    let max_x = std::cmp::max(x1, x2);
    let min_y = std::cmp::min(y1, y2);
    let max_y = std::cmp::max(y1, y2);
    
    // vertical line case
    if x1 == x2 {
        for w in (x1 - (thickness as u32 / 2))..=(x1 + (thickness as u32 / 2)) {
            for h in min_y..=max_y {
                image.set_pixel(w as u32, h as u32, color)
            }
        }
    }
    // horozontal line case
    if y1 == y2 {
        for w in min_x..=max_x {
            for h in (y1 - (thickness as u32 / 2))..=(y1 + (thickness as u32 / 2)) {
                image.set_pixel(w as u32, h as u32, color)
            }
        }
    }

    for w in min_x..=max_x {
        for h in min_y..=max_y {

            let mut numerator = (a * w as f64 + b * h as f64 + c).abs();
            let denominator = ((a * a) + (b * b)).sqrt();

            let d = numerator / denominator;

            if d <= thickness as f64 + 0.1 {
                image.set_pixel(w as u32, h as u32, color);
            }
        }
    }
}


