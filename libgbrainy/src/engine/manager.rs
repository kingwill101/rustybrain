use std::borrow::Borrow;

use crate::engine::game::{GameData, grab_game_data};
use crate::models::game::Game;
use std::fmt::{Display, Formatter};

pub struct Manager {
    pub games: Vec<GameData>,
}

impl Manager {

    pub fn new() -> Manager{
        Manager::default()
    }

    fn add_game(&mut self, game: GameData) {
        self.games.push(game)
    }

    pub fn load_games(&mut self, games: Vec<Game>) {
        for g in games.iter() {
            let mut game = Default::default();

            if g.variants.as_ref().is_some() {
                for variant in g.variants.as_ref().unwrap() {
                    game = grab_game_data(g, Some(variant));
                    self.add_game(game);
                }
            } else {
                game = grab_game_data(g, None);
                self.add_game(game);

            }
        }
    }
}

impl Display for Manager{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.games.iter().for_each(|g|{
            g.fmt(f);
        });

        write!(f, "")
    }
}

impl Default for Manager{
    fn default() -> Self {
        Manager {
            games: vec![]
        }
    }
}