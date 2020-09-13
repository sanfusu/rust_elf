pub mod dynamic;
pub mod program;
pub mod relocation;
pub mod section;
pub mod str_table;
pub mod sym_table;

use basic_type::BasicType;

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
    use super::basic_type::BasicType;
    use super::IDENT;

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
        pub fn shdr_table_range(&self) -> core::ops::Range<usize> {
            core::ops::Range {
                start: self.shoff as usize,
                end: self.shoff as usize + (self.shentsize * self.shnum) as usize,
            }
        }
        pub fn phdr_table_range(&self) -> core::ops::Range<usize> {
            core::ops::Range {
                start: self.phoff as usize,
                end: self.phoff as usize + (self.phentsize * self.phnum) as usize,
            }
        }
    }
}
