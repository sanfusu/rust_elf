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

use std::ops::{Deref, DerefMut};

/// 用来表示 section 中的数据块，一个 section 可能会有多个数据块，
/// 每一个数据块的大小由 [`Shdr::sh_entsize`](crate::elf32::section::Shdr::sh_entsize) 决定
///
/// 对 Vec<u8> 类型的简单封装，实现了 Deref。
#[derive(Default)]
pub struct DataChunk<T> {
    data: T,
}

impl<T> DataChunk<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Deref for DataChunk<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for DataChunk<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
