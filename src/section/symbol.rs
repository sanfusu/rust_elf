use core::{ops::RangeInclusive, panic};

use crate::{Elf64Addr, Elf64Half, Elf64Word, Elf64Xword};

#[accessor]
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
    /// 创建一个 os 指定的 SymBind。
    ///
    /// 该函数只会被显式调用，如果传入的值不在 [`Self::OS`] 范围内，则认定为是 Bug，会导致 Panic.
    pub fn os(value: u8) -> Self {
        if Self::OS.contains(&Self::new(value)) {
            Self::new(value)
        } else {
            panic!("The symbind value is not in os specified range")
        }
    }
    /// 类似于 [`Self::os`]，用于创建处理器特定的 SymBind
    pub fn proc(value: u8) -> Self {
        if Self::PROC.contains(&Self::new(value)) {
            Self::new(value)
        } else {
            panic!("The symbind value is not in processor specified range")
        }
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
    /// 创建一个 os 指定的 SymType。
    ///
    /// 该函数只会被显式调用，如果传入的值不在 [`Self::OS`] 范围内，则认定为是 Bug，会导致 Panic.
    pub fn os(value: u8) -> Self {
        if Self::OS.contains(&Self::new(value)) {
            Self::new(value)
        } else {
            panic!("The SymType value is not in os specified range")
        }
    }
    /// 类似于 [`Self::os`]，用于创建处理器特定的 SymType
    pub fn proc(value: u8) -> Self {
        if Self::PROC.contains(&Self::new(value)) {
            Self::new(value)
        } else {
            panic!("The SymType value is not in processor specified range")
        }
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
