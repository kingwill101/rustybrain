use libgbrainy::engine::context::GameContext;
use libgbrainy::engine::game::GameType;
use libgbrainy::engine::manager::Manager;

use std::sync::Mutex;
use wasm_bindgen::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    #[wasm_bindgen]
    static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::default());
}

#[wasm_bindgen]
pub fn init_manager() -> bool {
    let bytes = include_bytes!("../../data/games.xml");

    let collection = libgbrainy::reader::parse_game_data(&*String::from_utf8_lossy(bytes));

    return match collection {
        None => {
            println!("Parsing error");
            false
        }
        Some(data) => {
            MANAGER.lock().unwrap().load_games(data.games);
            true
        }
    };
}

#[wasm_bindgen]
pub struct WrappedContext {
    context: GameContext,
}

#[wasm_bindgen]
impl WrappedContext {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WrappedContext {
        WrappedContext {
            context: GameContext::default(),
        }
    }

    pub fn get_question(&mut self) -> String {
        self.context.get_question()
    }

    pub fn get_rationale(&mut self) -> String {
        self.context.get_rationale()
    }

    pub fn get_name(&mut self) -> String {
        self.context.get_name()
    }

    pub fn get_drawables(&mut self) -> String {
        serde_json::to_string(self.context.get_drawing_objects()).unwrap()
    }

    pub fn get_option_prefix(&mut self, index: Option<u8>, content: String) -> String {
        self.context.replace_option_answer_prefix(index.unwrap(), content.as_str())
    }
}

impl Default for WrappedContext {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn new_context() -> WrappedContext {
    let game_context = GameContext::new(MANAGER.lock().unwrap().random_game().to_owned());
    WrappedContext {
        context: game_context,
    }
}

pub fn new_by_category(cat: String) -> WrappedContext {
    let game_context = GameContext::new(
        MANAGER.lock().unwrap()
            .random_game_from_category(
                GameType::from_string(cat.as_str()
                )).to_owned());

    WrappedContext {
        context: game_context,
    }
}

#[wasm_bindgen]
pub fn new_context_by_category(cat: String) -> WrappedContext {
    let game_context = GameContext::new(
        MANAGER
            .lock()
            .unwrap()
            .random_game_from_category(GameType::from_string(&cat))
            .to_owned(),
    );
    WrappedContext {
        context: game_context,
    }
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
