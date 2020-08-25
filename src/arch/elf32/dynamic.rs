use super::BasicType;
pub use crate::arch::gabi::dynamic::{self, *};
pub type Dyn = dynamic::Dyn<BasicType>;

#[allow(non_snake_case)]
pub mod D_TAG {
    define_d_tag_basic_const!(<super::BasicType as crate::IBasicType>::Sxword);
}
