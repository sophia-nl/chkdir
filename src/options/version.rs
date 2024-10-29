use core::fmt::{Display, Formatter, Result};

pub struct VersionString;

impl Display for VersionString {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "chkdir 0.1.0")
    }
}
