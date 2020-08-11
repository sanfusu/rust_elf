pub use crate::arch::gabi::section::*;
use crate::arch::{elf64::Elf, gabi};

pub mod sh_type {
    pub use crate::arch::gabi::section::sh_type::*;
    /// elf-64 特有（特定环境使用）
    pub const LOOS: u32 = 0x6000_0000;
    /// elf-64 特有（特定环境使用）
    pub const HIOS: u32 = 0x6fff_ffff;
}

pub type Header = gabi::section::Header<Elf>;
