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

use std::ops::Range;

pub enum Type {
    None,
    Object,
    Func,
    Section,
    Processor(u8),
    File,
    Unknown(u8),
}

impl Into<u8> for Type {
    fn into(self) -> u8 {
        match self {
            Type::None => NOTYPE,
            Type::Object => OBJECT,
            Type::Func => FUNC,
            Type::Section => SECTION,
            Type::Processor(v) if PROCRANGE.contains(&v) => v,
            Type::Processor(_) => {
                panic!("You should not construct a processor sym type out of range")
            }
            Type::File => FILE,
            Type::Unknown(v) => v,
        }
    }
}

impl From<u8> for Type {
    fn from(v: u8) -> Self {
        match v {
            NOTYPE => Type::None,
            OBJECT => Type::Object,
            FUNC => Type::Func,
            SECTION => Type::Section,
            FILE => Type::File,
            LOPROC..=HIPROC => Type::Processor(v),
            _ => Type::Unknown(v),
        }
    }
}
const NOTYPE: u8 = 0;
const OBJECT: u8 = 1;
const FUNC: u8 = 2;
const SECTION: u8 = 3;
const FILE: u8 = 4;
const LOPROC: u8 = 13;
const HIPROC: u8 = 15;
const PROCRANGE: Range<u8> = LOPROC..HIPROC + 1;
