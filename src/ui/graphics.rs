use std::time::Duration;
use log::info;
use sdl2::render::WindowCanvas;
use sdl2::{EventPump, Sdl};
use sdl2::pixels::Color;


pub struct Graphics {
    sdl_descriptor: Box<SDLDescriptor>,
}

impl Graphics {
    pub fn new(app_name: &str, screen_width: u32, screen_height: u32) -> Box<Self> {
        return Box::new(Graphics {
            sdl_descriptor: SDLDescriptor::new(app_name, screen_width, screen_height)
        });
    }

    pub fn event_pump(&self) -> EventPump {
        return self.sdl_descriptor.sdl_context.event_pump().unwrap();
    }

    pub fn render(&mut self) {
        self.sdl_descriptor.sdl_canvas.present();

    }


}


struct SDLDescriptor {
    sdl_context: Sdl,
    sdl_canvas: WindowCanvas,
}

impl SDLDescriptor {
    pub fn new(app_name: &str, screen_width: u32, screen_height: u32) -> Box<Self> {
        info!("Initializing SDL subsystem");

        let context = match sdl2::init() {
            Err(e) => { panic!("Error creating SDL2 context:{}", e) }
            Ok(context) => context
        };

        let video = match context.video() {
            Err(e) => panic!("Error initializing video system:{}", e),
            Ok(video) => video
        };

        let window = match video.window(app_name, screen_width, screen_height)
            .position_centered()
            .build() {
            Err(e) => panic!("Error creating window:{}", e),
            Ok(window) => window
        };

        let canvas = match window.into_canvas().build() {
            Err(e) => panic!("Error creating canvas:{}", e),
            Ok(canvas) => canvas
        };

        return Box::new(SDLDescriptor {
            sdl_context: context,
            sdl_canvas: canvas,
        });
    }

    pub fn shutdown() {
        info!("Shutting down SDL subsystem");
    }
}
