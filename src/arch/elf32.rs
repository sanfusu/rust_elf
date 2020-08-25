pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;

use basic_type::*;
pub use elf::Elf;

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
    use super::{basic_type::BasicType, header::Ehdr, section};
    
    pub type Elf<'a> = crate::arch::gabi::Elf<'a, BasicType, Ehdr, section::header::Shdr>;
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
    use super::{section, Ident, IDENT};
    use crate::arch::elf32::basic_type::BasicType;
    use std::io;

    pub type Ehdr = crate::arch::gabi::header::Ehdr<BasicType, Ident>;

    impl crate::Validity for Ehdr {
        fn is_valid(&self) -> io::Result<()> {
            if usize::from(self.shentsize) != std::mem::size_of::<section::header::Shdr>() {
                return Err(crate::Error::InvalidShentSize.into());
            }
            if self.ident.class == IDENT::CLASS::CLASS32 {
                Ok(())
            } else {
                Err(crate::Error::InvalidClass.into())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Elf;
    use std::io;

    #[test]
    fn test_file_open() -> io::Result<()> {
        let mut file = std::fs::File::open("./test/elf64_example")?;
        let _elf = Elf::new(&mut file).expect_err("elf64_example 应当是 elf64 文件");
        Ok(())
    }
}
