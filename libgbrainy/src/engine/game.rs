use std::fmt::{Display, Formatter};
extern crate serde;
use crate::models::game::Game;
use crate::models::shared::{Question, Variant};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum GameType {
    None,
    LogicPuzzle,
    Memory,
    Calculation,
    VerbalAnalogy,
}

impl Clone for GameType {
    fn clone(&self) -> Self {
        match self {
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
        match g_type {
            "Logic" => GameType::LogicPuzzle,
            "Memory" => GameType::Memory,
            "Calculation" => GameType::Calculation,
            "Verbal" => GameType::VerbalAnalogy,
            _ => GameType::None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            GameType::None => "Unknown".to_string(),
            GameType::LogicPuzzle => "Logic".to_string(),
            GameType::Memory => "Memory".to_string(),
            GameType::Calculation => "Location".to_string(),
            GameType::VerbalAnalogy => "Verbal".to_string(),
        }
    }
}

impl Default for GameType {
    fn default() -> Self {
        GameType::None
    }
}

#[derive(Serialize, Deserialize)]
pub enum GameDifficulty {
    None,
    Easy,
    Medium,
    Master,
    All,
}

impl Clone for GameDifficulty {
    fn clone(&self) -> Self {
        match self {
            GameDifficulty::None => GameDifficulty::None,
            GameDifficulty::Easy => GameDifficulty::Easy,
            GameDifficulty::Medium => GameDifficulty::Medium,
            GameDifficulty::Master => GameDifficulty::Master,
            GameDifficulty::All => GameDifficulty::All,
        }
    }
}

impl GameDifficulty {
    fn from_string(difficulty: &str) -> GameDifficulty {
        match difficulty {
            "Easy" => GameDifficulty::Easy,
            "Medium" => GameDifficulty::Medium,
            "Master" => GameDifficulty::Master,
            "All" => GameDifficulty::All,
            _ => GameDifficulty::None,
        }
    }
    fn to_string(&self) -> String {
        match self {
            GameDifficulty::None => String::from("Unknown"),
            GameDifficulty::Easy => String::from("Easy"),
            GameDifficulty::Medium => String::from("Medium"),
            GameDifficulty::Master => String::from("Master"),
            GameDifficulty::All => String::from("All"),
        }
    }
}

impl Default for GameDifficulty {
    fn default() -> Self {
        GameDifficulty::All
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Image {
    position: Position,
    dimensions: Dimensions,
    path: String,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
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
            ObjectSize::Small => ObjectSize::Small,
            ObjectSize::Medium => ObjectSize::Medium,
            ObjectSize::Large => ObjectSize::Large,
            ObjectSize::XLarge => ObjectSize::XLarge,
            ObjectSize::XXLarge => ObjectSize::XXLarge,
        }
    }
}

impl ObjectSize {
    pub fn from_string(size: &str) -> ObjectSize {
        match size {
            "Small" => ObjectSize::Small,
            "Medium" => ObjectSize::Medium,
            "Large" => ObjectSize::Large,
            "XLarge" => ObjectSize::XLarge,
            "XXLarge" => ObjectSize::XXLarge,
            _ => ObjectSize::Medium,
        }
    }
}

impl Default for ObjectSize {
    fn default() -> Self {
        ObjectSize::Medium
    }
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum ObjectOrder {
    Randomized,
    InOut,
}

impl Clone for ObjectOrder {
    fn clone(&self) -> Self {
        match self {
            ObjectOrder::Randomized => ObjectOrder::Randomized,
            ObjectOrder::InOut => ObjectOrder::InOut,
        }
    }
}

impl ObjectOrder {
    pub fn from_string(size: &str) -> ObjectOrder {
        match size {
            "random" => ObjectOrder::Randomized,
            "same" => ObjectOrder::InOut,
            _ => ObjectOrder::InOut,
        }
    }
}

impl Default for ObjectOrder {
    fn default() -> Self {
        ObjectOrder::InOut
    }
}
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TextObject {
    position: Position,
    text: TObject,
    centered: bool,
    size: ObjectSize,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum QuestionType {
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

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TObject {
    pub singular: String,
    pub plural: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestionObject {
    pub text: TObject,
    pub question_ype: QuestionType,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct RationaleObject {
    pub text: TObject,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct AnswerObject {
    pub text: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct GameObject {
    pub position: Position,
    pub dimensions: Dimensions,
    pub order: ObjectOrder,
    pub text: TextObject,
    pub is_option: bool,
    pub is_correct: bool
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub name: String,
    pub game_type: GameType,
    pub difficulty: GameDifficulty,
    pub variables: String,
    pub image: Image,
    pub question: QuestionObject,
    pub rationale: RationaleObject,
    pub answer: AnswerObject,
    pub objects: Vec<GameObject>,
}

impl Display for GameData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Name: {} Type: {} Difficulty: {}",
            self.name,
            self.game_type.to_string(),
            self.difficulty.to_string()
        )
    }
}

/// only 2 questions are supported second question is only used to provide additional
/// data where in cases where a plural version of a question is needed
fn process_question(q: &Vec<Question>, question: &mut QuestionObject) {
    if q.len() > 1 {
        question.text.singular = q
            .get(0)
            .as_ref()
            .unwrap()
            .string
            .as_ref()
            .unwrap()
            .to_string();

        question.text.plural = q
            .get(1)
            .as_ref()
            .unwrap()
            .plural
            .as_ref()
            .unwrap()
            .to_string()
    } else {
        question.text.singular = q
            .get(0)
            .as_ref()
            .unwrap()
            .string
            .as_ref()
            .unwrap()
            .to_string()
    }
}

fn process_rationale(q: &Vec<String>, question: &mut RationaleObject) {
    if q.len() > 1 {
        question.text.singular = q.get(0).as_ref().unwrap().to_string();

        question.text.plural = q.get(1).as_ref().unwrap().to_string()
    } else {
        question.text.singular = q.get(0).as_ref().unwrap().to_string();
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

    let questions = vec![ques1, ques2];

    process_question(&questions, &mut q_obj);

    assert_eq!(q_obj.text.singular, "question 1");
    assert_eq!(q_obj.text.plural, "plural 2");

    q_obj = QuestionObject::default();
    ques1 = Question::default();
    ques1.string = Some("question 1".to_string());

    process_question(&vec![ques1], &mut q_obj);

    assert_eq!(q_obj.text.singular, "question 1");
    assert_eq!(q_obj.text.plural, "");
}

pub fn grab_game_data(game: &Game, variant: Option<&Variant>) -> GameData {
    let mut game_data = GameData {
        name: game.name.as_str().to_string(),
        game_type: GameType::from_string(game.type_.as_str()),
        difficulty: GameDifficulty::from_string(game.difficulty.as_str()),
        ..Default::default()
    };

    let mut question = QuestionObject {
        ..Default::default()
    };
    let mut rat = RationaleObject {
        ..Default::default()
    };

    //unique cases where variants don't have questions but questions may exist
    //on the parent Game object
    if variant.as_ref().is_some()
        && variant.as_ref().unwrap().question.is_none()
        && game.questions.is_some()
    {
        process_question(game.questions.as_ref().unwrap().as_ref(), &mut question);
    }

    if variant.as_ref().is_none() {
        game_data.variables = match game.variables.as_ref() {
            None => "".to_string(),
            Some(vars) => vars.to_string(),
        };

        if game.questions.as_ref().is_some() {
            process_question(game.questions.as_ref().unwrap(), &mut question);
        }

        if game.rationale.as_ref().is_some() {
            process_rationale(game.rationale.as_ref().unwrap(), &mut rat);
        }

        game_data.image = if game.svg.is_some() {
            let mut image = Image::default();
            image.position = Position {
                x: game.svg.as_ref().unwrap().get(0).unwrap().x,
                y: game.svg.as_ref().unwrap().get(0).unwrap().y,
            };

            image.dimensions = Dimensions {
                width: game.svg.as_ref().unwrap().get(0).unwrap().width,
                height: game.svg.as_ref().unwrap().get(0).unwrap().height,
            };
            image.path = game
                .svg
                .as_ref()
                .unwrap()
                .get(0)
                .unwrap()
                .file
                .as_str()
                .to_string();
            image
        } else {
            Image::default()
        }
    } else {
        let variant = variant.as_ref().unwrap();

        game_data.variables = if variant.variables.is_some() {
            variant.variables.as_ref().unwrap().to_string()
        } else {
            String::new()
        };

        if variant.question.as_ref().is_some() {
            process_question(variant.question.as_ref().unwrap(), &mut question);
        }

        if variant.rationale.as_ref().is_some() {
            process_rationale(variant.rationale.as_ref().unwrap(), &mut rat);
        }
        game_data.image = if variant.svg.is_some() {
            let mut image = Image::default();
            let svg = variant.svg.as_ref().unwrap();

            image.position = Position {
                x: svg.get(0).unwrap().x,
                y: svg.get(0).unwrap().y,
            };

            image.dimensions = Dimensions {
                width: svg.get(0).unwrap().width,
                height: svg.get(0).unwrap().height,
            };
            image.path = svg.get(0).unwrap().file.as_str().to_string();
            image
        } else {
            Image::default()
        };
        if variant.answer.is_some() {
            game_data.answer = AnswerObject {
                text: variant
                    .answer
                    .as_ref()
                    .unwrap()
                    .value
                    .as_ref()
                    .unwrap()
                    .to_string(),
            }
        }

        let mut game_objects: Vec<GameObject> = [].to_vec();

        if variant.text.is_some() {
            for opt in variant.text.as_ref().unwrap().iter() {
                game_objects.push(GameObject {
                    position: Position { x: 0.0, y: 0.0 },
                    dimensions: self::Dimensions {
                        width: 0.0,
                        height: 0.0,
                    },
                    order: ObjectOrder::InOut,
                    text: TextObject {
                        position: Position { x: opt.x, y: opt.y },
                        text: TObject {
                            singular: opt.text.as_ref().unwrap().to_string(),
                            ..Default::default()
                        },
                        centered: match opt.centered.as_ref() {
                            Some(size) => if size == "yes" { true}else{false},
                            None => false,
                        },
                        size: match opt.size.as_ref() {
                            None => ObjectSize::Small,
                            Some(ss) =>  ObjectSize::from_string(ss)
                        },
                    },
                    is_option: false,
                    is_correct: false
                });
            }
        }

        if variant.options.is_some() {
            for opt in variant.options.as_ref().unwrap().iter() {
                game_objects.push(GameObject {
                    position: Position { x: opt.x, y: opt.y },
                    dimensions: self::Dimensions {
                        width: opt.width,
                        height: opt.height,
                    },
                    order: ObjectOrder::from_string(opt.order.as_str()),
                    text: TextObject {
                        position: Position { x: opt.x, y: opt.y },
                        text: TObject {
                            singular: opt
                                .text
                                .as_ref()
                                .unwrap()
                                .text
                                .as_ref()
                                .unwrap()
                                .to_string(),
                            ..Default::default()
                        },
                        centered: false,
                        size: ObjectSize::Medium,
                    },
                    is_option: true,
                    is_correct:   if opt.correct == "yes" { true}else{false}
                });
            }
        } 
        game_data.objects = game_objects;
    }

    game_data.question = question;
    game_data
}
