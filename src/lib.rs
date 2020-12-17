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

#[macro_use]
extern crate elf_proc;
use std::ops::Index;

use elf32::ehdr::ident::encode::Encode;

pub mod elf32;
pub mod elf64;
pub mod interface;

pub struct EndWrapper<'a, T> {
    pub src: &'a T,
    pub endiness: Encode,
}

pub struct EndWrapperMut<'a, T> {
    pub src: &'a mut T,
    pub endiness: Encode,
}

pub struct Wrapper<'a, T> {
    pub src: &'a T,
}
pub struct WrapperMut<'a, T> {
    pub src: &'a mut T,
}
/// 直接通过索引来获取字符串表中的数值
/// # Example
/// ```
/// use elf::StrTab;
///
/// let tmp = ['a' as u8,'b' as u8,'c' as u8,'\0' as u8];
/// let str_tab = StrTab::new(&tmp);
/// println!("{}", &str_tab[0], &str_tab[1]);
/// ```
pub struct StrTab<'a> {
    src: &'a [u8],
}

impl<'a> StrTab<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Self { src }
    }
}

impl Index<usize> for StrTab<'_> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        let end = self
            .src
            .iter()
            .enumerate()
            .find(|x| *x.1 == 0)
            .map_or(self.src.len(), |x| x.0);
        std::str::from_utf8(&self.src[index..end]).map_or("", |x| x)
    }
}
