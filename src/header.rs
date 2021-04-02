use flassor::ByteOrder;
pub mod ident {
    use flassor::ByteOrder;

    #[derive(Accessor, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Ident {
        pub mag: [u8; 4],
        pub file_class: u8,
        pub data_encode: u8,
        pub file_version: u8,
        pub osabi: u8,
        pub abi_version: u8,
    }
    pub use ident_accessor::*;
    impl fields::FileClass {
        pub const CLSSS32: Self = Self::new(1);
        pub const CLASS64: Self = Self::new(2);
    }
    impl fields::DataEncode {
        pub const LSB: Self = Self::new(1);
        pub const MSB: Self = Self::new(2);
        pub fn is_lsb(&self) -> bool {
            *self == Self::LSB
        }
        pub fn is_msb(&self) -> bool {
            *self == Self::MSB
        }
    }
    impl fields::Osabi {
        pub const SYSV: Self = Self::new(0);
        pub const HPUX: Self = Self::new(1);
        pub const STANDLONE: Self = Self::new(255);
    }
}

use super::{Elf64Addr, Half, Elf64Off, Elf64Word};
#[derive(Accessor)]
#[repr(packed)]
pub struct Ehdr {
    pub ident: ident::Ident,
    _pad: [u8; 16 - core::mem::size_of::<ident::Ident>()],
    pub _type: Half,
    pub machine: Half,
    pub version: Elf64Word,
    pub entry: Elf64Addr,
    pub phoff: Elf64Off,
    pub shoff: Elf64Off,
    pub flags: Elf64Word,
    pub ehsize: Half,
    pub phentsize: Half,
    pub phnum: Half,
    pub shentsize: Half,
    pub shnum: Half,
    pub shstrndx: Half,
}

pub use ehdr_accessor::{EhdrFlat, EhdrFlatMut};

pub mod fields {
    use core::ops::Range;

    pub use super::ehdr_accessor::fields::*;
    struct Pad; // 用于隐藏 fields 中的 _pad

    impl Type {
        pub const NONE: Self = Self::new(0);
        pub const REL: Self = Self::new(1);
        pub const EXEC: Self = Self::new(2);
        pub const DYN: Self = Self::new(3);
        pub const CORE: Self = Self::new(4);
        pub const OS: Range<Self> = Self::new(0xfe00)..Self::new(0xfeff);
        pub const PROC: Range<Self> = Self::new(0xff00)..Self::new(0xffff);
    }
}
