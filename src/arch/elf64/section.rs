pub use crate::arch::gabi::section::*;
use crate::arch::{elf64::Elf, gabi};

pub mod sh_type {
    crate::define_sh_type_basic_const!(<super::Elf as crate::BasicType>::Word);
    /// elf-64 特有（特定环境使用）
    pub const LOOS: <super::Elf as crate::BasicType>::Word = 0x6000_0000;
    /// elf-64 特有（特定环境使用）
    pub const HIOS: <super::Elf as crate::BasicType>::Word = 0x6fff_ffff;
}

pub mod sh_flags {
    crate::define_sh_flags_basic_const!(<super::Elf as crate::BasicType>::Xword);
}
pub type Header = gabi::section::Header<Elf>;
