use std::any::Any;
use macroquad::prelude::*;

use super::ships::*;
use super::physics::*;
use super::camera::*;


pub struct Player {
    pub ship: PlayerShip, 
    xpos: f32,
    ypos: f32,
    xvel: f32,
    yvel: f32,
    mass: u64,
    radius: f32,
    force_vecs: Vec<ForceVector>,
}

impl Player {
    pub fn new() -> Player {
        let xpos = 0.;
        let ypos = 0.;
        let xvel = 0.;
        let yvel = 0.;
        let mass = 90999999999;
        let radius = 14.;
        let force_vecs = vec![(0., 0.,)];
        let ship = PlayerShip::new(
            xpos,
            ypos,
            xvel,
            yvel,
            mass,
            radius,
        );
        Player{
            ship,
            xpos,
            ypos,
            xvel,
            yvel,
            mass,
            radius,
            force_vecs,
        }
    }
    pub fn clone(&self) -> Player {
        Player {
            xpos: self.xpos,
            ypos: self.ypos,
            xvel: self.xvel,
            yvel: self.yvel,
            mass: self.mass,
            radius: self.radius,
            force_vecs: self.force_vecs.clone(),
            ship: self.ship.clone(),
        }
    }
}

impl PhysObj for Player {
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
        self.xvel += update_val;
    }

    fn update_yvel(&mut self, update_val: f32) {
        self.yvel += update_val;
    }
    
    fn update_xpos(&mut self, update_val: f32) {
        self.xpos += update_val;
    }

    fn update_ypos(&mut self, update_val: f32) {
        self.ypos += update_val;
    }
    fn force_vectors(&self) -> Vec<ForceVector> {
        self.force_vecs.clone()  
    }

    fn add_vector(&mut self, force_vec: ForceVector) {
        self.force_vecs.push(force_vec);
        
    }

    fn update(&mut self) {
        update_ship_velocity(&mut self.ship);
    }

    fn draw(&mut self, camera: &ZCamera) {
        self.ship.draw(camera);
    }
}

fn update_ship_velocity(mut player_ship: &mut PlayerShip) {
    if is_key_down(KeyCode::W) {
        player_ship.update_yvel(-45.);
    };
    if is_key_down(KeyCode::A) {
        player_ship.update_xvel(-45.);
    };
    if is_key_down(KeyCode::S) {
        player_ship.update_yvel(55.);
    };
    if is_key_down(KeyCode::D) {
        player_ship.update_xvel(45.);
    };
    if is_key_down(KeyCode::Space) {
        player_ship.update_xvel(-player_ship.xvel());
        player_ship.update_yvel(-player_ship.yvel());
    }
}

fn lerp(body: &mut Box<dyn PhysObj>, camera: &mut ZCamera) {
    let lerp_factor = 0.6; // Adjust lerp_factor as needed for smooth transitions

    // Get half the screen width and height in pixels
    let half_width_pixels = screen_width() / 2.0;
    let half_height_pixels = screen_height() / 2.0;


    camera.xpos = -(half_width_pixels / camera.zoom as f32) + body.xpos();
    camera.ypos = -(half_height_pixels / camera.zoom as f32) + body.ypos(); 

    

}


