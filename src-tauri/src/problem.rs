use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::path::Path;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::problem::Problem::Bare;
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
            "EX" => Ok(ProblemId::Ex),
            _ => Err("invalid problem id".into()),
        }
    }
}

#[derive(Clone)]
pub enum Problem {
    Bare(BareProblem),
    Full(FullProblem),
}

#[derive(Clone)]
pub struct BareProblem {
    pub contest_type: ContestType,
    pub contest_id: u16,
    pub problem_id: ProblemId,
    pub test_cases_link: String,
}

#[derive(Serialize, Clone)]
pub struct FullProblem {
    pub contest_type: ContestType,
    pub contest_id: u16,
    pub problem_id: ProblemId,
    pub title: String,
    pub description: String,
    pub time_limit: u32,
    pub memory_limit: u32,
    pub test_cases_link: String,
}

impl PartialEq<Self> for BareProblem {
    fn eq(&self, other: &Self) -> bool {
        self.contest_type == other.contest_type && self.contest_id == other.contest_id && self.problem_id == other.problem_id
    }
}

impl Eq for BareProblem {}

impl Problem {
    pub async fn scrape(&mut self) -> Result<(), String> {
        match self {
            Bare(bare_problem) => {
                let full_problem = bare_problem.scrape().await?;
                *self = Problem::Full(full_problem);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl FullProblem {
    pub fn new(
        bare_problem: &BareProblem,
        title: String,
        description: String,
        time_limit: u32,
        memory_limit: u32,
    ) -> FullProblem {
        FullProblem {
            contest_type: bare_problem.contest_type.clone(),
            contest_id: bare_problem.contest_id,
            problem_id: bare_problem.problem_id.clone(),
            title,
            description,
            time_limit,
            memory_limit,
            test_cases_link: bare_problem.test_cases_link.clone(),
        }
    }
}

impl BareProblem {
    pub fn new(
        contest_type: ContestType,
        contest_id: u16,
        problem_id: ProblemId,
        test_cases_link: String,
    ) -> BareProblem {
        BareProblem {
            contest_type,
            contest_id,
            problem_id,
            test_cases_link,
        }
    }

    pub async fn scrape(&self) -> Result<FullProblem, String> {
        match reqwest::get(format!(
            "https://atcoder.jp/contests/{}{:03}/tasks/{}{:03}_{}",
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
                    let limits_text = document
                        .select(&Selector::parse(".row > div:nth-child(2) > p").unwrap())
                        .next()
                        .unwrap()
                        .text()
                        .collect::<Vec<&str>>()
                        .join("\n");
                    let limits = limits_text.split("/");
                    let time_limit: u32 = limits
                        .clone()
                        .nth(0)
                        .unwrap()
                        .split(" ")
                        .nth(2)
                        .unwrap()
                        .parse()
                        .unwrap();
                    let memory_limit: u32 = limits
                        .clone()
                        .nth(1)
                        .unwrap()
                        .split(" ")
                        .nth(3)
                        .unwrap()
                        .parse()
                        .unwrap();

                    Ok(FullProblem::new(
                        self,
                        title,
                        description.inner_html(),
                        time_limit,
                        memory_limit,
                    ))
                }
                Err(err) => Err(format!("error while getting problem text: {}", err)),
            },
            Err(err) => Err(format!("error while getting problem response: {}", err)),
        }
    }
}

pub async fn get_problems_list() -> Result<Vec<BareProblem>, String> {
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
                    let (ct, cid, pid, link) =
                        (record.get(0), record.get(1), record.get(2), record.get(3));
                    if ct.is_none() || cid.is_none() || pid.is_none() || link.is_none() {
                        return Err("error while getting problem".into());
                    }
                    if let Ok(contest_id) = cid.unwrap().parse::<u16>() {
                        if let Ok(problem_id) = ProblemId::from_str(pid.unwrap()) {
                            let contest_type = ContestType::from_str(ct.unwrap())?;
                            let test_cases_link = link.unwrap().into();
                            let problem = BareProblem::new(
                                contest_type,
                                contest_id,
                                problem_id,
                                test_cases_link,
                            );
                            problem_set.push(problem);
                        } else {
                            return Err(format!(
                                "error while parsing problem_id: {} {}",
                                contest_id,
                                pid.unwrap()
                            ));
                        }
                    } else {
                        return Err("error while parsing contest_id".into());
                    }
                };
            }

            Ok(problem_set)
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

pub fn get_solved_problems() -> Result<Vec<BareProblem>, String> {
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
                    let (ct, cid, pid) = (record.get(0), record.get(1), record.get(2));

                    if ct.is_none() || cid.is_none() || pid.is_none() {
                        return Err("error while getting problem".into());
                    }

                    if let Ok(contest_id) = cid.unwrap().parse::<u16>() {
                        let contest_type = ContestType::from_str(ct.unwrap())?;
                        let problem_id = ProblemId::from_str(pid.unwrap())?;
                        let problem =
                            BareProblem::new(contest_type, contest_id, problem_id, "".into());
                        solved_problems.push(problem);
                    } else {
                        return Err("error while parsing contest_id".into());
                    }
                };
            }

            Ok(solved_problems)
        }
        Err(err) => Err(format!("error while opening solved_problems.csv: {}", err)),
    };
}

pub fn insert_solved_problem(problem: BareProblem) -> Result<(), String> {
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
