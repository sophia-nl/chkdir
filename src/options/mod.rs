extern crate alloc;
use alloc::borrow::Cow;
use core::ops::Not;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

mod error;
mod help;
mod version;

use self::ArgumentsResult::{Help, InvalidArguments, UserSelectDir, Version};
use error::ArgumentsError::{self, NoArgument, NotDirPath, Unsupported};
use help::HelpString;
use version::VersionString;

pub enum ArgumentsResult {
    Help(HelpString),
    InvalidArguments(ArgumentsError),
    UserSelectDir(PathBuf),
    Version(VersionString),
}

pub fn parse() -> ArgumentsResult {
    let mut arguments: Vec<OsString> = env::args_os().skip(1).collect();
    arguments
        .pop()
        .map_or(InvalidArguments(NoArgument), |arg: OsString| {
            if arguments.pop().is_some() {
                InvalidArguments(Unsupported)
            } else {
                let arg_string_lossy: Cow<'_, str> = arg.to_string_lossy();
                let arg_str: &str = arg_string_lossy.as_ref();
                match arg_str {
                    "-h" | "--help" => Help(HelpString),
                    "-V" | "--version" => Version(VersionString),
                    _ => {
                        let mut path: PathBuf = PathBuf::from(arg_str);
                        if path.is_dir() {
                            if cfg!(target_os = "windows") && arg_str.ends_with('\\').not() {
                                let mut string: String = arg_str.to_owned();
                                string.push('\\');
                                path = PathBuf::from(string);
                            }
                            UserSelectDir(path)
                        } else {
                            InvalidArguments(NotDirPath(arg))
                        }
                    }
                }
            }
        })
}
