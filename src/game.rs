use std::time::Duration;
use crate::ui::graphics::Graphics;
use crate::objects::player::Player;
use crate::objects::entity::EntityRenderer;
use log::info;
use sdl2::event::Event;
use crate::ui::inputstate::InputState;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;
const APP_NAME: &str = "project-zero";


pub struct Game {
    //input_manager: Option<Box<InputManager>>,
    player: Box<Player>,
    graphics: Box<Graphics>,
    inputs : Box<InputState>,
    is_active : bool
}

impl Game {
    pub fn new() -> Box<Self> {
        info!("Starting the game...");

        let game = Box::new(Self {
            graphics: Graphics::new(APP_NAME, WINDOW_WIDTH, WINDOW_HEIGHT),
            player: Player::new(),
            inputs : InputState::new(),
            is_active : true
        });
        return game;
    }

    pub fn run(&mut self) {
        info!("Running game loop");
        while self.is_active() {
            self.handle_inputs();
            self.player_update(&self.inputs);
            self.npcs_update();
            self.world_update();
            self.render();
            self.wait_vsync();

        }
    }

    fn is_active(&self) -> bool {
        return self.is_active;
    }

    fn player_update(&self, input_manager: &Box<InputState>) {
        self.player.update();
    }

    fn npcs_update(&self) {}

    fn world_update(&self) {}

    fn render(&mut self) {
        self.graphics.render();
    }

    fn handle_inputs(&mut self)  {
        let mut pump = self.graphics.event_pump();
        while let Some(event) = pump.poll_event() {
            match event {
                Event::Quit { .. } => self.is_active = false,
                Event::KeyDown {..} | Event::KeyUp {..} => {
                    self.inputs.handle_keyboard(pump.keyboard_state())
                },
                Event::MouseButtonDown {..} |
                Event::MouseButtonUp {..} |
                Event::MouseMotion {..} |
                Event::MouseWheel {..} => {
                    self.inputs.handle_mouse(pump.mouse_state())
                }
                _ => ()
            };
        }


    }

    fn wait_vsync(&self) {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }


    pub fn shutdown(&mut self) {
        info!("Shutting down the game")
    }

}

