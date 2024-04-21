use macroquad::prelude::*;
use macroquad::audio::*;
use crate::menu::VintageWindow;

use super::camera::*;
use super::star_system::*;
use super::rockybody::*;
use super::menu::*;
use super::player::*;
use super::physics::*;

// TODO WE SHALL CONSTRUCT A GAME WHICH HAS A GAMESTATE
pub struct Game {
    pub game_state: GameState,
    universe: Vec<StarSystem>,
    camera:  ZCamera,
    player: Player,
    //TEMP - TODO make a trait for ui so we can
    // do our signature fucked up 
    // solution : `Vec<Box<dyn UI>>`
    ui_elements: Vec<VintageWindow>,
    quitting: Quitter,
}

impl Game {
    pub async fn new() -> Game {

        set_pc_assets_folder("src");
        let mut limd_sound: &Sound = &load_sound("limd.wav").await.unwrap();
        set_sound_volume(limd_sound, 0.5);
        play_sound_once(limd_sound);
        let mut camera: ZCamera = ZCamera::new_origin();
        let mut universe: Vec<StarSystem> = Vec::new();
        let mut player = Player::new();
        universe.push(StarSystem::new_rand(player.clone()).await);
        let mut ui_elements = Vec::new();
        ui_elements.push(VintageWindow::new(
            400, 200, String::from("You really tryna quit?"),
            String::from("ERROR: QUITTER DETECTED"),
            String::from("you done?"),
            String::from("Yes"),
            WindowType::Error,
            load_sound("lcuterror98.wav").await.unwrap(),
        ));
        Game {
            game_state: GameState::Playing,
            universe,
            camera,
            player,
            ui_elements,
            quitting: Quitter::No,
        }
    }
    pub async fn update(&mut self) {
        match self.quitting {
            Quitter::No => { },
            Quitter::Maybe => { },
            Quitter::Yes => { self.game_state = GameState::Quit },
        }
        match self.game_state {
            GameState::StartingUp => {},
            GameState::Playing => {
                for system in self.universe.iter_mut() {
                    update_gravity_physics(&mut system.bodies);
                    check_collisions(&mut system.bodies).await;
                    system.update();
                }
            },
            GameState::AboutToQuit => {
                self.handle_quitter();
            },
            GameState::Quit => { std::process::exit(0) },
        }
    }
    pub fn draw(&mut self) {
        match self.game_state {
            GameState::StartingUp => {
                
            },
            GameState::Playing => {
                for system in self.universe.iter_mut() {
                    system.draw(&self.camera);
                }
                for e in self.ui_elements.iter_mut() {
                    e.update(&mut self.quitting);
                    e.draw();
                }
                self.handle_inputs();
            },
            GameState::AboutToQuit => {
                for e in self.ui_elements.iter_mut() {
                    e.draw();
                }
            },
            GameState::Quit => { std::process::exit(0) },
        }
    }
    pub fn handle_inputs(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            match &self.quitting{
                Quitter::No => { self.quitting = Quitter::Maybe },
                Quitter::Maybe => { self.quitting = Quitter::No },
                Quitter::Yes => { self.game_state = GameState::Quit },
            }
        } 
    }
    pub fn handle_quitter(&mut self) {
        for e in self.ui_elements.iter_mut() {
            e.update(&mut self.quitting);
        }  
    }
}
pub enum GameState {
    Playing,
    StartingUp,
    AboutToQuit,
    Quit,
}
pub enum Quitter {
    No, 
    Yes,
    Maybe,
}
