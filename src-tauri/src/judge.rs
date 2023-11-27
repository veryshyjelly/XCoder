use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
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

    pub fn judge(
        &mut self,
        problem: Problem,
        input_file: String,
        output_file: String,
        language: Language,
        directory: String,
    ) -> Result<(), String> {
        Ok(())
    }
}

pub async fn submit(
    problem: Problem,
    directory: String,
    language: Language,
) -> Result<Vec<Verdict>, String> {
    download_test_cases(&problem, &directory).await?;
    compile_solution(&problem, &directory, language)?;
    let paths = fs::read_dir(format!(
        "{}/test_cases/{}{}_{}",
        directory, problem.contest_type, problem.contest_id, problem.problem_id
    ));
    Err("not implemented".into())
}

pub async fn run(
    problem: Problem,
    directory: String,
    language: Language,
) -> Result<Vec<Verdict>, String> {
    download_test_cases(&problem, &directory).await?;
    compile_solution(&problem, &directory, language)?;
    Err("not implemented".into())
}

pub fn compile_solution(
    problem: &Problem,
    directory: &String,
    language: Language,
) -> Result<(), String> {
    Err("not implemented".into())
}

pub async fn download_test_cases(problem: &Problem, directory: &String) -> Result<(), String> {
    let mut link = problem.test_cases_link.clone().trim().to_string();
    link.pop();
    link.push('1');

    match reqwest::get(link).await {
        Ok(resp) => match resp.bytes().await {
            Ok(archive) => match zip_extract::extract(
                Cursor::new(archive),
                &PathBuf::from(format!(
                    "{}/test_cases/{}{}_{}",
                    directory, problem.contest_type, problem.contest_id, problem.problem_id
                )),
                true,
            ) {
                Ok(()) => Ok(()),
                Err(err) => Err(format!("error while extracting problems: {}", err)),
            },
            Err(err) => Err(format!("error while getting test_case: {}", err)),
        },
        Err(err) => Err(format!("error while getting test cases: {}", err)),
    }
}