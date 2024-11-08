use std::borrow::Cow;
use std::io::Error;
use std::ops::Not;
use std::path::PathBuf;

use self::WalkEntry::{CommonFile, EmptyDir};

#[derive(Clone)]
pub enum WalkEntry {
    CommonFile(PathBuf),
    EmptyDir(PathBuf),
}

pub struct WalkResult {
    pub contents: Vec<WalkEntry>,
    pub result_files: Option<Vec<PathBuf>>,
}

pub trait Walk {
    fn walk_root(&self) -> Result<WalkResult, Error>;
    fn walk_sub(&self) -> Result<Vec<WalkEntry>, Error>;
}

impl Walk for PathBuf {
    fn walk_root(&self) -> Result<WalkResult, Error> {
        let mut contents: Vec<WalkEntry> = vec![];
        let mut result_files: Option<Vec<PathBuf>> = None;
        let mut temporary_result_files: Vec<PathBuf> = vec![];
        self.read_dir()?
            .flatten()
            .map(|entry| entry.path())
            .for_each(|path| {
                if path.is_dir() {
                    if path.read_dir().unwrap().flatten().count().eq(&0) {
                        contents.push(EmptyDir(path));
                    } else {
                        contents.append(&mut path.walk_sub().unwrap());
                    }
                } else if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        let file_name_string_lossy: Cow<'_, str> = file_name.to_string_lossy();
                        let file_name_str: &str = file_name_string_lossy.as_ref();
                        if file_name_str.len().eq(&28)
                            && file_name_str.starts_with("checkresult-")
                            && file_name_str.ends_with(".txt")
                        {
                            if let Some(num_str) = file_name_str.get(12..24) {
                                match num_str.parse::<u64>() {
                                    Ok(_) => temporary_result_files.push(path),
                                    Err(_) => contents.push(CommonFile(path)),
                                }
                            };
                        } else {
                            contents.push(CommonFile(path))
                        }
                    }
                }
            });
        if temporary_result_files.is_empty().not() {
            result_files = Some(temporary_result_files);
        }
        Ok(WalkResult {
            contents,
            result_files,
        })
    }
    fn walk_sub(&self) -> Result<Vec<WalkEntry>, Error> {
        let mut contents: Vec<WalkEntry> = vec![];
        self.read_dir()?
            .flatten()
            .map(|entry| entry.path())
            .for_each(|path| {
                if path.is_dir() {
                    if path.read_dir().unwrap().flatten().count().eq(&0) {
                        contents.push(EmptyDir(path));
                    } else {
                        contents.append(&mut path.walk_sub().unwrap());
                    }
                } else if path.is_file() {
                    contents.push(CommonFile(path))
                }
            });
        Ok(contents)
    }
}
