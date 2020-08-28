pub mod arch;
pub mod utils;

use std::{
    cmp,
    convert::TryInto,
    fmt::{self, Debug},
    ops,
};
use utils::AsBytes;

#[derive(Debug)]
pub enum Error {
    DataLoss,
    InvalidShentSize,
    InvalidMagic,
    InvalidClass,
    InvalidEhdr,
    MissAligned,
    UnExpectedIoError(std::io::Error),
}

// impl std::convert::Into<io::Error> for Error {
//     fn into(self) -> io::Error {
//         let x = match self {
//             Error::DataLoss => (io::ErrorKind::InvalidInput, Error::DataLoss),
//             Error::InvalidMagic => (io::ErrorKind::InvalidInput, Error::DataLoss),
//             Error::InvalidClass => (io::ErrorKind::InvalidInput, Error::DataLoss),
//             Error::InvalidEhdr => (io::ErrorKind::InvalidData, Error::InvalidEhdr),
//             Error::InvalidShentSize => (io::ErrorKind::InvalidData, Error::InvalidShentSize),
//             Error::MissAligned => (io::ErrorKind::InvalidData, Error::MissAligned),
//             Error::UnExpectedIoError => (io::ErrorKind::InvalidData, Error::MissAligned),
//         };
//         io::Error::new(x.0, x.1)
//     }
// }
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

pub trait IBasicType {
    type Xword: Debug
        + Default
        + From<u32>
        + Copy
        + TryInto<usize, Error = std::num::TryFromIntError>
        + ops::BitOrAssign
        + ops::BitOr<Output = Self::Xword>
        + ops::BitAndAssign
        + ops::BitAnd<Output = Self::Xword>
        + cmp::PartialEq
        + fmt::LowerHex;
    type Sxword: Debug + fmt::LowerHex + Default + From<i32> + Copy;
    type Addr: Debug + Default + fmt::LowerHex + From<u32> + Into<u64> + Copy;
    type Half: Debug
        + Default
        + From<u16>
        + fmt::LowerHex
        + Copy
        + ops::Mul<Output = Self::Half>
        + Into<u64>;
    type Off: Debug + Default + From<u32> + Copy + fmt::LowerHex + Into<u64>;
    type Sword: Debug + Default + From<i32> + fmt::LowerHex + Copy;
    type Word: Debug
        + Default
        + Into<u32>
        + From<u32>
        + fmt::LowerHex
        + Copy
        + TryInto<usize, Error = std::num::TryFromIntError>;
}

#[cfg(test)]
mod test {}
