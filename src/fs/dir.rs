use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;

use crate::fs::PathType;

#[derive(Clone)]
pub struct DirItem {
    pub path: PathBuf,
}

impl DirItem {
    pub fn scan(&self) -> Result<Option<Vec<PathType>>, Error> {
        let path_items: Vec<PathType> = self
            .path
            .read_dir()?
            .flatten()
            .filter_map(|entry: DirEntry| PathType::from_pathbuf(entry.path()))
            .collect();
        if path_items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(path_items))
        }
    }
}
