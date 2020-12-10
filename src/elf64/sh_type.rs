// Copyright (C) 2020 sanfusu@foxmail.com
//
// This file is part of rust_elf.
//
// rust_elf is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rust_elf is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rust_elf.  If not, see <http://www.gnu.org/licenses/>.

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
    PROCESSOR(Word),
    OS(Word),
    UNKNOW(Word),
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
            LOOS..=HIOS => ShType::OS(val),
            _ => ShType::UNKNOW(val),
        }
    }
}

// elf64 中用户保留类型区间被移除，额外添加了 OS 保留类型区间

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
const LOOS: u32 = 0x60000000;
const HIOS: u32 = 0x6FFFFFFF;
const LOPROC: u32 = 0x70000000;
const HIPROC: u32 = 0x7FFFFFFF;
