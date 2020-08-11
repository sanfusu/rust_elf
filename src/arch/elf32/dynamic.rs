use super::Elf;
pub use crate::arch::gabi::dynamic::{self, *};
pub type Dyn = dynamic::Dyn<Elf>;

pub mod d_tag {
    crate::define_d_tag_basic_const!(<super::Elf as crate::BasicType>::Sxword);
}
