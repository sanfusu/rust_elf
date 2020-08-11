use super::Elf;
pub use crate::arch::gabi::program::*;

pub struct Header {
    pub r#type: <Elf as crate::BasicType>::Word,
    pub offset: <Elf as crate::BasicType>::Off,
    pub flags: <Elf as crate::BasicType>::Word,
    pub vaddr: <Elf as crate::BasicType>::Addr,
    pub paddr: <Elf as crate::BasicType>::Addr,
    pub filesz: <Elf as crate::BasicType>::Xword,
    pub memsz: <Elf as crate::BasicType>::Xword,
    pub align: <Elf as crate::BasicType>::Xword,
}
