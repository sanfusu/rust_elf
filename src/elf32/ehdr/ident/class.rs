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

pub enum Class {
    Class32,
    Class64,
    Invalid,
}
impl core::convert::From<u8> for Class {
    fn from(val: u8) -> Self {
        match val {
            CLASS32 => Class::Class32,
            CLASS64 => Class::Class64,
            _ => Class::Invalid,
        }
    }
}

impl core::convert::Into<u8> for Class {
    fn into(self) -> u8 {
        match self {
            Class::Class32 => CLASS32,
            Class::Class64 => CLASS64,
            Class::Invalid => {
                panic!("It's an invalid Class, you shouldn't construct an invalid Class")
            }
        }
    }
}

const CLASS32: u8 = 1;
const CLASS64: u8 = 2;
