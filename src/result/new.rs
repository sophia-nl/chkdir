use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::sync::mpsc;

use chrono::Local;
use threadpool::ThreadPool;

use crate::item::Item::{self, CommonFileItem, EmptyDirItem};
use crate::walk::WalkResult;

impl WalkResult {
    pub fn create_new_result(&self, working_dir: &Path) -> Vec<Item> {
        let mut new_result: Vec<Item> = vec![];
        let total_num: usize = self.contents.len();
        let mut temporary_num: u64 = 0;
        let commonpath_len: usize = working_dir.to_str().unwrap().len();
        print!("Calculating MD5...\t[{temporary_num}/{total_num}]\r");
        io::stdout().flush().unwrap();
        let thread_pool: ThreadPool = ThreadPool::new(num_cpus::get());
        let (tx, rx) = mpsc::channel();
        for entry in self.contents.clone() {
            let tx_: mpsc::Sender<Item> = tx.clone();
            thread_pool.execute(move || {
                tx_.send(entry.generate(commonpath_len).unwrap()).unwrap();
            });
        }
        drop(tx);
        for item in &rx {
            new_result.push(item);
            temporary_num += 1;
            print!("Calculating MD5...\t[{temporary_num}/{total_num}]\r");
            io::stdout().flush().unwrap();
        }
        println!("\n");
        new_result
    }
}

pub trait FileWrite {
    fn write(&self, working_dir: &Path);
}

impl FileWrite for Vec<Item> {
    fn write(&self, working_dir: &Path) {
        let mut contents: Vec<(String, String)> = vec![];
        for item in self {
            match item {
                CommonFileItem(file_item) => {
                    contents.push((
                        format!("{} {}", file_item.mtime, file_item.md5),
                        file_item.path.to_string(),
                    ));
                }
                EmptyDirItem(dir_item) => {
                    contents.push((
                        "               empty_directory               ".to_string(),
                        dir_item.to_string(),
                    ));
                }
            }
        }
        contents.sort_by_key(|tuple| tuple.1.clone());
        let mut text: String = String::new();
        for line in contents {
            text.push_str(format!("{} {}\n", line.0, line.1).as_str());
        }
        let mut new_result_file: File = File::create(working_dir.join(format!(
            "checkresult-{}.txt",
            Local::now().format("%y%m%d%H%M%S")
        ))).unwrap();
        new_result_file.write_all(text.as_bytes()).unwrap();
    }
}
