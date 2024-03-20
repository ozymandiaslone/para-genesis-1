use macroquad::prelude::*;
use std::time::{SystemTime, Instant, Duration};
// This is all just to create the images we load into textures.
// There has gotta be a better way :(
use image::{ImageBuffer, Rgba, ImageOutputFormat};
use std::io::Cursor;
use std::fs::File;
use std::io::Write;
use std::any::Any;
use ::rand::Rng;
use noise::{NoiseFn, Perlin};
use ::rand::distributions::{Distribution, Uniform};

use super::physics::*;
use super::camera::*;

const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;

pub struct Star {
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

impl PhysObj for Star {

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
        let (tex_x, tex_y) = (self.frames[self.frame_idx].width(), self.frames[self.frame_idx].height());
        
        /*
        ctx.draw(
            surface, 
            &self.frames[self.frame_idx], 
            (self.xpos as i32 - (tex_x as i32 * scl_x as i32 / 2), self.ypos as i32 - (tex_y as i32 * scl_y as i32 / 2)), 
            //(self.xpos as i32, self.ypos as i32),
            &draw_config,
        );
        */
        let draw_x = (self.xpos as f64 - 150. - camera.xpos as f64) * camera.zoom;
        let draw_y = (self.ypos as f64 - 150. - camera.ypos as f64) * camera.zoom;
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
    //fn set_xvel(&mut self, xvel: f32) { self.xvel = xvel }
    //fn set_yvel(&mut self, yvel: f32) { self.yvel = yvel }
}

impl Star {

