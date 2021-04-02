use core::ops::RangeInclusive;

use crate::{Elf64Addr, Elf64Off, Elf64Word, Elf64Xword};

#[derive(Accessor)]
#[repr(packed)]
pub struct Header {
    pub ptype: Elf64Word,
    pub flags: Elf64Word,
    pub offset: Elf64Off,
    pub vaddr: Elf64Addr,
    /// 物理寻址，这个字段是为那些可以物理寻址的系统保留的。
    pub paddr: Elf64Addr,
    pub filesz: Elf64Xword,
    pub memsz: Elf64Xword,
    pub align: Elf64Xword,
}

pub use header_accessor::*;

impl fields::Ptype {
    pub const NULL: Self = Self::new(0);
    pub const LOAD: Self = Self::new(1);
    pub const DYNAMIC: Self = Self::new(2);
    pub const INTERP: Self = Self::new(3);
    pub const NOTE: Self = Self::new(4);
    pub const SHLIB: Self = Self::new(5);
    pub const PHDR: Self = Self::new(6);
    pub const OS: RangeInclusive<Self> = Self::new(0x6000_0000)..=Self::new(0x6fff_ffff);
    pub const PROC: RangeInclusive<Self> = Self::new(0x7000_0000)..=Self::new(0x7fff_ffff);
}

impl fields::Flags {
    pub const X: Self = Self::new(1);
    pub const W: Self = Self::new(2);
    pub const R: Self = Self::new(4);
    pub(crate) const MASKOS: Self = Self::new(0x00ff_0000);
    pub(crate) const MASKPROC: Self = Self::new(0xff00_0000);

    pub const fn set_os(self, value: u32) -> Self {
        Self::new(self.raw() | (value & Self::MASKOS.raw()))
    }
    pub const fn set_proc(self, value: u32) -> Self {
        Self::new(self.raw() | (value & Self::MASKPROC.raw()))
    }
    pub const fn set_x(self, value: bool) -> Self {
        if value == true {
            Self::new(self.raw() | Self::X.raw())
        } else {
            Self::new(self.raw() & !Self::X.raw())
        }
    }
    pub const fn set_w(self, value: bool) -> Self {
        if value == true {
            Self::new(self.raw() | Self::W.raw())
        } else {
            Self::new(self.raw() & !Self::W.raw())
        }
    }
    pub const fn set_r(self, value: bool) -> Self {
        if value == true {
            Self::new(self.raw() | Self::R.raw())
        } else {
            Self::new(self.raw() & !Self::R.raw())
        }
    }
    /// 创建一个空 flag，所有 bit 均为 0
    pub const fn empty() -> Self {
        Self::new(0)
    }
}
