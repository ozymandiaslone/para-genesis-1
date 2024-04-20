use macroquad::prelude::*;
mod player;
mod game;
mod menu;
mod star;
mod rockybody;
mod physics;
mod camera;
mod ships;
mod texturetools;
mod mathtools;
mod life;
mod star_system;
use game::*;
use menu::*;
use physics::*;
use star::*;
use rockybody::*;
use camera::*;
use ships::*;
use life::*;
use texturetools::*;
use mathtools::*;
use star_system::*;
use player::*;

#[macroquad::main("Yuh")]
async fn main() {
    /* RELICT CODE
    *
    *
    let mut quitting = false;
*/

    let mut game = Game::new().await;

    loop {
        while screen_width() < 900. {
            set_fullscreen(true);
            next_frame().await
        }
        clear_background(BLACK);
        game.update().await;
        game.draw();
        next_frame().await

    /*
        
        
        if let Some(ship) = grav_objs.first_mut() {
            update_ship_velocity(ship);
            lerp(ship, &mut camera);
        }
        update_gravity_physics(&mut grav_objs);

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
//        update_ships_desired_pos(&mut grav_objs);
        vintage_window.update(&mut quitting);
        vintage_window.draw();

        if quitting {
            break
        }

    */

    }
}



fn log_clamp(value: f64, max_value: f64) -> f64 {
    if value <= 0.01 {
        return 0.1;
    }
    (max_value * f64::ln(value) / f64::ln(max_value)).min(max_value)
}
