use crate::arch::elf32::Elf;
pub use crate::arch::gabi::sym_table::*;
use crate::BasicType;

#[repr(C)]
pub struct Entry {
    pub name: <Elf as BasicType>::Word,
    pub value: <Elf as BasicType>::Addr,
    pub size: <Elf as BasicType>::Xword,
    pub info: u8,
    pub other: u8,
    pub shndx: <Elf as BasicType>::Half,
}
