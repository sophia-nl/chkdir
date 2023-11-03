use std::env;
use std::ffi::OsString;
use std::io::Error;
use std::path::PathBuf;
use std::process;

mod args;
mod fs;
mod result;
mod walk;

use args::ArgumentsResult::{HelpFlag, InvalidArguments, UserSelectDir, VersionFlag};
use result::{CheckResult, DiffSummary};
use walk::WalkSummary;

fn main() {
    let arguments: Vec<OsString> = env::args_os().skip(1).collect();
    match args::parse(arguments) {
        UserSelectDir(working_dir) => {
            let chkdir: Chkdir = Chkdir { working_dir };
            chkdir.run().unwrap()
        }
        InvalidArguments(error) => error.print_and_exit(),
        HelpFlag(help) => help.print(),
        VersionFlag(version) => version.print(),
    }
}

struct Chkdir {
    working_dir: PathBuf,
}

impl Chkdir {
    fn run(&self) -> Result<(), Error> {
        let walk_summary: WalkSummary = walk::walk(self.working_dir.clone())?;
        let check_result: CheckResult = result::generate(walk_summary)?;
        match check_result.last {
            Some(last_result) => {
                let diff_summary: DiffSummary = result::diff(&last_result, &check_result.new);
                if diff_summary.added.is_empty() && diff_summary.deleted.is_empty() {
                    println!("\x1B[1mNo change.\x1B[0m");
                    process::exit(0)
                }
                let _ = check_result.new.write(self.working_dir.clone());
                diff_summary.print();
                Ok(())
            }
            None => {
                let _ = check_result.new.write(self.working_dir.clone());
                println!("\x1B[1mThe first check is done.\x1B[0m");
                Ok(())
            }
        }
    }
}
