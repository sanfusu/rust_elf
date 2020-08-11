use crate::BasicType;
use crate::arch::elf64::Elf;
pub use crate::arch::gabi::sym_table::*;
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
#[repr(packed)]
pub struct Entry {
    pub name: <Elf as BasicType>::Word,
    pub info: u8,
    pub other: u8,
    pub shndx: <Elf as BasicType>::Half,
    pub value: <Elf as BasicType>::Addr,
    pub size: <Elf as BasicType>::Xword,
}
