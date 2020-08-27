use crate::arch::elf64::BasicType;
pub use header::Shdr;
#[allow(non_snake_case)]
pub mod IDX {
    pub use crate::arch::gabi::section::IDX::*;
}

#[allow(non_snake_case)]
pub mod TYPE {
    define_sh_type_basic_const!(<super::BasicType as crate::IBasicType>::Word);
    /// elf-64 特有（特定环境使用）
    pub const LOOS: <super::BasicType as crate::IBasicType>::Word = 0x6000_0000;
    /// elf-64 特有（特定环境使用）
    pub const HIOS: <super::BasicType as crate::IBasicType>::Word = 0x6fff_ffff;
}

#[allow(non_snake_case)]
pub mod FLAGS {
    define_sh_flags_basic_const!(<super::BasicType as crate::IBasicType>::Xword);
}

pub(crate) mod header {
    use super::BasicType;
    use crate::arch::gabi;
    pub type Shdr = gabi::section::header::Shdr<BasicType>;
    impl_convert_from_block_mem_for_plain_struct!(Shdr);
}
