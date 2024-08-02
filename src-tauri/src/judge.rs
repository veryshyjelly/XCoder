use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::{fs, io};

use serde::{Deserialize, Serialize};
use wait_timeout::ChildExt;
use zip::ZipArchive;

use crate::problem::*;
use crate::store::{LangType, Language};

#[derive(Serialize, Deserialize, Clone)]
pub struct Verdict {
    input: String,
    output: Option<String>,
    answer: String,
    status: Option<JudgeStatus>,
    time: Option<f32>,
    memory: Option<u64>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
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

    pub fn exec(
        &mut self,
        binary_path: String,
        input_file: PathBuf,
        output_file: PathBuf,
        timeout: Duration,
    ) -> Result<(), String> {
        let mut sol_process = Command::new("powershell")
            .current_dir(input_file.parent().unwrap())
            .args(["-Command", "-"])
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|err| format!("error while running solution: {}", err))?;

        sol_process
            .stdin
            .take()
            .ok_or(format!("error while taking stdin of powershell"))?
            .write_fmt(format_args!(
                "{}",
                format!("Start-Process '{}' -RedirectStandardInput '{}' -RedirectStandardOutput '{}' -NoNewWindow -Wait", binary_path, input_file.display(), output_file.display())
                    .replace("/", "\\")
                    .as_str()
            ))
            .map_err(|err| format!("error while giving command to powershell: {}", err))?;
        let now = Instant::now();

        match sol_process.wait_timeout(timeout).unwrap() {
            Some(x) => {
                if x.success() {
                    self.time = Some(now.elapsed().as_secs_f32());
                    self.output = Some(
                        fs::read_to_string(output_file)
                            .map_err(|err| {
                                format!("error while reading output from file: {}", err)
                            })?
                            .trim()
                            .to_string(),
                    );
                    if self.answer.eq(self.output.as_ref().unwrap()) {
                        self.status = Some(JudgeStatus::AC);
                    } else {
                        self.status = Some(JudgeStatus::WA);
                    }
                    if self.time.as_ref().unwrap() > &(timeout.as_secs_f32() - 2.0) {
                        self.status = Some(JudgeStatus::TLE);
                    }
                } else {
                    self.status = Some(JudgeStatus::RE);
                }
            }
            None => {
                self.status = Some(JudgeStatus::TLE);
            }
        };

        // TODO: check status, and memory
        Ok(())
    }
}

pub struct Judge {
    problem: FullProblem,
    directory: String,
    language: Language,
    binary_path: Option<String>,
}

impl Judge {
    fn new(problem: FullProblem, directory: String, language: Language) -> Judge {
        Judge {
            problem,
            directory,
            language,
            binary_path: None,
        }
    }

