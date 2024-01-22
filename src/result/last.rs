use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

pub struct LastResult {
    pub content: Vec<String>,
}

impl LastResult {
    pub fn gain(result_files_state: Option<Vec<PathBuf>>) -> Result<Option<Self>, Error> {
        match result_files_state {
            Some(result_files) => {
                let mut tmp_num: u64 = 0;
                let mut last_result_file: PathBuf = PathBuf::new();
                for file in result_files {
                    if let Some(name) = file.file_name() {
                        if let Some(name_str) = name.to_str() {
                            if let Ok(file_num) = name_str[12..24].to_string().parse::<u64>() {
                                if file_num > tmp_num {
                                    tmp_num = file_num;
                                    last_result_file = file;
                                }
                            }
                        }
                    }
                }
                let mut content: Vec<String> = vec![];
                for line in BufReader::new(File::open(last_result_file)?).lines() {
                    content.push(line?);
                }
                Ok(Some(Self { content }))
            }
            None => Ok(None),
        }
    }
}
