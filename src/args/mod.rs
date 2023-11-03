use std::borrow::Cow;
use std::ffi::OsString;
use std::path::PathBuf;

mod error;
mod help;
mod version;

pub use error::ArgumentsError::{NoInput, NotDirPath, Unsupported};
pub use help::HelpOption;
pub use version::VersionOption;

use error::ArgumentsError;

use self::ArgumentsResult::{HelpFlag, InvalidArguments, UserSelectDir, VersionFlag};

pub enum ArgumentsResult {
    UserSelectDir(PathBuf),
    InvalidArguments(ArgumentsError),
    HelpFlag(HelpOption),
    VersionFlag(VersionOption),
}

pub fn parse(arguments: Vec<OsString>) -> ArgumentsResult {
    match arguments.len() {
        0 => InvalidArguments(NoInput),
        1 => {
            let arg: OsString = arguments[0].clone();
            let arg_string_lossy: Cow<'_, str> = arg.to_string_lossy();
            match arg_string_lossy.as_ref() {
                "-h" | "--help" => HelpFlag(HelpOption),
                "-V" | "--version" => VersionFlag(VersionOption),
                _ => {
                    let mut input_path: PathBuf = PathBuf::from(&arg);
                    if input_path.is_dir() {
                        if arg_string_lossy.ends_with('\\') {
                            UserSelectDir(input_path)
                        } else {
                            let mut arg_string: String = arg_string_lossy.to_string();
                            arg_string.push('\\');
                            input_path = PathBuf::from(arg_string);
                            UserSelectDir(input_path)
                        }
                    } else {
                        InvalidArguments(NotDirPath(arg))
                    }
                }
            }
        }
        _ => InvalidArguments(Unsupported),
    }
}
