use std::fs::File;
use std::io;

use md5::{Digest, Md5};

use crate::walk::WalkEntry::{self, CommonFile, EmptyDir, UnavailableDir};

pub struct ResultItem {
    pub md5: String,
    pub path: String,
}

impl ResultItem {
    #[cfg(target_os = "windows")]
    fn format_path(&self, commonpath_len: usize) -> Self {
        let mut path = String::new();
        self.path.chars().for_each(|char: char| {
            if char == '\\' {
                path.push('/');
            } else {
                path.push(char);
            }
        });
        if let Some(sub_path) = path.get(commonpath_len..) {
            path = format!("./{}", sub_path.to_owned());
        } else {
            path = self.path.clone();
        }
        Self {
            md5: self.md5.clone(),
            path,
        }
    }
    #[cfg(target_os = "linux")]
    fn format_path(&self, commonpath_len: usize) -> Self {
        let path: String;
        if let Some(sub_path) = self.path.get(commonpath_len..) {
            path = format!(".{}", sub_path.to_owned());
        } else {
            path = self.path.clone();
        }
        Self {
            md5: self.md5.clone(),
            path,
        }
    }
}

impl WalkEntry {
    pub fn generate(self, commonpath_len: usize) -> ResultItem {
        let md5: String;
        let path: String;
        match self {
            CommonFile(entry) => {
                path = entry.to_string_lossy().to_string();
                if let Ok(mut file) = File::open(entry) {
                    let mut hasher = Md5::new();
                    if io::copy(&mut file, &mut hasher).is_ok() {
                        md5 = format!("{:x}", hasher.finalize());
                    } else {
                        md5 = String::from("        unavailable_file        ");
                    }
                } else {
                    md5 = String::from("        unavailable_file        ");
                }
            }
            EmptyDir(entry) => {
                md5 = String::from("         empty_directory        ");
                path = entry.to_string_lossy().to_string();
            }
            UnavailableDir(entry) => {
                md5 = String::from("      unavailable_directory     ");
                path = entry.to_string_lossy().to_string();
            }
        }
        ResultItem { md5, path }.format_path(commonpath_len)
    }
}
