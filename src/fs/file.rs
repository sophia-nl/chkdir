use std::borrow::Cow;
use std::ops::Not;
use std::path::PathBuf;

#[derive(Clone)]
pub struct FileItem {
    pub path: PathBuf,
}

impl FileItem {
    pub fn is_result_file(&self) -> bool {
        match self.path.file_name() {
            Some(file_name) => {
                let file_name_string_lossy: Cow<'_, str> = file_name.to_string_lossy();
                let file_name_str: &str = file_name_string_lossy.as_ref();
                if file_name_str.len().eq(&28)
                    && file_name_str.starts_with("checkresult-")
                    && file_name_str.ends_with(".txt")
                {
                    for char in file_name_str[12..24].chars() {
                        if char.is_ascii_digit().not() {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}
