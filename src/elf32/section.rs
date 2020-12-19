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

pub mod reloc;
pub mod sh_flags;
pub mod sh_type;
pub mod sym;

use super::basic_type::*;

#[derive(MetaData, Default, Shdr, Layout)]
#[repr(packed)]
pub struct Header {
    #[name_idx]
    pub sh_name: Word,
    pub sh_type: Word,
    pub sh_flags: Word,
    pub sh_addr: Addr,
    #[offset]
    pub sh_offset: Off,
    #[size]
    pub sh_size: Word,
    pub sh_link: Word,
    pub sh_info: Word,
    pub sh_addralign: Word,
    #[entsize]
    pub sh_entsize: Word,
}

pub struct Wrapper<'a> {
    shdr: &'a Header,
}

impl Wrapper<'_> {
    pub fn sh_type(&self) -> sh_type::Type {
        self.shdr.sh_type.into()
    }
    pub fn sh_flags(&self) -> sh_flags::Wrapper {
        sh_flags::Wrapper { shdr: &self.shdr }
    }
}

pub struct WrapperMut<'a> {
    shdr: &'a mut Header,
}
impl WrapperMut<'_> {
    pub fn sh_type(&mut self, val: sh_type::Type) {
        self.shdr.sh_type = val.into();
    }
    pub fn sh_flags(&mut self) -> sh_flags::WrapperMut {
        sh_flags::WrapperMut {
            shdr: &mut self.shdr,
        }
    }
}
