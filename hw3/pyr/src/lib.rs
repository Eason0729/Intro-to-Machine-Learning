mod app;
mod data;

use std::slice;

use app::{App, TickState};
use data::parser::*;
use data::prelude::*;
use simple_logger::SimpleLogger;

#[no_mangle]
pub unsafe extern "C" fn tick(
    app: *mut App,
    overall: &RawOverall,
    food: *mut RawFood,
    len: u64,
) -> i32 {
    let app = &mut *app;

    let state = {
        let foods: Vec<Food> = slice::from_raw_parts(food, len as usize)
            .into_iter()
            .map(|x| x.to_owned().into())
            .collect();
        TickState {
            frame: overall.frame,
            player: overall.get_player(),
            opponent: overall.get_opponent(),
            foods,
        }
    };

    app.run(state) as i32
}

#[no_mangle]
pub unsafe extern "C" fn check_point(app: *mut App) {
    let app = &mut *app;
    app.check_point();
}

#[no_mangle]
pub unsafe extern "C" fn new_app() -> *const App {
    SimpleLogger::new().init().unwrap();
    log::info!("Initializing App...");
    let a = Box::into_raw(Box::new(App::new()));
    a
}

#[no_mangle]
pub unsafe extern "C" fn drop_app(app: *mut App) {
    drop(Box::from_raw(app))
}
