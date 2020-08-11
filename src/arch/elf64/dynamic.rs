use super::Elf;
pub use crate::arch::gabi::dynamic::{self, *};

pub mod d_tag {
    pub use crate::arch::gabi::dynamic::d_tag::*;
    pub const INIT_ARRAY: u32 = 25;
    pub const FINI_ARRAY: u32 = 26;
    pub const INIT_ARRAYSZ: u32 = 27;
    pub const FINI_ARRAYSZ: u32 = 28;
    pub const LOOS: u32 = 0x6000_0000;
    pub const HIOS: u32 = 0x6fff_ffff;
}

pub type Dyn = dynamic::Dyn<Elf>;
