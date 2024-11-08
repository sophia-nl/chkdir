#![allow(clippy::if_same_then_else)]

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(value_parser = parse_path)]
    pub target_directory: PathBuf,
    #[arg(long, short)]
    pub quick: bool,
}

fn parse_path(input: &str) -> Result<PathBuf, String> {
    let mut path: PathBuf = PathBuf::from(input);
    let mut string: String = input.to_owned();
    if path.is_dir() {
        if cfg!(target_os = "windows") && input.ends_with('\\') {
            string.pop();
            path = PathBuf::from(string);
        } else if cfg!(target_os = "linux") && input.ends_with('/') {
            string.pop();
            path = PathBuf::from(string);
        }
        Ok(path)
    } else {
        Err(format!("'{}' was not an available directory path.", input))
    }
}
