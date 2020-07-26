pub mod ehdr;
pub mod shdr;
pub mod spsec;
pub mod symtable;
use std::convert::TryInto;
use std::fmt::Debug;
pub trait ElfBasicType {
    type Xword: Debug
        + Default
        + From<u32>
        + Copy
        + TryInto<usize, Error = std::num::TryFromIntError>;
    type Sxword: Debug + Default + From<i32> + Copy;
    type Addr: Debug + Default + From<u32> + Into<u64> + Copy;
    type Half: Debug + Default + From<u16> + Copy;
    type Off: Debug + Default + From<u32> + Copy;
    type Sword: Debug + Default + From<i32> + Copy;
    type Word: Debug
        + Default
        + From<u32>
        + Copy
        + TryInto<usize, Error = std::num::TryFromIntError>;
}
