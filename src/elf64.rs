pub mod ehdr;
pub mod shdr;
pub mod spsec;
pub mod symtable;
use super::gabi41;

pub struct Elf {
    pub elf_header: ehdr::Ehdr,
}

impl gabi41::ElfBasicType for Elf {
    type Addr = u64;
    type Half = u16;
    type Off = u64;
    type Word = u32;
    type Sword = i32;
    type Xword = u64;
    type Sxword = i64;
}
