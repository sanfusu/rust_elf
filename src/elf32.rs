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

pub mod basic_type;
pub mod chunk;
pub mod ehdr;
pub mod err;
pub mod section;
pub mod segment;

use ehdr::Ehdr;
use section::{Section, Shdr};
use segment::{Phdr, Segment};

use crate::interface::{Ehdr as IEhdr, MetaData};

#[derive(Default)]
pub struct Elf {
    header: ehdr::Ehdr,
    pub secs: Option<Vec<section::Section>>,
    pub segs: Option<Vec<segment::Segment>>,
}

impl Elf {
    pub fn getter(&self) -> ehdr::Wrapper {
        ehdr::Wrapper {
            header: &self.header,
        }
    }
    pub fn setter(&mut self) -> ehdr::WrapperMut {
        ehdr::WrapperMut {
            header: &mut self.header,
        }
    }
    pub fn from_le_slice(_src: &[u8]) -> Result<Elf, err::Err> {
        todo!()
    }
    pub unsafe fn from_le_slice_uncheck(src: &[u8]) -> Result<Elf, err::Err> {
        let mut elf = Elf::default();

        if src.len() < Ehdr::len() {
            return Err(err::Err::OutofRange);
        };
        elf.header.read_from_slice(&src[0..Ehdr::len()]);
        elf.header = Ehdr::from_le(elf.header);

        let shdr_tab_range = elf.header.shdr_table_range();
        if src.len() < *shdr_tab_range.end() {
            return Err(err::Err::OutofRange);
        }
        if !shdr_tab_range.is_empty() {
            let mut secs: Vec<Section> = Vec::new();
            for shdr_src in src[shdr_tab_range].chunks(Shdr::len()) {
                let sec = Section::from_le_slice(src, shdr_src);
                //TODO 解析数据部分
                secs.push(sec);
            }
            elf.secs = Some(secs);
        }

        todo! {}
    }
    pub fn from_be_slice(_src: &[u8]) -> Result<Elf, err::Err> {
        todo!()
    }
    pub unsafe fn from_be_slice_uncheck(_src: &[u8]) -> Elf {
        todo!()
    }
    pub fn from_slice(_src: &[u8]) -> Result<Elf, err::Err> {
        todo!()
    }
}

pub struct LinkView {
    pub hdr: Ehdr,
    pub secs: Vec<Section>,
    pub phdrs: Option<Vec<Phdr>>,
}

pub struct RuntimeView {
    pub hdr: Ehdr,
    pub segs: Vec<Segment>,
    pub shdrs: Option<Vec<Shdr>>,
}
