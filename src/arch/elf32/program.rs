use super::ElfBasicType;
pub use crate::arch::gabi::program::*;
use derive::AsSlice;
#[repr(C)]
#[derive(AsSlice)]
pub struct Header {
    pub r#type: <ElfBasicType as crate::BasicType>::Word,
    pub offset: <ElfBasicType as crate::BasicType>::Off,
    pub flags: <ElfBasicType as crate::BasicType>::Word,
    pub vaddr: <ElfBasicType as crate::BasicType>::Addr,
    pub paddr: <ElfBasicType as crate::BasicType>::Addr,
    pub filesz: <ElfBasicType as crate::BasicType>::Xword,
    pub memsz: <ElfBasicType as crate::BasicType>::Xword,
    pub align: <ElfBasicType as crate::BasicType>::Xword,
}

pub mod p_type {
    crate::define_p_flags_basic_const!(<super::ElfBasicType as crate::BasicType>::Word);
}
pub mod p_flags {
    crate::define_p_flags_basic_const!(<super::ElfBasicType as crate::BasicType>::Word);
}
