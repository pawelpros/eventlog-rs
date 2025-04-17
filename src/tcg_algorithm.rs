use std::fmt;

use num_enum::TryFromPrimitive;
use crate::utils;

#[repr(u32)]
#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq, TryFromPrimitive)]
pub enum TcgAlgorithm {
    TpmAlgRsa = 0x1,
    TpmAlgTdes = 0x3,
    TpmAlgSha1 = 0x4,
    TpmAlgSha256 = 0xB,
    TpmAlgSha384 = 0xC,
    TpmAlgSha512 = 0xD,
}

impl TcgAlgorithm {
    fn format_name(&self) -> String {
        let name = format!("{:?}", self);

        utils::format_name(name)
    }
}

impl fmt::Display for TcgAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_name())
    }
}
