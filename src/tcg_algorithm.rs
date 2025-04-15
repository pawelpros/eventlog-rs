use std::fmt;

use num_enum::TryFromPrimitive;

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
    fn variant_name(&self) -> String {
        let name = format!("{:?}", self);

        let mut result = String::new();
        for (i, ch) in name.chars().enumerate() {
            if ch.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(ch.to_ascii_uppercase());
        }

        result
    }
}

impl fmt::Display for TcgAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.variant_name())
    }
}
