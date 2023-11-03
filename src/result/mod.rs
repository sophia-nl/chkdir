use std::io::Error;

mod diff;
mod last;
mod new;

pub use diff::{diff, DiffSummary};

use last::LastResult;
use new::NewResult;

use crate::walk::WalkSummary;

pub struct CheckResult {
    pub last: Option<LastResult>,
    pub new: NewResult,
}

pub fn generate(walk_summary: WalkSummary) -> Result<CheckResult, Error> {
    let last: Option<LastResult> = LastResult::gain(walk_summary.result_files)?;
    let new: NewResult = NewResult::create(walk_summary.contained_paths, walk_summary.walked_dir)?;
    Ok(CheckResult { last, new })
}
