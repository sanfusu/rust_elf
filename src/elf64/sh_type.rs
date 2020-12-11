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

pub struct Wrapper<'a> {
    shdr: &'a super::Shdr,
}
impl Wrapper<'_> {
    pub fn get(&self) -> Type {
        self.shdr.sh_type.into()
    }
    pub fn any_unknown(&self) -> Option<u32> {
        let sh_type = self.shdr.sh_type;
        if sh_type == NULL {
            return None;
        }
        if let Type::Null = sh_type.into() {
            Some(sh_type)
        } else {
            None
        }
    }
}
pub struct WrapperMut<'a> {
    shdr: &'a mut super::Shdr,
}
impl WrapperMut<'_> {
    pub fn with(&mut self, val: Type) {
        self.shdr.sh_type = val.into();
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Null,
    Progbits,
    Symtab,
    Strtab,
    Rela,
    Hash,
    Dynamic,
    Note,
    Nobits,
    Rel,
    Shlib,
    Dynsym,
    Processor(Word),
    Os(Word),
}

impl std::convert::From<u32> for Type {
    fn from(val: u32) -> Self {
        match val {
            NULL => Type::Null,
            PROGBITS => Type::Progbits,
            SYMTAB => Type::Symtab,
            STRTAB => Type::Strtab,
            RELA => Type::Rela,
            HASH => Type::Hash,
            DYNAMIC => Type::Dynamic,
            NOTE => Type::Note,
            NOBITS => Type::Nobits,
            REL => Type::Rel,
            SHLIB => Type::Shlib,
            DYNSYM => Type::Dynsym,
            LOPROC..=HIPROC => Type::Processor(val),
            LOOS..=HIOS => Type::Os(val),
            _ => Type::Null,
        }
    }
}

impl std::convert::Into<u32> for Type {
    fn into(self) -> u32 {
        match self {
            Type::Null => NULL,
            Type::Progbits => PROGBITS,
            Type::Symtab => SYMTAB,
            Type::Strtab => STRTAB,
            Type::Rela => RELA,
            Type::Hash => HASH,
            Type::Dynamic => DYNAMIC,
            Type::Note => NOTE,
            Type::Nobits => NOBITS,
            Type::Rel => REL,
            Type::Shlib => SHLIB,
            Type::Dynsym => DYNSYM,
            Type::Processor(v) | Type::Os(v) => v,
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
