use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;

use chrono::Local;

use crate::new::NewCheckResult;

impl NewCheckResult {
    pub fn write(&self, path: &Path) {
        let mut text = String::new();
        self.contents.iter().for_each(|line: &String| {
            text.push_str(format!("{line}\n").as_str());
        });
        File::create(path.join(format!(
            "checkresult-{}.txt",
            Local::now().format("%y%m%d%H%M%S")
        )))
        .map_or_else(
            |_| {
                eprintln!(
                    "\x1B[91;1merror\x1B[0;1m:\x1B[0m the new check result file cannot be created."
                );
                process::exit(exit_code::EX_CANTCREAT)
            },
            |mut file| {
                if file.write_all(text.as_bytes()).is_err() {
                    eprintln!(
                        "\x1B[91;1merror\x1B[0;1m:\x1B[0m the new check result cannot be written."
                    );
                    process::exit(exit_code::EX_IOERR)
                }
            },
        );
    }
}

mod exit_code {
    pub const EX_CANTCREAT: i32 = 73;
    pub const EX_IOERR: i32 = 74;
}
