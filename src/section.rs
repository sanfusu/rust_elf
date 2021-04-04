use core::{marker::PhantomData, ops::RangeInclusive};
pub mod relocation;
pub mod str_tab;
pub mod symbol;

pub mod header {
    use core::ops::RangeInclusive;

    use super::super::{Elf64Addr, Elf64Off, Elf64Word, Elf64Xword};

    #[derive(Accessor)]
    #[repr(packed)]
    pub struct Header {
        pub name: Elf64Word,
        pub sec_type: Elf64Word,
        pub flags: Elf64Xword,
        pub addr: Elf64Addr,
        pub offset: Elf64Off,
        pub size: Elf64Xword,
        pub link: Elf64Word,
        pub info: Elf64Word,
        pub addralign: Elf64Xword,
        pub entsize: Elf64Xword,
    }

    pub use header_accessor::*;
    impl fields::SecType {
        pub const NULL: Self = Self::new(0);
        pub const PROGBITS: Self = Self::new(1);
        pub const SYMTAB: Self = Self::new(2);
        pub const STRTAB: Self = Self::new(3);
        pub const RELA: Self = Self::new(4);
        pub const HASH: Self = Self::new(5);
        pub const DYNAMIC: Self = Self::new(6);
        pub const NOTE: Self = Self::new(7);
        pub const NOBITS: Self = Self::new(8);
        pub const REL: Self = Self::new(9);
        pub const SHLIB: Self = Self::new(10);
        pub const DYNSYM: Self = Self::new(11);
        pub const OS: RangeInclusive<Self> = Self::new(0x6000_0000)..=Self::new(0x6fff_ffff);
        pub const PROC: RangeInclusive<Self> = Self::new(0x7000_0000)..=Self::new(0x7fff_ffff);
        pub const X86_64_UNWIND: Self = Self::proc(0x7000_0001);
        pub const fn os(value: Elf64Word) -> Self {
            debug_assert!(0x6000_0000 <= value && value <= 0x6fff_ffff);
            Self::new(value)
        }
        pub const fn proc(value: Elf64Word) -> Self {
            debug_assert!(0x7000_0000 <= value && value <= 0x7fff_ffff);
            Self::new(value)
        }
    }
    impl fields::Flags {
        pub const WRITE: Self = Self::new(1);
        pub const ALLOC: Self = Self::new(2);
        pub const EXECINSTR: Self = Self::new(4);
        pub const AMD64_LARGE: Self = Self::empty().set_proc(0x1000_0000);

        const MASKOS: Self = Self::new(0x0f00_0000);
        const MASKPROC: Self = Self::new(0xf000_0000);
        pub const fn empty() -> Self {
            Self::new(0)
        }
        pub const fn set_proc(self, value: Elf64Xword) -> Self {
            debug_assert!(value & Self::MASKPROC.raw() == 0);
            Self::new(value | self.raw())
        }
        pub const fn set_os(self, value: Elf64Xword) -> Self {
            debug_assert!(value & Self::MASKOS.raw() == 0);
            Self::new(value | self.raw())
        }
        pub const fn execinstr(self, value: bool) -> Self {
            if value == true {
                Self::new(self.raw() | Self::EXECINSTR.raw())
            } else {
                Self::new(self.raw() & !Self::EXECINSTR.raw())
            }
        }
        pub const fn writeable(self, value: bool) -> Self {
            if value == true {
                Self::new(self.raw() | Self::WRITE.raw())
            } else {
                Self::new(self.raw() & !Self::WRITE.raw())
            }
        }
        pub const fn allocable(self, value: bool) -> Self {
            if value == true {
                Self::new(self.raw() | Self::ALLOC.raw())
            } else {
                Self::new(self.raw() & !Self::ALLOC.raw())
            }
        }
        pub const fn is_execinstr(&self) -> bool {
            self.raw() & Self::EXECINSTR.raw() != 0
        }
        pub const fn is_writeable(&self) -> bool {
            self.raw() & Self::WRITE.raw() != 0
        }
        pub const fn is_allocable(&self) -> bool {
            self.raw() & Self::ALLOC.raw() != 0
        }
        pub const fn get_os(&self) -> Self {
            Self::new(self.raw() & Self::MASKOS.raw())
        }
        pub const fn get_proc(&self) -> Self {
            Self::new(self.raw() & Self::MASKPROC.raw())
        }
    }
}

pub struct Section<'a, End: flassor::Endianess<'a>> {
    header: header::HeaderFlat<'a, End>,
}

pub struct SectionMut<'a, End: flassor::Endianess<'a>> {
    data: &'a mut [u8],
    /// 整个文件的数据都应该在里面
    phantom_data: PhantomData<End>,
}

pub const SHN_UNDEF: u16 = 0;
pub const SHN_PROC: RangeInclusive<u16> = 0xff00..=0xff1f;
pub const SHN_OS: RangeInclusive<u16> = 0xff20..=0xff3f;
pub const SHN_ABS: u16 = 0xfff1;
pub const SHN_COMMON: u16 = 0xfff2;
