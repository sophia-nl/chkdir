use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::mpsc;

use chrono::Local;
use md5::{Digest, Md5};
use threadpool::ThreadPool;

use crate::path::PathItem;
use crate::path::PathItem::DirItem;
use crate::path::PathItem::FileItem;
use crate::WalkSummary;

struct ResultItem {
    md5: String,
    path: String,
}

fn unix_style(path: String) -> String {
    let mut unix_style_path: String = String::new();
    path.chars().for_each(|c: char| {
        if c == '\\' {
            unix_style_path.push('/');
        } else {
            unix_style_path.push(c);
        }
    });
    unix_style_path
}

impl PathItem {
    fn hexdigest(&self, commonpath_len: usize) -> ResultItem {
        let md5: String;
        let mut path: String;
        match self {
            DirItem(dir_item) => {
                md5 = String::from("         empty_directory        ");
                path = dir_item.path.to_string_lossy().to_string();
            }
            FileItem(file_item) => {
                let mut file: File = File::open(file_item.path.clone()).unwrap();
                let mut hasher = Md5::new();
                io::copy(&mut file, &mut hasher).unwrap();
                md5 = format!("{:x}", hasher.finalize());
                path = file_item.path.to_string_lossy().to_string();
            }
        }
        path = format!("./{}", unix_style(path[commonpath_len..].to_string()));
        ResultItem { md5, path }
    }
}

pub struct NewResult {
    pub content: Vec<String>,
}

fn print_progress(tmp_num: u64, total_num: usize) {
    print!("Calculating MD5...\t[{}/{}]\r", tmp_num, total_num);
    io::stdout().flush().unwrap();
}

impl WalkSummary {
    pub fn new_result(&self) -> NewResult {
        let mut result_items: Vec<ResultItem> = vec![];
        let total_num: usize = self.path_items.len();
        let mut tmp_num: u64 = 0;
        let commonpath_len: usize = self.walked_dir.to_str().unwrap().len();
        print_progress(tmp_num, total_num);
        let pool: ThreadPool = ThreadPool::new(num_cpus::get());
        let (tx, rx) = mpsc::channel();
        self.path_items.clone().into_iter().for_each(|e: PathItem| {
            let tx: mpsc::Sender<ResultItem> = tx.clone();
            pool.execute(move || {
                tx.send(e.hexdigest(commonpath_len)).unwrap();
            });
        });
        drop(tx);
        rx.iter().for_each(|f: ResultItem| {
            result_items.push(f);
            tmp_num += 1;
            print_progress(tmp_num, total_num);
        });
        println!("\n");
        result_items.sort_by(|i: &ResultItem, j: &ResultItem| i.path.cmp(&j.path));
        let mut content: Vec<String> = vec![];
        result_items
            .into_iter()
            .for_each(|m: ResultItem| content.push(format!("{} {}", m.md5, m.path)));
        NewResult { content }
    }
}

impl NewResult {
    pub fn write(&self, path: PathBuf) {
        let mut text: String = String::new();
        self.content.iter().for_each(|e: &String| {
            text.push_str(&format!("{}\n", e));
        });
        let mut file: File = File::create(path.join(format!(
            "checkresult-{}.txt",
            Local::now().format("%y%m%d%H%M%S")
        )))
        .unwrap();
        file.write_all(text.as_bytes()).unwrap();
    }
}
