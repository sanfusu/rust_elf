pub use crate::arch::gabi::section::*;
use crate::arch::{elf32::Elf, gabi};

pub type Header = gabi::section::Header<Elf>;

pub mod sh_type {
    crate::define_sh_type_basic_const!(<super::Elf as crate::BasicType>::Word);
}
pub mod sh_flags {
    crate::define_sh_flags_basic_const!(<super::Elf as crate::BasicType>::Xword);
}
