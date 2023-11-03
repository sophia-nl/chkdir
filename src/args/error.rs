use std::ffi::OsString;
use std::process;

use crate::args::HelpOption;

use self::ArgumentsError::{NoInput, NotDirPath, Unsupported};

pub enum ArgumentsError {
    NoInput,
    NotDirPath(OsString),
    Unsupported,
}

impl ArgumentsError {
    pub fn print_and_exit(&self) {
        match self {
            NoInput => {
                eprintln!("\x1B[91;1merror\x1B[0;1m:\x1B[0m the arguments were not provided.\n");
                HelpOption.print();
                process::exit(exit_code::EX_NOINPUT)
            }
            NotDirPath(input_string) => {
                eprintln!(
                    "\x1B[91;1merror\x1B[0;1m:\x1B[0m \"\x1B[0;1m{}\x1B[0m\" was not an available folder path.",
                    input_string.to_string_lossy()
                );
                process::exit(exit_code::EX_DATAERR)
            }
            Unsupported => {
                eprintln!("\x1B[91;1merror\x1B[0;1m:\x1B[0m the user inputs were not supported.\n");
                HelpOption.print();
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
