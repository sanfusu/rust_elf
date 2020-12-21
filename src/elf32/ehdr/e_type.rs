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

use core::ops::{Range, RangeInclusive};

pub enum Type {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Processor(u16),
    Unknown(u16),
}

impl Into<u16> for Type {
    fn into(self) -> u16 {
        match self {
            Type::None => NONE,
            Type::Rel => REL,
            Type::Exec => EXEC,
            Type::Dyn => DYN,
            Type::Core => CORE,
            Type::Processor(v) if PROCRANGE.contains(&v) => v,
            Type::Processor(_) => {
                panic!("You should not construct a out range processor specified Elf type")
            }
            Type::Unknown(v) if UNKNRANGE.contains(&v) => v,
            Type::Unknown(_) => {
                panic!("The unknown specified Elf type is not in the range")
            }
        }
    }
}

impl From<u16> for Type {
    fn from(v: u16) -> Self {
        match v {
            NONE => Type::None,
            REL => Type::Rel,
            EXEC => Type::Exec,
            DYN => Type::Dyn,
            CORE => Type::Core,
            LOPROC..=HIPROC => Type::Processor(v),
            _ => Type::Unknown(v),
        }
    }
}

const NONE: u16 = 0;
const REL: u16 = 1;
const EXEC: u16 = 2;
const DYN: u16 = 3;
const CORE: u16 = 4;
const LOPROC: u16 = 0xff00;
const HIPROC: u16 = 0xffff;
const PROCRANGE: RangeInclusive<u16> = LOPROC..=HIPROC;
const UNKNRANGE: Range<u16> = CORE..LOPROC;
