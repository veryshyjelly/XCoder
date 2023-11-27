use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::path::Path;

use scraper::{Html, Selector};
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
    Ex,
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
            "Ex" => Ok(ProblemId::Ex),
            _ => Err("invalid problem id".into()),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Problem {
    pub contest_type: ContestType,
    pub contest_id: u16,
    pub problem_id: ProblemId,
    pub title: Option<String>,
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
            title: None,
            description: None,
            time_limit: None,
            memory_limit: None,
            test_cases_link,
        }
    }

    pub async fn scrape(&mut self) -> Result<(), String> {
        match reqwest::get(format!(
            "https://atcoder.jp/contests/{}{}/tasks/{}{}_{}",
            self.contest_type, self.contest_id, self.contest_type, self.contest_id, self.problem_id
        ))
        .await
        {
            Ok(resp) => match resp.text().await {
                Ok(text) => {
                    let document = Html::parse_document(&text);
                    let description_selector = Selector::parse(".lang-en").unwrap();
                    let description = document.select(&description_selector).next().unwrap();
                    let title = document
                        .select(&Selector::parse(".h2").unwrap())
                        .next()
                        .unwrap()
                        .text()
                        .collect::<Vec<&str>>()
                        .join("\n");
                    let mut limits_text = document
                        .select(&Selector::parse(".row > div:nth-child(2) > p").unwrap())
                        .next()
                        .unwrap()
                        .text()
                        .collect::<Vec<&str>>()
                        .join("\n");
                    let limits = limits_text.split("/");

                    self.description = Some(description.html());
                    self.title = Some(title.split("Editorial").next().unwrap().trim().into());
                    Ok(())
                }
                Err(err) => Err(format!("error while getting problem text: {}", err)),
            },
            Err(err) => Err(format!("error while getting problem response: {}", err)),
        }
    }
}

pub async fn get_problems_list() -> Result<Vec<Problem>, String> {
    if !Path::new("problems.csv").exists() {
        return Err("problems list does not exist".into());
    }

    let file = File::open("problems.csv");
    match file {
        Ok(f) => {
            let mut problem_set = vec![];
            let mut rdr = csv::Reader::from_reader(f);

            for r in rdr.records() {
                if let Ok(record) = r {
                    let (ct, cid, pid, tl) =
                        (record.get(0), record.get(1), record.get(2), record.get(3));

                    match ct {
                        None => return Err("error while getting contest_type".into()),
                        Some(contest_type) => match cid {
                            None => return Err("error while getting contest_id".into()),
                            Some(contest_id) => match pid {
                                None => return Err("error while getting problem id".into()),
                                Some(problem_id) => match tl {
                                    None => {
                                        return Err("error while getting test_cases_link".into());
                                    }
                                    Some(test_case_link) => {
                                        let contest_type = ContestType::from_str(contest_type)?;
                                        let problem_id = ProblemId::from_str(problem_id)?;
                                        let problem = Problem::new(
                                            contest_type,
                                            contest_id
                                                .parse()
                                                .expect("error while parsing contest_id"),
                                            problem_id,
                                            test_case_link.into(),
                                        );
                                        problem_set.push(problem);
                                    }
                                },
                            },
                        },
                    };
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
        let file = File::create("solved_problems.csv");
        return match file {
            Ok(f) => {
                let mut wtr = csv::Writer::from_writer(f);
                match wtr.write_record(["contest_type", "contest_id", "problem_id"]) {
                    Ok(()) => Ok(vec![]),
                    Err(err) => Err(format!("error while writing csv record: {}", err)),
                }
            }
            Err(err) => Err(format!("error while creating solved_problems.csv: {}", err)),
        };
    }

    let file = File::open("solved_problems.csv");
    return match file {
        Ok(f) => {
            let mut solved_problems = vec![];

            let mut rdr = csv::Reader::from_reader(f);

            for r in rdr.records() {
                if let Ok(record) = r {
                    let (ct, cid, pid, tl) =
                        (record.get(0), record.get(1), record.get(2), record.get(3));

                    match ct {
                        None => return Err("error while getting contest_type".into()),
                        Some(contest_type) => match cid {
                            None => return Err("error while getting contest_id".into()),
                            Some(contest_id) => match pid {
                                None => return Err("error while getting problem id".into()),
                                Some(problem_id) => {
                                    let contest_type = ContestType::from_str(contest_type)?;
                                    let problem_id = ProblemId::from_str(problem_id)?;
                                    let problem = Problem::new(
                                        contest_type,
                                        contest_id.parse().expect("error while parsing contest_id"),
                                        problem_id,
                                        "".into(),
                                    );
                                    solved_problems.push(problem);
                                }
                            },
                        },
                    }
                };
            }

            Ok(solved_problems)
        }
        Err(err) => Err(format!("error while opening solved_problems.csv: {}", err)),
    };
}

pub fn insert_solved_problem(problem: Problem) -> Result<(), String> {
    let file = File::open("solved_problems.csv");
    match file {
        Ok(f) => {
            let mut wtr = csv::Writer::from_writer(f);
            match wtr.write_record([
                problem.contest_type.to_string(),
                problem.contest_id.to_string(),
                problem.problem_id.to_string(),
            ]) {
                Ok(()) => Ok(()),
                Err(err) => Err(format!("error writing to solved_problems.csv: {}", err)),
            }
        }
        Err(err) => {
            return Err(format!("error while opening solved_problems.csv: {}", err));
        }
    }
}