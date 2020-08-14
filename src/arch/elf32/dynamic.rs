use super::ElfBasicType;
pub use crate::arch::gabi::dynamic::{self, *};
pub type Dyn = dynamic::Dyn<ElfBasicType>;

pub mod d_tag {
    crate::define_d_tag_basic_const!(<super::ElfBasicType as crate::BasicType>::Sxword);
}
