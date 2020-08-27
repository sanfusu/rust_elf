use super::basic_type;

#[derive(Debug, Copy, Clone)]
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
impl_convert_from_block_mem_for_plain_struct!(Phdr);

#[allow(non_snake_case)]
pub mod TYPE {
    use super::basic_type;

    define_p_type_basic_const!(basic_type::Word);
    pub const LOOS: basic_type::Word = 0x6000_0000;
    pub const HIOS: basic_type::Word = 0x6fff_ffff;
}

#[allow(non_snake_case)]
pub mod FLAGS {
    use super::basic_type;

    define_p_flags_basic_const!(basic_type::Word);
    pub const MASKOS: basic_type::Word = 0x00ff_0000;
}
