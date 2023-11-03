use std::fs::File;
use std::io::{self, Error, Write};
use std::path::PathBuf;
use std::sync::mpsc;

use chrono::Local;
use md5::{Digest, Md5};
use threadpool::ThreadPool;

use crate::fs::PathType::{self, DirType, FileType};

pub struct NewResult {
    pub content: Vec<String>,
}

impl NewResult {
    pub fn create(contained_paths: Vec<PathType>, walked_dir: PathBuf) -> Result<NewResult, Error> {
        let mut new_result_items: Vec<NewResultItem> = vec![];
        let total_num: usize = contained_paths.len();
        let mut tmp_num: u64 = 0;
        let commonpath_len: usize = walked_dir.to_str().unwrap().len();
        print_progress(tmp_num, total_num);
        let thread_pool: ThreadPool = ThreadPool::new(num_cpus::get());
        let (tx, rx) = mpsc::channel();
        contained_paths
            .clone()
            .into_iter()
            .for_each(|path_item: PathType| {
                let tx: mpsc::Sender<NewResultItem> = tx.clone();
                thread_pool.execute(move || {
                    tx.send(NewResultItem::from_pathtype(path_item, commonpath_len).unwrap())
                        .unwrap();
                });
            });
        drop(tx);
        rx.iter().for_each(|item: NewResultItem| {
            new_result_items.push(item);
            tmp_num += 1;
            print_progress(tmp_num, total_num);
        });
        println!("\n");
        new_result_items.sort_by(|i: &NewResultItem, j: &NewResultItem| i.path.cmp(&j.path));
        let mut content: Vec<String> = vec![];
        new_result_items
            .into_iter()
            .for_each(|result: NewResultItem| {
                content.push(format!("{} {}", result.md5, result.path))
            });
        Ok(NewResult { content })
    }
    pub fn write(&self, path: PathBuf) -> Result<(), Error> {
        let mut text: String = String::new();
        self.content.iter().for_each(|line: &String| {
            text.push_str(&format!("{}\n", line));
        });
        let mut file: File = File::create(path.join(format!(
            "checkresult-{}.txt",
            Local::now().format("%y%m%d%H%M%S")
        )))?;
        file.write_all(text.as_bytes())?;
        Ok(())
    }
}

struct NewResultItem {
    md5: String,
    path: String,
}

impl NewResultItem {
    fn from_pathtype(path_item: PathType, commonpath_len: usize) -> Result<NewResultItem, Error> {
        let md5: String;
        let mut path: String;
        match path_item {
            DirType(dir_item) => {
                md5 = String::from("         empty_directory        ");
                path = dir_item.path.to_string_lossy().to_string();
            }
            FileType(file_item) => {
                let mut file: File = File::open(file_item.path.clone())?;
                let mut hasher = Md5::new();
                io::copy(&mut file, &mut hasher)?;
                md5 = format!("{:x}", hasher.finalize());
                path = file_item.path.to_string_lossy().to_string();
            }
        }
        path = format!("./{}", to_unix_style(path[commonpath_len..].to_string()));
        Ok(NewResultItem { md5, path })
    }
}

fn to_unix_style(path: String) -> String {
    let mut unix_style_path: String = String::new();
    path.chars().for_each(|char: char| {
        if char == '\\' {
            unix_style_path.push('/');
        } else {
            unix_style_path.push(char);
        }
    });
    unix_style_path
}

fn print_progress(tmp_num: u64, total_num: usize) {
    print!("Calculating MD5...\t[{}/{}]\r", tmp_num, total_num);
    io::stdout().flush().unwrap();
}
