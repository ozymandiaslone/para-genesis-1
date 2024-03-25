use macroquad::prelude::*;
use std::time::{SystemTime, Instant, Duration};
use std::any::Any;
use ::rand::Rng;
use noise::{NoiseFn, Perlin};
use ::rand::distributions::{Distribution, Uniform};

use super::physics::*;
use super::camera::*;

const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;


