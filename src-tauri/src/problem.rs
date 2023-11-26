use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::path::Path;

use reqwest::Client;
use scraper::Html;
use serde::{Deserialize, Serialize};

use crate::store::ContestType;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum ProblemId {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl fmt::Display for ProblemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ProblemId {
    pub fn from_str(id: &str) -> Result<ProblemId, String> {
        match id.to_uppercase().as_str() {
            "A" => Ok(ProblemId::A),
            "B" => Ok(ProblemId::B),
            "C" => Ok(ProblemId::C),
            "D" => Ok(ProblemId::D),
            "E" => Ok(ProblemId::E),
            "F" => Ok(ProblemId::F),
            "G" => Ok(ProblemId::G),
            "H" => Ok(ProblemId::H),
            _ => Err("invalid problem id".into()),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Problem {
    pub contest_type: ContestType,
    pub contest_id: u16,
    pub problem_id: ProblemId,
    pub description: Option<String>,
    pub time_limit: Option<u32>,
    pub memory_limit: Option<u32>,
    pub test_cases_link: String,
}

impl PartialEq<Self> for Problem {
    fn eq(&self, other: &Self) -> bool {
        self.contest_type.eq(&other.contest_type)
            && self.contest_id.eq(&other.contest_id)
            && self.problem_id.eq(&other.problem_id)
    }
}

impl Eq for Problem {}

impl Problem {
    pub fn new(
        contest_type: ContestType,
        contest_id: u16,
        problem_id: ProblemId,
        test_cases_link: String,
    ) -> Problem {
        Problem {
            contest_type,
            contest_id,
            problem_id,
            description: None,
            time_limit: None,
            memory_limit: None,
            test_cases_link,
        }
    }

    pub async fn scrape(&mut self) -> Result<(), String> {
        let text = &Client::new()
            .get(format!(
                "https://atcoder.jp/contests/{}{}/tasks/{}{}_{}",
                self.contest_type,
                self.contest_id,
                self.contest_type,
                self.contest_id,
                self.problem_id
            ))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("{}", text);
        let _document = Html::parse_document(&text);
        Ok(())
    }
}

pub async fn get_problems_list() -> Result<Vec<Problem>, String> {
    if !Path::new("problems.csv").exists() {
        update_problems_list().await?;
    }

    let file = File::open("problems.csv");
    match file {
        Ok(f) => {
            let mut problem_set = vec![];
            let mut rdr = csv::Reader::from_reader(f);

            for r in rdr.records() {
                if let Ok(record) = r {
                    let (contest_type, problem_id);
                    if let Some(ct) = record.get(0) {
                        contest_type = ContestType::from_str(ct)?;
                    } else {
                        return Err("error while getting contest_type".into());
                    }
                    if let Some(pi) = record.get(2) {
                        problem_id = ProblemId::from_str(pi)?;
                    } else {
                        return Err("error while getting problem id".into());
                    }
                    let problem = Problem::new(
                        contest_type,
                        record
                            .get(1)
                            .expect("error getting contest_id")
                            .parse()
                            .expect("error while parsing contest_id"),
                        problem_id,
                        record
                            .get(3)
                            .expect("error getting test_cases_link")
                            .to_string(),
                    );

                    problem_set.push(problem);
                }
            }

            Ok(problem_set)
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

pub fn get_solved_problems() -> Result<Vec<Problem>, String> {
    if !Path::new("solved_problems.csv").exists() {
        let mut file = File::create("solved_problems.csv");
        return match file {
            Ok(f) => {
                let mut wtr = csv::Writer::from_writer(f);
                match wtr.write_record(["contest_type", "contest_id", "problem_id"]) {
                    Ok(()) => Ok(vec![]),
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        };
    }

    let file = File::open("solved_problems.csv");
    return match file {
        Ok(f) => {
            let mut problem_set = vec![];

            let mut rdr = csv::Reader::from_reader(f);

            rdr.records().for_each(|r| {
                if let Ok(record) = r {
                    let problem = Problem::new(
                        ContestType::from_str(record.get(0).expect("error getting contest_id"))
                            .unwrap(),
                        record
                            .get(1)
                            .expect("error getting contest_id")
                            .parse()
                            .expect("error while parsing contest_id"),
                        ProblemId::from_str(record.get(2).expect("error getting problem_id"))
                            .unwrap(),
                        "".into(),
                    );

                    problem_set.push(problem);
                }
            });

            Ok(problem_set)
        }
        Err(err) => Err(format!("error while opening solved_problems.csv: {}", err)),
    };
}

pub fn insert_solved_problem(problem: Problem) -> Result<(), String> {
    let mut file = File::open("solved_problems.csv");
    match file {
        Ok(f) => {
            let mut wtr = csv::Writer::from_writer(f);
            match wtr.write_record([
                problem.contest_type.to_string(),
                problem.contest_id.to_string(),
                problem.problem_id.to_string(),
            ]) {
                Ok(()) => Ok(()),
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => {
            return Err(format!("error while opening solved_problems.csv: {}", err));
        }
    }
}

pub async fn update_problems_list() -> Result<(), String> {
    // here we have to use side car build in go to update the problem set
    Err("not implemented".into())
}