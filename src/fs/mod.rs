use std::path::PathBuf;

mod dir;
mod file;

use self::PathType::{DirType, FileType};
use dir::DirItem;
use file::FileItem;

pub enum PathType {
    DirType(DirItem),
    FileType(FileItem),
}

impl PathType {
    pub fn from_pathbuf(path: PathBuf) -> Option<Self> {
        if path.is_dir() {
            Some(DirType(DirItem { path }))
        } else if path.is_file() {
            Some(FileType(FileItem { path }))
        } else {
            None
        }
    }
}
