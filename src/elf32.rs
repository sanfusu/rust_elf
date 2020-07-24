pub mod ehdr;
pub mod shdr;
pub mod spsec;
pub mod symtable;
use super::gabi41;

pub struct Elf {}
impl gabi41::ElfBasicType for Elf {
    type Addr = u32;
    type Half = u16;
    type Off = u32;
    type Word = u32;
    type Sword = i32;
    type Xword = Self::Word;
    type Sxword = Self::Sword;
}
