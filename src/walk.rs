use std::fs::DirEntry;
use std::io::Error;
use std::ops::Not;
use std::path::PathBuf;

use crate::path::PathItem;
use crate::path::PathItem::DirItem;
use crate::path::PathItem::FileItem;
use crate::UserSelectedFolder;

pub struct WalkSummary {
    pub path_items: Vec<PathItem>,
    pub result_files: Option<Vec<PathBuf>>,
    pub walked_dir: PathBuf,
}

fn walk_subdir(path: PathBuf) -> Vec<PathItem> {
    let mut path_items: Vec<PathItem> = vec![];
    path.read_dir()
        .unwrap()
        .filter_map(|e: Result<DirEntry, Error>| PathItem::from(e.unwrap().path()))
        .for_each(|f: PathItem| match &f {
            DirItem(dir_item) => {
                if dir_item.is_empty() {
                    path_items.push(f)
                } else {
                    path_items.append(&mut walk_subdir(dir_item.path.clone()))
                }
            }
            FileItem(_) => path_items.push(f),
        });
    path_items
}

impl UserSelectedFolder {
    pub fn walk_dir(&self) -> WalkSummary {
        let walked_dir: PathBuf = self.path.clone();
        let mut path_items: Vec<PathItem> = vec![];
        let mut result_files: Option<Vec<PathBuf>> = None;
        let mut tmp_result_files: Vec<PathBuf> = vec![];
        walked_dir
            .read_dir()
            .unwrap()
            .filter_map(|e: Result<DirEntry, Error>| PathItem::from(e.unwrap().path()))
            .for_each(|f: PathItem| match &f {
                DirItem(dir_item) => {
                    if dir_item.is_empty() {
                        path_items.push(f)
                    } else {
                        path_items.append(&mut walk_subdir(dir_item.path.clone()))
                    }
                }
                FileItem(file_item) => {
                    if file_item.is_result_file() {
                        tmp_result_files.push(file_item.path.clone())
                    } else {
                        path_items.push(f)
                    }
                }
            });
        if tmp_result_files.is_empty().not() {
            result_files = Some(tmp_result_files)
        }
        WalkSummary {
            path_items,
            result_files,
            walked_dir,
        }
    }
}
