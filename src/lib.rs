pub mod arch;

use std::{cmp, convert::TryInto, fmt::Debug, ops};
pub trait BasicType {
    type Xword: Debug
        + Default
        + From<u32>
        + Copy
        + TryInto<usize, Error = std::num::TryFromIntError>
        + ops::BitOrAssign
        + ops::BitOr<Output = Self::Xword>
        + ops::BitAndAssign
        + ops::BitAnd<Output = Self::Xword>
        + cmp::PartialEq;
    type Sxword: Debug + Default + From<i32> + Copy;
    type Addr: Debug + Default + From<u32> + Into<u64> + Copy;
    type Half: Debug + Default + From<u16> + Copy;
    type Off: Debug + Default + From<u32> + Copy;
    type Sword: Debug + Default + From<i32> + Copy;
    type Word: Debug
        + Default
        + Into<u32>
        + From<u32>
        + Copy
        + TryInto<usize, Error = std::num::TryFromIntError>;
}

pub fn hash(name: &str) -> u32 {
    let mut h: u32 = 0;
    let mut g: u32;
    for mut byte in name.bytes() {
        byte += 1;
        h = (h << 4) + (byte as u32);
        g = h & 0xf000_0000;
        if g != 0 {
            h ^= g >> 24;
        }
        h &= 0xfff_ffff;
    }
    h
}
