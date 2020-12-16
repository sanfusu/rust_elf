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

use crate::{Wrapper, WrapperMut};

use self::{class::Class, encode::Encode, version::Version};

pub(crate) const MAGIC: [u8; 4] = [0x7f, 'E' as u8, 'L' as u8, 'F' as u8];
pub(crate) const CLASS_IDX: usize = 4;
pub(crate) const DATA_IDX: usize = 5;
pub(crate) const VERSION_IDX: usize = 6;

impl Wrapper<'_, Ident> {
    /// Version 只能是 [`Version::Current`]，所以不提供写入访问
    pub fn version(&self) -> Version {
        (self.src.src[VERSION_IDX] as u32).into()
    }
    /// Class 只能是 [`Class::Class32`]，所以不提供写入访问
    pub fn class(&self) -> Class {
        self.src.src[CLASS_IDX].into()
    }
    pub fn encode(&self) -> Encode {
        self.src.src[DATA_IDX].into()
    }
}

impl WrapperMut<'_, Ident> {
    pub fn encode(&mut self, ec: Encode) {
        self.src.src[DATA_IDX] = ec.into();
    }
}

#[derive(MetaData)]
#[repr(packed)]
pub struct Ident {
    src: [u8; 16],
}

impl Default for Ident {
    /// 提供默认的 Magic, Class, Version
    fn default() -> Self {
        let mut ret = Self { src: [0; 16] };
        ret.src.copy_from_slice(&MAGIC);
        ret.src[CLASS_IDX] = Class::Class32.into();
        ret.src[VERSION_IDX] = Version::Current.into();
        ret
    }
}

impl Ident {
    pub fn getter(&self) -> Wrapper<Ident> {
        Wrapper::<Ident> { src: self }
    }
}
