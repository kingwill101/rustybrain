use std::fmt::{Display, Formatter};

use rand::{self, Rng};

use crate::engine::game::{grab_game_data, GameData};
use crate::models::game::Game;

pub struct Manager {
    games: Vec<GameData>,
}

impl Manager {
    pub fn available_games(&self) -> &Vec<GameData> {
        self.games.as_ref()
    }
    pub fn game_count(&self) -> u32 {
        self.available_games().len() as u32
    }
    pub fn new() -> Manager {
        Manager::default()
    }

    fn add_game(&mut self, game: GameData) {
        self.games.push(game)
    }

    pub fn load_games(&mut self, games: Vec<Game>) {
        for g in games.iter() {
            if g.variants.as_ref().is_some() {
                for variant in g.variants.as_ref().unwrap() {
                    let game = grab_game_data(g, Some(variant));
                    self.add_game(game);
                }
            } else {
                let game = grab_game_data(g, None);
                self.add_game(game);
            }
        }
    }

    pub fn random_game(&self) -> &GameData {
        self.games
            .get(rand::thread_rng().gen_range(0..self.games.len()))
            .unwrap()
    }
}

impl Display for Manager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.games.iter().for_each(|g| if g.fmt(f).is_ok() {});
        write!(f, "")
    }
}

impl Default for Manager {
    fn default() -> Self {
        Manager { games: vec![] }
    }
}
