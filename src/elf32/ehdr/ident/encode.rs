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

#[derive(Clone, Copy)]
pub enum Encode {
    Lsb,
    Msb,
    Invalid,
}

pub struct Lsb {}
impl Lsb {
    pub fn encode() -> Encode {
        Encode::Lsb
    }
}

pub struct Msb {}
impl Msb {
    pub fn encode() -> Encode {
        Encode::Msb
    }
}

impl core::convert::From<u8> for Encode {
    fn from(val: u8) -> Self {
        match val {
            LSB => Encode::Lsb,
            MSB => Encode::Msb,
            _ => Encode::Invalid,
        }
    }
}

impl core::convert::Into<u8> for Encode {
    fn into(self) -> u8 {
        match self {
            Encode::Lsb => LSB,
            Encode::Msb => MSB,
            Encode::Invalid => {
                panic!("It's an invalid Encode, you shouldn't construct an invalid encode")
            }
        }
    }
}

const LSB: u8 = 1;
const MSB: u8 = 2;
