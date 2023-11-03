use std::ops::Not;

use crate::result::LastResult;
use crate::result::NewResult;

pub struct DiffSummary {
    pub added: Vec<String>,
    pub deleted: Vec<String>,
}

pub fn diff(last_result: &LastResult, new_result: &NewResult) -> DiffSummary {
    let mut added: Vec<String> = vec![];
    let mut deleted: Vec<String> = vec![];
    let mut same: Vec<String> = vec![];
    last_result.content.iter().for_each(|last_item: &String| {
        if new_result.content.contains(last_item) {
            same.push(last_item.to_string())
        } else {
            deleted.push(last_item.to_string())
        }
    });
    new_result
        .content
        .iter()
        .filter(|new_item: &&String| same.contains(new_item).not())
        .for_each(|new_item: &String| added.push(new_item.to_string()));
    DiffSummary { added, deleted }
}

impl DiffSummary {
    pub fn print(&self) {
        if self.added.is_empty().not() {
            println!("\x1B[92;1mNewly Added:\x1B[0m")
        }
        self.added
            .iter()
            .for_each(|added_item: &String| println!("{}", added_item));
        if self.added.is_empty().not() && self.deleted.is_empty().not() {
            println!()
        }
        if self.deleted.is_empty().not() {
            println!("\x1B[96;1mRemoved:\x1B[0m")
        }
        self.deleted
            .iter()
            .for_each(|deleted_item: &String| println!("{}", deleted_item));
    }
}
