extern crate colog;
extern crate log;

use libgbrainy::engine::{game::GameType, manager::Manager};

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
            // let game = game_manager.find_game_by_cat_n_name(
            //     GameType::Calculation, "Multiple number");
            // println!("{}",game.unwrap());
            // let mut context =
            //     libgbrainy::engine::context::GameContext::new(game_manager.random_game().clone());
            //
            // println!("{}", context.get_drawing_objects().len());
            // println!("{}", context.options_answers_interop())

            let game2 = game_manager
                .get_game_from_category_with_name(GameType::LogicPuzzle, "Boxes".to_string());
            // assert_eq!(game2.unwrap().objects.len(), 4)

            println!("{}", game2.unwrap());
        }
    }
}
