use std::ops::Not;

use crate::last_result::LastResult;
use crate::NewResult;

pub struct DiffSummary {
    pub added: Vec<String>,
    pub deleted: Vec<String>,
}

pub fn diff(last_result: &LastResult, new_result: &NewResult) -> DiffSummary {
    let mut added: Vec<String> = vec![];
    let mut deleted: Vec<String> = vec![];
    let mut same: Vec<String> = vec![];
    last_result.content.iter().for_each(|e: &String| {
        if new_result.content.contains(e) {
            same.push(e.to_string())
        } else {
            deleted.push(e.to_string())
        }
    });
    new_result
        .content
        .iter()
        .filter(|i: &&String| same.contains(i).not())
        .for_each(|j: &String| added.push(j.to_string()));
    DiffSummary { added, deleted }
}

impl DiffSummary {
    pub fn print(&self) {
        if self.added.is_empty().not() {
            println!("\x1B[92;1mNewly Added:\x1B[0m")
        }
        self.added.iter().for_each(|i: &String| println!("{}", i));
        if self.added.is_empty().not() && self.deleted.is_empty().not() {
            println!()
        }
        if self.deleted.is_empty().not() {
            println!("\x1B[96;1mRemoved:\x1B[0m")
        }
        self.deleted.iter().for_each(|j: &String| println!("{}", j));
    }
}
