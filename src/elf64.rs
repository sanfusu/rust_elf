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

pub type Addr = u64;
pub type Off = u64;
pub type Half = u16;
pub type Word = u32;
pub type Sword = u32;
pub type Xword = u64;
pub type Sxword = i64;

#[derive(MetaData, Ehdr)]
#[repr(packed)]
pub struct Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: Half,
    pub e_machine: Half,
    pub e_version: Word,
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

#[derive(MetaData)]
#[repr(packed)]
pub struct Shdr {
    pub sh_name: Word,
    pub sh_type: Word,
    pub sh_flags: Xword,
    pub sh_addr: Addr,
    pub sh_offset: Off,
    pub sh_size: Xword,
    pub sh_link: Word,
    pub sh_info: Word,
    pub sh_addralign: Xword,
    pub sh_entsize: Xword,
}

#[derive(MetaData)]
#[repr(packed)]
pub struct Sym {
    pub st_name: Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Half,
    pub st_value: Addr,
    pub st_size: Xword,
}

#[derive(MetaData)]
#[repr(packed)]
pub struct Phdr {
    pub p_type: Word,
    pub p_flags: Word,
    pub p_offset: Off,
    pub p_vaddr: Addr,
    pub p_paddr: Addr,
    pub p_filesz: Xword,
    pub p_memsz: Xword,
    pub p_align: Xword,
}

pub struct Segment {
    pub header: Phdr,
    pub section: Section,
}

pub struct Section {
    pub header: Shdr,
    pub name: String,
    pub data: Vec<u8>,
}
