use std::time::Duration;
use crate::ui::graphics::Graphics;
use crate::objects::player::Player;
use crate::objects::entity::EntityRenderer;
use log::info;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use crate::ui::inputstate::InputState;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;
const FPS : u32 = 60;
const APP_NAME: &str = "project-zero";


pub struct Game {
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
            let mut tick = self.graphics.get_ticks();
            let next_tick : u32 = tick + 1000 / FPS;

            while (tick <= next_tick) {
                self.handle_inputs();
                let InputState = self.inputs.as_ref();
                self.world_update(InputState);
                tick = self.graphics.get_ticks();
            }

            self.render();

        }
    }

    fn is_active(&self) -> bool {
        return self.is_active;
    }

    fn world_update(&mut self, input_state: &InputState) {
        self.player.update();
    }

    fn render(&mut self) {
        let canvas = self.graphics.start_renderer();
        self.player.render(canvas);
        self.graphics.commit_renderer();

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

    pub fn shutdown(&mut self) {
        info!("Shutting down the game")
    }

}

