use crate::arch::elf32::BasicType;

pub mod header {
    use crate::arch::{elf32::basic_type::BasicType, gabi};

    pub type Shdr = gabi::section::header::Shdr<BasicType>;
}

pub mod sh_type {
    crate::define_sh_type_basic_const!(<super::BasicType as crate::IBasicType>::Word);
}
pub mod sh_flags {
    crate::define_sh_flags_basic_const!(<super::BasicType as crate::IBasicType>::Xword);
}

pub use crate::arch::gabi::section::sh_idx;
