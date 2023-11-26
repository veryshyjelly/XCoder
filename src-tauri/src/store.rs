use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::problem::*;

#[derive(Serialize, Deserialize)]
pub struct Store {
    pub contest_type: ContestType,
    pub problem_types: Vec<ProblemId>,
    pub language: Language,
    pub directory: String,
    pub show_solved: bool,
    #[serde(skip)]
    pub problems_list: Option<Vec<Problem>>,
    #[serde(skip)]
    pub filtered_problems: Option<Vec<Problem>>,
    #[serde(skip)]
    pub solved_problems: Option<Vec<Problem>>,
    #[serde(skip)]
    pub index: usize,
}

pub struct StoreState(pub Mutex<Store>);

impl StoreState {
    pub fn new() -> StoreState {
        StoreState(Mutex::new(Store::read_or_create().unwrap()))
    }
}

impl Store {
    pub fn new(directory: String) -> Store {
        Store {
            contest_type: ContestType::ABC,
            problem_types: vec![
                ProblemId::A,
                ProblemId::B,
                ProblemId::C,
                ProblemId::D,
                ProblemId::E,
                ProblemId::F,
                ProblemId::G,
                ProblemId::H,
            ],
            language: Language::Cpp,
            directory,
            show_solved: true,
            problems_list: None,
            filtered_problems: None,
            solved_problems: None,
            index: 0,
        }
    }

    pub fn read_or_create() -> Result<Store, String> {
        if Path::new("store.json").exists() {
            Store::read()
        } else {
            Store::create("".into())
        }
    }

    pub fn create(directory: String) -> Result<Store, String> {
        let file = File::create("store.json");
        match file {
            Ok(f) => {
                let mut v = Store::new(directory);
                match serde_json::to_writer(f, &v) {
                    Ok(_) => Ok(v),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn read() -> Result<Store, String> {
        let file = File::open("store.json");
        match file {
            Ok(f) => match serde_json::from_reader(f) {
                Ok(v) => Ok(v),
                Err(err) => Err(err.to_string()),
            },
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn save(self) -> Result<(), String> {
        let file = File::open("store.json");
        match file {
            Ok(f) => match serde_json::to_writer(f, &self) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string()),
            },
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn fill_problems(&mut self) -> Result<(), String> {
        if self.solved_problems.is_none() {
            self.solved_problems = Some(get_solved_problems()?)
        }

        if self.problems_list.is_none() {
            self.problems_list = Some(get_problems_list().await?);
            self.filter_problems()?;
        }

        Ok(())
    }

    pub fn filter_problems(&mut self) -> Result<(), String> {
        let mut filtered_problems = self.problems_list.clone().unwrap();
        if !self.show_solved {
            filtered_problems = filtered_problems
                .into_iter()
                .filter(|x| !self.solved_problems.as_ref().unwrap().contains(x))
                .collect();
        }

        filtered_problems = filtered_problems
            .into_iter()
            .filter(|x| {
                x.contest_type.eq(&self.contest_type) && self.problem_types.contains(&x.problem_id)
            })
            .collect();

        self.filtered_problems = Some(filtered_problems);
        self.index = 0;

        Ok(())
    }

    pub async fn get_problem(&mut self) -> Result<Problem, String> {
        self.fill_problems().await?;
        if let Some(problem) = self.filtered_problems.as_ref().unwrap().get(self.index) {
            let mut pr = problem.clone();
            pr.scrape().await?;
            Ok(pr)
        } else {
            Err("index out of range".into())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum ContestType {
    ABC,
    AGC,
    ARC,
    AHC,
}

impl fmt::Display for ContestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ContestType {
    pub fn from_str(str: &str) -> Result<ContestType, String> {
        match str.to_uppercase().as_str() {
            "ABC" => Ok(ContestType::ABC),
            "AGC" => Ok(ContestType::AGC),
            "ARC" => Ok(ContestType::ARC),
            "AHC" => Ok(ContestType::AHC),
            _ => Err("invalid contest type".into()),
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub enum Language {
    C,
    Cpp,
    Go,
    Rust,
    Kotlin,
    Zig,
    Node,
    R,
    Swift,
    Dart,
    Julia,
    Haskell,
    Fortran,
    Python,
    Ocaml,
}

impl Language {
    pub fn from_str(str: &str) -> Result<Language, String> {
        match str.to_lowercase().as_str() {
            "c" => Ok(Language::C),
            "cpp" => Ok(Language::Cpp),
            "go" => Ok(Language::Go),
            "rust" => Ok(Language::Rust),
            "kotlin" => Ok(Language::Kotlin),
            "zig" => Ok(Language::Zig),
            "node" => Ok(Language::Node),
            "r" => Ok(Language::R),
            "swift" => Ok(Language::Swift),
            "dart" => Ok(Language::Dart),
            "julia" => Ok(Language::Julia),
            "haskell" => Ok(Language::Haskell),
            "fortran" => Ok(Language::Fortran),
            "python" => Ok(Language::Python),
            "ocaml" => Ok(Language::Ocaml),
            _ => Err("invalid language".into()),
        }
    }
}