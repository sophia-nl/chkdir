extern crate alloc;
use alloc::borrow::Cow;
use core::ops::Not;
use std::path::PathBuf;

pub struct FileItem {
    pub path: PathBuf,
}

#[allow(clippy::case_sensitive_file_extension_comparisons)]
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
                    match file_name_str.get(12..24) {
                        Some(num_str) => {
                            for char in num_str.chars() {
                                if char.is_ascii_digit().not() {
                                    return false;
                                }
                            }
                        }
                        None => return false,
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
