extern crate log;

use std::borrow::BorrowMut;

use boa::Context;
use log::{info, warn};

use crate::engine::utils::{clean_var_name, get_variables};


mod utils;
pub mod game;
pub mod manager;

pub struct Engine {
    context: Context,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            context: Context::new()
        }
    }

    fn parse(&mut self, src: String) {
        log::info!("parsing: \n {}", src);
        match self.context.eval(src) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", format!("{}", e.display()))
            }
        }
    }

    pub fn parse_variables(&mut self, variables: &str) {
        self.parse(variables.trim().to_string())
    }

    pub fn find_var(&mut self, var: &str) -> Option<String> {
        let result = self.context.eval(var);

        match result {
            Ok(val) => {
                if val.is_null_or_undefined() {
                    return None;
                }
                Some(String::from(val.to_string(self.context.borrow_mut()).unwrap().as_str()))
            }
            Err(_) => None,
        }
    }

    pub fn interop(&mut self, content: &str) -> String {
        let mut replaced = String::from(content);
        let mut variable_name: String;
        for v in get_variables(content).iter() {
            info!("processing {}", v);
            variable_name = clean_var_name(v);

            let result = self.find_var(variable_name.as_str());

            if result.is_none() {
                warn!("variable {} was not found", v);
                continue;
            }
            info!("updating {} with  {}", v,result.as_ref().unwrap().as_str());
            replaced = replaced.replace(v, result.as_ref().unwrap().as_str());
        }

        replaced
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::engine::Engine;

    #[test]
    fn value_lookup() {
        let mut map = HashMap::new();
        map.insert("let a=1;", "1");
        map.insert("let a=2;", "2");
        map.insert("let a=3;", "3");
        map.insert("let a=4;", "4");

        for i in map.iter() {
            let mut engine = Engine::new();

            engine.parse(i.0.to_string());
            let val = engine.find_var("a");
            assert!(val.is_some());
            assert_eq!(val.unwrap(), i.1.to_string());
        }

        let mut engine = Engine::new();
        let val = engine.find_var("a");
        assert!(val.is_none());
    }

    #[test]
    fn variable_parsing() {
        let mut engine = Engine::new();
        engine.parse_variables("let num= 10");
        let val = engine.find_var("num");
        assert_eq!("10", val.unwrap())
    }

    #[test]
    fn interop_test() {
        let mut engine = Engine::new();
        engine.parse_variables(r##"
        let age= 10;
        let first_name = "Glenford";
        let last_name = "Williams"
        "##);
        let mut result = engine.interop("[age]");
        assert_eq!(result, "10");

        result = engine.interop("My name is [first_name] [last_name] and i am [age] years old");
        assert_eq!(result, "My name is Glenford Williams and i am 10 years old");
    }
}
