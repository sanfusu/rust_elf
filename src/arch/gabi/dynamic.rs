pub union Du<T: crate::BasicType> {
    pub val: T::Xword,
    pub ptr: T::Addr,
}

#[repr(C)]
pub struct Dyn<T: crate::BasicType> {
    pub tag: T::Sxword,
    pub un: Du<T>,
}

pub mod d_tag {
    pub const NULL: i32 = 0;
    pub const NEEDED: i32 = 1;
    pub const PLTRELSZ: i32 = 2;
    pub const PLTGOT: i32 = 3;
    pub const HASH: i32 = 4;
    pub const STRTAB: i32 = 5;
    pub const SYMTAB: i32 = 6;
    pub const RELA: i32 = 7;
    pub const RELASZ: i32 = 8;
    pub const RELAENT: i32 = 9;
    pub const STRSZ: i32 = 10;
    pub const SYMENT: i32 = 11;
    pub const INIT: i32 = 12;
    pub const FINI: i32 = 13;
    pub const SONAME: i32 = 14;
    pub const RPATH: i32 = 15;
    pub const SYMBOLIC: i32 = 16;
    pub const REL: i32 = 17;
    pub const RELSZ: i32 = 18;
    pub const RELENT: i32 = 19;
    pub const PLTREL: i32 = 20;
    pub const DEBUG: i32 = 21;
    pub const TEXTREL: i32 = 22;
    pub const JMPREL: i32 = 23;
    pub const BIND_NOW: i32 = 24;
    pub const LOPROC: i32 = 0x7000_0000;
    pub const HIPROC: i32 = 0x7fff_ffff;
}
