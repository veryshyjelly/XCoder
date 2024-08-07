use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::path::Path;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::problem::Problem::Bare;
use crate::store::ContestType;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
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

#[derive(Clone, Hash)]
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
    pub time_limit: u64,
    pub memory_limit: u64,
    pub test_cases_link: String,
}

impl PartialEq<Self> for BareProblem {
    fn eq(&self, other: &Self) -> bool {
        self.contest_type == other.contest_type
            && self.contest_id == other.contest_id
            && self.problem_id == other.problem_id
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
        time_limit: u64,
        memory_limit: u64,
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
        let mut text = reqwest::get(format!(
            "https://atcoder.jp/contests/{}{:03}/tasks/{}{:03}_{}",
            self.contest_type, self.contest_id, self.contest_type, self.contest_id, self.problem_id
        ))
        .await
        .map_err(|err| format!("error while getting problem response: {}", err))?
        .text()
        .await
        .map_err(|err| format!("error while getting problem text: {}", err))?;

        text = text.replace("\\leq", "≤");
        let document = Html::parse_document(&text);
        let description_selector = Selector::parse(".lang-en")
            .map_err(|err| format!("error while parsing description selector: {}", err))?;

        let description = document
            .select(&description_selector)
            .next()
            .ok_or(format!("error while getting description"))?;

        let mut title = document
            .select(
                &Selector::parse(".h2")
                    .map_err(|err| format!("error while parsing title selector: {}", err))?,
            )
            .next()
            .ok_or(format!("error while getting problem title"))?
            .text()
            .collect::<Vec<&str>>()
            .join("\n");

        title = title.split("Editorial").nth(0).unwrap().trim().to_string();

        let limits_text = document
            .select(
                &Selector::parse(".row > div:nth-child(2) > p")
                    .map_err(|err| format!("error while parsing limits selector: {}", err))?,
            )
            .next()
            .ok_or(format!("error while getting time and memory limit"))?
            .text()
            .collect::<Vec<&str>>()
            .join("\n");

        let limits = limits_text.split("/");
        let time_limit: u64 = limits
            .clone()
            .nth(0)
            .ok_or(format!("error while getting time limit"))?
            .split(" ")
            .nth(2)
            .ok_or(format!("error while getting time limit"))?
            .parse()
            .map_err(|err| format!("error while parsing time limit: {}", err))?;
        let memory_limit: u64 = limits
            .clone()
            .nth(1)
            .ok_or(format!("error while getting memory limit"))?
            .split(" ")
            .nth(3)
            .ok_or(format!("error while getting memory limit"))?
            .parse()
            .map_err(|err| format!("error while parsing memory limit: {}", err))?;

        Ok(FullProblem::new(
            self,
            title,
            description.inner_html(),
            time_limit * 2,
            memory_limit,
        ))
    }
}

pub async fn get_problems_list() -> Result<Vec<BareProblem>, String> {
    if !Path::new("problems.csv").exists() {
        return Err("problems list does not exist".into());
    }

    let file = File::open("problems.csv")
        .map_err(|err| format!("error while opening problems.csv: {}", err))?;
    let mut problem_set = vec![];
    let mut rdr = csv::Reader::from_reader(file);

    for r in rdr.records() {
        if let Ok(record) = r {
            let (ct, cid, pid, link) = (
                record
                    .get(0)
                    .ok_or("error while getting contest type".to_string())?,
                record
                    .get(1)
                    .ok_or("error while getting contest id".to_string())?,
                record
                    .get(2)
                    .ok_or("error while getting problem id".to_string())?,
                record
                    .get(3)
                    .ok_or("error while getting problem link".to_string())?,
            );
            let contest_id = cid
                .parse::<u16>()
                .map_err(|err| format!("error while parsing contest_id: {}", err))?;
            let problem_id = ProblemId::from_str(pid).map_err(|err| {
                format!(
                    "error while parsing problem_id: {} contest_id: {}",
                    err, contest_id
                )
            })?;
            let contest_type = ContestType::from_str(ct)?;
            let test_cases_link = link.into();
            let problem = BareProblem::new(contest_type, contest_id, problem_id, test_cases_link);
            problem_set.push(problem);
        };
    }

    Ok(problem_set)
}

pub fn get_solved_problems(directory: String) -> Result<Vec<BareProblem>, String> {
    if !Path::new(&format!("{}/solved_problems.csv", directory)).exists() {
        let file = File::create(&format!("{}/solved_problems.csv", directory))
            .map_err(|err| format!("error while creating solved_problems.csv: {}", err))?;
        let mut wtr = csv::Writer::from_writer(file);
        wtr.write_record(["contest_type", "contest_id", "problem_id"])
            .map_err(|err| format!("error while writing csv record: {}", err))?;
        return Ok(vec![]);
    }
    let file = File::open(format!("{}/solved_problems.csv", directory))
        .map_err(|err| format!("error while opening solved_problems.csv: {}", err))?;

    let mut solved_problems = vec![];

    let mut rdr = csv::Reader::from_reader(file);

    for r in rdr.records() {
        if let Ok(record) = r {
            let (ct, cid, pid) = (record.get(0), record.get(1), record.get(2));

            if ct.is_none() || cid.is_none() || pid.is_none() {
                return Err("error while getting problem".into());
            }

            if let Ok(contest_id) = cid.unwrap().parse::<u16>() {
                let contest_type = ContestType::from_str(ct.unwrap())?;
                let problem_id = ProblemId::from_str(pid.unwrap())?;
                let problem = BareProblem::new(contest_type, contest_id, problem_id, "".into());
                solved_problems.push(problem);
            } else {
                return Err("error while parsing contest_id".into());
            }
        };
    }

    Ok(solved_problems)
}

pub fn insert_solved_problem(problem: BareProblem, directory: String) -> Result<(), String> {
    let mut recs_map = HashMap::new();
    get_solved_problems(directory.clone())?
        .into_iter()
        .for_each(|x| {
            recs_map.insert(x, true);
        });
    recs_map.insert(problem, true);

    let mut wtr = csv::Writer::from_path(format!("{}/solved_problems.csv", directory))
        .map_err(|err| format!("error while opening solved_problems.csv: {}", err))?;

    wtr.write_record(["contest_type", "contest_id", "problem_id"])
        .map_err(|err| format!("error while writing csv record: {}", err))?;

    for (rec, _) in recs_map.iter() {
        wtr.write_record([
            rec.contest_type.to_string(),
            rec.contest_id.to_string(),
            rec.problem_id.to_string(),
        ])
        .map_err(|err| format!("error writing to solved_problems.csv: {}", err))?;
    }

    wtr.flush()
        .map_err(|err| format!("error writing to solved_prbolems.csv: {}", err))?;
    Ok(())
}
