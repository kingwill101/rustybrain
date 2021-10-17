extern crate lazy_static;
extern crate serde;
extern crate serde_json;

use std::borrow::Borrow;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

use lazy_static::lazy_static;
use libgbrainy::engine::context::GameContext;

use libgbrainy::engine::Engine;
use libgbrainy::engine::game::{ GameData, GameObject};
use libgbrainy::engine::manager::Manager;


lazy_static! {
    static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::default());
}

#[no_mangle]
pub extern "C" fn engine_init_game_manager() -> bool {
    let bytes = include_bytes!("../../data/games.xml");

    let collection =
        libgbrainy::reader::parse_game_data(Box::from(&*String::from_utf8_lossy(bytes)));

    match collection {
        None => return false,
        Some(data) => {
            MANAGER.lock().unwrap().load_games(data.games);
            return true;
        }
    }
}

#[no_mangle]
pub extern "C" fn engine_get_games_count() -> u32 {
    MANAGER.lock().unwrap().game_count()
}

#[no_mangle]
pub extern "C" fn engine_context_new() -> *mut GameContext {
    let game_context = GameContext::new(MANAGER.lock().unwrap().random_game().to_owned());

    Box::into_raw(Box::new(game_context))
}

#[no_mangle]
pub extern "C" fn engine_context_get_question(ptr: *mut GameContext) -> *mut c_char {
    if ptr.is_null() {
        return CString::new(String::from("")).unwrap().into_raw();
    }

    let context = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let question_c = CString::new(context.get_question()).unwrap();
    question_c.into_raw()
}

#[no_mangle]
pub extern "C" fn engine_context_get_drawables(ptr: *mut GameContext) -> *mut c_char {
    if ptr.is_null() {
        return CString::new(String::from("")).unwrap().into_raw();
    }

    let context = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let json = serde_json::to_string(&context.get_drawing_objects()).unwrap();
    println!("{}", json);
    let objects = CString::new(json).unwrap();
    objects.into_raw()
}

#[no_mangle]
pub extern "C" fn engine_context_check_answer(ptr: *mut GameContext, s: *const c_char) -> bool {
    if ptr.is_null() {
        return false;
    }

    let context = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let c_str = unsafe {
        if s.is_null() {
            return false;
        }

        CStr::from_ptr(s)
    };

    context.check_answer(c_str.to_str().unwrap())
}

#[no_mangle]
pub extern "C" fn engine_free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

// #[no_mangle]
// pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
//     let c_str = unsafe { CStr::from_ptr(to) };
//     let recipient = match c_str.to_str() {
//         Err(_) => "there",
//         Ok(string) => string,
//     };
//
//     CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
// }
//
// #[no_mangle]
// pub extern fn rust_cstr_free(s: *mut c_char) {
//     unsafe {
//         if s.is_null() { return }
//         CString::from_raw(s)
//     };
// }

// #[no_mangle]
// pub extern "C" fn generate_question() -> *mut c_char {
//     let bytes = include_bytes!("../../data/games.xml");
//
//     let collection = libgbrainy::reader::parse_game_data(
//         Box::from(
//             &*String::from_utf8_lossy(bytes)
//         )
//     );
//
//     let mut manager = libgbrainy::engine::manager::Manager::new();
//     MANAGER.load_games(collection.unwrap().games);
//     match serde_json::to_string(MANAGER.random_game()) {
//         Ok(json) => {
//             CString::new(json).unwrap().into_raw()
//         }
//         Err(_) => {
//             CString::new("{'error': true}").unwrap().into_raw()
//         }
//     }
// }
//
// #[no_mangle]
// pub extern "C" fn get_question() -> Box<GameData> {
//     let bytes = include_bytes!("../../data/games.xml");
//
//     let collection = libgbrainy::reader::parse_game_data(
//         Box::from(
//             &*String::from_utf8_lossy(bytes)
//         )
//     );
//
//     let mut manager = libgbrainy::engine::manager::Manager::new();
//     MANAGER.load_games(collection.unwrap().games);
//     let mut game = MANAGER.random_game();
//     Box::new(game.clone())
// }
