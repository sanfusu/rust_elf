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

use crate::interface::MetaData;
use ident::version::Version;

pub mod ident;
pub mod section;
pub mod segment;

pub type Addr = u32;
pub type Off = u32;
pub type Half = u16;
pub type Word = u32;
pub type Sword = u32;

#[derive(MetaData, Ehdr)]
#[repr(packed)]
pub struct Ehdr {
    e_ident: [u8; 16],
    e_type: Half,
    e_machine: Half,
    e_version: Word,
    pub e_entry: Addr,
    #[phoff]
    pub e_phoff: Off,
    #[shoff]
    pub e_shoff: Off,
    pub e_flags: Word,
    pub e_ehsize: Half,
    #[phentsize]
    pub e_phentsize: Half,
    #[phnum]
    pub e_phnum: Half,
    #[shentsize]
    pub e_shentsize: Half,
    #[shnum]
    pub e_shnum: Half,
    pub e_shstrndx: Half,
}

impl Default for Ehdr {
    fn default() -> Self {
        let tmp = [0u8; std::mem::size_of::<Ehdr>()];
        let mut ret = Ehdr::from_le_bytes(tmp);
        ret.e_version = Version::Current.into();
        ret.as_mut_slice()[0..std::mem::size_of_val(&ident::MAGIC)].copy_from_slice(&ident::MAGIC);
        ret
    }
}

pub struct Elf {
    header: Ehdr,
}

pub struct Wrapper<'a> {
    header: &'a Ehdr,
}

pub struct WrapperMut<'a> {
    header: &'a mut Ehdr,
}

impl Elf {
    pub fn getter(&self) -> Wrapper {
        Wrapper {
            header: &self.header,
        }
    }
    pub fn setter(&mut self) -> WrapperMut {
        WrapperMut {
            header: &mut self.header,
        }
    }
}

impl Wrapper<'_> {
    pub fn id(&self) -> ident::Wrapper {
        ident::Wrapper {
            id: &self.header.e_ident,
        }
    }
    pub fn version(&self) -> Version {
        self.header.e_version.into()
    }
}

impl WrapperMut<'_> {
    pub fn id(&mut self) -> ident::WrapperMut {
        ident::WrapperMut {
            id: &mut self.header.e_ident,
        }
    }
}
