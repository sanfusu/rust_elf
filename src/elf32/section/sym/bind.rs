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

pub enum Bind {
    Local,
    Global,
    Weak,
    Processor(u8),
    Unknown(u8),
}

impl Into<u8> for Bind {
    fn into(self) -> u8 {
        match self {
            Bind::Local => LOCAL,
            Bind::Global => GLOBAL,
            Bind::Weak => WEAK,
            Bind::Processor(v) if v >= LOPROC && v <= HIPROC => v,
            Bind::Processor(_) => {
                panic!("You should not construct a out range Processor Sym Bind variant")
            }
            Bind::Unknown(v) => v,
        }
    }
}

impl From<u8> for Bind {
    fn from(v: u8) -> Self {
        match v {
            LOCAL => Bind::Local,
            GLOBAL => Bind::Global,
            WEAK => Bind::Weak,
            LOPROC..=HIPROC => Bind::Processor(v),
            _ => Bind::Unknown(v),
        }
    }
}

const LOCAL: u8 = 0;
const GLOBAL: u8 = 1;
const WEAK: u8 = 2;
const LOPROC: u8 = 13;
const HIPROC: u8 = 15;
