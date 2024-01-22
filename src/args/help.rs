use core::fmt::{Display, Formatter, Result};

pub struct HelpOption;

impl Display for HelpOption {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "\x1B[1mUsage:\x1B[0m chkdir <folder path>")
    }
}
