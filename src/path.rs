use std::fs::DirEntry;
use std::io::Error;
use std::ops::Not;
use std::path::PathBuf;

#[derive(Clone)]
pub struct DirItem {
    pub path: PathBuf,
}

impl DirItem {
    pub fn is_empty(&self) -> bool {
        let dir_content: Vec<PathBuf> = self
            .path
            .read_dir()
            .unwrap()
            .map(|e: Result<DirEntry, Error>| e.unwrap().path())
            .collect();
        dir_content.is_empty()
    }
}

#[derive(Clone)]
pub struct FileItem {
    pub path: PathBuf,
}

impl FileItem {
    pub fn is_result_file(&self) -> bool {
        let file_name: String = self.path.file_name().unwrap().to_string_lossy().to_string();
        if file_name.len().eq(&28)
            && file_name.starts_with("checkresult-")
            && file_name.ends_with(".txt")
        {
            for char in file_name[12..24].chars() {
                if char.is_ascii_digit().not() {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub enum PathItem {
    DirItem(DirItem),
    FileItem(FileItem),
}

impl PathItem {
    pub fn from(path: PathBuf) -> Option<Self> {
        if path.is_dir() {
            Some(PathItem::DirItem(DirItem { path }))
        } else if path.is_file() {
            Some(PathItem::FileItem(FileItem { path }))
        } else {
            None
        }
    }
}
