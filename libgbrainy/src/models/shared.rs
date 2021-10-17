use serde_derive::{Deserialize, Serialize};

use crate::models::game::GameOption;

#[derive(Debug, Serialize, Deserialize, )]
pub struct Variant {
    #[serde(rename = "variables")]
    pub variables:  Option<String>,

    // #[serde(rename = "_question")]
    // pub _question: Option<Vec<String>>,

    pub question: Option<Vec<Question>>,

    #[serde(rename = "rationale")]
    pub rationale: Option<Vec<String>>,
    //
    // #[serde(rename = "_rationale")]
    // pub rationale: Option<String>,

    #[serde(rename = "string")]
    pub text: Option<Vec<Text>>,

    #[serde(rename = "svg")]
    pub svg: Option<Vec<Svg>>,

    #[serde(rename = "tip")]
    pub tip: Option<String>,
    //
    // #[serde(rename = "_tip")]
    // pub tip: Option<String>,

    #[serde(rename = "answer")]
    pub answer: Option<Answer>,

    #[serde(rename = "answer_checkattributes")]
    pub answer_checkattributes: Option<String>,

    #[serde(rename = "answer_expression")]
    pub answer_expression: Option<String>,

    #[serde(rename = "answer_show")]
    pub answer_show: Option<String>,

    #[serde(rename = "option")]
    pub options: Option<Vec<GameOption>>
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct Svg {
    pub file: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct Text {
    // #[serde(rename = "_text")]
    // pub _text:  Option<String>,

    #[serde(rename = "text")]
    pub text:  Option<String>,
    pub size: Option<String>,
    pub x: f64,
    pub y: f64,
    pub centered: Option<String>,
}

#[derive(Default,Debug, Serialize, Deserialize, )]
pub struct Question {
    #[serde(rename = "$value")]
    pub string: Option<String>,

    #[serde(rename = "plural")]
    pub plural: Option<String>,

    #[serde(rename = "type")]
    pub question_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct Answer {
    #[serde(rename = "$value")]
    pub value: Option<String>,

    #[serde(rename = "correct")]
    pub correct: Option<String>,
}

