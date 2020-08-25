use crate::arch::elf32::BasicType;
use crate::IBasicType;

#[allow(non_snake_case)]
pub mod BIND {
    pub use crate::arch::gabi::sym_table::BIND::*;
}
#[allow(non_snake_case)]
pub mod TYPE {
    pub use crate::arch::gabi::sym_table::TYPE::*;
}

#[repr(C)]
pub struct Entry {
    pub name: <BasicType as IBasicType>::Word,
    pub value: <BasicType as IBasicType>::Addr,
    pub size: <BasicType as IBasicType>::Xword,
    pub info: u8,
    pub other: u8,
    pub shndx: <BasicType as IBasicType>::Half,
}
