use super::Elf;
pub use crate::arch::gabi::program::*;

#[repr(packed)]
pub struct Header {
    pub r#type: <Elf as crate::BasicType>::Word,
    pub flags: <Elf as crate::BasicType>::Word,
    pub offset: <Elf as crate::BasicType>::Off,
    pub vaddr: <Elf as crate::BasicType>::Addr,
    pub paddr: <Elf as crate::BasicType>::Addr,
    pub filesz: <Elf as crate::BasicType>::Xword,
    pub memsz: <Elf as crate::BasicType>::Xword,
    pub align: <Elf as crate::BasicType>::Xword,
}

pub mod p_type {
    pub use crate::arch::gabi::program::p_type::*;
    pub const LOOS: u32 = 0x6000_0000;
    pub const HIOS: u32 = 0x6fff_ffff;
}

pub mod p_flags {
    pub use crate::arch::gabi::program::p_flags::*;
    pub const MASKOS: u32 = 0x00ff_0000;
}
