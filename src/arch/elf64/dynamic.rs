use super::ElfBasicType;
use crate::arch::gabi::dynamic;

pub mod d_tag {
    pub const INIT_ARRAY: <super::ElfBasicType as crate::BasicType>::Sxword = 25;
    pub const FINI_ARRAY: <super::ElfBasicType as crate::BasicType>::Sxword = 26;
    pub const INIT_ARRAYSZ: <super::ElfBasicType as crate::BasicType>::Sxword = 27;
    pub const FINI_ARRAYSZ: <super::ElfBasicType as crate::BasicType>::Sxword = 28;
    pub const LOOS: <super::ElfBasicType as crate::BasicType>::Sxword = 0x6000_0000;
    pub const HIOS: <super::ElfBasicType as crate::BasicType>::Sxword = 0x6fff_ffff;
    crate::define_d_tag_basic_const!(<super::ElfBasicType as crate::BasicType>::Sxword);
}

pub type Dyn = dynamic::Dyn<ElfBasicType>;
pub type Du = dynamic::Du<ElfBasicType>;
