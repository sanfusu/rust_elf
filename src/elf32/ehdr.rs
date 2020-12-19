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

use elface::MetaData;
use ident::Ident;

use self::ident::version::Version;
use super::basic_type::*;
use crate::{EndWrapper, EndWrapperMut, Wrapper, WrapperMut};

#[derive(MetaData, Ehdr, Layout)]
#[repr(packed)]
pub struct Header {
    e_ident: Ident,
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

impl Default for Header {
    fn default() -> Self {
        let tmp = [0u8; std::mem::size_of::<Header>()];
        let mut ret = Header::from_le_bytes(tmp);
        ret.e_version = Version::Current.into();
        ret.as_mut_slice()[0..std::mem::size_of_val(&ident::MAGIC)].copy_from_slice(&ident::MAGIC);
        ret
    }
}

impl Header {
    pub fn getter(&self) -> crate::Wrapper<Self> {
        crate::Wrapper { src: self }
    }
    pub fn setter(&mut self) -> crate::WrapperMut<Self> {
        crate::WrapperMut { src: self }
    }

    pub fn getter_le(&self) -> EndWrapper<Self> {
        EndWrapper::<Self> {
            src: self,
            endiness: ident::encode::Encode::Lsb,
        }
    }
    pub fn setter_le(&mut self) -> EndWrapperMut<Self> {
        EndWrapperMut::<Self> {
            src: self,
            endiness: ident::encode::Encode::Lsb,
        }
    }
}

impl<'a> EndWrapper<'a, Header> {
    pub fn version(&self) -> Version {
        match self.endiness {
            ident::encode::Encode::Lsb => u32::from_le(self.src.e_version).into(),
            ident::encode::Encode::Msb => u32::from_be(self.src.e_version).into(),
            ident::encode::Encode::Invalid => {
                panic! {"Invalid encode"}
            }
        }
    }
    pub fn ident(&self) -> EndWrapper<Ident> {
        EndWrapper::<Ident> {
            src: &self.src.e_ident,
            endiness: self.endiness,
        }
    }
}

impl crate::Wrapper<'_, Header> {
    pub fn version(&self) -> Version {
        self.src.e_version.into()
    }
    pub fn ident(&self) -> Wrapper<Ident> {
        Wrapper::<Ident> {
            src: &self.src.e_ident,
        }
    }
    pub fn obj_type(&self) -> e_type::Type {
        self.src.e_type.into()
    }
}

impl crate::WrapperMut<'_, Header> {
    pub fn ident(&mut self) -> WrapperMut<Ident> {
        WrapperMut::<Ident> {
            src: &mut self.src.e_ident,
        }
    }
    pub fn obj_type(&mut self, ty: e_type::Type) {
        self.src.e_type = ty.into();
    }
}
