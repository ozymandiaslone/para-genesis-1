use macroquad::prelude::*;
use std::time::Instant;
use ::rand::Rng;
use std::any::Any;
use noise::{NoiseFn, Perlin};
use ::rand::distributions::{Distribution, Uniform};

use super::ships::*;
use super::physics::*;
use super::camera::*;

pub struct Civilization {
    pub energy_output: f64,
    pub dexterity: f64,
    pub strength: f64,
    pub constitution: f64,
    pub damage: f64,
    pub size: f64,
    pub seed: u32,
    pub density_field: Image,
} 

impl Civilization {
    pub fn new(
        energy_output: f64,
        dexterity: f64,
        strength: f64,
        constitution: f64,
        active_ships: Vec<usize>,
        damage: f64,
        size: f64,
        seed: u32,
        density_field: Image,
    ) -> Civilization{
        Civilization {
            energy_output,
            dexterity,
            strength,
            constitution,
            damage,
            size,
            seed,
            density_field,
        }
    }

    pub fn new_rand( bodies: &mut Vec<Box<dyn PhysObj>> ) -> Civilization {

        let mut rng = ::rand::thread_rng();
        let energy_output = rng.gen_range(0..1000000000) as f64 / 1000000000.;
        let dexterity = rng.gen_range(0..1000000000) as f64 / 1000000000.;
        let strength = rng.gen_range(0..1000000000) as f64 / 1000000000.;
        let constitution = rng.gen_range(0..1000000000) as f64 / 1000000000.;
        let size = rng.gen_range(0..1000000000) as f64 / 1000000000.;
        let damage = 0.;
        let seed = 0;
        let density_field = Image::gen_image_color(1000, 1000, WHITE);
/*
        let active_ships = create_num_active_ships(
            ((strength + energy_output) / 2.) * size * 200.,
            bodies
        );
*/   
        Civilization { 
            energy_output, 
            dexterity,
            strength,
            constitution,
            damage,
            size,
            seed,
            density_field,
        }
    }

    pub fn ship_ap(&self) -> f64 {
        (self.strength + self.energy_output) / 2.
    } 

}

impl PhysObj for Civilization {

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    // We don't really wanna be able to change 
    // or meaningfully access any of
    // these kinds values for a Civlization
    //
    // they don't really conceptually apply
    
    fn xpos(&self) -> f32 { 0. }
    fn ypos(&self) -> f32 { 0. }
    fn xvel(&self) -> f32 { 0. }
    fn yvel(&self) -> f32 { 0. }
    fn mass(&self) -> u64 { 0 }
    fn radius(&self) -> f32 { 0. } 
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
        Vec::new()
    }

    fn update(&mut self) {

    }

    fn draw(&mut self, camera: &mut ZCamera) {
        // TODO draw everything related to the civlization
        let (width, height) = (self.density_field.width(), self.density_field.height());
        // WARNING - This is temp. In the future we will want to fix its center pos to the 
        // actual map center of the solar system. Right now i havent specified anything about the
        // camera
        //
        // maybe im being silly though and forgetting how the camera works tho so ill have to look
        // into it.
        draw_texture_ex(
            &Texture2D::from_image(&self.density_field),
            (screen_width() as f32 / 2.) - (width as f32 / 2.),
            (screen_height() as f32 / 2.) - (height as f32 / 2.),
            WHITE,
            DrawTextureParams {
                ..Default::default()
            }
        )
    }


}

pub fn create_n_active_ships(
    n: f64,
    bodies: &mut Vec<Box<dyn PhysObj>>,

) -> Vec<usize> {
    let mut ships_idx: Vec<usize> = Vec::new();
    let mut rng = ::rand::thread_rng();
    let width = screen_width();
//    let height = screen_height();
    let n = n as usize;
    for _ in 0..n {
        bodies.push(
            Box::new(
                Ship {
                    xpos: rng.gen_range(-width * 2. .. width * 2.),
                    ypos: rng.gen_range(-width * 2. .. width * 2.),
                    desired_x: 0.,
                    desired_y: 0.,
                    xvel: 0.,
                    yvel: 0.,
                    force_vectors: Vec::new(),
                    last_update: Instant::now(),
                    mass: 100000000,
                    radius: 22.,
                    max_dv: 40.,
                    texture: create_ship_texture(22.),
                }
            )
        );
        ships_idx.push(bodies.len() - 1 );
    }
    ships_idx
}

pub fn load_civilization(
    bodies: &mut Vec<Box<dyn PhysObj>>,
) {
    Civilization::new_rand(bodies);
}


pub fn update_ships_desired_pos(
    bodies: &mut Vec<Box<dyn PhysObj>>,
) {
    if let Some(ship) = bodies.first_mut() {
        let (x, y) = (ship.xpos(), ship.ypos());

        for body in bodies.iter_mut().skip(1) {
            if let Some(ship) = body.as_any_mut().downcast_mut::<Ship>() {
                ship.desired_x = x;
                ship.desired_y = y;
            }
        }
    }
}
