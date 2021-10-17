extern crate serde;
extern crate serde_json;

use crate::engine::Engine;
use crate::engine::game::{GameData, GameObject};

pub struct GameContext {
    engine: crate::engine::Engine,
    game: GameData,
}

impl GameContext {
    const SEPARATOR: &'static str = "|";
    const OPTION_PREFIX: &'static str = "option_prefix";

    pub fn new(game: GameData) -> GameContext {
        let mut engine = Engine::new();
        engine.parse_variables(&game.variables);

        engine.find_var(&game.answer.to_owned().text.as_str());

        GameContext { engine, game }
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

    pub fn get_drawing_objects(&mut self) -> Vec<GameObject> {
        println!("{} objects found", self.game.objects.len());
        self.game.objects.clone()
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
            if right.to_string() == str_answer {
                return true;
            }
        }
        false
    }

    pub fn replace_option_answer_prefix(&mut self, index: u8, content: &str) -> String {
        let int_var: u8 = 65 + index;
        let char_var: char = int_var as char;

        self.engine.set_str_var(GameContext::OPTION_PREFIX, format!("{}", char_var).as_str());
        self.engine.interop(content)
    }

    pub fn options_possible_answers(&mut self) -> String {
        let parts: Vec<&str> = self.game.answer.text.split(GameContext::SEPARATOR).collect();
        let mut builder = string_builder::Builder::default();
        let count = parts.len();

        if count == 1 || count == 2 {
            match count {
                1 => builder.append(format!("{}", parts[0].trim())),
                2 => {
                    builder.append(format!("{} or {}", parts[0].trim(), parts[1].trim()));
                }
                _ => {}
            };
        } else {
            for val in 0..count {
                if val == (count - 1) {
                    builder.append(format!(" or {}", parts[val].trim()));
                } else {
                    if val != 0 {
                        if val == count - 2 {
                            builder.append(format!("{}", parts[val].trim()));
                        } else {
                            builder.append(format!("{}, ", parts[val].trim()));
                        }
                    } else {
                        builder.append(format!("{}, ", parts[val].trim()));
                    }
                }
            }
        }
        builder.string().unwrap()
    }

    pub fn options_possible_answers_interop(&mut self) -> String {
        let s = self.options_possible_answers();
        self.engine.interop(s.as_str())
    }
}

impl Default for GameContext {
    fn default() -> Self {
        GameContext {
            engine: Engine::new(),
            game: GameData::default(),
        }
    }
}

#[test]
pub fn multiple_option_answer_prefix_test() {
    let mut context = GameContext::default();

    let res = context.replace_option_answer_prefix(0, "[option_prefix] a question");

    assert_eq!(res, "A a question");
}

#[test]
pub fn options_possible_answers_test() {
    let mut context = GameContext::default();

    context.game.answer.text = "[a]".parse().unwrap();
    assert_eq!(context.options_possible_answers(), "[a]");

    context.game.answer.text = "[a]|[b]".parse().unwrap();
    assert_eq!(context.options_possible_answers(), "[a] or [b]");

    context.game.answer.text = "[a]|[b]|[c]".parse().unwrap();
    assert_eq!(context.options_possible_answers(), "[a], [b] or [c]");

    context.game.answer.text = "[a]|[b]|[c]|[d]".parse().unwrap();
    assert_eq!(context.options_possible_answers(), "[a], [b], [c] or [d]");

    context.game.answer.text = "[a]|[b]|[c]|[d]|[e]".parse().unwrap();
    assert_eq!(context.options_possible_answers(), "[a], [b], [c], [d] or [e]");
}

#[test]
pub fn options_possible_answers_interop_test() {
    let mut context = GameContext::default();
    context.game.answer.text = "[a]|[b]".parse().unwrap();
    context.engine.set_str_var("a", "1");
    context.engine.set_str_var("b", "2");
    assert_eq!(context.options_possible_answers_interop(), "1 or 2");

    context.game.answer.text = "[a]|[b]|[c]|[d]".parse().unwrap();
    context.engine.set_str_var("a", "1");
    context.engine.set_str_var("b", "2");
    context.engine.set_str_var("c", "3");
    context.engine.set_str_var("d", "4");
    assert_eq!(context.options_possible_answers_interop(), "1, 2, 3 or 4");
}