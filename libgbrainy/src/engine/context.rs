extern crate serde;
extern crate serde_json;
use crate::engine::Engine;
use crate::engine::game::{GameData, GameObject, Image};

fn make_context(game: GameData) -> GameContext {
    let engine = Engine::new();
    let mut context = GameContext { engine, game };
    context.init_vars();

    context
}

pub struct GameContext {
    engine: crate::engine::Engine,
    game: GameData,
}

impl GameContext {
    // const SEPARATOR: &'static str = "|";
    const OPTION_PREFIX: &'static str = "option_prefix";

    pub fn new(game: GameData) -> GameContext {
        make_context(game)
    }

    pub fn init_vars(&mut self) {

        self.engine.parse_variables(self.game.variables.as_str());

        if self.get_options_count() >= 1 {
            let answer = self.options_answers_interop();
            self.engine
                .set_str_var("option_answers", answer.as_str());
        }
    }

    pub fn get_name(&mut self) -> String {
        self.game.name.clone()
    }

    pub fn get_image(&self) -> Image {
        Image::default()
    }

    pub fn get_question(&mut self) -> String {
        self.engine.interop(
            crate::engine::utils::pluralize(
                self.game.question.text.singular.as_str(),
                self.game.question.text.plural.as_str(),
                1,
            )
                .as_str(),
        )
    }
    pub fn get_rationale(&mut self) -> String {
        self.engine.interop(
            crate::engine::utils::pluralize(
                self.game.rationale.text.singular.as_str(),
                self.game.rationale.text.plural.as_str(),
                1,
            )
                .as_str(),
        )
    }

    pub fn get_drawing_objects(&self) -> &Vec<GameObject> {
        println!("{} objects found", self.game.objects.len());
        &self.game.objects
    }

    pub fn check_answer(&mut self, answer: &str) -> bool {
        if answer.is_empty() {
            return false;
        }
        let str_answer = answer.to_string();

        let mut right_answers: Vec<&str> = self
            .game
            .answer
            .text
            .split(GameContext::OPTION_PREFIX)
            .collect();

        for right in right_answers.iter_mut() {
            *right = right.trim();
        }

        for right in right_answers.iter() {
            if *right == str_answer {
                return true;
            }
        }
        false
    }

    pub fn get_options_count(&self) -> u32 {
        let mut count = 0;
        for obj in self.game.objects.iter() {
            if obj.is_option {
                count += 1;
            }
        }
        count
    }

    fn get_char(&mut self, index: u8) -> String {
        let int_var: u8 = 65 + index;
        let char_var: char = int_var as char;

        char_var.to_string()
    }

    pub fn replace_option_answer_prefix(&mut self, index: u8, content: &str) -> String {
        let char = self.get_char(index);
        self.engine
            .set_str_var(GameContext::OPTION_PREFIX, char.as_str());
        self.engine.interop(content)
    }

    fn multi_option_answers(&mut self) -> String {
        let mut builder = string_builder::Builder::default();
        let count = self.get_options_count();

        let mut get_index_value = |index| self.get_char(index as u8);

        if count == 1 || count == 2 {
            match count {
                1 => builder.append(get_index_value(0)),
                2 => {
                    builder.append(format!("{} or {}", get_index_value(0), get_index_value(1)));
                }
                _ => {}
            };
        } else {
            for val in 0..count {
                if val == (count - 1) {
                    builder.append(format!(" or {}", get_index_value(val)));
                } else if val != 0 {
                    if val == count - 2 {
                        builder.append(get_index_value(val));
                    } else {
                        builder.append(format!("{}, ", get_index_value(val)));
                    }
                } else {
                    builder.append(format!("{}, ", get_index_value(val)));
                }
            }
        }
        builder.string().unwrap()
    }

    pub fn options_answers_interop(&mut self) -> String {
        if self.get_options_count() >= 1 {
            let answers = self.multi_option_answers();
            return self.engine.interop(answers.as_str());
        }
        String::new()
    }
}

impl Default for GameContext {
    fn default() -> Self {
        make_context(GameData::default())
    }
}

#[test]
pub fn multiple_option_answer_prefix_test() {
    let mut context = GameContext::default();

    assert_eq!(
        context.replace_option_answer_prefix(0, "[option_prefix] a question"),
        "A a question"
    );
    assert_eq!(
        context.replace_option_answer_prefix(1, "[option_prefix] a question"),
        "B a question"
    );
    assert_eq!(
        context.replace_option_answer_prefix(2, "[option_prefix] a question"),
        "C a question"
    );
    assert_eq!(
        context.replace_option_answer_prefix(3, "[option_prefix] a question"),
        "D a question"
    );
}
//
// #[test]
// pub fn options_possible_answers_test() {
//     let mut context = GameContext::default();
//
//     context.game.objects = Default::default();
//
//     context.game.answer.text = "[a]".parse().unwrap();
//
//     assert_eq!(context.options_possible_answers(), "[a]");
//
//     context.game.answer.text = "[a]|[b]".parse().unwrap();
//     assert_eq!(context.options_possible_answers(), "[a] or [b]");
//
//     context.game.answer.text = "[a]|[b]|[c]".parse().unwrap();
//     assert_eq!(context.options_possible_answers(), "[a], [b] or [c]");
//
//     context.game.answer.text = "[a]|[b]|[c]|[d]".parse().unwrap();
//     assert_eq!(context.options_possible_answers(), "[a], [b], [c] or [d]");
//
//     context.game.answer.text = "[a]|[b]|[c]|[d]|[e]".parse().unwrap();
//     assert_eq!(
//         context.options_possible_answers(),
//         "[a], [b], [c], [d] or [e]"
//     );
// }
//
// #[test]
// pub fn options_possible_answers_interop_test() {
//     let mut context = GameContext::default();
//     context.game.answer.text = "[a]|[b]".parse().unwrap();
//     assert_eq!(context.engine.set_str_var("a", "1"), true);
//     assert_eq!(context.engine.set_str_var("b", "2"), true);
//     assert_eq!(context.options_possible_answers_interop(), "1 or 2")
//
//     // context.game.answer.text = "[a]|[b]|[c]|[d]".parse().unwrap();
//     // context.engine.set_str_var("a", "1");
//     // context.engine.set_str_var("b", "2");
//     // context.engine.set_str_var("c", "3");
//     // context.engine.set_str_var("d", "4");
//     // assert_eq!(context.options_possible_answers_interop(), "1, 2, 3 or 4");
// }
