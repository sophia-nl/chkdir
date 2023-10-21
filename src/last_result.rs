use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

use crate::WalkSummary;

pub struct LastResult {
    pub content: Vec<String>,
}

impl WalkSummary {
    pub fn last_result(&self) -> Option<LastResult> {
        let result_files: Vec<PathBuf> = self.result_files.clone()?;
        let mut tmp_num: u64 = 0;
        let mut last_result_file: PathBuf = PathBuf::new();
        result_files.iter().for_each(|e: &PathBuf| {
            let file_num: u64 = e.file_name().unwrap().to_str().unwrap()[12..24]
                .to_string()
                .parse::<u64>()
                .unwrap();
            if file_num > tmp_num {
                tmp_num = file_num;
                last_result_file = e.clone();
            }
        });
        let mut content: Vec<String> = vec![];
        BufReader::new(File::open(last_result_file).unwrap())
            .lines()
            .for_each(|i: Result<String, Error>| content.push(i.unwrap()));
        Some(LastResult { content })
    }
}
