use super::Elf;
pub use crate::arch::gabi::program::*;

#[repr(C)]
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
    crate::define_p_type_basic_const!(<super::Elf as crate::BasicType>::Word);
    pub const LOOS: <super::Elf as crate::BasicType>::Word = 0x6000_0000;
    pub const HIOS: <super::Elf as crate::BasicType>::Word = 0x6fff_ffff;
}

pub mod p_flags {
    crate::define_p_flags_basic_const!(<super::Elf as crate::BasicType>::Word);
    pub const MASKOS: <super::Elf as crate::BasicType>::Word = 0x00ff_0000;
}
