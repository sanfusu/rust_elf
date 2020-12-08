pub type Addr = u32;
pub type Off = u32;
pub type Half = u16;
pub type Word = u32;
pub type Sword = u32;

#[derive(MetaData)]
#[repr(packed)]
pub struct Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: Half,
    pub e_machine: Half,
    pub e_version: Word,
    pub e_entry: Addr,
    pub e_phoff: Off,
    pub e_shoff: Off,
    pub e_flags: Word,
    pub e_ehsize: Half,
    pub e_phentsize: Half,
    pub e_phnum: Half,
    pub e_shentsize: Half,
    pub e_shnum: Half,
    pub e_shstrndx: Half,
}

#[derive(MetaData)]
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

#[derive(MetaData)]
#[repr(packed)]
pub struct Sym {
    pub st_name: Word,
    pub st_value: Addr,
    pub st_size: Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Half,
}

#[derive(MetaData)]
#[repr(packed)]
pub struct Phdr {
    pub p_type: Word,
    pub p_offset: Off,
    pub p_vaddr: Addr,
    pub p_paddr: Addr,
    pub p_filesz: Word,
    pub p_memsz: Word,
    pub p_flags: Word,
    pub p_align: Word,
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