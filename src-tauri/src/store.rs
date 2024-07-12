use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::{fmt, fs};

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
    pub editor: String,
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
            editor: String::new(),
        }
    }

    pub fn read_or_create() -> Result<Store, String> {
        let data_dir = tauri::api::path::data_dir().unwrap();
        if Path::new(&format!("{}/xcoder_store.json", data_dir.display())).exists() {
            Store::read()
        } else {
            Store::create("".into())
        }
    }

    pub fn create(directory: String) -> Result<Store, String> {
        let data_dir = tauri::api::path::data_dir().unwrap();
        let file = File::create(&format!("{}/xcoder_store.json", data_dir.display()))
            .map_err(|err| format!("error while creating store.json: {}", err))?;
        let v = Store::new(directory);
        serde_json::to_writer(file, &v)
            .map_err(|err| format!("error while writing store.json: {}", err))?;
        Ok(v)
    }

    pub fn read() -> Result<Store, String> {
        let data_dir = tauri::api::path::data_dir().unwrap();
        serde_json::from_reader(
            File::open(&format!("{}/xcoder_store.json", data_dir.display()))
                .map_err(|err| format!("error while opening store.json: {}", err))?,
        )
        .map_err(|err| format!("error while parsing store.json: {}", err))
    }

    pub fn save(&self) -> Result<(), String> {
        let data_dir = tauri::api::path::data_dir().unwrap();
        serde_json::to_writer(
            File::create(&format!("{}/xcoder_store.json", data_dir.display()))
                .map_err(|err| format!("error while saving store.json cannot create: {}", err))?,
            &self,
        )
        .map_err(|err| format!("error while writing store.json: {}", err))
    }

    pub fn filter_problems(&mut self) -> Result<(), String> {
        self.solved_problems = Some(get_solved_problems(self.directory.clone())?);
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
        if let Problem::Bare(problem) = problem {
            let mut file_path = PathBuf::from(format!(
                "{}/{}/{}{}_{}",
                self.directory,
                self.language.source_directory(),
                problem.contest_type.to_string().to_lowercase(),
                problem.contest_id,
                problem.problem_id,
            ));
            file_path.set_extension(self.language.extension());
            if file_path.exists() {
                return Ok(());
            }
            fs::create_dir_all(file_path.parent().unwrap())
                .map_err(|err| format!("error while creating directory: {}", err))?;
            File::create(file_path).map_err(|err| format!("error while creating file: {}", err))?;
            Ok(())
        } else {
            Err("got invalid problem while creating file".into())
        }
    }

    pub fn open_file_in_editor(&self) -> Result<(), String> {
        let problem = self.get_problem()?;
        if let Problem::Bare(problem) = problem {
            let file_path = PathBuf::from(format!(
                "{}/{}/{}{}_{}",
                self.directory,
                self.language.source_directory(),
                problem.contest_type.to_string().to_lowercase(),
                problem.contest_id,
                problem.problem_id,
            ));
            Command::new(self.editor.clone()).arg(file_path);
            Ok(())
        } else {
            Err("got invalid problem while opening editor".into())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
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

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
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
    Elixir,
    Fortran,
    Ocaml,
    Python,
    Julia,
    Fsharp,
    Csharp,
}

#[derive(PartialEq, Eq)]
pub enum LangType {
    Compiled,
    Interpreted,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Language {
    pub fn from_str(str: &str) -> Result<Language, String> {
        use Language::*;
        match str.to_lowercase().as_str() {
            "c" => Ok(C),
            "cpp" => Ok(Cpp),
            "go" => Ok(Go),
            "rust" => Ok(Rust),
            "kotlin" => Ok(Kotlin),
            "zig" => Ok(Zig),
            "node" => Ok(Node),
            "swift" => Ok(Swift),
            "dart" => Ok(Dart),
            "haskell" => Ok(Haskell),
            "elixir" => Ok(Elixir),
            "fortran" => Ok(Fortran),
            "ocaml" => Ok(Ocaml),
            "python" => Ok(Python),
            "julia" => Ok(Julia),
            "c#" => Ok(Csharp),
            "f#" => Ok(Fsharp),
            _ => Err("invalid language".into()),
        }
    }

    pub fn extension(&self) -> String {
        use Language::*;
        match self {
            C => "c".into(),
            Cpp => "cpp".into(),
            Go => "go".into(),
            Rust => "rs".into(),
            Kotlin => "kt".into(),
            Zig => "zig".into(),
            Node => "js".into(),
            Swift => "swift".into(),
            Dart => "dart".into(),
            Haskell => "hs".into(),
            Elixir => "exs".into(),
            Fortran => "f90".into(),
            Ocaml => "ml".into(),
            Python => "py".into(),
            Julia => "jl".into(),
            Fsharp => "fsx".into(),
            Csharp => "cs".into(),
        }
    }

    pub fn source_directory(&self) -> String {
        use Language::*;
        match self {
            Elixir => String::from("lib"),
            Rust => String::from("src/bin"),
            Kotlin => String::from("src"),
            _ => String::from("."),
        }
    }

    pub fn compiler(&self) -> Command {
        use Language::*;
        match self {
            C => Command::new("gcc"),
            Cpp => Command::new("g++"),
            Rust => Command::new("rustc"),
            Kotlin => Command::new("kotlinc"),
            Zig => Command::new("zig"),
            Swift => Command::new("swiftc"),
            Dart => Command::new("dart"),
            Haskell => Command::new("ghc"),
            Fortran => Command::new("gfortran"),
            Ocaml => Command::new("ocamlc"),
            _ => panic!("compiler not found"),
        }
    }

    pub fn interpreter(&self) -> String {
        use Language::*;
        match self {
            Elixir => String::from("elixir"),
            Node => String::from("node"),
            Go => String::from("go run"),
            Python => String::from("python"),
            Julia => String::from("julia"),
            Fsharp => String::from("dotnet fsx"),
            Csharp => String::from("dotnet script"),
            _ => panic!("interpretor not found"),
        }
    }

    pub fn lang_type(&self) -> LangType {
        use Language::*;
        match self {
            Python | Elixir | Go | Node | Fsharp => LangType::Interpreted,
            _ => LangType::Compiled,
        }
    }
}
