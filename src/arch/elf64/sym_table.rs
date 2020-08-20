use crate::arch::elf64::ElfBasicType;
use crate::BasicType;
pub mod st_bind {
    pub use crate::arch::gabi::sym_table::st_bind::*;
    pub const LOOS: u8 = 10;
    pub const HIOS: u8 = 12;
}
pub mod st_type {
    pub use crate::arch::gabi::sym_table::st_type::*;
    pub const LOOS: u8 = 10;
    pub const HIOS: u8 = 12;
}
#[repr(C)]
pub struct Entry {
    pub name: <ElfBasicType as BasicType>::Word,
    pub info: u8,
    pub other: u8,
    pub shndx: <ElfBasicType as BasicType>::Half,
    pub value: <ElfBasicType as BasicType>::Addr,
    pub size: <ElfBasicType as BasicType>::Xword,
}
