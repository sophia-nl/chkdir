mod diff;
mod fs;
mod item;
mod last;
mod new;
mod options;
mod walk;

use options::ArgumentsResult::{Help, InvalidArguments, UserSelectDir, Version};

fn main() {
    match options::parse() {
        Help(help) => println!("{help}"),
        InvalidArguments(error) => error.suggestion(),
        UserSelectDir(working_dir) => {
            if let Ok(walk_result) = walk::walk(working_dir.as_path()) {
                let new_check_result = new::create(walk_result.contents, working_dir.as_path());
                last::find(&walk_result.result_files).map_or_else(
                    || {
                        new_check_result.write(&working_dir);
                        println!("\x1B[1mThe first check is done.\x1B[0m");
                    },
                    |last_check_result| {
                        let diff_result = diff::diff(&last_check_result, &new_check_result);
                        if diff_result.added.is_empty() && diff_result.deleted.is_empty() {
                            println!("\x1B[1mNo change.\x1B[0m");
                        } else {
                            new_check_result.write(&working_dir);
                            diff_result.print();
                        }
                    },
                );
            } else {
                eprintln!(
                    "\x1B[91;1merror\x1B[0;1m:\x1B[0m \"\x1B[0;1m{}\x1B[0m\" can not be read.",
                    working_dir.display()
                );
            }
        }
        Version(version) => println!("{version}"),
    }
}
