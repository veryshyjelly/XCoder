use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Write};
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::problem::*;
use crate::store::Language;

#[derive(Serialize, Deserialize)]
pub struct Verdict {
    input: String,
    output: Option<String>,
    answer: String,
    status: Option<JudgeStatus>,
    time: Option<Duration>,
    memory: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub enum JudgeStatus {
    CE,
    MLE,
    TLE,
    RE,
    OLE,
    IE,
    WA,
    AC,
}

impl Verdict {
    fn new(input: String, answer: String) -> Verdict {
        Verdict {
            input,
            output: None,
            answer,
            status: None,
            time: None,
            memory: None,
        }
    }

    pub fn exec(&mut self, binary_path: &PathBuf) -> Result<(), String> {
        let mut sol_process = Command::new(binary_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .creation_flags(0x08000000)
            .spawn()
            .unwrap();
        sol_process
            .stdin
            .take()
            .unwrap()
            .write_fmt(format_args!("{}", self.input))
            .unwrap();
        let sol_output = String::from_utf8(sol_process.wait_with_output().unwrap().stdout)
            .unwrap()
            .trim()
            .to_string();
        self.output = Some(sol_output);
        if self.output.unwrap().cmp(&self.answer) {
            self.status = Some(JudgeStatus::AC);
        } else {
            self.status = Some(JudgeStatus::WA);
        }
        // TODO: check status, time and memory
        Ok(())
    }
}

pub struct Judge {
    problem: Problem,
    directory: String,
    language: Language,
    binary_path: Option<PathBuf>,
}

impl Judge {
    fn new(problem: Problem, directory: String, language: Language) -> Judge {
        Judge {
            problem,
            directory,
            language,
            binary_path: None,
        }
    }

    pub fn compile(&mut self) -> Result<(), String> {
        let mut binary_path = PathBuf::from(format!(
            "{}/bin/{}{}_{}.exe",
            self.directory,
            self.problem.contest_type,
            self.problem.contest_id,
            self.problem.problem_id
        ));
        let mut file_path = PathBuf::from(format!(
            "{}/{}{}_{}",
            self.directory,
            self.problem.contest_type,
            self.problem.contest_id,
            self.problem.problem_id
        ));
        file_path.set_extension(self.language.extension());

        if !file_path.exists() {
            return Err(format!("{} does not exist", file_path.display()));
        }

        match Command::new(self.language.compiler())
            .current_dir(&self.directory)
            .arg(file_path)
            .arg("-o")
            .arg(binary_path.clone())
            .creation_flags(0x08000000)
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    self.binary_path = Some(binary_path);
                    Ok(())
                } else {
                    Err(format!(
                        "error while compiling: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ))
                }
            }
            Err(err) => Err(format!("error while compiling: {}", err)),
        }
    }

    pub async fn download_test_cases(&self) -> Result<(), String> {
        let test_cases_path = PathBuf::from(format!(
            "{}/test_cases/{}{}_{}",
            self.directory,
            self.problem.contest_type,
            self.problem.contest_id,
            self.problem.problem_id
        ));

        if test_cases_path.exists() {
            return Ok(());
        }

        let mut link = self.problem.test_cases_link.clone().trim().to_string();
        link.pop();
        link.push('1');

        match reqwest::get(link).await {
            Ok(resp) => match resp.bytes().await {
                Ok(archive) => {
                    match zip_extract::extract(Cursor::new(archive), &test_cases_path, true) {
                        Ok(()) => Ok(()),
                        Err(err) => Err(format!("error while extracting problems: {}", err)),
                    }
                }
                Err(err) => Err(format!("error while getting test_case: {}", err)),
            },
            Err(err) => Err(format!("error while getting test cases: {}", err)),
        }
    }

    pub fn judge_by_filename(&mut self, file_name: String) -> Result<Verdict, String> {
        if self.binary_path.is_none() {
            self.compile()?;
        }
        let input_file = format!(
            "{}/test_cases/{}{}_{}/in/{}",
            self.directory,
            self.problem.contest_type,
            self.problem.contest_id,
            self.problem.problem_id,
            file_name
        );
        let output_file = format!(
            "{}/test_cases/{}{}_{}/out/{}",
            self.directory,
            self.problem.contest_type,
            self.problem.contest_id,
            self.problem.problem_id,
            file_name
        );
        let mut verdict = Verdict::new(
            fs::read_to_string(input_file).unwrap(),
            fs::read_to_string(output_file).unwrap(),
        );
        verdict.exec(&self.binary_path.unwrap())?;
        Ok(verdict)
    }

    pub fn judge_by_filenames(&mut self, file_names: Vec<String>) -> Result<Vec<Verdict>, String> {
        let mut verdicts = vec![];
        for file_name in file_names {
            verdicts.push(self.judge_by_filename(file_name)?);
        }
        Ok(verdicts)
    }
}

pub async fn submit(
    problem: Problem,
    directory: String,
    language: Language,
) -> Result<Vec<Verdict>, String> {
    let in_paths = fs::read_dir(format!(
        "{}/test_cases/{}{}_{}/in",
        directory, problem.contest_type, problem.contest_id, problem.problem_id
    ))
    .unwrap();
    let out_paths = fs::read_dir(format!(
        "{}/test_cases/{}{}_{}/out",
        directory, problem.contest_type, problem.contest_id, problem.problem_id
    ))
    .unwrap();

    let mut judge = Judge::new(problem, directory, language);
    judge.download_test_cases().await?;

    let mut file_names_map = HashMap::new();
    for path in in_paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        file_names_map.insert(file_name, 1);
    }
    for path in out_paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if file_names_map.contains_key(&file_name) {
            file_names_map.insert(file_name, 2);
        }
    }

    let mut file_names = vec![];
    for (file_name, status) in file_names_map {
        if status == 2 {
            file_names.push(file_name);
        }
    }

    let verdicts = judge.judge_by_filenames(file_names)?;

    Ok(verdicts)
}

pub async fn run(
    problem: Problem,
    directory: String,
    language: Language,
) -> Result<Vec<Verdict>, String> {
    let in_paths = fs::read_dir(format!(
        "{}/test_cases/{}{}_{}/in",
        directory, problem.contest_type, problem.contest_id, problem.problem_id
    ))
    .unwrap();
    let out_paths = fs::read_dir(format!(
        "{}/test_cases/{}{}_{}/out",
        directory, problem.contest_type, problem.contest_id, problem.problem_id
    ))
    .unwrap();

    let mut judge = Judge::new(problem, directory, language);
    judge.download_test_cases().await?;

    let mut file_names_map = HashMap::new();
    for path in in_paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if !file_name.starts_with("sample") {
            continue;
        }
        file_names_map.insert(file_name, 1);
    }
    for path in out_paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if !file_name.starts_with("sample") {
            continue;
        }
        if file_names_map.contains_key(&file_name) {
            file_names_map.insert(file_name, 2);
        }
    }

    let mut file_names = vec![];
    for (file_name, status) in file_names_map {
        if status == 2 {
            file_names.push(file_name);
        }
    }

    let verdicts = judge.judge_by_filenames(file_names)?;

    Ok(verdicts)
}