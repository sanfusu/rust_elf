use super::basic_type::BasicType;
use crate::arch::gabi::dynamic;

pub mod d_tag {
    pub const INIT_ARRAY: <super::BasicType as crate::IBasicType>::Sxword = 25;
    pub const FINI_ARRAY: <super::BasicType as crate::IBasicType>::Sxword = 26;
    pub const INIT_ARRAYSZ: <super::BasicType as crate::IBasicType>::Sxword = 27;
    pub const FINI_ARRAYSZ: <super::BasicType as crate::IBasicType>::Sxword = 28;
    pub const LOOS: <super::BasicType as crate::IBasicType>::Sxword = 0x6000_0000;
    pub const HIOS: <super::BasicType as crate::IBasicType>::Sxword = 0x6fff_ffff;
    crate::define_d_tag_basic_const!(<super::BasicType as crate::IBasicType>::Sxword);
}

pub type Dyn = dynamic::Dyn<BasicType>;
pub type Du = dynamic::Du<BasicType>;
