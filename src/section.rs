pub mod cie;
pub mod relocation;
pub mod str_tab;
pub mod symbol;
use super::{Elf64Addr, Elf64Off, Elf64Word, Elf64Xword};

#[repr(C)]
pub struct Header {
    pub name: Elf64Word,
    pub sec_type: Elf64Word,
    pub flags: Elf64Xword,
    pub addr: Elf64Addr,
    pub offset: Elf64Off,
    pub size: Elf64Xword,
    pub link: Elf64Word,
    pub info: Elf64Word,
    pub addralign: Elf64Xword,
    pub entsize: Elf64Xword,
}

impl_borrow!(Header);
