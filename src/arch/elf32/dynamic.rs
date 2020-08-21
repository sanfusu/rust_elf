use super::BasicType;
pub use crate::arch::gabi::dynamic::{self, *};
pub type Dyn = dynamic::Dyn<BasicType>;

pub mod d_tag {
    crate::define_d_tag_basic_const!(<super::BasicType as crate::IBasicType>::Sxword);
}
