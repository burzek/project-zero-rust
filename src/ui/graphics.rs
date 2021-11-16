use std::time::Duration;
use log::info;
use sdl2::render::WindowCanvas;
use sdl2::{EventPump, Sdl, TimerSubsystem};
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

    pub fn get_ticks(&self) -> u32 {
        return self.sdl_descriptor.sdl_timer.ticks();
    }

    pub fn start_renderer(&mut self) -> &mut WindowCanvas {
        self.sdl_descriptor.sdl_canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.sdl_descriptor.sdl_canvas.clear();
        return &mut self.sdl_descriptor.sdl_canvas;
    }

    pub fn commit_renderer(&mut self) {
        self.sdl_descriptor.sdl_canvas.present();
    }


}


struct SDLDescriptor {
    sdl_context: Sdl,
    sdl_canvas: WindowCanvas,
    sdl_timer : TimerSubsystem
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
        let timer = context.timer().unwrap();

        return Box::new(SDLDescriptor {
            sdl_context: context,
            sdl_canvas: canvas,
            sdl_timer : timer
        });
    }


    pub fn shutdown() {
        info!("Shutting down SDL subsystem");
    }
}
