use std::fmt::{Display, Formatter, Result};
use std::ops::Not;

use crate::item::Item::{self, CommonFileItem, EmptyDirItem};

mod last;
pub mod new;

pub struct ChangedResult {
    added: Vec<String>,
    deleted: Vec<String>,
}

pub enum DiffResult {
    Changed(ChangedResult),
    NoChange,
}

pub trait Difference {
    fn diff(&self, new_result: &[Item]) -> DiffResult;
}

impl Difference for Vec<Item> {
    fn diff(&self, new_result: &[Item]) -> DiffResult {
        let mut last_inners: Vec<String> = vec![];
        self.iter().for_each(|last_item| match last_item {
            CommonFileItem(last_file_info) => {
                last_inners.push(format!(
                    "{} {} {}",
                    last_file_info.mtime, last_file_info.md5, last_file_info.path
                ));
            }
            EmptyDirItem(last_dir_path) => {
                last_inners.push(format!(
                    "               empty_directory                 {}",
                    last_dir_path
                ));
            }
        });
        let mut new_inners: Vec<String> = vec![];
        new_result.iter().for_each(|new_item| match new_item {
            CommonFileItem(new_file_info) => {
                new_inners.push(format!(
                    "{} {} {}",
                    new_file_info.mtime, new_file_info.md5, new_file_info.path
                ));
            }
            EmptyDirItem(new_dir_path) => {
                new_inners.push(format!(
                    "               empty_directory                 {}",
                    new_dir_path
                ));
            }
        });
        let mut added: Vec<String> = vec![];
        let mut deleted: Vec<String> = vec![];
        let mut same: Vec<String> = vec![];
        last_inners.iter().for_each(|last_inner: &String| {
            if new_inners.contains(last_inner) {
                same.push(last_inner.to_string());
            } else {
                deleted.push(last_inner.to_string());
            }
        });
        new_inners
            .iter()
            .filter(|new_inner: &&String| same.contains(new_inner).not())
            .for_each(|new_inner: &String| added.push(new_inner.to_string()));
        if added.is_empty() && deleted.is_empty() {
            DiffResult::NoChange
        } else {
            DiffResult::Changed(ChangedResult { added, deleted })
        }
    }
}

impl Display for ChangedResult {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let mut text: String = String::new();
        if self.added.is_empty().not() {
            text.push_str("\x1B[92;1mNewly Added:\x1B[0m\n");
        }
        self.added
            .iter()
            .for_each(|added_item: &String| text.push_str(&format!("{}\n", added_item)));
        if self.added.is_empty().not() && self.deleted.is_empty().not() {
            text.push('\n');
        }
        if self.deleted.is_empty().not() {
            text.push_str("\x1B[96;1mRemoved:\x1B[0m\n");
        }
        self.deleted
            .iter()
            .for_each(|deleted_item: &String| text.push_str(&format!("{}\n", deleted_item)));
        write!(formatter, "{text}")
    }
}
