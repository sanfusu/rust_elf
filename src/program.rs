use crate::{Elf64Addr, Elf64Off, Elf64Word, Elf64Xword};

#[derive(Accessor)]
#[repr(packed)]
pub struct Header {
    pub ptype: Elf64Word,
    pub flags: Elf64Word,
    pub offset: Elf64Off,
    pub vaddr: Elf64Addr,
    pub paddr: Elf64Addr,
    pub filesz: Elf64Xword,
    pub memsz: Elf64Xword,
    pub align: Elf64Xword,
}
