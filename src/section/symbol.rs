use crate::{Elf64Addr, Elf64Half, Elf64Word, Elf64Xword};

#[repr(C)]
pub struct Sym {
    pub name: Elf64Word,
    pub info: u8,
    pub other: u8,
    pub shndx: Elf64Half,
    pub value: Elf64Addr,
    pub size: Elf64Xword,
}

#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub struct SymBind {
    value: u8,
}

#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub struct SymType {
    value: u8,
}

impl_borrow!(Sym, SymBind, SymType);
