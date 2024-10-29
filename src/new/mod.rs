use std::io::{self, Write};
use std::path::Path;
use std::sync::mpsc;

use threadpool::ThreadPool;

mod write;

use crate::item::ResultItem;
use crate::walk::WalkEntry;

pub struct NewCheckResult {
    pub contents: Vec<String>,
}

fn print_progress(tmp_num: u64, total_num: usize) {
    #![allow(clippy::unwrap_used)]
    print!("Calculating MD5...\t[{tmp_num}/{total_num}]\r");
    io::stdout().flush().unwrap();
}

pub fn create(walk_result_contents: Vec<WalkEntry>, working_dir: &Path) -> NewCheckResult {
    #![allow(clippy::unwrap_used)]
    let mut new_result_items: Vec<ResultItem> = vec![];
    let total_num: usize = walk_result_contents.len();
    let mut tmp_num: u64 = 0;
    let commonpath_len = working_dir.to_str().unwrap().len();
    print_progress(tmp_num, total_num);
    let thread_pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = mpsc::channel();
    for path_item in walk_result_contents {
        let tx_: mpsc::Sender<ResultItem> = tx.clone();
        thread_pool.execute(move || {
            tx_.send(path_item.generate(commonpath_len)).unwrap();
        });
    }
    drop(tx);
    for item in &rx {
        new_result_items.push(item);
        tmp_num += 1;
        print_progress(tmp_num, total_num);
    }
    println!("\n");
    new_result_items.sort_by(|i: &ResultItem, j: &ResultItem| i.path.cmp(&j.path));
    let mut contents: Vec<String> = vec![];
    new_result_items
        .into_iter()
        .for_each(|result: ResultItem| contents.push(format!("{} {}", result.md5, result.path)));
    NewCheckResult { contents }
}
