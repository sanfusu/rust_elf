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

// Ident 单独列为一个模块是因为，ident 作为 u8 数组和字节顺序无关。
pub mod class;
pub mod encode;
pub mod machine;
pub mod version;

use self::{class::Class, encode::Encode, version::Version};

pub(crate) const MAGIC: [u8; 4] = [0x7f, 'E' as u8, 'L' as u8, 'F' as u8];
pub(crate) const CLASS_IDX: usize = 4;
pub(crate) const DATA_IDX: usize = 5;
pub(crate) const VERSION_IDX: usize = 6;

pub struct Wrapper<'a> {
    pub(crate) id: &'a [u8; 16],
}

impl Wrapper<'_> {
    pub fn version(&self) -> Version {
        (self.id[VERSION_IDX] as u32).into()
    }
    /// Class 只能是 [`Class::Class32`]，所以不提供写入访问
    pub fn class(&self) -> Class {
        self.id[CLASS_IDX].into()
    }
    pub fn encode(&self) -> Encode {
        self.id[DATA_IDX].into()
    }
}

pub struct WrapperMut<'a> {
    pub(crate) id: &'a mut [u8; 16],
}

impl WrapperMut<'_> {
    pub fn encode(&mut self, ec: Encode) {
        self.id[DATA_IDX] = ec.into();
    }
}
