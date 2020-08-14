use crate::arch::elf32::ElfBasicType;
pub use crate::arch::gabi::sym_table::*;
use crate::BasicType;

#[repr(C)]
pub struct Entry {
    pub name: <ElfBasicType as BasicType>::Word,
    pub value: <ElfBasicType as BasicType>::Addr,
    pub size: <ElfBasicType as BasicType>::Xword,
    pub info: u8,
    pub other: u8,
    pub shndx: <ElfBasicType as BasicType>::Half,
}
