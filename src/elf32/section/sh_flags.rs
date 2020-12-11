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

pub enum Flags {
    Write,
    Alloc,
    ExecInstr,
    Processor(u32),
    Unknown(u32),
}

pub struct Wrapper<'a> {
    pub(crate) shdr: &'a super::Shdr,
}
impl Wrapper<'_> {
    pub fn is_writeable(&self) -> bool {
        self.shdr.sh_flags & WRITE != 0
    }
    pub fn is_allocable(&self) -> bool {
        self.shdr.sh_flags & ALLOC != 0
    }
    pub fn any_processor(&self) -> Option<u32> {
        let processor = self.shdr.sh_flags & MASK_PROCESSOR;

        if processor != 0 {
            Some(processor)
        } else {
            None
        }
    }
    pub fn any_unknown(&self) -> Option<u32> {
        let unknown = self.shdr.sh_flags & MASK_PROCESSOR;

        if unknown != 0 {
            Some(unknown)
        } else {
            None
        }
    }
}

pub struct WrapperMut<'a> {
    pub(crate) shdr: &'a mut super::Shdr,
}
impl<'a> WrapperMut<'a> {
    /// 设置 Ehdr 中 sh_flags 的值。
    ///
    /// val 如果是 Flags::Unknown(xxx)，则会被忽略。
    pub fn with(&'a mut self, val: Flags) -> &'a mut Self {
        match val {
            Flags::Write => self.shdr.sh_flags |= WRITE,
            Flags::Alloc => self.shdr.sh_flags |= ALLOC,
            Flags::ExecInstr => self.shdr.sh_flags |= EXECINSTR,
            Flags::Processor(v) => self.shdr.sh_flags |= v & MASK_PROCESSOR,
            Flags::Unknown(v) => self.shdr.sh_flags |= v & MASK_UNKNOWN,
        };
        self
    }
}

impl std::convert::Into<u32> for Flags {
    fn into(self) -> u32 {
        match self {
            Flags::Write => WRITE,
            Flags::Alloc => ALLOC,
            Flags::ExecInstr => EXECINSTR,
            Flags::Processor(v) if (v & !MASK_PROCESSOR) != 0 => {
                panic! {"Invalid processor bit"}
            }
            Flags::Unknown(v) if (v & !MASK_UNKNOWN) != 0 => {
                panic!("Invalid unknown bit")
            }
            Flags::Processor(v) => v & MASK_PROCESSOR,
            Flags::Unknown(v) => v & MASK_UNKNOWN,
        }
    }
}

const WRITE: u32 = 0x1;
const ALLOC: u32 = 0x2;
const EXECINSTR: u32 = 0x4;
const MASK_PROCESSOR: u32 = 0xf0000000;
const MASK_UNKNOWN: u32 = (!WRITE) & (!ALLOC) & (!EXECINSTR) & (!MASK_PROCESSOR);
