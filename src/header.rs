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

use super::{Elf64Addr, Elf64Half, Elf64Off, Elf64Word};
#[derive(Accessor)]
#[repr(packed)]
pub struct Header {
    pub ident: ident::Ident,
    _pad: [u8; 16 - ident::Ident::flat_size()],
    pub file_type: Elf64Half,
    pub machine: Elf64Half,
    pub version: Elf64Word,
    pub entry: Elf64Addr,
    pub phoff: Elf64Off,
    pub shoff: Elf64Off,
    pub flags: Elf64Word,
    pub ehsize: Elf64Half,
    pub phentsize: Elf64Half,
    pub phnum: Elf64Half,
    pub shentsize: Elf64Half,
    pub shnum: Elf64Half,
    pub shstrndx: Elf64Half,
}

pub use header_accessor::{HeaderFlat, HeaderFlatMut};

pub mod fields {
    use core::ops::Range;

    pub use super::header_accessor::fields::*;
    struct Pad; // 用于隐藏 fields 中的 _pad

    impl FileType {
        pub const NONE: Self = Self::new(0);
        pub const REL: Self = Self::new(1);
        pub const EXEC: Self = Self::new(2);
        pub const DYN: Self = Self::new(3);
        pub const CORE: Self = Self::new(4);
        pub const OS: Range<Self> = Self::new(0xfe00)..Self::new(0xfeff);
        pub const PROC: Range<Self> = Self::new(0xff00)..Self::new(0xffff);
    }
}
