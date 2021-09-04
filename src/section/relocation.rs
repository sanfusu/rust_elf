use crate::{Elf64Addr, Elf64Sxword, Elf64Xword};

#[repr(C)]
pub struct ReloType {
    value: Elf64Xword,
}
#[repr(C)]
pub struct Rel {
    pub offset: Elf64Addr,
    pub info: Elf64Xword,
}
#[repr(C)]
pub struct Rela {
    pub offset: Elf64Addr,
    pub info: Elf64Xword,
    pub addend: Elf64Sxword,
}

impl_borrow!(ReloType, Rel, Rela);
