use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::item::{FileInfo, Item};
use crate::walk::WalkResult;

impl WalkResult {
    pub fn get_last_result(&self) -> Option<Vec<Item>> {
        self.result_files
            .as_ref()
            .map(|result_files: &Vec<PathBuf>| {
                let mut temporary_num: u64 = 0;
                let mut last_result_file = PathBuf::new();
                for result_file in result_files {
                    if let Some(file_name) = result_file.file_name() {
                        if let Some(name_str) = file_name.to_str() {
                            if let Some(num_str) = name_str.get(12..24) {
                                if let Ok(num) = num_str.to_owned().parse::<u64>() {
                                    if num > temporary_num {
                                        temporary_num = num;
                                        last_result_file = result_file.clone();
                                    }
                                }
                            }
                        }
                    }
                }
                let mut last_result: Vec<Item> = vec![];
                if let Ok(file) = File::open(last_result_file) {
                    for line in BufReader::new(file).lines().map_while(Result::ok) {
                        if line.starts_with("               empty_directory               ") {
                            last_result
                                .push(Item::EmptyDirItem(line.get(46..).unwrap().to_string()));
                        } else {
                            let mtime: String = line.get(..12).unwrap().to_string();
                            let md5: String = line.get(13..45).unwrap().to_string();
                            let path: String = line.get(46..).unwrap().to_string();
                            last_result.push(Item::CommonFileItem(FileInfo { mtime, md5, path }));
                        }
                    }
                }
                last_result
            })
    }
}
