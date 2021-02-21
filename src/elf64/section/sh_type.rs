use core::{convert::TryFrom, ops::RangeInclusive};

use crate::elf64::Word;

#[repr(transparent)]
#[derive(MetaData, Default, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct ShType {
    value: Word,
}
impl ShType {
    const fn new(value: Word) -> Self {
        Self { value }
    }
}
impl Into<Word> for ShType {
    fn into(self) -> Word {
        self.value
    }
}
impl TryFrom<Word> for ShType {
    type Error = &'static str;
    fn try_from(value: Word) -> Result<Self, Self::Error> {
        let ret = ShType::new(value);
        if SHT_RESERVED_RANGE.contains(&ret)
            || SHT_OS_RANGE.contains(&ret)
            || SHT_PROC_RANGE.contains(&ret)
        {
            Ok(ret)
        } else {
            Err("out of range")
        }
    }
}

pub const SHT_NULL: ShType = ShType::new(0); // Marks an unused section header
pub const SHT_PROGBITS: ShType = ShType::new(1); // Contains information defined by the program
pub const SHT_SYMTAB: ShType = ShType::new(2); // Contains a linker symbol table
pub const SHT_STRTAB: ShType = ShType::new(3); // Contains a string table
pub const SHT_RELA: ShType = ShType::new(4); // Contains "Rela" type relocation entries
pub const SHT_HASH: ShType = ShType::new(5); // Contains a symbol hash table
pub const SHT_DYNAMIC: ShType = ShType::new(6); // Contains dynamic linking tables
pub const SHT_NOTE: ShType = ShType::new(7); // Contains note information
pub const SHT_NOBITS: ShType = ShType::new(8); // Contains uninitialized space; does not occupy any space in the file
pub const SHT_REL: ShType = ShType::new(9); // Contains "Rel" type relocation entries
pub const SHT_SHLIB: ShType = ShType::new(10); // Reserved
pub const SHT_DYNSYM: ShType = ShType::new(11); // Contains a dynamic loader symbol table
pub const SHT_LOOS: ShType = ShType::new(0x60000000); // Environment-specific use
pub const SHT_HIOS: ShType = ShType::new(0x6FFFFFFF); //
pub const SHT_LOPROC: ShType = ShType::new(0x70000000); // Processor-specific use
pub const SHT_HIPROC: ShType = ShType::new(0x7FFFFFFF); //

const SHT_RESERVED_RANGE: RangeInclusive<ShType> = SHT_NULL..=SHT_DYNSYM;
const SHT_OS_RANGE: RangeInclusive<ShType> = SHT_LOOS..=SHT_HIOS;
const SHT_PROC_RANGE: RangeInclusive<ShType> = SHT_LOPROC..=SHT_HIPROC;
