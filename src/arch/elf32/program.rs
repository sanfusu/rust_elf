use super::BasicType;
use derive::AsSlice;
#[repr(C)]
#[derive(AsSlice)]
pub struct Header {
    pub r#type: <BasicType as crate::IBasicType>::Word,
    pub offset: <BasicType as crate::IBasicType>::Off,
    pub flags: <BasicType as crate::IBasicType>::Word,
    pub vaddr: <BasicType as crate::IBasicType>::Addr,
    pub paddr: <BasicType as crate::IBasicType>::Addr,
    pub filesz: <BasicType as crate::IBasicType>::Xword,
    pub memsz: <BasicType as crate::IBasicType>::Xword,
    pub align: <BasicType as crate::IBasicType>::Xword,
}

pub mod p_type {
    crate::define_p_type_basic_const!(<super::BasicType as crate::IBasicType>::Word);
}
pub mod p_flags {
    crate::define_p_flags_basic_const!(<super::BasicType as crate::IBasicType>::Word);
}
