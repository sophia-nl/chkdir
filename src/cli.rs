use std::env;
use std::path::PathBuf;
use std::process;

struct UserInput {
    content: String,
}

fn print_usage() {
    println!("\x1B[1mUsage:\x1B[0m chkdir <directory>")
}

impl UserInput {
    fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        match args.len() {
            1 => {
                eprintln!("\x1B[91;1merror\x1B[0;1m:\x1B[0m the arguments were not provided.\n");
                print_usage();
                process::exit(66)
            }
            2 => Self {
                content: args[1].clone(),
            },
            _ => {
                eprintln!("\x1B[91;1merror\x1B[0;1m:\x1B[0m the input was not valid.\n");
                print_usage();
                process::exit(64)
            }
        }
    }
    fn to_pathbuf(&self) -> PathBuf {
        let mut input_content: String = self.content.clone();
        if input_content.ends_with('\\') {
            input_content.into()
        } else {
            input_content.push('\\');
            input_content.into()
        }
    }
}

pub struct UserSelectedFolder {
    pub path: PathBuf,
}

fn print_version() {
    println!("chkdir 0.1.0")
}

impl UserSelectedFolder {
    pub fn new() -> Self {
        let user_input: UserInput = UserInput::new();
        match user_input.content.as_str() {
            "-h" | "--help" => {
                print_version();
                println!();
                print_usage();
                process::exit(0)
            }
            "-V" | "--version" => {
                print_version();
                process::exit(0)
            }
            _ => {
                let path: PathBuf = user_input.to_pathbuf();
                if path.is_dir() {
                    Self { path }
                } else {
                    eprintln!(
                        "\x1B[91;1merror\x1B[0;1m: \"{}\" \x1B[0mwas not available.",
                        user_input.content
                    );
                    process::exit(65)
                }
            }
        }
    }
}
