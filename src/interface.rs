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

pub trait MetaData: Sized {
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
}

pub trait Ehdr {
    fn shdr_table_range(&self) -> Range<usize>;
    fn phdr_table_range(&self) -> Range<usize>;
}
pub trait Section {
    fn name(&self) -> String;
}
pub trait Elf {
    fn sections<T: Section>(&self) -> Vec<T>;
    fn programs(&self);
}
