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

#[allow(non_snake_case)]
pub mod TYPE {
    define_p_type_basic_const!(<super::BasicType as crate::IBasicType>::Word);
}
#[allow(non_snake_case)]
pub mod FLAGS {
    define_p_flags_basic_const!(<super::BasicType as crate::IBasicType>::Word);
}
