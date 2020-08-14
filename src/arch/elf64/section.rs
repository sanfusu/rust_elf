pub use crate::arch::gabi::section::*;
use crate::arch::{elf64::ElfBasicType, gabi};

pub mod sh_type {
    crate::define_sh_type_basic_const!(<super::ElfBasicType as crate::BasicType>::Word);
    /// elf-64 特有（特定环境使用）
    pub const LOOS: <super::ElfBasicType as crate::BasicType>::Word = 0x6000_0000;
    /// elf-64 特有（特定环境使用）
    pub const HIOS: <super::ElfBasicType as crate::BasicType>::Word = 0x6fff_ffff;
}

pub mod sh_flags {
    crate::define_sh_flags_basic_const!(<super::ElfBasicType as crate::BasicType>::Xword);
}
pub type Header = gabi::section::Header<ElfBasicType>;
