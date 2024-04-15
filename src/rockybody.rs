use macroquad::prelude::*;
use std::time::{SystemTime, Instant, Duration};
use std::any::Any;
use ::rand::Rng;
use noise::{NoiseFn, Perlin};
use ::rand::distributions::{Distribution, Uniform};

use super::physics::*;
use super::camera::*;
use super::texturetools::*;
use super::mathtools::*;

const WIDTH: u32 = 290;
const HEIGHT: u32 = 290;

pub struct RockyBody {
    xpos: f32,
    ypos: f32,
    xvel: f32,
    yvel: f32,
    mass: u64,
    radius: f32,
    force_vectors: Vec<ForceVector>,
    frames: Vec<Texture2D>,
    frame_idx: usize,
    last_update: Instant,
    last_frame_update: Instant,
}

impl PhysObj for RockyBody {

    fn as_any(&self) -> &dyn Any { self }

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

    fn update(&mut self) {
        let now = Instant::now();
        let elapsed_frame = now.duration_since(self.last_frame_update);
        // update the frame to display 7x per second
        if elapsed_frame >= Duration::from_secs_f32(1. / 7.) {
            self.last_frame_update = now;
            if self.frame_idx < self.frames.len() - 1  {
                self.frame_idx += 1;
            } else {
                self.frame_idx = 0;
            }
        }
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

    fn add_vector(&mut self, force_vec: ForceVector) {
        self.force_vectors.push(force_vec);
    }

    fn force_vectors(&self) -> Vec<ForceVector> {
        self.force_vectors.clone()
    }

    fn draw(
        &mut self, 
        camera: &mut ZCamera,
    ) {
        /*
        * TODO
        * I want to check and see if the bodyis within the bounds
        * of the camera, and only draw it if it is. 
        */
        let draw_x = (self.xpos as f64 - (WIDTH as f64 / 2.) - camera.xpos as f64) * camera.zoom;
        let draw_y = (self.ypos as f64 - (HEIGHT as f64 / 2.) - camera.ypos as f64) * camera.zoom;
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
    }
}

impl RockyBody {
    
    pub  async fn new(
        xpos: f32,
        ypos: f32,
        xvel: f32,
        yvel: f32,
        mass: u64,
        radius: f32,
    ) -> RockyBody {
        let frames = gen_rand_rocky_body_textures(mass, radius).await;
        RockyBody {
            xpos: xpos,
            ypos: ypos,
            xvel,
            yvel,
            frames,
            mass,
            radius,
            force_vectors: Vec::new(),
            frame_idx: 0,
            last_update: Instant::now(),
            last_frame_update: Instant::now(),
        }
    }
}

pub async fn load_rocky_bodies(
    loaded: &mut bool,
    bodies: &mut Vec<Box<dyn PhysObj>>,
    win_width: f32,
    win_height: f32,
    orbit_px: f32,
    orbit_py: f32,
    m: u64
) {
    let num_rocky_bodies = 450;
    for _ in 0..num_rocky_bodies{
        bodies.push(Box::new(gen_random_rocky_body(win_width, win_height, orbit_px, orbit_py, m).await));
    }
    *loaded = true;
}

async fn gen_random_rocky_body(win_width: f32, win_height: f32, ox: f32, oy: f32, sm: u64) -> RockyBody {
    let mut rng = ::rand::thread_rng();
    let mass = rng.gen_range(10000000..10000000000000000);
    let r = r_from_mass(mass as f32, (10000000., 10000000000000000.), (5., 90.));
    let win_width = win_width as i32;
    let win_height = win_height as i32;
    let vel_distribution = Uniform::new(0.0f32, 2.0f32);
    let right_angle = std::f32::consts::PI / 2.;
    let (xpos, ypos) = (
        rng.gen_range(-win_width * 10..(win_width * 2) * 10) as f32, 
        rng.gen_range(-win_width * 10..(win_width * 2) * 10) as f32
    );
    let dx = (xpos - ox).abs();
    let dy = (ypos - oy).abs();
    let d = ((dx * dx) + (dy * dy)).sqrt();
    let t_1 = f32::atan(dx / dy);
    let t_2 = right_angle - t_1;
    let g = 0.000000001;
    let vo = ((g * sm as f32) / d).sqrt();
    let pixel_conversion = 1.;
    let mut x_component = f32::sin(t_2) * vo * pixel_conversion;
    let mut y_component = f32::cos(t_2) * vo * pixel_conversion;
    if ypos < oy {
        x_component = -x_component;
        y_component = -y_component;
    }
    if x_component == 0. {
        if y_component == 0. {
            print!("ERROR: UH OH THEY SPAWNED WITH NO VELOCITY");
        }
    }
    RockyBody::new(
        xpos,
        ypos,
        x_component,
        y_component,
        mass,
        r,
    ).await
}

async fn gen_rand_rocky_body_textures(mass: u64, radius: f32) -> Vec<Texture2D> {
    let mut textures: Vec<Texture2D> = Vec::new();
    textures.push(create_rocky_body(mass, radius).await);
    textures 
}

async fn create_rocky_body(mass: u64, radius: f32) -> Texture2D {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");
    let seed = (
        since_epoch.as_secs() * 1_000_000_000 + since_epoch
            .subsec_nanos() as u64 
    ) as u32;
    let perlin = Perlin::new(seed);
    let clear_color = Color{
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    let cloud_perlin = Perlin::new(seed + 1);
    let (width, height) = (WIDTH as u16, HEIGHT as u16);
    let (cx, cy) = (width as u16 / 2, height as u16 / 2);
    let mut base_img_texture = Image::gen_image_color(width, height, clear_color);
    let mut cloud_layer = Image::gen_image_color(width, height, clear_color);
    // LAND LAYER
    for w in 0..width {
        for h in 0..height {
            let dx = (w - cx);
            let dy = (h - cy);
            let d = (((dx * dx) + (dy * dy)) as f32).sqrt();

            let p = radius + radius * 0.3;
            let v = p - radius;


            if d <= radius {
                let val = perlin.get([w as f64 / 91., h as f64 / 92.]);
                let val = (val + 1.0) / 2.0;
                let mut color: Color = Color {
                    r: 0.,
                    g: 1. - val as f32,
                    b: val as f32,
                    a: 1.,
                };
                base_img_texture.set_pixel(w as u32, h as u32, color);

            }

            if d <= p {
                let q = d - radius;
                let mut cloud_val = cloud_perlin.get([w as f64 / 50., h as f64 / 20.]);
                cloud_val = (cloud_val + 1.) / 2.;
                let cloud_cover: Color = Color {
                    r: 0.7 + 0.3 * cloud_val as f32,
                    g: 0.7 + 0.3 * cloud_val as f32,
                    b: 1.,
                    a: (0.5 * cloud_val as f32) * (1. - q / v),
                };
                cloud_layer.set_pixel(w as u32, h as u32, cloud_cover)
            } 
        }
    }

    base_img_texture.overlay(&cloud_layer);
    Texture2D::from_image(&base_img_texture)
}

