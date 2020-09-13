#![no_std]

pub mod arch;
pub mod utils;

use core::{
    cmp,
    convert::TryInto,
    fmt::{self, Debug},
    ops,
};

#[derive(Debug)]
pub enum Error {
    DataLoss,
    InvalidShentSize,
    InvalidMagic,
    InvalidClass,
    InvalidEhdr,
    MissAligned,
    UnExpectedIoError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub trait IBasicType {
    type Xword: Debug
        + Default
        + From<u32>
        + Copy
        + TryInto<usize, Error = core::num::TryFromIntError>
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
        + TryInto<usize, Error = core::num::TryFromIntError>;
}

#[cfg(test)]
mod test {}
