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

pub enum Version {
    Current,
    Invalid,
}

impl core::convert::Into<u32> for Version {
    fn into(self) -> u32 {
        match self {
            Version::Current => 1,
            Version::Invalid => panic! {"You should not construct a invalid version"},
        }
    }
}

impl core::convert::From<u32> for Version {
    fn from(val: u32) -> Self {
        match val {
            CURRENT => Version::Current,
            _ => Version::Invalid,
        }
    }
}
impl core::convert::Into<u8> for Version {
    fn into(self) -> u8 {
        match self {
            Version::Current => 1,
            Version::Invalid => panic! {"You should not construct a invalid version"},
        }
    }
}

impl core::convert::From<u8> for Version {
    fn from(val: u8) -> Self {
        match val as u32 {
            CURRENT => Version::Current,
            _ => Version::Invalid,
        }
    }
}
const CURRENT: u32 = 1;
