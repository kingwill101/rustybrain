use std::fmt::{Display, Formatter};

use crate::engine::Engine;
use crate::models::game::Game;
use crate::models::shared::{Question, Variant};

pub struct GameContext {
    engine: Engine,
    game: Game,
}

impl GameContext {
    pub fn new(game: Game) -> GameContext {
        GameContext {
            engine: Engine::new(),
            game,
        }
    }
}

enum GameType {
    None,
    LogicPuzzle,
    Memory,
    Calculation,
    VerbalAnalogy,
}

impl Clone for GameType {
    fn clone(&self) -> Self {
        match self
        {
            GameType::None => GameType::None,
            GameType::LogicPuzzle => GameType::LogicPuzzle,
            GameType::Memory => GameType::Memory,
            GameType::Calculation => GameType::Calculation,
            GameType::VerbalAnalogy => GameType::VerbalAnalogy,
        }
    }
}

impl GameType {
    fn from_string(g_type: &str) -> GameType {
        match g_type
        {
            "Logic" => { GameType::LogicPuzzle }
            "Memory" => { GameType::Memory }
            "Calculation" => { GameType::Calculation }
            "Verbal" => { GameType::VerbalAnalogy }
            _ => { GameType::None }
        }
    }

    fn to_string(&self) -> String {
        match self
        {
            GameType::None => { "Unknown".to_string() }
            GameType::LogicPuzzle => { "Logic".to_string() }
            GameType::Memory => { "Memory".to_string() }
            GameType::Calculation => { "Location".to_string() }
            GameType::VerbalAnalogy => { "Verbal".to_string() }
        }
    }
}


impl Default for GameType {
    fn default() -> Self {
        GameType::None
    }
}

enum GameDifficulty {
    None,
    Easy,
    Medium,
    Master,
    All,
}

impl Clone for GameDifficulty {
    fn clone(&self) -> Self {
        match self {
            GameDifficulty::None => { GameDifficulty::None }
            GameDifficulty::Easy => { GameDifficulty::Easy }
            GameDifficulty::Medium => { GameDifficulty::Medium }
            GameDifficulty::Master => { GameDifficulty::Master }
            GameDifficulty::All => { GameDifficulty::All }
        }
    }
}

impl GameDifficulty {
    fn from_string(difficulty: &str) -> GameDifficulty {
        match difficulty
        {
            "Easy" => { GameDifficulty::Easy }
            "Medium" => { GameDifficulty::Medium }
            "Master" => { GameDifficulty::Master }
            "All" => { GameDifficulty::All }
            _ => { GameDifficulty::None }
        }
    }
    fn to_string(&self) -> String {
        match self {
            GameDifficulty::None => { String::from("Unknown") }
            GameDifficulty::Easy => { String::from("Easy") }
            GameDifficulty::Medium => { String::from("Medium") }
            GameDifficulty::Master => { String::from("Master") }
            GameDifficulty::All => { String::from("All") }
        }
    }
}

impl Default for GameDifficulty {
    fn default() -> Self {
        GameDifficulty::All
    }
}

#[derive(Default, Clone)]
struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Clone)]
struct Dimensions {
    pub width: f64,
    pub height: f64,
}

#[derive(Default, Clone)]
struct Image {
    position: Position,
    dimensions: Dimensions,
    path: String,
}


enum ObjectSize {
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
}

impl Clone for ObjectSize {
    fn clone(&self) -> Self {
        match self {
            ObjectSize::Small => { ObjectSize::Small }
            ObjectSize::Medium => { ObjectSize::Medium }
            ObjectSize::Large => { ObjectSize::Large }
            ObjectSize::XLarge => { ObjectSize::XLarge }
            ObjectSize::XXLarge => { ObjectSize::XXLarge }
        }
    }
}

impl ObjectSize {
    pub fn from_string(size: &str) -> ObjectSize {
        match size {
            "Small" => { ObjectSize::Small }
            "Medium" => { ObjectSize::Medium }
            "Large" => { ObjectSize::Large }
            "XLarge" => { ObjectSize::XLarge }
            "XXLarge" => { ObjectSize::XXLarge }
            _ => { ObjectSize::Medium }
        }
    }
}

impl Default for ObjectSize {
    fn default() -> Self {
        ObjectSize::Medium
    }
}

#[derive(Default)]
struct TextObject {
    position: Position,
    text: String,
    centered: bool,
    size: ObjectSize,
}

#[derive(Clone)]
enum QuestionType {
    MultipleOptions,
    PairOfWordsOptions,
    PairOfWordsCompare,
    None,
}

impl Default for QuestionType {
    fn default() -> Self {
        QuestionType::None
    }
}

#[derive(Default, Clone, )]
struct QuestionObject {
    value: String,
    plural: String,
    question_ype: QuestionType,
}

#[derive(Default, Clone, )]
pub struct GameData {
    name: String,
    game_type: GameType,
    difficulty: GameDifficulty,
    variables: String,
    image: Image,
    question: QuestionObject,
}

impl Display for GameData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name: {} Type: {} Difficulty: {}", self.name, self.game_type.to_string(), self.difficulty.to_string())
    }
}

/// only 2 questions are supported second question is only used to provide additional
/// data where in cases where a plural version of a question is needed
fn process_question(q: &Vec<Question>, question: &mut QuestionObject) {
    if q.len() > 1 {
        question.value = q.get(0).as_ref()
            .unwrap().string.as_ref().unwrap().to_string();

        question.plural = q.get(1).as_ref()
            .unwrap().plural.as_ref().unwrap().to_string()
    } else {
        question.value = q.get(0).as_ref()
            .unwrap().string.as_ref().unwrap().to_string()
    }
}

#[test]
fn test_process_selection() {
    let mut q_obj = QuestionObject::default();

    let mut ques1 = Question::default();
    ques1.string = Some("question 1".to_string());
    ques1.plural = Some("plural 1".to_string());

    let mut ques2 = Question::default();
    ques2.string = Some("question 2".to_string());
    ques2.plural = Some("plural 2".to_string());

    let mut questions = vec![ques1, ques2];

    process_question(&questions, &mut q_obj);

    assert_eq!(q_obj.value, "question 1");
    assert_eq!(q_obj.plural, "plural 2");

    q_obj = QuestionObject::default();
    ques1 = Question::default();
    ques1.string = Some("question 1".to_string());

    process_question(&vec![ques1], &mut q_obj);

    assert_eq!(q_obj.value, "question 1");
    assert_eq!(q_obj.plural, "");
}

pub fn grab_game_data(game: &Game, variant: Option<&Variant>) -> GameData {
    let mut game_data = GameData {
        name: game.name.as_str().to_string(),
        game_type: GameType::from_string(game.type_.as_str()),
        difficulty: GameDifficulty::from_string(game.difficulty.as_str()),
        ..Default::default()
    };

    let mut question = QuestionObject { ..Default::default() };

    if variant.as_ref().is_none() {
        game_data.variables = match game.variables.as_ref() {
            None => { "".to_string() }
            Some(vars) => { vars.to_string() }
        };

        if game.questions.as_ref().is_some() {
            process_question(game.questions.as_ref().unwrap(), &mut question);
        }
    } else {
        game_data.variables = if variant.as_ref().unwrap().variables.is_some() {
            variant.as_ref().unwrap().variables.as_ref().unwrap().to_string()
        } else {
            String::new()
        };

        if variant.as_ref().unwrap().question.as_ref().is_some() {
            process_question(variant.as_ref().unwrap().question.as_ref().unwrap(), &mut question);
        }
    }

    game_data.question = question;
    game_data
}
