use macroquad::prelude::*;

mod menu;
mod star;
mod rockybody;
mod physics;
mod camera;
mod ships;
mod texturetools;
mod mathtools;
mod life;

use menu::*;
use physics::*;
use star::*;
use rockybody::*;
use camera::*;
use ships::*;
use life::*;
use texturetools::*;
use mathtools::*;

#[macroquad::main("Yuh")]
async fn main() {
    let mut draw_vintage_window = false;
    let mut quitting = false;
    let mut loaded = false;
    let mut fs = false;
    let mut camera: ZCamera = ZCamera::new_origin();
    let mut vintage_window = VintageWindow::new(
        400,
        220,
        String::from("Vintage Window Pop-Up"),
        String::from("ERROR: APATHY DETECTED!"),
        String::from("GIVE UP"),
        String::from("Ok"),
        WindowType::Error
    );

    let mut player_ship = PlayerShip::new(
        0.,
        0.,
        0.,
        0.,
        90999999999,
        13.,
    );


    let mut grav_objs: Vec<Box<dyn PhysObj>> = Vec::new();
    grav_objs.push(Box::new(player_ship));

    loop {
        if !loaded {
            set_fullscreen(true);
            if screen_width() > 900. {
                let (ox, oy, m) = load_stars(
                    &mut grav_objs,
                    screen_width() as f32,
                    screen_height() as f32
                ).await;
                load_civilization(&mut grav_objs);
                load_rocky_bodies(
                    &mut loaded,
                    &mut grav_objs,
                    screen_width() as f32,
                    screen_height() as f32,
                    ox,
                    oy,
                    m
                ).await;
            }

        }
        clear_background(BLACK);
        if let Some(ship) = grav_objs.first_mut() {
            update_ship_velocity(ship);
            lerp(ship, &mut camera);
        }
        update_gravity_physics(&mut grav_objs);

        check_collisions(&mut grav_objs).await;
        let (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
        let (mouse_x, mouse_y) = mouse_position();
        let (mwx, mwy) = ((mouse_x / camera.zoom as f32) + camera.xpos, (mouse_y / camera.zoom as f32) + camera.ypos);
        let o_zoom = camera.zoom;
        if mouse_wheel_y != 0. {
            let dz = 0.01;
            camera.zoom += mouse_wheel_y as f64 * dz as f64;
            if camera.zoom < 0.01 {
                camera.zoom = 0.01;
            }

            if let Some(ship) = grav_objs.first_mut() {
                let (new_x, new_y) = ((ship.xpos() / camera.zoom as f32) + camera.xpos, (ship.ypos() / camera.zoom as f32) + camera.ypos);
            }
        }
        for obj in grav_objs.iter_mut().rev() {
            obj.update();
            obj.draw(&mut camera);
        }
        update_ships_desired_pos(&mut grav_objs);
        vintage_window.update(&mut quitting);
        vintage_window.draw();

        if quitting {
            break
        }
        next_frame().await
    }
}


fn update_ship_velocity(mut player_ship: &mut Box<dyn PhysObj>) {
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
}

fn lerp(body: &mut Box<dyn PhysObj>, camera: &mut ZCamera) {
    let lerp_factor = 0.6; // Adjust lerp_factor as needed for smooth transitions

    // Get half the screen width and height in pixels
    let half_width_pixels = screen_width() / 2.0;
    let half_height_pixels = screen_height() / 2.0;


    camera.xpos = -(half_width_pixels / camera.zoom as f32) + body.xpos();
    camera.ypos = -(half_height_pixels / camera.zoom as f32) + body.ypos(); 

    

}


fn log_clamp(value: f64, max_value: f64) -> f64 {
    if value <= 0.01 {
        return 0.1;
    }
    (max_value * f64::ln(value) / f64::ln(max_value)).min(max_value)
}
