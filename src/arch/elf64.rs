pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;

use basic_type::BasicType;
pub use elf::Elf;
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
    #[derive(Default, Debug, Copy, Clone)]
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

pub mod header {
    use super::IDENT;
    use super::{basic_type::BasicType, section};
    use std::io;

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Ident {
        pub magic: [u8; 4],
        pub class: u8,
        pub data: u8,
        pub version: u8,
        pub os_abi: u8,
        pub abi_version: u8,
        pub pad: [u8; IDENT::IDX::NIDENT - IDENT::IDX::PAD],
    }
    impl_convert_from_block_mem_for_plain_struct!(Ident);

    pub type Ehdr = crate::arch::gabi::header::Ehdr<BasicType, Ident>;
    impl_convert_from_block_mem_for_plain_struct!(Ehdr);
    impl Ehdr {
        pub fn shdr_table_range(&self) -> std::ops::Range<usize> {
            std::ops::Range {
                start: self.shoff as usize,
                end: self.shoff as usize + (self.shentsize * self.shnum) as usize,
            }
        }
        pub fn phdr_table_range(&self) -> std::ops::Range<usize> {
            std::ops::Range {
                start: self.phoff as usize,
                end: self.phoff as usize + (self.phentsize * self.phnum) as usize,
            }
        }
    }

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
    use super::{
        header::{Ehdr, Ident},
        IDENT,
    };
    use std::convert::TryInto;

    #[derive(Debug)]
    pub struct Elf<'a> {
        pub ident: &'a super::header::Ident,
        pub ehdr: &'a super::header::Ehdr,
        pub shdr_table: Option<Box<Vec<&'a super::section::Shdr>>>,
        pub phdr_table: Option<Box<Vec<&'a super::program::Phdr>>>,
        // _data: &'a [u8],
    }
    impl<'a> Elf<'a> {}

    impl<'a> std::convert::TryFrom<&'a [u8]> for Elf<'a> {
        type Error = crate::Error;
        fn try_from(src: &'a [u8]) -> Result<Self, Self::Error> {
            let ident: &Ident = src.try_into()?;

            if ident.class != IDENT::CLASS::CLASS64
                && ident.magic
                    != [
                        IDENT::MAGIC::MAG0,
                        IDENT::MAGIC::MAG1,
                        IDENT::MAGIC::MAG2,
                        IDENT::MAGIC::MAG3,
                    ]
            {
                return Err(crate::Error::InvalidClass);
            }
            let ehdr: &Ehdr = src.try_into()?;
            let shdr_table = match ehdr.shoff {
                0 => None,
                _ => {
                    let mut tmp = Box::new(Vec::<&super::section::Shdr>::new());
                    for shdr in src
                        .get(ehdr.shdr_table_range())
                        .ok_or(crate::Error::DataLoss)?
                        .chunks_exact(ehdr.shentsize as usize)
                    {
                        tmp.push(shdr.try_into()?);
                    }
                    Some(tmp)
                }
            };
            let phdr_table = match ehdr.phoff {
                0 => None,
                _ => {
                    let mut tmp = Box::new(Vec::<&super::program::Phdr>::new());
                    for shdr in src
                        .get(ehdr.phdr_table_range())
                        .ok_or(crate::Error::DataLoss)?
                        .chunks_exact(ehdr.phentsize as usize)
                    {
                        tmp.push(shdr.try_into()?);
                    }
                    Some(tmp)
                }
            };
            Ok(Elf {
                ident,
                ehdr,
                shdr_table,
                phdr_table,
                // _data: src,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::header::{Ehdr, Ident};
    use std::convert::{TryFrom, TryInto};

    const MAGIC_0X55: u8 = 0x55;
    const MAGIC_0XAA: u8 = 0xaa;
    #[test]
    fn test_ident_from_array() -> Result<(), crate::Error> {
        let mut test_data = [MAGIC_0X55; std::mem::size_of::<Ident>()];
        let ident_move = Ident::try_from(test_data)?;
        let ident_borrow: &Ident = (&test_data).try_into().unwrap();

        test_data[0] = MAGIC_0XAA;
        assert_eq!(ident_borrow.as_ref(), test_data);
        println!("{:?}", ident_borrow);

        assert_eq!(
            ident_move.as_ref(),
            &[MAGIC_0X55; std::mem::size_of::<Ident>()][..]
        );
        println!("{:?}", ident_move);
        Ok(())
    }

    #[test]
    fn test_ehdr_from_array() -> Result<(), crate::Error> {
        let mut test_data = [MAGIC_0X55; std::mem::size_of::<Ehdr>()];
        let ehdr_move = Ehdr::try_from(test_data)?;
        let ehdr_borrow: &Ehdr = (&test_data).try_into()?;

        test_data[0] = MAGIC_0XAA;
        println!("{:?}", ehdr_borrow.as_ref());
        assert_eq!(ehdr_borrow.as_ref(), &test_data[..]);

        println!("{:?}", ehdr_move.as_ref());
        assert_eq!(
            ehdr_move.as_ref(),
            &[MAGIC_0X55; std::mem::size_of::<Ehdr>()][..]
        );

        println!("{:b}", test_data.as_ptr() as usize);
        use std::any::Any;
        println!("{:?}", (&test_data).type_id());
        println!("{:?}", (&test_data[..]).type_id());
        Ok(())
    }
    #[test]
    fn test_elf_from_slice() -> Result<(), crate::Error> {
        let test_data = std::fs::read("./test/elf64_example").unwrap();
        let elf = super::Elf::try_from(test_data.as_slice());
        println!(
            "{},{},{:#x?}",
            std::mem::size_of_val(&elf),
            test_data.as_slice().len(),
            elf?
        );
        Ok(())
    }
}
