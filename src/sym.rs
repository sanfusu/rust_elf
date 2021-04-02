use super::{Elf64Addr, Half, Elf64Word, Elf64Xword};

#[derive(Accessor)]
#[repr(packed)]
pub struct Sym {
    pub st_name: Elf64Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Half,
    pub st_value: Elf64Addr,
    pub st_size: Elf64Xword,
}

pub use sym_accessor::*;
