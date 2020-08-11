pub use crate::arch::gabi::*;
pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;
pub mod e_ident {
    pub mod idx {
        pub use crate::arch::gabi::e_ident::idx::*;
        pub const EI_PAD: u8 = 7;
    }
}
#[repr(C)]
pub struct Ident {
    pub mag: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub pad: [u8; (e_ident::idx::EI_NIDENT - e_ident::idx::EI_PAD) as usize],
}
#[repr(C)]
#[derive(Default, Debug)]
pub struct Elf {}
impl crate::BasicType for Elf {
    type Addr = u32;
    type Half = u16;
    type Off = u32;
    type Word = u32;
    type Sword = i32;
    type Xword = Self::Word;
    type Sxword = Self::Sword;
}

pub type Header = crate::arch::gabi::Header<Elf, Ident>;
