use std::time::Instant;
use macroquad::prelude::*;
use macroquad::texture::*;
use super::physics::*;

pub fn intersection(
    a1: f64, b1: f64,
    c1: f64, a2: f64,
    b2: f64, c2: f64
) -> Option<(f64, f64)> {
    let det = a1*b2 - a2*b1;
    if det == 0.0 {
        // The lines are parallel
        None
    } else {
        let x = (b1*c2 - b2*c1) / det;
        let y = (a2*c1 - a1*c2) / det;
        Some((x, y))
    }
}

pub fn line_equation(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64, f64) {
    let a = y2 - y1;
    let b = x1 - x2;
    let c = x2*y1 - x1*y2;
    
    (a, b, c)
}


fn fast_inverse_sqrt(n: f32) -> f32 {
    let i = unsafe { std::mem::transmute::<f32, i32>(n) };
    let j = 0x5f3759df - (i >> 1);
    let y = unsafe { std::mem::transmute::<i32, f32>(j) };
    y * (1.5f32 - 0.5f32 * n * y * y)
}

pub fn fast_root(n :f32) -> f32 {
    1. / fast_inverse_sqrt(n)
}


pub fn r_from_mass(mass: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
    let (from_min, from_max) = from_range;
    let (to_min, to_max) = to_range;

    (mass - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
}