    pub fn compile(&mut self) -> Result<(), String> {
        let mut file_path = PathBuf::from(format!(
            "{}/{}/{}{}_{}",
            self.directory,
            self.language.source_directory(),
            self.problem.contest_type.to_string().to_lowercase(),
            self.problem.contest_id,
            self.problem.problem_id,
        ));
        file_path.set_extension(self.language.extension());

        if !file_path.exists() {
            return Err(format!("the file {} does not exist", file_path.display()));
        }

        if self.language.lang_type() == LangType::Interpreted {
            self.binary_path = Some(format!(
                "{} '{}'",
                self.language.interpreter(),
                file_path.display()
            ));
            return Ok(());
        }

        if !Path::new(&format!("{}/bin", self.directory)).exists() {
            fs::create_dir(&format!("{}/bin", self.directory))
                .map_err(|err| format!("error while creating bin folder: {}", err))?;
        }

        let binary_path = PathBuf::from(format!(
            "{}/bin/{}{}_{}.exe",
            self.directory,
            self.problem.contest_type.to_string().to_lowercase(),
            self.problem.contest_id,
            self.problem.problem_id
        ));

        let output = self
            .language
            .compiler()
            .current_dir(&self.directory)
            .arg("-o")
            .arg(binary_path.clone())
            .arg(file_path)
            .creation_flags(0x08000000)
            .output()
            .map_err(|err| format!("error while compiling: {}", err))?;
        if output.status.success() {
            self.binary_path = Some(binary_path.to_str().unwrap().into());
            Ok(())
        } else {
            Err(format!(
                "error while compiling: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    pub async fn download_test_cases(&self) -> Result<(), String> {
        if !Path::new(&format!("{}/test_cases", self.directory)).exists() {
            fs::create_dir(&format!("{}/test_cases", self.directory))
                .map_err(|err| format!("error while creating test_cases folder: {}", err))?;
        }

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
        let mut archive = ZipArchive::new(Cursor::new(
            reqwest::get(link)
                .await
                .map_err(|err| format!("error while getting test cases response: {}", err))?
                .bytes()
                .await
                .map_err(|err| format!("error while getting test cases bytes: {}", err))?,
        ))
        .map_err(|err| format!("error while creating zip archive: {}", err))?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let output_path = match file.enclosed_name() {
                Some(path) => test_cases_path.join(path.to_owned()),
                None => continue,
            };
            if file.name().ends_with('/') {
                fs::create_dir_all(&output_path).unwrap();
            } else {
                if let Some(p) = output_path.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&output_path)
                    .map_err(|err| format!("error while creating file: {}", err))?;
                io::copy(&mut file, &mut outfile)
                    .map_err(|err| format!("error while copying file data: {}", err))?;
            }
        }
        Ok(())
    }

    pub fn judge_by_filenames(&mut self, file_names: Vec<String>) -> Result<Vec<Verdict>, String> {
        if self.binary_path.is_none() {
            self.compile()?;
        }
        let binary_path = self.binary_path.as_ref().unwrap().clone();

        let directory = self.directory.clone();
        let contest_type = self.problem.contest_type.clone();
        let contest_id = self.problem.contest_id.clone();
        let problem_id = self.problem.problem_id.clone();
        let timeout = Duration::from_secs(self.problem.time_limit + 2);

        let mut verdicts: Vec<Verdict> = vec![];
        let output_dir = PathBuf::from(format!(
            "{}/output/{}{}_{}",
            directory, contest_type, contest_id, problem_id
        ));

        if !output_dir.exists() {
            fs::create_dir_all(output_dir.clone())
                .map_err(|err| format!("error while creating output directory: {}", err))?;
        }

        for file_name in file_names {
            let input_file = PathBuf::from(format!(
                "{}/test_cases/{}{}_{}/in/{}",
                directory, contest_type, contest_id, problem_id, file_name
            ));

            let mut output_file = output_dir.clone();
            output_file.push(file_name.clone());

            let mut input = fs::read_to_string(&input_file)
                .map_err(|err| format!("error while reading input file: {}", err))?;
            let mut output = fs::read_to_string(format!(
                "{}/test_cases/{}{}_{}/out/{}",
                directory, contest_type, contest_id, problem_id, file_name
            ))
            .map_err(|err| format!("error while reading output file: {}", err))?;

            input = input.trim().replace("\r\n", "\n");
            output = output.trim().replace("\r\n", "\n");

            let binary_path = binary_path.clone();

            let mut verdict = Verdict::new(input, output);
            verdict.exec(binary_path, input_file, output_file, timeout.clone())?;
            verdicts.push(verdict);
        }

        Ok(verdicts)
    }
}

pub async fn submit(
    problem: FullProblem,
    directory: String,
    language: Language,
) -> Result<Vec<Verdict>, String> {
    let mut judge = Judge::new(problem.clone(), directory.clone(), language);
    judge.download_test_cases().await?;

    let in_paths = fs::read_dir(format!(
        "{}/test_cases/{}{}_{}/in",
        directory, problem.contest_type, problem.contest_id, problem.problem_id
    ))
    .map_err(|err| format!("error while reading in directory: {}", err))?;
    let out_paths = fs::read_dir(format!(
        "{}/test_cases/{}{}_{}/out",
        directory, problem.contest_type, problem.contest_id, problem.problem_id
    ))
    .map_err(|err| format!("error while reading out directory: {}", err))?;

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

    if verdicts.iter().all(|x| {
        x.status
            .as_ref()
            .unwrap_or(&JudgeStatus::CE)
            .eq(&JudgeStatus::AC)
    }) {
        insert_solved_problem(
            BareProblem::new(
                problem.contest_type,
                problem.contest_id,
                problem.problem_id,
                "".into(),
            ),
            directory,
        )?;
    }

    Ok(verdicts)
}

pub async fn run(
    problem: FullProblem,
    directory: String,
    language: Language,
) -> Result<Vec<Verdict>, String> {
    let mut judge = Judge::new(problem.clone(), directory.clone(), language);
    judge.download_test_cases().await?;

    let in_paths = fs::read_dir(format!(
        "{}/test_cases/{}{}_{}/in",
        directory, problem.contest_type, problem.contest_id, problem.problem_id
    ))
    .map_err(|err| format!("error while reading in directory: {}", err))?;
    let out_paths = fs::read_dir(format!(
        "{}/test_cases/{}{}_{}/out",
        directory, problem.contest_type, problem.contest_id, problem.problem_id
    ))
    .map_err(|err| format!("error while reading out directory: {}", err))?;

    let mut file_names_map = HashMap::new();
    for path in in_paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if !file_name.contains("sample") && !file_name.contains("example") {
            continue;
        }
        file_names_map.insert(file_name, 1);
    }
    for path in out_paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if !file_name.contains("sample") {
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
