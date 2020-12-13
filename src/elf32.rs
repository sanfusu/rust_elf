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
pub mod ehdr;
pub mod section;
pub mod segment;

use ehdr::Ehdr;
use section::{Section, Shdr};
use segment::{Phdr, Segment};

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
