use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct LastCheckResult {
    pub contents: Vec<String>,
}

pub fn find(result_files_option: &Option<Vec<PathBuf>>) -> Option<LastCheckResult> {
    result_files_option
        .as_ref()
        .and_then(|result_files: &Vec<PathBuf>| {
            let mut tmp_num: u64 = 0;
            let mut last_result_file = PathBuf::new();
            for result_file in result_files {
                if let Some(file_name) = result_file.file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        if let Some(num_str) = name_str.get(12..24) {
                            if let Ok(num) = num_str.to_owned().parse::<u64>() {
                                if num > tmp_num {
                                    tmp_num = num;
                                    last_result_file = result_file.clone();
                                }
                            }
                        }
                    }
                }
            }
            let mut contents: Vec<String> = vec![];
            match File::open(last_result_file) {
                Ok(file) => {
                    for line in BufReader::new(file).lines().map_while(Result::ok) {
                        contents.push(line);
                    }
                }
                Err(_) => return None,
            }
            Some(LastCheckResult { contents })
        })
}
