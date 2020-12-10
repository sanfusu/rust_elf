use super::Word;

#[derive(Debug)]
pub enum ShType {
    NULL,
    PROGBITS,
    SYMTAB,
    STRTAB,
    RELA,
    HASH,
    DYNAMIC,
    NOTE,
    NOBITS,
    REL,
    SHLIB,
    DYNSYM,
    UNKNOW(Word),
    PROCESSOR(Word),
    USER(Word),
}

impl std::convert::From<u32> for ShType {
    fn from(val: u32) -> Self {
        match val {
            NULL => ShType::NULL,
            PROGBITS => ShType::PROGBITS,
            SYMTAB => ShType::SYMTAB,
            STRTAB => ShType::STRTAB,
            RELA => ShType::RELA,
            HASH => ShType::HASH,
            DYNAMIC => ShType::DYNAMIC,
            NOTE => ShType::NOTE,
            NOBITS => ShType::NOBITS,
            REL => ShType::REL,
            SHLIB => ShType::SHLIB,
            DYNSYM => ShType::DYNSYM,
            LOPROC..=HIPROC => ShType::PROCESSOR(val),
            LOUSER..=HIUSER => ShType::USER(val),
            _ => ShType::UNKNOW(val),
        }
    }
}

const NULL: u32 = 0;
const PROGBITS: u32 = 1;
const SYMTAB: u32 = 2;
const STRTAB: u32 = 3;
const RELA: u32 = 4;
const HASH: u32 = 5;
const DYNAMIC: u32 = 6;
const NOTE: u32 = 7;
const NOBITS: u32 = 8;
const REL: u32 = 9;
const SHLIB: u32 = 10;
const DYNSYM: u32 = 11;
const LOPROC: u32 = 0x70000000;
const HIPROC: u32 = 0x7fffffff;
const LOUSER: u32 = 0x80000000;
const HIUSER: u32 = 0xffffffff;
