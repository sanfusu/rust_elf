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
#![no_std]
#![allow(dead_code)]

#[macro_use]
extern crate flassor;

pub mod header;
pub mod section;
pub mod program;
pub mod str_tab;
pub mod sym;

pub type Elf64Addr = u64;
pub type Elf64Off = u64;
pub type Half = u16;
pub type Elf64Word = u32;
pub type Sword = u32;
pub type Elf64Xword = u64;
pub type Sxword = i64;

