use macroquad::prelude::*;
use crate::menu::VintageWindow;

use super::camera::*;
use super::star_system::*;
use super::rockybody::*;
use super::menu::*;
use super::player::*;
use super::physics::*;


// TODO WE SHALL CONSTRUCT A GAME WHICH HAS A GAMESTATE
pub enum GameState {
    Playing,
    StartingUp,
    AboutToQuit,
}

pub struct Game {
    game_state: GameState,
    universe: Vec<StarSystem>,
    camera:  ZCamera,
    player: Player,
    //TEMP - TODO make a trait for ui
    ui_elements: Vec<VintageWindow>,
}

impl Game {

    pub async fn new() -> Game {

        let mut camera: ZCamera = ZCamera::new_origin();
        let mut universe: Vec<StarSystem> = Vec::new();
        let mut player = Player::new();
        universe.push(StarSystem::new_rand(player.clone()).await);
        let mut ui_elements = Vec::new();
        ui_elements.push(VintageWindow::new(
            400, 200, String::from("Test"), String::from("Line 1"), String::from("Line 2"),
            String::from("Ok"), WindowType::Error 
        ));

        Game {
            game_state: GameState::Playing,
            universe,
            camera,
            player,
            ui_elements,
        }
    }

    pub async fn update(&mut self) {
        match self.game_state {
            GameState::StartingUp => {},
            GameState::Playing => {
                for system in self.universe.iter_mut() {
                    update_gravity_physics(&mut system.bodies);
                    check_collisions(&mut system.bodies).await;
                    system.update();
                }
            },
            GameState::AboutToQuit => {},
        }
    }
    pub fn draw(&mut self) {
        match self.game_state {
            GameState::StartingUp => {},
            GameState::Playing => {
                for system in self.universe.iter_mut() {
                    system.draw(&self.camera);
                }
                for e in self.ui_elements.iter_mut() {
                    e.draw();
                }
            },
            GameState::AboutToQuit => {},
        }
    }


}

