use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use rand::{self, Rng};

use crate::engine::game::{grab_game_data, GameData, GameType};
use crate::models::game::Game;

pub struct Manager {
    games: HashMap<String, Vec<GameData>>,
}

impl Manager {
    pub fn available_games(&self) -> &HashMap<String, Vec<GameData>> {
        &self.games
    }

    pub fn game_category_count(&self, game_type: GameType) -> u32 {
        let mut count: u32 = 0;
        if !self.available_games().contains_key(&game_type.to_string()) {
            return 0;
        }
        self.games[&game_type.to_string()].iter().for_each(|_| {
            count += 1;
        });

        count
    }

    pub fn game_count(&self) -> u32 {
        let mut count: u32 = 0;
        for k in self.available_games().keys() {
            self.available_games()[k].iter().for_each(|_| {
                count += 1;
            });
        }
        count
    }

    pub fn new() -> Manager {
        Manager::default()
    }

    fn add_game(&mut self, game: GameData) {
        self.available_games()
            .to_owned()
            .entry(game.game_type.to_string())
            .or_insert_with(|| vec![game.clone()])
            .push(game);
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

    pub fn random_game(&self) -> GameData {
       self.random_game_from_category(GameType::LogicPuzzle)

    }

    fn random_target(&self, registry: Vec<GameData>) -> GameData {
        registry
            .get(rand::thread_rng().gen_range(0..registry.len()))
            .unwrap()
            .to_owned()
    }

    pub fn random_game_from_category(&self, game_type:GameType) -> GameData{
        let registry = self
            .available_games()
            .get(game_type.to_string().as_str())
            .unwrap()
            .clone();
        self.random_target(registry)
    }
}

impl Display for Manager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.available_games().keys().for_each(|k| {
            match writeln!(
                f,
                "{}: {} games",
                k,
                self.game_category_count(GameType::from_string(k))
            ) {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
        });
        write!(f, "")
    }
}

impl Default for Manager {
    fn default() -> Self {
        Manager {
            games: HashMap::new(),
        }
    }
}
