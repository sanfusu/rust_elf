use super::ElfBasicType;
pub use crate::arch::gabi::program::*;

use derive::AsSlice;
#[derive(AsSlice)]
#[repr(C)]
pub struct Header {
    pub r#type: <ElfBasicType as crate::BasicType>::Word,
    pub flags: <ElfBasicType as crate::BasicType>::Word,
    pub offset: <ElfBasicType as crate::BasicType>::Off,
    pub vaddr: <ElfBasicType as crate::BasicType>::Addr,
    pub paddr: <ElfBasicType as crate::BasicType>::Addr,
    pub filesz: <ElfBasicType as crate::BasicType>::Xword,
    pub memsz: <ElfBasicType as crate::BasicType>::Xword,
    pub align: <ElfBasicType as crate::BasicType>::Xword,
}

pub mod p_type {
    crate::define_p_type_basic_const!(<super::ElfBasicType as crate::BasicType>::Word);
    pub const LOOS: <super::ElfBasicType as crate::BasicType>::Word = 0x6000_0000;
    pub const HIOS: <super::ElfBasicType as crate::BasicType>::Word = 0x6fff_ffff;
}

pub mod p_flags {
    crate::define_p_flags_basic_const!(<super::ElfBasicType as crate::BasicType>::Word);
    pub const MASKOS: <super::ElfBasicType as crate::BasicType>::Word = 0x00ff_0000;
}
