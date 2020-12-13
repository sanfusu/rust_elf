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

pub mod e_type;
pub mod ident;

use self::ident::version::Version;
use super::basic_type::*;
use crate::interface::MetaData;

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

pub struct Wrapper<'a> {
    pub(crate) header: &'a Ehdr,
}

pub struct WrapperMut<'a> {
    pub(crate) header: &'a mut Ehdr,
}

impl Wrapper<'_> {
    pub fn version(&self) -> Version {
        self.header.e_version.into()
    }
    pub fn ident(&self) -> ident::Wrapper {
        ident::Wrapper {
            id: &self.header.e_ident,
        }
    }
    pub fn obj_type(&self) -> e_type::Type {
        self.header.e_type.into()
    }
}

impl WrapperMut<'_> {
    pub fn ident(&mut self) -> ident::WrapperMut {
        ident::WrapperMut {
            id: &mut self.header.e_ident,
        }
    }
    pub fn obj_type(&mut self, ty: e_type::Type) {
        self.header.e_type = ty.into();
    }
}
