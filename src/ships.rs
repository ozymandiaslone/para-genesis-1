use std::time::Instant;
use macroquad::prelude::*;
use macroquad::texture::*;
use super::physics::*;
use super::camera::*;
use std::any::Any;

pub struct PlayerShip {
    pub xpos: f32,
    pub ypos: f32,
    pub xvel: f32,
    pub yvel: f32,
    pub mass: u64,
    pub radius: f32,
    pub force_vectors: Vec<ForceVector>,
    //frames: Vec<Texture2D>,
    //frame_idx: usize,
    pub last_update: Instant,
    pub texture: Texture2D,
}

impl PhysObj for PlayerShip {

    fn as_any(&self) -> &dyn Any {
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

    fn add_vector(&mut self, force_vec: ForceVector) {
        self.force_vectors.push(force_vec);
    }
    fn force_vectors(&self) -> Vec<ForceVector> {
        self.force_vectors.clone()
    }

    fn update(
        &mut self,
    ) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        
        let mut final_vector: ForceVector = (0., 0.);
        //assume this self as a force_vectors full of vectors
        for i in 0..self.force_vectors.len() {
            final_vector.0 += self.force_vectors[i].0;
            final_vector.1 += self.force_vectors[i].1;
        }
        self.force_vectors = Vec::new();

        let ax = final_vector.0 / self.mass as f32;
        let ay = final_vector.1 / self.mass as f32;
        
        // v = at
        self.xvel += ax * elapsed.as_secs_f32(); 
        self.yvel += ay * elapsed.as_secs_f32();

        self.xpos += self.xvel * elapsed.as_secs_f32();
        self.ypos += self.yvel * elapsed.as_secs_f32();
        
        self.last_update = now;
    }
fn draw(
        &mut self,
        camera: &mut ZCamera, 
    ) {
        //let (tex_x, tex_y) = (self.frames[self.frame_idx].width(), self.frames[self.frame_idx].height());
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
        /*
        draw_hexagon(
            draw_x as f32,
            draw_y as f32,
            self.radius * camera.zoom as f32,
            0.,
            true,
            WHITE,
            GRAY,
        )
        //draw_texture(&self.frames[self.frame_idx], draw_x as f32 - (tex_x as f32 * scl_x as f32 / 2.), draw_y as f32 - (tex_y as f32 * scl_y as f32 / 2.), WHITE);
        draw_texture_ex(
            &self.frames[self.frame_idx],
            draw_x as f32,
            draw_y as f32,
            WHITE,
            DrawTextureParams {
                //dest_size: Some(vec2(screen_width() * camera.zoom as f32, screen_height() * camera.zoom as f32)),
                dest_size: Some(vec2(WIDTH as f32 * camera.zoom as f32, HEIGHT as f32 * camera.zoom as f32)),
                ..Default::default()
            }
        )
        */
    }
}

impl PlayerShip {
    pub fn new (
        xpos: f32,
        ypos: f32,
        xvel: f32,
        yvel: f32,
        mass: u64,
        radius: f32,
    ) -> PlayerShip {
        let passthru_rad = radius as f32;
        PlayerShip {
            xpos,
            ypos,
            xvel,
            yvel,
            mass,
            radius,
            force_vectors: Vec::new(),
            last_update: Instant::now(),
            texture: create_ship_texture(passthru_rad),
        }
    }
}

pub fn create_ship_texture(radius: f32) -> Texture2D {
    // Step 1: create a macroquad::texture::Image
    // Step 2: turn that in to a Texture2D
    // step 3: return that shit and profit
    //
    

    let clear_color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };

    let (width, height) = (300, 300);

    let (cx, cy) = (width as u16 / 2, height as u16 / 2);

    let mut base_img_texture = Image::gen_image_color(width, height, clear_color);
    
    for w in 0..width {
        for h in 0..height {
            let (dx, dy) = (cx - w, cy - h);
            let d = ((dx * dx) as f32 + (dy * dy) as f32).sqrt();
            if d <= radius {
                base_img_texture.set_pixel(w as u32, h as u32, WHITE);
            }
        }
    };

    Texture2D::from_image(&base_img_texture)
}
