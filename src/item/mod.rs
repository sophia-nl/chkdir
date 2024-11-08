use std::fs::File;
use std::io::{self, Error};

use chrono::{DateTime, Local};
use md5::{Digest, Md5};

use crate::walk::WalkEntry::{self, CommonFile, EmptyDir};

pub struct FileInfo {
    pub mtime: String,
    pub md5: String,
    pub path: String,
}

pub enum Item {
    CommonFileItem(FileInfo),
    EmptyDirItem(String),
}

trait Uniform {
    fn uniform(&self, commonpath_len: usize) -> String;
}

impl Uniform for String {
    #[cfg(target_os = "windows")]
    fn uniform(&self, commonpath_len: usize) -> String {
        let mut path: String = String::new();
        self.chars().for_each(|char: char| {
            if char == '\\' {
                path.push('/');
            } else {
                path.push(char);
            }
        });
        if let Some(sub_path) = path.get(commonpath_len..) {
            path = format!(".{}", sub_path);
        }
        path
    }
}

impl WalkEntry {
    pub fn generate(&self, commonpath_len: usize) -> Result<Item, Error> {
        match self {
            CommonFile(file_path) => {
                let last_modification_time: DateTime<Local> =
                    file_path.metadata()?.modified()?.into();
                let mtime: String = last_modification_time.format("%y%m%d%H%M%S").to_string();
                let mut hasher = Md5::new();
                io::copy(&mut File::open(file_path)?, &mut hasher)?;
                let md5: String = format!("{:x}", hasher.finalize());
                let path = file_path
                    .to_string_lossy()
                    .to_string()
                    .uniform(commonpath_len);
                Ok(Item::CommonFileItem(FileInfo { mtime, md5, path }))
            }
            EmptyDir(dir_path) => Ok(Item::EmptyDirItem(
                dir_path
                    .to_string_lossy()
                    .to_string()
                    .uniform(commonpath_len),
            )),
        }
    }
}
