use crate::problem::{Problem, ProblemId};
use crate::store::{ContestType, Language, StoreState};

#[tauri::command]
pub fn set_directory(directory: String, store: tauri::State<'_, StoreState>) -> Result<(), String> {
    store.0.lock().unwrap().directory = directory;
    Ok(())
}

#[tauri::command]
pub fn get_directory(store: tauri::State<'_, StoreState>) -> Result<String, ()> {
    Ok(store.0.lock().unwrap().directory.clone())
}

#[tauri::command]
pub fn set_contest_type(
    store: tauri::State<'_, StoreState>,
    contest_type: String,
) -> Result<(), String> {
    store.0.lock().unwrap().contest_type = ContestType::from_str(contest_type.as_str())?;
    store.0.lock().unwrap().filter_problems()
}

#[tauri::command]
pub fn set_problem_type(
    store: tauri::State<'_, StoreState>,
    problem_types: Vec<String>,
) -> Result<(), String> {
    let mut pts = vec![];
    for pt in problem_types {
        let problem_type = ProblemId::from_str(pt.as_str())?;
        pts.push(problem_type);
    }

    store.0.lock().unwrap().problem_types = pts;
    store.0.lock().unwrap().filter_problems()
}

#[tauri::command]
pub fn set_language(store: tauri::State<'_, StoreState>, language: String) -> Result<(), String> {
    store.0.lock().unwrap().language = Language::from_str(language.as_str())?;
    Ok(())
}

#[tauri::command]
pub fn set_show_solved(
    store: tauri::State<'_, StoreState>,
    show_solved: bool,
) -> Result<(), String> {
    store.0.lock().unwrap().show_solved = show_solved;
    store.0.lock().unwrap().filter_problems()
}

#[tauri::command]
pub fn next(store: tauri::State<'_, StoreState>) {
    store.0.lock().unwrap().index += 1;
}

#[tauri::command]
pub async fn get_problem(store: tauri::State<'_, StoreState>) -> Result<Problem, String> {
    store.0.lock().unwrap().get_problem().await
}

#[tauri::command]
pub fn previous(store: tauri::State<'_, StoreState>) {
    store.0.lock().unwrap().index = usize::max(1, store.0.lock().unwrap().index) - 1
}

#[tauri::command]
pub fn run() {}

#[tauri::command]
pub fn submit() {}