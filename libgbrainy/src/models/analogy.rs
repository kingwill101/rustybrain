use serde_derive::{Deserialize, Serialize};

use crate::models::shared::{Answer, Question};

#[derive(Debug, Serialize, Deserialize, )]
pub struct AnalogyCollection {
    #[serde(rename = "analogy")]
    pub analogies: Option<Vec<Analogy>>,
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct Analogy {
    #[serde(rename = "_question")]
    pub question: Option<Question>,

    #[serde(rename = "_answer")]
    pub plural: Option<Answer>,
}
