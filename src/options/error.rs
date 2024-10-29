use std::ffi::OsString;
use std::process;

use self::ArgumentsError::{NoArgument, NotDirPath, Unsupported};
use crate::options::HelpString;

pub enum ArgumentsError {
    NoArgument,
    NotDirPath(OsString),
    Unsupported,
}

impl ArgumentsError {
    pub fn suggestion(self) {
        match self {
            NoArgument => {
                eprintln!("\x1B[91;1merror\x1B[0;1m:\x1B[0m the arguments were not provided.\n\n{HelpString}");
                process::exit(exit_code::EX_NOINPUT)
            }
            NotDirPath(user_input) => {
                eprintln!("\x1B[91;1merror\x1B[0;1m:\x1B[0m \"\x1B[0;1m{}\x1B[0m\" was not an available folder path.", user_input.to_string_lossy());
                process::exit(exit_code::EX_DATAERR)
            }
            Unsupported => {
                eprintln!("\x1B[91;1merror\x1B[0;1m:\x1B[0m the user inputs were not supported.\n\n{HelpString}");
                process::exit(exit_code::EX_USAGE)
            }
        }
    }
}

mod exit_code {
    pub const EX_USAGE: i32 = 64;
    pub const EX_DATAERR: i32 = 65;
    pub const EX_NOINPUT: i32 = 66;
}
