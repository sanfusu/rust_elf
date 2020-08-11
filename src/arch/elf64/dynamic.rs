use super::Elf;
pub use crate::arch::gabi::dynamic::{self, *};

pub mod d_tag {
    pub const INIT_ARRAY: <super::Elf as crate::BasicType>::Sxword = 25;
    pub const FINI_ARRAY: <super::Elf as crate::BasicType>::Sxword = 26;
    pub const INIT_ARRAYSZ: <super::Elf as crate::BasicType>::Sxword = 27;
    pub const FINI_ARRAYSZ: <super::Elf as crate::BasicType>::Sxword = 28;
    pub const LOOS: <super::Elf as crate::BasicType>::Sxword = 0x6000_0000;
    pub const HIOS: <super::Elf as crate::BasicType>::Sxword = 0x6fff_ffff;
    crate::define_d_tag_basic_const!(<super::Elf as crate::BasicType>::Sxword);
}

pub type Dyn = dynamic::Dyn<Elf>;
