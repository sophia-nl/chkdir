use core::fmt::{Display, Formatter, Result};

pub struct VersionOption;

impl Display for VersionOption {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "chkdir 0.1.0")
    }
}
