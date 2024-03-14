use macroquad::prelude::*;

mod star;
mod physics;
mod camera;
mod ships;

use physics::*;
use star::*;
use camera::*;
use ships::*;

#[macroquad::main("Yuh")]
async fn main() {
    let mut loaded = false;
    let mut fs = false;
    let mut camera: ZCamera = ZCamera::new_origin();

    let mut player_ship = PlayerShip::new(
        0.,
        0.,
        0.,
        0.,
        10000,
        5.,
    );

    let mut grav_objs: Vec<Box<dyn PhysObj>> = Vec::new();
    grav_objs.push(Box::new(player_ship));

    loop {
        if !loaded {
            set_fullscreen(true);
            if screen_width() > 900. {
                load_stars(
                    &mut loaded,
                    &mut grav_objs,
                    screen_width() as f32,
                    screen_height() as f32
                ).await;
            }
        }
        clear_background(BLACK);
        if let Some(ship) = grav_objs.first_mut() {
            update_ship_velocity(ship);
            lerp(ship, &mut camera);
        }
        update_gravity_physics(&mut grav_objs);

        check_collisions(&mut grav_objs);
        let (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
        let (mouse_x, mouse_y) = mouse_position();
        let (mwx, mwy) = ((mouse_x / camera.zoom as f32) + camera.xpos, (mouse_y / camera.zoom as f32) + camera.ypos);
        let o_zoom = camera.zoom;
        if mouse_wheel_y != 0. {
            let dz = 0.01;
            camera.zoom += mouse_wheel_y as f64 * dz as f64;
            if camera.zoom < 0.1 {
                camera.zoom = 0.1;
            }
            //camera.zoom = log_clamp(camera.zoom, 10.);
            let (new_mwx, new_mwy) = ((mouse_x / camera.zoom as f32) + camera.xpos, (mouse_y / camera.zoom as f32) + camera.ypos);
            let dx = mwx - new_mwx;
            let dy = mwy - new_mwy;

            camera.xpos += dx;
            camera.ypos += dy;
        
        }
        for obj in grav_objs.iter_mut() {
            obj.update();
            obj.draw(&mut camera);
        }
        next_frame().await
    }
}


fn update_ship_velocity(mut player_ship: &mut Box<dyn PhysObj>) {
    if is_key_down(KeyCode::W) {
        player_ship.update_yvel(-1.);
    };
    if is_key_down(KeyCode::A) {
        player_ship.update_xvel(-1.);
    };
    if is_key_down(KeyCode::S) {
        player_ship.update_yvel(1.);
    };
    if is_key_down(KeyCode::D) {
        player_ship.update_xvel(1.);
    };
}

fn lerp(body: &mut Box<dyn PhysObj>, camera: &mut ZCamera) {
    let lerp_factor = 0.6;
    camera.xpos = body.xpos() - screen_width() / 2.;
    camera.ypos = body.ypos() - screen_height() / 2.;
}

fn log_clamp(value: f64, max_value: f64) -> f64 {
    if value <= 0.01 {
        return 0.1;
    }
    (max_value * f64::ln(value) / f64::ln(max_value)).min(max_value)
}
