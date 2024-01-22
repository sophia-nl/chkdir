use std::path::PathBuf;

mod dir;
mod file;

pub use dir::DirItem;
pub use file::FileItem;

use self::PathType::{DirType, FileType};

#[derive(Clone)]
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
