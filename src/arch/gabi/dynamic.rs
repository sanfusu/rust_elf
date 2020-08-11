pub union Du<T: crate::BasicType> {
    pub val: T::Xword,
    pub ptr: T::Addr,
}

pub struct Dyn<T: crate::BasicType> {
    pub tag: T::Sxword,
    pub un: Du<T>,
}

pub mod d_tag {
    pub const NULL: u32 = 0;
    pub const NEEDED: u32 = 1;
    pub const PLTRELSZ: u32 = 2;
    pub const PLTGOT: u32 = 3;
    pub const HASH: u32 = 4;
    pub const STRTAB: u32 = 5;
    pub const SYMTAB: u32 = 6;
    pub const RELA: u32 = 7;
    pub const RELASZ: u32 = 8;
    pub const RELAENT: u32 = 9;
    pub const STRSZ: u32 = 10;
    pub const SYMENT: u32 = 11;
    pub const INIT: u32 = 12;
    pub const FINI: u32 = 13;
    pub const SONAME: u32 = 14;
    pub const RPATH: u32 = 15;
    pub const SYMBOLIC: u32 = 16;
    pub const REL: u32 = 17;
    pub const RELSZ: u32 = 18;
    pub const RELENT: u32 = 19;
    pub const PLTREL: u32 = 20;
    pub const DEBUG: u32 = 21;
    pub const TEXTREL: u32 = 22;
    pub const JMPREL: u32 = 23;
    pub const BIND_NOW: u32 = 24;
    pub const LOPROC: u32 = 0x7000_0000;
    pub const HIPROC: u32 = 0x7fff_ffff;
}
