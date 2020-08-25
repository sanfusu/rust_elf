use crate::arch::elf64::BasicType;
use crate::IBasicType;
#[allow(non_snake_case)]
pub mod BIND {
    pub use crate::arch::gabi::sym_table::BIND::*;
    pub const LOOS: u8 = 10;
    pub const HIOS: u8 = 12;
}

#[allow(non_snake_case)]
pub mod TYPE {
    pub use crate::arch::gabi::sym_table::TYPE::*;
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
