// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::{
    create_file, get_contest_type, get_directory, get_language, get_problem, get_problem_type,
    get_show_solved, new_directory, next, previous, run, save_state, set_contest_type,
    set_directory, set_language, set_problem_type, set_show_solved, submit, update_problems_list,
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
            new_directory,
            set_directory,
            get_directory,
            set_contest_type,
            get_contest_type,
            set_problem_type,
            get_problem_type,
            set_language,
            get_language,
            set_show_solved,
            get_show_solved,
            get_problem,
            next,
            previous,
            run,
            submit,
            update_problems_list,
            save_state,
            create_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
