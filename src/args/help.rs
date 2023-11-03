pub struct HelpOption;

impl HelpOption {
    pub fn print(&self) {
        println!("\x1B[1mUsage:\x1B[0m chkdir <folder path>")
    }
}
