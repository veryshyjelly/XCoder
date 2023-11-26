use std::time::Duration;

use crate::problem::*;
use crate::store::Language;

pub struct Verdict {
    input: String,
    output: Option<String>,
    answer: String,
    status: Option<JudgeStatus>,
    time: Option<Duration>,
    memory: Option<u64>,
}

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
