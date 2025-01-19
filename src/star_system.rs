use macroquad::prelude::*;
use std::time::{SystemTime, Instant, Duration};
use std::any::Any;
use ::rand::Rng;
use noise::{NoiseFn, Perlin};
use ::rand::distributions::{Distribution, Uniform};

use super::physics::*;
use super::star::*;
use super::rockybody::*;
use super::camera::*;
use super::player::*;
use super::mathtools::*;
use super::life::*;
use super::ships::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;


pub struct StarSystem {
    pub bodies: Vec<Box<dyn PhysObj>>,
    pub life: Civilization,
    pub xpos: f32,
    pub ypos: f32,
    pub xvel: f32,
    pub yvel: f32,
    pub mass: u64,
    pub radius: f32,
    pub camera: ZCamera,
    pub force_vectors: Vec<ForceVector>,
}

impl StarSystem {
    pub async fn new_rand(player: Player) -> StarSystem {
        let mut loaded = false;
        let mut fs = false;

       
        let mut bodies: Vec<Box< dyn PhysObj>> = Vec::new();
        bodies.push(Box::new(player));
        let mut life: Civilization = load_civilization();
        let cam = ZCamera::new_origin();

        let (ox, oy, m) = load_stars(
            &mut bodies,
            screen_width() as f32,
            screen_height() as f32
        ).await;

        load_rocky_bodies(
            &mut loaded,
            &mut bodies,
            screen_width() as f32,
            screen_height() as f32,
            ox,
            oy,
            m
        ).await;

        StarSystem {
            bodies,
            life,
            mass: 0,
            force_vectors: Vec::new(),
            radius: 0.,
            xpos: 0.,
            ypos: 0.,
            xvel: 0.,
            yvel: 0.,
            camera: cam,
        }
    }

    // Once there are multiple 'versions' of the player
    // when there are multiple star systems,
    // we should set the overall games player to equal
    // the most recently updated player in all the star systems
    // then, we make all those other star system players equal
    // to the game's player
    //
    // Okay im from over a year in the future and past me
    // was really on one when making this code base
    //
    // I think in the modern day i would just hide 
    // everything behind traits and RefCell Boxes & stuff.
    pub fn sync_player(player: &mut Player) {
         
    }
}

impl PhysObj for StarSystem {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn xpos(&self) -> f32 { self.xpos }
    fn ypos(&self) -> f32 { self.ypos }
    fn xvel(&self) -> f32 { self.xvel }
    fn yvel(&self) -> f32 { self.yvel }
    fn mass(&self) -> u64 { self.mass }
    fn radius(&self) -> f32 { self.radius } 
    fn update_xvel(&mut self, update_val: f32) {
    }
    fn update_yvel(&mut self, update_val: f32) {
    }
    fn update_xpos(&mut self, update_val: f32) {
    }
    fn update_ypos(&mut self, update_val: f32) {
    }
    fn add_vector(&mut self, force_vec: ForceVector) {
    }
    fn force_vectors(&self) -> Vec<ForceVector> {
        vec![(0., 0.)]
    }
    fn update(&mut self){
        for body in self.bodies.iter_mut() {
            body.update();
        }
        if let Some(ship) = self.bodies.first_mut() {
            follow_ship(ship, &mut self.camera);
        }
             
    }
    fn draw(&mut self, dummy_cam: &ZCamera) {
        for body in self.bodies.iter_mut() {
            body.draw(&self.camera);
        }
    }
}
 
