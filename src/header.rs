use super::{Elf64Addr, Elf64Half, Elf64Off, Elf64Word};

#[repr(C)]
pub struct Ident {
    pub mag: [u8; 4],
    pub file_class: u8,
    pub data_encode: u8,
    pub file_version: u8,
    pub osabi: u8,
    pub abi_version: u8,
}

#[repr(C)]
pub struct Header {
    pub ident: Ident,
    _pad: [u8; 16 - core::mem::size_of::<Ident>()],
    pub file_type: Elf64Half,
    pub machine: Elf64Half,
    pub version: Elf64Word,
    pub entry: Elf64Addr,
    pub phoff: Elf64Off,
    pub shoff: Elf64Off,
    pub flags: Elf64Word,
    pub ehsize: Elf64Half,
    pub phentsize: Elf64Half,
    pub phnum: Elf64Half,
    pub shentsize: Elf64Half,
    pub shnum: Elf64Half,
    pub shstrndx: Elf64Half,
}

impl_borrow!(Header, Ident);
