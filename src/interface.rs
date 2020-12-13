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

use std::ops::RangeInclusive;

pub trait MetaData<T: AsRef<[u8]> + Sized>: Sized {
    fn as_slice<'a>(&'a self) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
    fn as_mut_slice<'a>(&'a mut self) -> &'a mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self as *mut Self as *mut u8,
                std::mem::size_of::<Self>(),
            )
        }
    }

    fn read_from_slice(&mut self, src: &[u8]) {
        self.as_mut_slice().copy_from_slice(src);
    }
    fn to_be_bytes(self) -> T {
        todo! {}
    }
    fn to_le_bytes(self) -> T {
        todo! {}
    }
    fn to_ne_bytes(self) -> T {
        todo! {}
    }
    fn from_be_bytes(_: T) -> Self {
        todo! {}
    }
    fn from_le_bytes(_: T) -> Self {
        todo! {}
    }
    fn from_ne_bytes(_: T) -> Self {
        todo! {}
    }
    fn from_be(_: Self) -> Self {
        todo! {}
    }
    fn from_le(_: Self) -> Self {
        todo! {}
    }
    fn to_be(self) -> Self {
        todo! {}
    }
    fn to_le(self) -> Self {
        todo! {}
    }
    fn len() -> usize {
        std::mem::size_of::<Self>()
    }
}

pub trait Ehdr {
    fn shdr_table_range(&self) -> RangeInclusive<usize>;
    fn phdr_table_range(&self) -> RangeInclusive<usize>;
}

pub trait Section {
    fn name(&self) -> String;
}

pub trait Shdr {
    fn data_range(&self) -> RangeInclusive<usize>;
}
pub trait Elf {
    fn sections<T: Section>(&self) -> Vec<T>;
    fn programs(&self);
}
