use macroquad::prelude::*;
use std::time::Instant;
use ::rand::Rng;
use noise::{NoiseFn, Perlin};
use ::rand::distributions::{Distribution, Uniform};
use super::ships::*;
use super::physics::*;

pub struct Civilization {
    pub energy_output: f64,
    pub dexterity: f64,
    pub strength: f64,
    pub constitution: f64,
    pub active_ships: Vec<usize>,
    pub damage: f64,
    pub size: f64,
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
    ) -> Civilization{
        Civilization {
            energy_output,
            dexterity,
            strength,
            constitution,
            active_ships,
            damage,
            size,
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

        let active_ships = create_num_active_ships(
            ((strength + energy_output) / 2.) * size * 200.,
            bodies
        );
    
        Civilization { 
            energy_output, 
            dexterity,
            strength,
            constitution,
            active_ships,
            damage,
            size
        }
    }

    pub fn ship_ap(&self) -> f64 {
        (self.strength + self.energy_output) / 2.
    } 

}

pub fn create_num_active_ships(
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
