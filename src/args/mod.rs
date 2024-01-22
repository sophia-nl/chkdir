extern crate alloc;
use alloc::borrow::Cow;
use core::ops::Not;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

mod error;
mod help;
mod version;

use self::ArgumentsResult::{HelpFlag, InvalidArguments, UserSelectDir, VersionFlag};
use error::ArgumentsError::{self, NoInput, NotDirPath, Unsupported};
use help::HelpOption;
use version::VersionOption;

pub enum ArgumentsResult {
    UserSelectDir(PathBuf),
    InvalidArguments(ArgumentsError),
    HelpFlag(HelpOption),
    VersionFlag(VersionOption),
}

pub fn parse() -> ArgumentsResult {
    let mut arguments: Vec<OsString> = env::args_os().skip(1).collect();
    arguments
        .pop()
        .map_or(InvalidArguments(NoInput), |arg: OsString| {
            match arguments.pop() {
                Some(_) => InvalidArguments(Unsupported),
                None => analyze(&arg),
            }
        })
}

#[cfg(target_os = "windows")]
fn analyze(arg: &OsString) -> ArgumentsResult {
    let arg_str: Cow<'_, str> = arg.to_string_lossy();
    match arg_str.as_ref() {
        "-h" | "--help" => HelpFlag(HelpOption),
        "-V" | "--version" => VersionFlag(VersionOption),
        _ => {
            let mut input_path: PathBuf = PathBuf::from(&arg);
            if input_path.is_dir() {
                if arg_str.ends_with('\\').not() {
                    let mut input_text: String = arg_str.to_string();
                    input_text.push('\\');
                    input_path = PathBuf::from(input_text);
                }
                UserSelectDir(input_path)
            } else {
                InvalidArguments(NotDirPath(arg.clone()))
            }
        }
    }
}