    pub  async fn new(
        xpos: f32,
        ypos: f32,
        xvel: f32,
        yvel: f32,
        mass: u64,
        radius: f32,
        //ctx: &mut Context 
    ) -> Star {
        let frames = gen_rand_star_textures(radius).await;
        Star {
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

pub async fn gen_rand_star_textures(radius: f32) -> Vec<Texture2D> {
    let n_frames = 10;
    let mut textures: Vec<Texture2D> = Vec::new();
    let star_temp = ::rand::thread_rng().gen_range(4000..11000);

    for _ in 0..n_frames {
        textures.push(create_star_texture(radius, star_temp as f32));
    }
    textures
}

pub fn create_star_texture(r: f32, temp: f32) -> Texture2D {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");
    let seed = (
        since_epoch.as_secs() * 1_000_000_000 + since_epoch
            .subsec_nanos() as u64 
    ) as u32;
    let clear_color = Color{
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };

    let (width, height) = (300, 300);

    let (cx, cy) = (width as u16 / 2, height as u16 / 2);

    let mut base_img_texture = Image::gen_image_color(width, height, clear_color);

    let perlin = Perlin::new(seed);
    let base_color = temp_to_color(temp);
    let (red, green, blue) = base_color;
    let p = r + r * 0.3;
    let v = p - r;

    for w in 0..width {
        for h in 0..height {

            let (dx, dy) = (cx - w, cy - h);
            let d = ((dx * dx) as f32 + (dy * dy) as f32).sqrt();

            //if this pixel is inside the radius of the circle...
            if d <= r {
                let noise_val = perlin.get([w as f64 / 2., h as f64 / 2.]);


                let mod_red = (red as f64 * (noise_val + 1.4) / 2.) as f32;
                let mod_green = (green as f64 * (noise_val + 1.4) / 2.) as f32;
                let mod_blue = (blue as f64 * (noise_val + 2.3) / 2.) as f32;
                base_img_texture.set_pixel(w as u32, h as u32, Color{r: mod_red, g: mod_green, b: mod_blue, a: 1.});

            // if this pixel is outside the radius of the circle, but inside the radius p
            // (p defined above. p~r )
            } else if d <= p {
                let q = d - r;
                let noise_val = perlin.get([w as f64 / 2., h as f64 / 2.]);
                let alpha = (0.75 * (1. - q / v)) as f32;
                let mod_red = (red as f64 * (noise_val + 1.4) / 2.) as f32;
                let mod_green = (green as f64 * (noise_val + 1.4) / 2.) as f32;
                let mod_blue = (blue as f64 * (noise_val + 2.3) / 2.) as f32;
                base_img_texture.set_pixel(w as u32, h as u32, Color{r: mod_red, g: mod_green, b: mod_blue, a: alpha});
            }
        }
    };

    Texture2D::from_image(&base_img_texture)
}

// Some really messed up stuff
// but it works, i guess
// and i don't want to spend more time
// thinking about star colors
fn temp_to_color(temp: f32) -> (f32, f32, f32) {
    // convert temperature to celsius (for no reason)
    let temp_celsius = temp - 273.15;

    // define temperature ranges for RGB channels
    let red_range = (1500.0, 3500.0);
    let green_range = (4000.0, 7000.0);
    let blue_range = (7500.0, 12000.0);
 
    // calculate normalized values for each channel
    let red = (temp_celsius - red_range.0) / (red_range.1 - red_range.0);
    let green = (temp_celsius - green_range.0) / (green_range.1 - green_range.0);
    let blue = (temp_celsius - blue_range.0) / (blue_range.1 - blue_range.0);

    // clamp values to the range [0, 1]
    let red = red.max(0.0).min(1.0);
    let green = green.max(0.0).min(1.0);
    let blue = blue.max(0.0).min(1.0);

    (red, green, blue)
}

fn fast_inverse_sqrt(n: f32) -> f32 {
    let i = unsafe { std::mem::transmute::<f32, i32>(n) };
    let j = 0x5f3759df - (i >> 1);
    let y = unsafe { std::mem::transmute::<i32, f32>(j) };
    y * (1.5f32 - 0.5f32 * n * y * y)
}

fn fast_root(n :f32) -> f32 {
    1. / fast_inverse_sqrt(n)
}

async fn initialize_rand_star(win_width: f32, win_height: f32) -> Star {
    let win_width = win_width as i32;
    let win_height = win_height as i32;
    let vel_distribution = Uniform::new(0.0f32, 2.0f32);
    let mut rng = ::rand::thread_rng();
    let mass = rng.gen_range(10000..1000000000000);
    let r = r_from_mass(mass as f32, (10000., 1000000000000.), (5., 20.)) / 2.;
    Star::new(
        rng.gen_range(-win_width..win_width * 2) as f32,
        rng.gen_range(-win_height..win_height * 2) as f32,
        (vel_distribution.sample(&mut rng) - 1.) * 15.,
        (vel_distribution.sample(&mut rng) - 1.) * 15.,
        mass,
        r,
    ).await
}

async fn initialize_particle(win_width: i32, win_height: i32) -> Star {
    let mut rng = ::rand::thread_rng();
    Star::new(
        rng.gen_range(0..win_width) as f32,
        rng.gen_range(0..win_height) as f32,
        0.,
        0.,
        1000000,
        1.,
    ).await
}
pub async fn load_stars(
    loaded: &mut bool,
    stars: &mut Vec<Box<dyn PhysObj>>,
    win_width: f32,
    win_height: f32
) {
    let desired_stars = 500;
    /*
    stars.push(
        Box::new(
            Star::new(
                (win_width / 2.) + 400.,
                win_height / 2., -15., -20.,
                9999999999999,
                55.)
            .await
        )
    );
    stars.push(
        Box::new(
            Star::new(
                (win_width / 2.) - 400.,
                win_height / 2.,
                15.,
                20.,
                99999999999999,
                55.)
            .await
        )
    );
    */
    stars.push(
        Box::new(
            Star::new(
                win_width / 2.,
                win_height / 2.,
                0.,
                0.,
                999999999999999,
                55.)
            .await
        )
    );

    for _ in 0..desired_stars {
        stars.push(Box::new(initialize_rand_star(win_width, win_height).await));
        //stars.push(initialize_particle(win_width as i32, win_height as i32).await);
    }
    *loaded = true;

}

fn r_from_mass(mass: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
    let (from_min, from_max) = from_range;
    let (to_min, to_max) = to_range;

    (mass - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
}
