pub use crate::arch::gabi::*;
pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod sym_table;

#[repr(C)]
#[derive(Default, Debug)]
pub struct ElfBasicType {}
#[repr(C)]
#[derive(Debug, Default)]
pub struct Ident {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub pad: [u8; e_ident::idx::EI_NIDENT - e_ident::idx::EI_PAD],
}
pub type Header = crate::arch::gabi::Header<ElfBasicType, Ident>;

impl crate::BasicType for ElfBasicType {
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
        pub const EI_OSABI: usize = 7;
        pub const EI_ABIVERSION: usize = 8;
        pub const EI_PAD: usize = 9;
    }
    pub mod ei_osabi {
        pub const ELFOSABI_SYSV: u8 = 0;
        pub const ELFOSABI_HPUX: u8 = 1;
        pub const ELFOSABI_STANDLONE: u8 = 255;
    }
    pub use crate::arch::gabi::e_ident::*;
}

/// 可用作 [`Ehdr::r#type`](crate::arch::gabi::Header::type) 的值
pub mod e_type {
    crate::define_e_type_basic_const!(<super::ElfBasicType as crate::BasicType>::Half);
    /// 特定环境使用的下限
    pub const ET_LOOS: <super::ElfBasicType as crate::BasicType>::Half = 0xFE00;
    /// 特定环境使用的上限
    pub const ET_HIOS: <super::ElfBasicType as crate::BasicType>::Half = 0xFEFF;
}
pub type Elf<'a> = crate::arch::gabi::Elf<'a, Header>;
use std::io;
impl crate::Validity for Header {
    fn is_valid(&self) -> io::Result<()> {
        if self.ident.class == e_ident::ei_class::ELFCLASS64 {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                crate::Error::InvalidClass,
            ))
        }
    }
}
#[test]
fn test_file_open() {
    let mut file = std::fs::File::open("./test/elf64_example").unwrap();
    let mut elf = Elf::new(&mut file).unwrap();
    println!("{:?}", elf.read_ehdr());
}
