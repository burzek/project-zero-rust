use std::borrow::BorrowMut;
use std::time::Duration;
use crate::ui::graphics::Graphics;
use crate::objects::player::Player;
use crate::objects::entity::EntityRenderer;
use crate::objects::entity::EntityUpdater;
use log::info;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use crate::ui::inputstate::InputState;



pub struct Game {
    player: Player,
    graphics: Graphics,
    inputs : InputState,
    is_active : bool
}

impl Game {
    pub fn new() -> Self {
        info!("Starting the game...");

        let game = Self {
            graphics: Graphics::new(crate::APP_NAME, crate::WINDOW_WIDTH, crate::WINDOW_HEIGHT),
            player: Player::new(),
            inputs : InputState::new(),
            is_active : true
        };
        return game;
    }

    pub fn run(&mut self) {
        info!("Running game loop");

        let mut tick = self.graphics.get_ticks();
        let mut prev_tick = tick;

        while self.is_active() {
            let next_tick : u32 = tick + (1000 / crate::FPS);     //render every 1/FPS s

            while tick <= next_tick {       //wait for next frame, but world is still updated
                let dt = tick - prev_tick;
                if dt != 0 {
                    self.handle_inputs();
                    self.world_update(dt);
                }
                prev_tick = tick;
                tick = self.graphics.get_ticks();
            }

            //render every 1/FPS secs
            self.render();

        }
    }

    fn is_active(&self) -> bool {
        return self.is_active;
    }

    fn world_update(&mut self, dt : u32) {
        let f_dt = dt as f32;
        self.player.update(&self.inputs, f_dt);
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

