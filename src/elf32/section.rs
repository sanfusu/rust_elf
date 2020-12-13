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

use super::{basic_type::*, chunk::DataChunk};

#[derive(MetaData, Default)]
#[repr(packed)]
pub struct Shdr {
    pub sh_name: Word,
    pub sh_type: Word,
    pub sh_flags: Word,
    pub sh_addr: Addr,
    pub sh_offset: Off,
    pub sh_size: Word,
    pub sh_link: Word,
    pub sh_info: Word,
    pub sh_addralign: Word,
    pub sh_entsize: Word,
}

#[derive(Default)]
pub struct Section {
    pub header: Shdr,
    pub name: String,
    pub data: Vec<DataChunk>,
}

impl Section {
    /// Consume self and return the data
    pub fn take_data(self) -> Vec<DataChunk> {
        self.data
    }
    /// getter 访问器效率上可能不会很高（看编译器优化），但形式上会更安全一点
    pub fn getter(&self) -> Wrapper {
        Wrapper { sec: self }
    }
    /// setter 访问器效率上可能不会很高（看编译器优化），但形式上会更安全一点
    /// # Example
    /// ```
    /// use elf::elf32::section::*;
    /// use elf::elf32::section::sh_type::*;
    ///
    /// let mut sec = Section{..Default::default()};
    /// sec.setter().sh_type(Type::Progbits);
    ///
    /// assert!(match sec.getter().sh_type() {Type::Progbits => true, _ => false});
    /// ```
    pub fn setter(&mut self) -> WrapperMut {
        WrapperMut { sec: self }
    }
}

pub struct Wrapper<'a> {
    sec: &'a Section,
}

impl Wrapper<'_> {
    pub fn sh_type(&self) -> sh_type::Type {
        self.sec.header.sh_type.into()
    }
    pub fn sh_flags(&self) -> sh_flags::Wrapper {
        sh_flags::Wrapper {
            shdr: &self.sec.header,
        }
    }
}

pub struct WrapperMut<'a> {
    sec: &'a mut Section,
}
impl WrapperMut<'_> {
    pub fn sh_type(&mut self, val: sh_type::Type) {
        self.sec.header.sh_type = val.into();
    }
    pub fn sh_flags(&mut self) -> sh_flags::WrapperMut {
        sh_flags::WrapperMut {
            shdr: &mut self.sec.header,
        }
    }
}
