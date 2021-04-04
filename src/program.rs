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
    pub const GNU_EH_FRAME: Self = Self::proc(0x6474_e550);
    /// Same as GNU_EH_FRAME
    pub const SUNW_UNWIND: Self = Self::proc(0x6474_e550);
    pub const OS: RangeInclusive<Self> = Self::new(0x6000_0000)..=Self::new(0x6fff_ffff);
    pub const PROC: RangeInclusive<Self> = Self::new(0x7000_0000)..=Self::new(0x7fff_ffff);

    pub const fn proc(value: Elf64Word) -> Self {
        debug_assert!(value >= Self::PROC.start().raw() && value <= Self::PROC.end().raw());
        Self::new(value)
    }
    pub const fn os(value: Elf64Word) -> Self {
        debug_assert!(value >= Self::OS.start().raw() && value <= Self::OS.end().raw());
        Self::new(value)
    }
}

impl fields::Flags {
    pub const X: Self = Self::new(1);
    pub const W: Self = Self::new(2);
    pub const R: Self = Self::new(4);
    pub(crate) const MASKOS: Self = Self::new(0x00ff_0000);
    pub(crate) const MASKPROC: Self = Self::new(0xff00_0000);

    pub const fn set_os(self, value: u32) -> Self {
        debug_assert!(value & Self::MASKOS.raw() == 0);
        Self::new(self.raw() | (value & Self::MASKOS.raw()))
    }
    pub const fn set_proc(self, value: u32) -> Self {
        debug_assert!(value & Self::MASKPROC.raw() == 0);
        Self::new(self.raw() | (value & Self::MASKPROC.raw()))
    }
    pub const fn executable(self, value: bool) -> Self {
        if value == true {
            Self::new(self.raw() | Self::X.raw())
        } else {
            Self::new(self.raw() & !Self::X.raw())
        }
    }
    pub const fn writeable(self, value: bool) -> Self {
        if value == true {
            Self::new(self.raw() | Self::W.raw())
        } else {
            Self::new(self.raw() & !Self::W.raw())
        }
    }
    pub const fn readable(self, value: bool) -> Self {
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
    pub const fn is_executable(&self) -> bool {
        self.raw() & Self::X.raw() != 0
    }
    pub const fn is_writeable(&self) -> bool {
        self.raw() & Self::W.raw() != 0
    }
    pub const fn is_readable(&self) -> bool {
        self.raw() & Self::R.raw() != 0
    }
    pub const fn get_os(&self) -> Self {
        Self::new(self.raw() & Self::MASKOS.raw())
    }
    pub const fn get_proc(&self) -> Self {
        Self::new(self.raw() & Self::MASKPROC.raw())
    }
}
