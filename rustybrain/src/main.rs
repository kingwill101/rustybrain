extern crate colog;
extern crate log;

use libgbrainy::engine::{manager::Manager, game::GameType};

fn main() {
    colog::init();
    let bytes = include_bytes!("../../data/games.xml");

    let collection = libgbrainy::reader::parse_game_data(&*String::from_utf8_lossy(bytes));

    match collection {
        None => {
            log::warn!("Parsing error");
        }
        Some(data) => {
            let mut game_manager = Manager::new();

            game_manager.load_games(data.games);
            // println!("Games found - {}", game_manager.game_count());
            // println!("{}", game_manager);
            // println!("fsafasdfad");
            let game = game_manager.find_game_by_cat_n_name(
                GameType::Calculation, "Multiple number");
            println!("{}",game.unwrap());
            // let mut context =
            //     libgbrainy::engine::context::GameContext::new(game_manager.random_game().clone());
            //
            // println!("{}", context.get_drawing_objects().len());
            // println!("{}", context.options_answers_interop())
        }
    }
}
