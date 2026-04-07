mod checks;
mod report;

use anyhow::Result;
use std::path::Path;

use checks::run_assessments;
use report::{print_json, print_report};

#[derive(Debug)]
pub(crate) struct Assessment {
    pub category: &'static str,
    pub requirement: &'static str,
    pub level: u8,
    pub obligation: Obligation,
    pub status: Status,
    pub detail: String,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Obligation {
    Must,
    Should,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Status {
    Pass,
    Fail,
    Partial,
}

pub fn run(project_root: &Path, json: bool) -> Result<()> {
    let assessments = run_assessments(project_root);

    if json {
        print_json(project_root, &assessments)?;
    } else {
        print_report(project_root, &assessments);
    }

    Ok(())
}
