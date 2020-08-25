use crate::arch::elf32::BasicType;

pub mod header {
    use crate::arch::{elf32::basic_type::BasicType, gabi};

    pub type Shdr = gabi::section::header::Shdr<BasicType>;
}

#[allow(non_snake_case)]
pub mod TYPE {
    define_sh_type_basic_const!(<super::BasicType as crate::IBasicType>::Word);
}

#[allow(non_snake_case)]
pub mod FLAGS {
    define_sh_flags_basic_const!(<super::BasicType as crate::IBasicType>::Xword);
}

#[allow(non_snake_case)]
pub mod IDX {
    pub use crate::arch::gabi::section::IDX::*;
}
