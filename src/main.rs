mod ui;
mod game;
mod objects;

use simple_logger::SimpleLogger;
use game::Game;
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;
const FPS : u32 = 60;
const APP_NAME: &str = "project-zero";


fn main() {
    //init logger
    SimpleLogger::new().init().expect("Error initializing logger");

    let mut game = Game::new();
    game.run();
    game.shutdown();

}
