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
