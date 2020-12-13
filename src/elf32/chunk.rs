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

use std::sync::{Arc, RwLock};

/// 用来表示 section 中的数据块，一个 section 可能会有多个数据块，
/// 每一个数据块的大小由 [`Shdr::sh_entsize`](crate::elf32::section::Shdr::sh_entsize) 决定
#[derive(Default, Debug)]
pub struct DataChunk {
    pub data: Arc<RwLock<Vec<u8>>>,
}
