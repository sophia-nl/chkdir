use core::ops::Not;
use std::fs::DirEntry;
use std::io::Error;
use std::path::{Path, PathBuf};

use self::WalkEntry::{CommonFile, EmptyDir, UnavailableDir};
use crate::fs::PathType::{self, DirType, FileType};

pub enum WalkEntry {
    CommonFile(PathBuf),
    EmptyDir(PathBuf),
    UnavailableDir(PathBuf),
}

pub struct WalkResult {
    pub contents: Vec<WalkEntry>,
    pub result_files: Option<Vec<PathBuf>>,
}

fn walk_subdirs(path_types: Vec<PathType>) -> Vec<WalkEntry> {
    let mut contents: Vec<WalkEntry> = vec![];
    for path_type in path_types {
        match path_type {
            DirType(dir) => match dir.scan() {
                Ok(path_types_option) => match path_types_option {
                    Some(inner_path_types) => contents.append(&mut walk_subdirs(inner_path_types)),
                    None => contents.push(EmptyDir(dir.path)),
                },
                Err(_) => contents.push(UnavailableDir(dir.path)),
            },
            FileType(file) => contents.push(CommonFile(file.path)),
        }
    }
    contents
}

pub fn walk(path: &Path) -> Result<WalkResult, Error> {
    let mut contents: Vec<WalkEntry> = vec![];
    let mut result_files: Option<Vec<PathBuf>> = None;
    let mut tmp_result_files: Vec<PathBuf> = vec![];
    match path.read_dir() {
        Ok(read_dir) => {
            let path_types = read_dir
                .flatten()
                .filter_map(|entry: DirEntry| PathType::from_pathbuf(entry.path()));
            for path_type in path_types {
                match path_type {
                    DirType(dir) => match dir.scan() {
                        Ok(path_types_option) => match path_types_option {
                            Some(inner_path_types) => {
                                contents.append(&mut walk_subdirs(inner_path_types));
                            }
                            None => contents.push(EmptyDir(dir.path)),
                        },
                        Err(_) => contents.push(UnavailableDir(dir.path)),
                    },
                    FileType(file) => {
                        if file.is_result_file() {
                            tmp_result_files.push(file.path);
                        } else {
                            contents.push(CommonFile(file.path));
                        }
                    }
                }
            }
            if tmp_result_files.is_empty().not() {
                result_files = Some(tmp_result_files);
            }
            Ok(WalkResult {
                contents,
                result_files,
            })
        }
        Err(error) => Err(error),
    }
}
