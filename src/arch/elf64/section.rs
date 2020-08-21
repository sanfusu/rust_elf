use crate::arch::elf64::BasicType;

pub use crate::arch::gabi::section::sh_idx;
pub mod sh_type {
    crate::define_sh_type_basic_const!(<super::BasicType as crate::IBasicType>::Word);
    /// elf-64 特有（特定环境使用）
    pub const LOOS: <super::BasicType as crate::IBasicType>::Word = 0x6000_0000;
    /// elf-64 特有（特定环境使用）
    pub const HIOS: <super::BasicType as crate::IBasicType>::Word = 0x6fff_ffff;
}

pub mod sh_flags {
    crate::define_sh_flags_basic_const!(<super::BasicType as crate::IBasicType>::Xword);
}

pub mod header {
    use super::BasicType;
    use crate::arch::gabi;
    pub type Shdr = gabi::section::header::Shdr<BasicType>;
}
