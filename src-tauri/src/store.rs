use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::path::{Path, PathBuf};
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
    pub problems_list: Option<Vec<BareProblem>>,
    #[serde(skip)]
    pub filtered_problems: Option<Vec<BareProblem>>,
    #[serde(skip)]
    pub solved_problems: Option<Vec<BareProblem>>,
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
        let file = File::create("store.json")
            .map_err(|err| format!("error while creating store.json: {}", err))?;
        let v = Store::new(directory);
        serde_json::to_writer(file, &v)
            .map_err(|err| format!("error while writing store.json: {}", err))?;
        Ok(v)
    }

    pub fn read() -> Result<Store, String> {
        serde_json::from_reader(
            File::open("store.json")
                .map_err(|err| format!("error while opening store.json: {}", err))?,
        )
        .map_err(|err| format!("error while parsing store.json: {}", err))
    }

    pub fn save(&self) -> Result<(), String> {
        serde_json::to_writer(
            File::create("store.json")
                .map_err(|err| format!("error while saving store.json cannot create: {}", err))?,
            &self,
        )
        .map_err(|err| format!("error while writing store.json: {}", err))
    }

    pub fn filter_problems(&mut self) -> Result<(), String> {
        match self.problems_list.as_ref() {
            Some(problems) => {
                let mut filtered_problems = problems.clone();
                if !self.show_solved {
                    filtered_problems = filtered_problems
                        .into_iter()
                        .filter(|x| !self.solved_problems.as_ref().unwrap().contains(x))
                        .collect();
                }

                filtered_problems = filtered_problems
                    .into_iter()
                    .filter(|x| {
                        x.contest_type.eq(&self.contest_type)
                            && self.problem_types.contains(&x.problem_id)
                    })
                    .collect();

                self.filtered_problems = Some(filtered_problems);
                self.index = 0;

                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn get_problem(&self) -> Result<Problem, String> {
        if let Some(problem) = self.filtered_problems.as_ref().unwrap().get(self.index) {
            let pr = problem.clone();
            Ok(Problem::Bare(pr))
        } else {
            Err("index out of range".into())
        }
    }

    pub fn create_file(&self) -> Result<(), String> {
        let problem = self.get_problem()?;
        match problem {
            Problem::Bare(problem) => {
                let mut file_path = PathBuf::from(format!(
                    "{}/{}{}_{}/{}",
                    self.directory,
                    problem.contest_type,
                    problem.contest_id,
                    problem.problem_id,
                    problem.problem_id
                ));
                file_path.set_extension(self.language.extension());
                match File::create(file_path) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(format!("error while creating file: {}", err)),
                }
            }
            _ => Err("get problem shouldn't return anything else".into()),
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

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Language {
    C,
    Cpp,
    Go,
    Rust,
    Kotlin,
    Zig,
    Node,
    Swift,
    Dart,
    Haskell,
    Fortran,
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
            "swift" => Ok(Language::Swift),
            "dart" => Ok(Language::Dart),
            "haskell" => Ok(Language::Haskell),
            "fortran" => Ok(Language::Fortran),
            "ocaml" => Ok(Language::Ocaml),
            _ => Err("invalid language".into()),
        }
    }

    pub fn extension(&self) -> String {
        match self {
            Language::C => "c".into(),
            Language::Cpp => "cpp".into(),
            Language::Go => "go".into(),
            Language::Rust => "rs".into(),
            Language::Kotlin => "kt".into(),
            Language::Zig => "zig".into(),
            Language::Node => "js".into(),
            Language::Swift => "swift".into(),
            Language::Dart => "dart".into(),
            Language::Haskell => "hs".into(),
            Language::Fortran => "f90".into(),
            Language::Ocaml => "ml".into(),
        }
    }

    pub fn compiler(&self) -> String {
        match self {
            Language::C => "gcc".into(),
            Language::Cpp => "g++".into(),
            Language::Go => "go".into(),
            Language::Rust => "rustc".into(),
            Language::Kotlin => "kotlinc".into(),
            Language::Zig => "zig".into(),
            Language::Node => "node".into(),
            Language::Swift => "swift".into(),
            Language::Dart => "dart".into(),
            Language::Haskell => "ghc".into(),
            Language::Fortran => "gfortran".into(),
            Language::Ocaml => "ocamlc".into(),
        }
    }
}