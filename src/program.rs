use crate::{Elf64Addr, Elf64Off, Elf64Word, Elf64Xword};

#[repr(C)]
pub struct Header {
    pub ptype: Elf64Word,
    pub flags: Elf64Word,
    pub offset: Elf64Off,
    pub vaddr: Elf64Addr,
    /// 物理寻址，这个字段是为那些可以物理寻址的系统保留的。
    pub paddr: Elf64Addr,
    pub filesz: Elf64Xword,
    pub memsz: Elf64Xword,
    pub align: Elf64Xword,
}

impl_borrow!(Header);
