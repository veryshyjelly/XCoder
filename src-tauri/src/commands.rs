use std::fs;

use tauri::api::process::{Command, CommandEvent};

use crate::judge;
use crate::judge::Verdict;
use crate::problem::{get_problems_list, get_solved_problems, FullProblem, Problem, ProblemId};
use crate::store::{ContestType, Language, StoreState};

#[tauri::command]
pub fn new_directory(directory: String, store: tauri::State<'_, StoreState>) -> Result<(), String> {
    fs::create_dir(&directory).map_err(|e| e.to_string())?;
    store.0.lock().unwrap().directory = directory;
    Ok(())
}

#[tauri::command]
pub fn set_directory(directory: String, store: tauri::State<'_, StoreState>) -> Result<(), String> {
    store.0.lock().unwrap().directory = directory;
    Ok(())
}

#[tauri::command]
pub fn set_editor(editor: String, store: tauri::State<'_, StoreState>) -> Result<(), String> {
    store.0.lock().unwrap().editor = editor;
    Ok(())
}

#[tauri::command]
pub fn get_directory(store: tauri::State<'_, StoreState>) -> Result<String, ()> {
    Ok(store.0.lock().unwrap().directory.clone())
}

#[tauri::command]
pub fn get_editor(store: tauri::State<'_, StoreState>) -> Result<String, ()> {
    Ok(store.0.lock().unwrap().editor.clone())
}

#[tauri::command]
pub fn set_contest_type(
    store: tauri::State<'_, StoreState>,
    contest_type: String,
) -> Result<(), String> {
    store.0.lock().unwrap().contest_type = ContestType::from_str(contest_type.as_str())?;
    store
        .0
        .lock()
        .unwrap()
        .filter_problems()
        .map_err(|err| format!("error while filtering problems: {}", err))
}

#[tauri::command]
pub fn get_contest_type(store: tauri::State<'_, StoreState>) -> Result<String, ()> {
    Ok(store
        .0
        .lock()
        .unwrap()
        .contest_type
        .to_string()
        .to_lowercase())
}

#[tauri::command]
pub fn set_problem_type(
    store: tauri::State<'_, StoreState>,
    problem_types: Vec<String>,
) -> Result<(), String> {
    if problem_types.len() == 0 {
        return Err("problem types cannot be empty".into());
    }

    let mut pts = vec![];
    for pt in problem_types {
        let problem_type = ProblemId::from_str(pt.as_str())?;
        pts.push(problem_type);
    }

    store.0.lock().unwrap().problem_types = pts;
    store
        .0
        .lock()
        .unwrap()
        .filter_problems()
        .map_err(|err| format!("error while filtering problems: {}", err))
}

#[tauri::command]
pub fn get_problem_type(store: tauri::State<'_, StoreState>) -> Result<Vec<String>, ()> {
    let mut pts = vec![];
    for pt in store.0.lock().unwrap().problem_types.clone() {
        pts.push(pt.to_string().to_lowercase());
    }
    Ok(pts)
}

#[tauri::command]
pub fn set_language(store: tauri::State<'_, StoreState>, language: String) -> Result<(), String> {
    store.0.lock().unwrap().language = Language::from_str(language.as_str())
        .map_err(|err| format!("error while setting language: {}", err))?;
    Ok(())
}

#[tauri::command]
pub fn get_language(store: tauri::State<'_, StoreState>) -> Result<String, ()> {
    Ok(store.0.lock().unwrap().language.to_string().to_lowercase())
}

#[tauri::command]
pub fn set_show_solved(
    store: tauri::State<'_, StoreState>,
    show_solved: bool,
) -> Result<(), String> {
    store.0.lock().unwrap().show_solved = show_solved;
    store
        .0
        .lock()
        .unwrap()
        .filter_problems()
        .map_err(|err| format!("error while filtering problems: {}", err))
}

#[tauri::command]
pub fn get_show_solved(store: tauri::State<'_, StoreState>) -> Result<bool, ()> {
    Ok(store.0.lock().unwrap().show_solved)
}

#[tauri::command]
pub fn next(store: tauri::State<'_, StoreState>) -> Result<(), String> {
    store.0.lock().unwrap().index += 1;
    Ok(())
}

#[tauri::command]
pub fn previous(store: tauri::State<'_, StoreState>) -> Result<(), String> {
    if store.0.lock().unwrap().index != 0 {
        store.0.lock().unwrap().index -= 1;
        Ok(())
    } else {
        Err("reached at top of list".into())
    }
}

#[tauri::command]
pub async fn get_problem(store: tauri::State<'_, StoreState>) -> Result<FullProblem, String> {
    let directory = store.0.lock().unwrap().directory.clone();

    if store.0.lock().unwrap().solved_problems.is_none() {
        store.0.lock().unwrap().solved_problems = Some(get_solved_problems(directory)?);
    }

    if store.0.lock().unwrap().problems_list.is_none() {
        store.0.lock().unwrap().problems_list = Some(get_problems_list().await?);
        store
            .0
            .lock()
            .unwrap()
            .filter_problems()
            .map_err(|err| format!("error while filtering problems: {}", err))?;
    }

    let mut problem = store.0.lock().unwrap().get_problem()?;
    problem.scrape().await?;

    match problem {
        Problem::Full(full_problem) => Ok(full_problem),
        _ => Err("error while getting full problem".into()),
    }
}

#[tauri::command]
pub async fn update_problems_list() -> Result<(), String> {
    // here we have to use side car build in go to update the problem set
    let (mut rx, _) = Command::new_sidecar("problems_sidecar")
        .expect("failed to create `problems_sidecar` binary command")
        .spawn()
        .map_err(|err| format!("Failed to spawn sidecar: {}", err))?;

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                println!("message from sidecar: {}", line)
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn run(store: tauri::State<'_, StoreState>) -> Result<Vec<Verdict>, String> {
    let mut problem = store.0.lock().unwrap().get_problem()?.clone();
    let directory = store.0.lock().unwrap().directory.clone();
    let language = store.0.lock().unwrap().language.clone();
    problem.scrape().await?;
    match problem {
        Problem::Full(problem) => judge::run(problem, directory, language).await,
        _ => Err("error while getting full problem".into()),
    }
}

#[tauri::command]
pub async fn submit(store: tauri::State<'_, StoreState>) -> Result<Vec<Verdict>, String> {
    let mut problem = store.0.lock().unwrap().get_problem()?.clone();
    let directory = store.0.lock().unwrap().directory.clone();
    let language = store.0.lock().unwrap().language.clone();
    problem.scrape().await?;
    match problem {
        Problem::Full(problem) => {
            let res = judge::submit(problem, directory, language).await;
            store.0.lock().unwrap().filter_problems()?;
            res
        }
        _ => Err("error while getting full problem".into()),
    }
}

#[tauri::command]
pub fn create_file(store: tauri::State<'_, StoreState>) -> Result<(), String> {
    store.0.lock().unwrap().create_file()
}

#[tauri::command]
pub fn open_file(store: tauri::State<'_, StoreState>) -> Result<(), String> {
    store.0.lock().unwrap().open_file_in_editor()
}

#[tauri::command]
pub fn save_state(store: tauri::State<'_, StoreState>) -> Result<(), String> {
    println!("saving state");
    store.0.lock().unwrap().save()
}
