use core::ops::RangeInclusive;

use super::{Elf64Addr, Elf64Word, Elf64Xword, Elf64Half};

#[derive(Accessor)]
#[repr(packed)]
pub struct Sym {
    pub name: Elf64Word,
    pub info: u8,
    pub other: u8,
    pub shndx: Elf64Half,
    pub value: Elf64Addr,
    pub size: Elf64Xword,
}

pub use sym_accessor::*;

#[derive(PartialEq, PartialOrd)]
pub struct SymBind {
    value: u8,
}
impl SymBind {
    pub const LOCAL: Self = Self::new(0);
    pub const GLOBAL: Self = Self::new(1);
    pub const WEAK: Self = Self::new(2);
    pub const OS: RangeInclusive<Self> = Self::new(10)..=Self::new(12);
    pub const PROC: RangeInclusive<Self> = Self::new(13)..=Self::new(15);
    const fn new(value: u8) -> Self {
        Self { value }
    }
}
#[derive(PartialEq, PartialOrd)]
pub struct SymType {
    value: u8,
}
impl SymType {
    pub const NOTYPE: Self = Self::new(0);
    pub const OBJECT: Self = Self::new(1);
    pub const FUNC: Self = Self::new(2);
    pub const SECTION: Self = Self::new(3);
    pub const FILE: Self = Self::new(4);
    pub const OS: RangeInclusive<Self> = Self::new(10)..=Self::new(12);
    pub const PROC: RangeInclusive<Self> = Self::new(13)..=Self::new(15);
    const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl fields::Info {
    pub fn bind(&self) -> SymBind {
        SymBind::new(self.raw() >> 4)
    }
    pub fn sym_type(&self) -> SymType {
        SymType::new(self.raw() & 0xf)
    }
}
