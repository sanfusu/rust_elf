pub mod ehdr;
pub mod shdr;
pub mod spsec;
pub mod symtable;
use std::convert::TryInto;
pub trait ElfBasicType {
    type Xword: Default + From<u32> + Copy + TryInto<usize, Error = std::num::TryFromIntError>;
    type Sxword: Default + From<i32> + Copy;
    type Addr: Default + From<u32> + Into<u64> + Copy;
    type Half: Default + From<u16> + Copy;
    type Off: Default + From<u32> + Copy;
    type Sword: Default + From<i32> + Copy;
    type Word: Default + From<u32> + Copy + TryInto<usize, Error = std::num::TryFromIntError>;
}
