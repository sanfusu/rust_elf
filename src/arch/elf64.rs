pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;

use basic_type::BasicType;
pub use elf::Elf;
pub use ident::Ident;

#[allow(non_snake_case)]
pub mod IDENT {
    pub mod IDX {
        pub use crate::arch::gabi::IDENT::IDX::*;
        pub const OSABI: usize = 7;
        pub const ABIVERSION: usize = 8;
        pub const PAD: usize = 9;
    }
    pub mod OSABI {
        pub const SYSV: u8 = 0;
        pub const HPUX: u8 = 1;
        pub const STANDLONE: u8 = 255;
    }
    pub use crate::arch::gabi::IDENT::*;
}

/// 可用作 [`Ehdr::r#type`](crate::arch::gabi::header::Ehdr::type) 的值
#[allow(non_snake_case)]
pub mod TYPE {
    use super::basic_type;

    define_e_type_basic_const!(basic_type::Half);
    /// 特定环境使用的下限
    pub const LOOS: basic_type::Half = 0xFE00;
    /// 特定环境使用的上限
    pub const HIOS: basic_type::Half = 0xFEFF;
}

#[allow(non_snake_case)]
pub mod MACHINE {
    use super::basic_type;

    pub const EM_X86_64: basic_type::Half = 62;
    define_e_machine_basic_constant!(basic_type::Half);
}

pub mod basic_type {
    #[repr(C)]
    #[derive(Default, Debug)]
    pub struct BasicType {}

    impl crate::IBasicType for BasicType {
        type Addr = u64;
        type Half = u16;
        type Off = u64;
        type Word = u32;
        type Sword = i32;
        type Xword = u64;
        type Sxword = i64;
    }
    pub type Addr = <BasicType as crate::IBasicType>::Addr;
    pub type Off = <BasicType as crate::IBasicType>::Off;
    pub type Half = <BasicType as crate::IBasicType>::Half;
    pub type Word = <BasicType as crate::IBasicType>::Word;
    pub type Sword = <BasicType as crate::IBasicType>::Sword;
    pub type Xword = <BasicType as crate::IBasicType>::Xword;
    pub type Sxword = <BasicType as crate::IBasicType>::Sxword;
}

pub(crate) mod ident {
    use super::IDENT;

    #[repr(C)]
    #[derive(Debug, Default)]
    pub struct Ident {
        pub magic: [u8; 4],
        pub class: u8,
        pub data: u8,
        pub version: u8,
        pub os_abi: u8,
        pub abi_version: u8,
        pub pad: [u8; IDENT::IDX::NIDENT - IDENT::IDX::PAD],
    }
}

pub mod header {
    use super::{basic_type::BasicType, section, Ident, IDENT};
    use std::io;

    pub type Ehdr = crate::arch::gabi::header::Ehdr<BasicType, Ident>;

    impl crate::Validity for Ehdr {
        fn is_valid(&self) -> io::Result<()> {
            if usize::from(self.shentsize) != std::mem::size_of::<section::header::Shdr>() {
                return Err(crate::Error::InvalidShentSize.into());
            }
            if self.ident.class == IDENT::CLASS::CLASS64 {
                Ok(())
            } else {
                Err(crate::Error::InvalidClass.into())
            }
        }
    }
}

pub(crate) mod elf {
    use super::{basic_type::BasicType, header::Ehdr, section};

    pub type Elf<'a> = crate::arch::gabi::Elf<'a, BasicType, Ehdr, section::header::Shdr>;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    #[test]
    fn test_file_open() -> io::Result<()> {
        let mut file = std::fs::File::open("./test/elf64_example")?;
        let mut elf = Elf::new(&mut file)?;
        println!("{:#x?}", elf.read_ehdr()?);
        let shdr_table = elf.read_shdr_table()?;
        for shdr in shdr_table.iter() {
            println!("{:#x?}", shdr);
        }
        Ok(())
    }
}
