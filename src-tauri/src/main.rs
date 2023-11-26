// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::{
    get_directory, get_problem, next, previous, run, set_contest_type, set_directory, set_language,
    set_problem_type, submit,
};

use crate::store::StoreState;

mod commands;
mod judge;
mod problem;
mod store;

fn main() {
    tauri::Builder::default()
        .manage(StoreState::new())
        .invoke_handler(tauri::generate_handler![
            set_directory,
            get_directory,
            set_contest_type,
            set_problem_type,
            set_language,
            next,
            get_problem,
            previous,
            run,
            submit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
