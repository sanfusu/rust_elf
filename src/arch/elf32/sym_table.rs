use crate::arch::elf32::BasicType;
pub use crate::arch::gabi::sym_table::*;
use crate::IBasicType;

#[repr(C)]
pub struct Entry {
    pub name: <BasicType as IBasicType>::Word,
    pub value: <BasicType as IBasicType>::Addr,
    pub size: <BasicType as IBasicType>::Xword,
    pub info: u8,
    pub other: u8,
    pub shndx: <BasicType as IBasicType>::Half,
}
