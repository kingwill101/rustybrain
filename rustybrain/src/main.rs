extern crate colog;
extern crate log;

use libgbrainy;

use libgbrainy::engine::manager::Manager;

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
            let mut context = libgbrainy::engine::context::GameContext::new(game_manager.random_game().clone());

            println!("{}", context.get_drawing_objects().len());
            println!("{}", context.options_possible_answers_interop())
        }
    }
}