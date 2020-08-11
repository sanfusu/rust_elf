pub use crate::arch::gabi::*;
pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;
#[repr(C)]
#[derive(Default, Debug)]
pub struct Elf {}
#[repr(C)]
pub struct Ident {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub pad: [u8; (e_ident::idx::EI_NIDENT - e_ident::idx::EI_PAD) as usize],
}
pub type Header = crate::arch::gabi::Header<Elf, Ident>;

impl crate::BasicType for Elf {
    type Addr = u64;
    type Half = u16;
    type Off = u64;
    type Word = u32;
    type Sword = i32;
    type Xword = u64;
    type Sxword = i64;
}
pub mod e_ident {
    pub mod idx {
        pub use crate::arch::gabi::e_ident::idx::*;
        pub const EI_OSABI: u8 = 7;
        pub const EI_ABIVERSION: u8 = 8;
        pub const EI_PAD: u8 = 9;
    }
    pub mod ei_osabi {
        pub const ELFOSABI_SYSV: u8 = 0;
        pub const ELFOSABI_HPUX: u8 = 1;
        pub const ELFOSABI_STANDLONE: u8 = 255;
    }
    pub use crate::arch::gabi::e_ident::*;
}

pub mod e_type {
    pub use crate::arch::gabi::e_type::*;
    /// 特定环境使用的下限
    pub const ET_LOOS: u16 = 0xFE00;
    /// 特定环境使用的上限
    pub const ET_HIOS: u16 = 0xFEFF;
}

#[test]
pub fn test1() {}
