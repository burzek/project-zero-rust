mod ui;
mod game;
mod objects;

use simple_logger::SimpleLogger;
use game::Game;

fn main() {
    //init logger
    SimpleLogger::new().init().expect("Error initializing logger");

    let mut game = Game::new();
    game.run();
    game.shutdown();

}
