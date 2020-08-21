use super::basic_type;

use derive::AsSlice;
#[derive(AsSlice)]
#[repr(C)]
pub struct Phdr {
    pub r#type: basic_type::Word,
    pub flags: basic_type::Word,
    pub offset: basic_type::Off,
    pub vaddr: basic_type::Addr,
    pub paddr: basic_type::Addr,
    pub filesz: basic_type::Xword,
    pub memsz: basic_type::Xword,
    pub align: basic_type::Xword,
}

pub mod p_type {
    use super::basic_type;

    crate::define_p_type_basic_const!(basic_type::Word);
    pub const LOOS: basic_type::Word = 0x6000_0000;
    pub const HIOS: basic_type::Word = 0x6fff_ffff;
}

pub mod p_flags {
    use super::basic_type;

    crate::define_p_flags_basic_const!(basic_type::Word);
    pub const MASKOS: basic_type::Word = 0x00ff_0000;
}
