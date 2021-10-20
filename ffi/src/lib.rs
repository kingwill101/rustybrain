extern crate lazy_static;
extern crate serde;
extern crate serde_json;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

use lazy_static::lazy_static;

use libgbrainy::engine::context::GameContext;
use libgbrainy::engine::manager::Manager;

lazy_static! {
    static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::default());
}

#[no_mangle]
/// # Safety
///
/// This should be called before any other ffi functions are invoked
pub unsafe extern "C" fn engine_init_game_manager() -> bool {
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

/// # Safety
///
/// Never call before initializing with engine_init_game_manager
#[no_mangle]
pub unsafe extern "C" fn engine_get_games_count() -> u32 {
    MANAGER.lock().unwrap().game_count()
}

/// # Safety
///
/// Never call before initializing with engine_init_game_manager
#[no_mangle]
pub unsafe extern "C" fn engine_context_new() -> *mut GameContext {
    let game_context = GameContext::new(MANAGER.lock().unwrap().random_game().to_owned());

    Box::into_raw(Box::new(game_context))
}

/// # Safety
///
/// Always make sure that GameContext ptr is always valid before passing
#[no_mangle]
pub unsafe extern "C" fn engine_context_get_question(ptr: *mut GameContext) -> *mut c_char {
    if ptr.is_null() {
        return to_c_str("".to_string());
    }

    let context = &mut *ptr;

    let question_c = CString::new(context.get_question()).unwrap();
    question_c.into_raw()
}

/// # Safety
///
/// Always make sure that GameContext ptr is always valid before passing
#[no_mangle]
pub unsafe extern "C" fn engine_context_get_rationale(ptr: *mut GameContext) -> *mut c_char {
    if ptr.is_null() {
        return to_c_str("".to_string());
    }

    let context = &mut *ptr;

    let question_c = CString::new(context.get_rationale()).unwrap();
    question_c.into_raw()
}

/// # Safety
///
/// Always make sure that GameContext ptr is always valid before passing
#[no_mangle]
pub unsafe extern "C" fn engine_context_get_name(ptr: *mut GameContext) -> *mut c_char {
    if ptr.is_null() {
        return to_c_str("".to_string());
    }

    let context = &mut *ptr;

    let name_c = CString::new(context.get_name()).unwrap();
    name_c.into_raw()
}

/// # Safety
///
/// Always make sure that GameContext ptr is always valid before passing
#[no_mangle]
pub unsafe extern "C" fn engine_context_get_drawables(ptr: *mut GameContext) -> *mut c_char {
    if ptr.is_null() {
        return to_c_str("".to_string());
    }

    let context = &mut *ptr;

    let json = serde_json::to_string(&context.get_drawing_objects()).unwrap();
    let objects = CString::new(json).unwrap();
    objects.into_raw()
}

/// # Safety
///
/// Always make sure that GameContext ptr is always valid before passing
#[no_mangle]
pub unsafe extern "C" fn engine_context_get_image(ptr: *mut GameContext) -> *mut c_char {
    if ptr.is_null() {
        return to_c_str("".to_string());
    }

    let context = &mut *ptr;

    let json = serde_json::to_string(&context.get_image()).unwrap();

    let objects = CString::new(json).unwrap();
    objects.into_raw()
}

/// # Safety
///
/// Always make sure that GameContext ptr is always valid before passing
#[no_mangle]
pub unsafe extern "C" fn engine_context_get_possible_answers(ptr: *mut GameContext) -> *mut c_char {
    if ptr.is_null() {
        return to_c_str("".to_string());
    }

    let context = &mut *ptr;

    let json = serde_json::to_string(&context.get_drawing_objects()).unwrap();
    println!("{}", json);
    let objects = CString::new(json).unwrap();
    objects.into_raw()
}

/// # Safety
///
/// Always make sure that GameContext ptr is always valid before passing
#[no_mangle]
pub unsafe extern "C" fn engine_context_check_answer(
    ptr: *mut GameContext,
    s: *const c_char,
) -> bool {
    if ptr.is_null() {
        return false;
    }

    let context = &mut *ptr;

    let c_str = {
        if s.is_null() {
            return false;
        }

        CStr::from_ptr(s)
    };

    context.check_answer(c_str.to_str().unwrap())
}

/// # Safety
///
/// Always make sure that GameContext ptr is always valid before passing
#[no_mangle]
pub unsafe extern "C" fn engine_context_get_option_prefix(
    ptr: *mut GameContext,
    index: u8,
    content: *const c_char,
) -> *mut c_char {
    if ptr.is_null() {
        return to_c_str("".to_string());
    }

    let context = &mut *ptr;

    let c_str = {
        assert!(!content.is_null());
        CStr::from_ptr(content)
    };

    to_c_str(context.replace_option_answer_prefix(index, c_str.to_str().unwrap()))
}

/// # Safety
///
/// This should always be called after consuming any rust strings
#[no_mangle]
pub unsafe extern "C" fn engine_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    CString::from_raw(s);
}

pub fn to_c_str(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}
