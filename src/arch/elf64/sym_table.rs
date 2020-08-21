use crate::arch::elf64::BasicType;
use crate::IBasicType;
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
    pub name: <BasicType as IBasicType>::Word,
    pub info: u8,
    pub other: u8,
    pub shndx: <BasicType as IBasicType>::Half,
    pub value: <BasicType as IBasicType>::Addr,
    pub size: <BasicType as IBasicType>::Xword,
}
