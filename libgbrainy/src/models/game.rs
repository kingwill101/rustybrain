use serde_derive::{Deserialize, Serialize};

use crate::models::shared::{Question, Svg, Text, Variant};

#[derive(Debug, Serialize, Deserialize, )]
pub struct GameCollection {
    #[serde(rename = "game")]
    pub games: Vec<Game>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub difficulty: String,
    pub variables: Option<String>,
    #[serde(rename = "rationale")]
    pub rationale: Option<String>,
    #[serde(rename = "variant")]
    pub variants: Option<Vec<Variant>>,
    #[serde(rename = "svg")]
    pub svg: Option<Vec<Svg>>,
    #[serde(rename = "question")]
    pub questions: Option<Vec<Question>>,
    #[serde(rename = "option")]
    pub options: Option<Vec<GameOption>>
}

impl Game {

    pub fn get_question(&self) -> String{
        crate::engine::game::grab_game_data(self, None).question.value
    }
}

impl Game {
    pub fn new() -> Game {
        Game::default()
    }
}


#[derive(Debug, Serialize, Deserialize, )]
pub struct                  GameOption {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub order:String,

    #[serde(rename = "string")]
    pub text: Option<Text>
}

