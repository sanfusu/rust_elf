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

pub enum Machine {
    None,
    M32,
    Sparc,
    I386,
    M68k,
    M88k,
    I860,
    Mips,
    MipsRs4Be,
    Unknown(u8),
}

impl core::convert::Into<u8> for Machine {
    fn into(self) -> u8 {
        match self {
            Machine::None => ET_NONE,
            Machine::M32 => EM_M32,
            Machine::Sparc => EM_SPARC,
            Machine::I386 => EM_386,
            Machine::M68k => EM_68K,
            Machine::M88k => EM_88K,
            Machine::I860 => EM_860,
            Machine::Mips => EM_MIPS,
            Machine::MipsRs4Be => EM_MIPS_RS4_BE,
            Machine::Unknown(v) => v,
        }
    }
}

impl core::convert::From<u8> for Machine {
    fn from(v: u8) -> Self {
        match v {
            ET_NONE => Machine::None,
            EM_M32 => Machine::M32,
            EM_SPARC => Machine::Sparc,
            EM_386 => Machine::I386,
            EM_68K => Machine::M68k,
            EM_88K => Machine::M88k,
            EM_860 => Machine::I860,
            EM_MIPS => Machine::Mips,
            EM_MIPS_RS4_BE => Machine::MipsRs4Be,
            _ => Machine::Unknown(v),
        }
    }
}

const ET_NONE: u8 = 0; // No machine
const EM_M32: u8 = 1; // AT&T WE 32100
const EM_SPARC: u8 = 2; // SPARC
const EM_386: u8 = 3; // Intel Architecture
const EM_68K: u8 = 4; // Motorola 68000
const EM_88K: u8 = 5; // Motorola 88000
const EM_860: u8 = 7; // Intel 80860
const EM_MIPS: u8 = 8; // MIPS RS3000 Big-Endian
const EM_MIPS_RS4_BE: u8 = 10; // MIPS RS4000 Big-Endian
