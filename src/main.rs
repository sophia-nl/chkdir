use std::path::PathBuf;

use clap::Parser;

mod cli;
mod item;
mod result;
mod walk;
use cli::Args;
use item::Item;
use result::new::FileWrite;
use result::DiffResult::{Changed, NoChange};
use result::Difference;
use walk::{Walk, WalkResult};

fn main() {
    let args: Args = Args::parse();
    let working_dir: PathBuf = args.target_directory;
    let walk_result: WalkResult = working_dir.walk_root().unwrap();
    match walk_result.get_last_result() {
        Some(last_result) => {
            let new_result: Vec<Item> = if args.quick {
                todo!()
            } else {
                walk_result.create_new_result(&working_dir)
            };
            match last_result.diff(&new_result) {
                Changed(changed_result) => {
                    new_result.write(&working_dir);
                    println!("{}", changed_result)
                }
                NoChange => println!("\x1B[1mNo change.\x1B[0m"),
            }
        }
        None => {
            let new_result: Vec<Item> = walk_result.create_new_result(&working_dir);
            new_result.write(&working_dir);
            println!("\x1B[1mThe first check is done.\x1B[0m");
        }
    }
}
