use super::{gabi41::ElfBasicType, Elf};
use crate::sym_info_accessor;
/// 各字段含义等同于 [`Sym32`](super::super::elf32::symtable::Sym)，唯一不同的是各字段大小和内存布局。
#[repr(packed)]
pub struct Sym {
    pub name: <Elf as ElfBasicType>::Word,
    info: u8,
    pub other: u8,
    pub shndx: <Elf as ElfBasicType>::Half,
    pub value: <Elf as ElfBasicType>::Addr,
    pub size: <Elf as ElfBasicType>::Xword,
}

sym_info_accessor!(Sym);
