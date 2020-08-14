pub use crate::arch::gabi::section::*;
use crate::arch::{elf32::ElfBasicType, gabi};

pub type Header = gabi::section::Header<ElfBasicType>;

pub mod sh_type {
    crate::define_sh_type_basic_const!(<super::ElfBasicType as crate::BasicType>::Word);
}
pub mod sh_flags {
    crate::define_sh_flags_basic_const!(<super::ElfBasicType as crate::BasicType>::Xword);
}
