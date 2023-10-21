use std::process;

mod cli;
mod diff;
mod last_result;
mod new_result;
mod path;
mod walk;

use cli::UserSelectedFolder;
use diff::DiffSummary;
use new_result::NewResult;
use walk::WalkSummary;

fn main() {
    let user_selected_folder: UserSelectedFolder = UserSelectedFolder::new();
    let walk_summary: WalkSummary = user_selected_folder.walk_dir();
    let new_result: NewResult = walk_summary.new_result();
    match walk_summary.last_result() {
        Some(last_result) => {
            let diff_summary: DiffSummary = diff::diff(&last_result, &new_result);
            if diff_summary.added.is_empty() && diff_summary.deleted.is_empty() {
                println!("\x1B[1mNo change.\x1B[0m");
                process::exit(0)
            }
            new_result.write(user_selected_folder.path);
            diff_summary.print()
        }
        None => {
            new_result.write(user_selected_folder.path);
            println!("\x1B[1mThe first check is done.\x1B[0m")
        }
    }
}
