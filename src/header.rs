use crate::ident::Ident;

use super::{Elf64Addr, Elf64Half, Elf64Off, Elf64Word};

#[repr(C)]
pub struct Header {
    pub ident: Ident,
    _pad: [u8; 16 - core::mem::size_of::<Ident>()],
    pub obj_file_type: ObjectFileType,
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

#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjectFileType {
    data: u16,
}

const_enum::const_enum! {
    pub ObjectFileTypeBasic [ObjectFileType::data: u16] {
        NONE: 0,
        /// 可重定位的对象文件
        REL: 1,
        /// 可执行文件
        EXEC: 2,
        /// 可共享的对象文件
        DYN: 3,
        /// 核心文件
        CORE: 4
    }
}

impl ObjectFileType {
    pub fn is_env_spec(&self) -> bool {
        match self.data {
            0xFE00..=0xFEFF => true,
            _ => false,
        }
    }
    pub fn is_processor_spec(&self) -> bool {
        match self.data {
            0xFF00..=0xFFFF => true,
            _ => false,
        }
    }
    pub fn raw(&self) -> u16 {
        self.data
    }
}
