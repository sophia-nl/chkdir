use std::fs::DirEntry;
use std::io::Error;
use std::ops::Not;
use std::path::PathBuf;

use crate::fs::PathType::{self, DirType, FileType};

pub struct WalkSummary {
    pub contained_paths: Vec<PathType>,
    pub result_files: Option<Vec<PathBuf>>,
    pub walked_dir: PathBuf,
}

pub fn walk(walked_dir: PathBuf) -> Result<WalkSummary, Error> {
    let mut contained_paths: Vec<PathType> = vec![];
    let mut result_files: Option<Vec<PathBuf>> = None;
    let mut tmp_result_files: Vec<PathBuf> = vec![];
    let path_items = walked_dir
        .read_dir()?
        .flatten()
        .filter_map(|entry: DirEntry| PathType::from_pathbuf(entry.path()));
    for item in path_items {
        match &item {
            DirType(dir) => match dir.scan()? {
                Some(paths) => contained_paths.append(&mut walk_subdirs(paths)?),
                None => contained_paths.push(item),
            },
            FileType(file) => {
                if file.is_result_file() {
                    tmp_result_files.push(file.path.clone());
                } else {
                    contained_paths.push(item);
                }
            }
        }
    }
    if tmp_result_files.is_empty().not() {
        result_files = Some(tmp_result_files);
    }
    Ok(WalkSummary {
        contained_paths,
        result_files,
        walked_dir,
    })
}

fn walk_subdirs(path_items: Vec<PathType>) -> Result<Vec<PathType>, Error> {
    let mut contained_paths: Vec<PathType> = vec![];
    for item in path_items {
        match &item {
            DirType(dir) => match dir.scan()? {
                Some(paths) => contained_paths.append(&mut walk_subdirs(paths)?),
                None => contained_paths.push(item),
            },
            FileType(_) => contained_paths.push(item),
        }
    }
    Ok(contained_paths)
}
