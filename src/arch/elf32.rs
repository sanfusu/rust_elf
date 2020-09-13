pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;

pub use self::elf::Elf;
use basic_type::*;

#[allow(non_snake_case)]
pub mod IDENT {
    pub use crate::arch::gabi::IDENT::*;
    pub mod IDX {
        pub use crate::arch::gabi::IDENT::IDX::*;
        pub const PAD: usize = 7;
    }
}
#[allow(non_snake_case)]
pub mod TYPE {
    define_e_type_basic_const!(super::Half);
}

#[allow(non_snake_case)]
pub mod MACHINE {
    pub const RESERVED_LO: super::Half = 11;
    pub const RESERVED_HI: super::Half = 16;
    define_e_machine_basic_constant!(super::Half);
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct Ident {
    pub mag: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub pad: [u8; IDENT::IDX::NIDENT - IDENT::IDX::PAD],
}

mod elf {
    pub struct Elf {}
}

pub mod basic_type {
    #[repr(C)]
    #[derive(Default, Debug)]
    pub struct BasicType {}

    impl crate::IBasicType for BasicType {
        type Addr = u32;
        type Half = u16;
        type Off = u32;
        type Word = u32;
        type Sword = i32;
        type Xword = Self::Word;
        type Sxword = Self::Sword;
    }
    pub type Addr = <BasicType as crate::IBasicType>::Addr;
    pub type Off = <BasicType as crate::IBasicType>::Off;
    pub type Half = <BasicType as crate::IBasicType>::Half;
    pub type Word = <BasicType as crate::IBasicType>::Word;
    pub type Sword = <BasicType as crate::IBasicType>::Sword;
    pub type Xword = <BasicType as crate::IBasicType>::Xword;
    pub type Sxword = <BasicType as crate::IBasicType>::Sxword;
}

pub mod header {
    use super::Ident;
    use crate::arch::elf32::basic_type::BasicType;

    pub type Ehdr = crate::arch::gabi::header::Ehdr<BasicType, Ident>;
}

#[cfg(test)]
mod test {}
