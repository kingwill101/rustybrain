use libgbrainy;
use libgbrainy::engine::manager::Manager;
use libgbrainy::engine::game::GameData;
use crate::data::get_game_xml;

mod data;
mod rusty;

fn main() {
    let collection = libgbrainy::reader::parse_game_data(
        get_game_xml()
    );

    let mut game_manager = Manager::new();
    game_manager.load_games(collection.games);

    println!("{} games found", game_manager.games.len());
    println!("{}", game_manager);


    let mut engine = libgbrainy::engine::Engine::new();
    let game = game_manager.random_game();
    engine.parse_variables(game.variables.as_str());
    println!("{}", game);
    println!("{}", engine.interop(game.question.value.as_str()));

}