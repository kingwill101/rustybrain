extern crate colog;
extern crate log;

use libgbrainy;
use libgbrainy::engine::game::GameData;
use libgbrainy::engine::manager::Manager;
use libgbrainy::models::game::GameCollection;

fn main() {

    colog::init();
    let bytes = include_bytes!("../../data/games.xml");

    let collection = libgbrainy::reader::parse_game_data(
        Box::from(
            &*String::from_utf8_lossy(bytes)
        )
    );

    match collection {
        None => {
            log::warn!("Parsing error");
            return
        }
        Some(data) => {
            let mut game_manager = Manager::new();

            game_manager.load_games(data.games);


            let mut engine = libgbrainy::engine::Engine::new();
            let game = game_manager.random_game();
            engine.parse_variables(game.variables.as_str());
            println!("{}", game);
            println!("{}", engine.interop(game.question.text.singular.as_str()));
        }
    }
}