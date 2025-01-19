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

#[macroquad::main("PARA-GENESIS")]
async fn main() {
    let mut game = Game::new().await;
    loop {
        while screen_width() < 900. {
            set_fullscreen(true);
            next_frame().await
        }
        clear_background(BLACK);
        game.update().await;
        game.draw();

        match game.game_state {
            GameState::Playing => {},
            GameState::AboutToQuit => {},
            GameState::StartingUp => {},
            GameState::Quit => { break },
        }
        next_frame().await
    /*
        TODO REIMPLEMENT ZOOMING        
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

    */

    }
}

