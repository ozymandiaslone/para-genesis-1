use std::any::Any;
use macroquad::prelude::*;
use std::time::*;

use super::ships::*;
use super::physics::*;
use super::camera::*;


pub struct Player {
    xpos: f32,
    ypos: f32,
    xvel: f32,
    yvel: f32,
    mass: u64,
    radius: f32,
    force_vecs: Vec<ForceVector>,
    last_update: Instant,
    texture: Texture2D,
//    create_ship_texture(passthru_rad),
}

impl Player {
    pub fn new() -> Player {
        let xpos = 0.;
        let ypos = 0.;
        let xvel = 0.;
        let yvel = 0.;
        let mass = 90999999999;
        let radius = 30.;
        let force_vecs = vec![(0., 0.,)];
        Player{
            xpos,
            ypos,
            xvel,
            yvel,
            mass,
            radius,
            force_vecs,
            last_update: Instant::now(),
            texture: create_ship_texture(radius),
        }
    }

    fn update_ship_velocity(&mut self) {
        if is_key_down(KeyCode::W) {
            self.update_yvel(-45.);
        };
        if is_key_down(KeyCode::A) {
            self.update_xvel(-45.);
        };
        if is_key_down(KeyCode::S) {
            self.update_yvel(55.);
        };
        if is_key_down(KeyCode::D) {
            self.update_xvel(45.);
        };
        if is_key_down(KeyCode::Space) {
            self.update_xvel(-self.xvel());
            self.update_yvel(-self.yvel());
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
            last_update: self.last_update,
            texture: self.texture.clone(),
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
        
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);

        
        let mut final_vector: ForceVector = (0., 0.);
        //assume this self as a force_vectors full of vectors
        for i in 0..self.force_vecs.len() {
            final_vector.0 += self.force_vecs[i].0;
            final_vector.1 += self.force_vecs[i].1;
        }
        self.force_vecs = Vec::new();

        let ax = final_vector.0 / self.mass as f32;
        let ay = final_vector.1 / self.mass as f32;
        
        // v = at
        self.xvel += ax * elapsed.as_secs_f32(); 
        self.yvel += ay * elapsed.as_secs_f32();

        self.xpos += self.xvel * elapsed.as_secs_f32();
        self.ypos += self.yvel * elapsed.as_secs_f32();
        
        self.last_update = now;
        
        self.update_ship_velocity();
    }

    fn draw(&mut self, camera: &ZCamera) {

        let draw_x = (self.xpos as f64 - camera.xpos as f64) * camera.zoom - 150. * camera.zoom;
        let draw_y = (self.ypos as f64 - camera.ypos as f64) * camera.zoom - 150. * camera.zoom;
        
        draw_texture_ex(
            &self.texture,
            draw_x as f32,
            draw_y as f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(300. * camera.zoom as f32, 300. as f32 * camera.zoom as f32)),
                ..Default::default()
            }
        );
        

    }
}

pub fn follow_ship(body: &mut Box<dyn PhysObj>, camera: &mut ZCamera) {
    let lerp_factor = 0.6; // Adjust lerp_factor as needed for smooth transitions

    // Get half the screen width and height in pixels
    let half_width_pixels = screen_width() / 2.0;
    let half_height_pixels = screen_height() / 2.0;


    camera.xpos = -(half_width_pixels / camera.zoom as f32) + body.xpos();
    camera.ypos = -(half_height_pixels / camera.zoom as f32) + body.ypos(); 

    

}


