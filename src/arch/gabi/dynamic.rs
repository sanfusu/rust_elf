pub union Du<T: crate::BasicType> {
    pub val: T::Xword,
    pub ptr: T::Addr,
}

use derive::AsSlice;
#[derive(AsSlice)]
#[repr(C)]
pub struct Dyn<T: crate::BasicType> {
    pub tag: T::Sxword,
    pub un: Du<T>,
}
pub(crate) mod d_tag {
    #[macro_export]
    macro_rules! define_d_tag_basic_const {
        ($elf:ty) => {
            pub const NULL: $elf = 0;
            pub const NEEDED: $elf = 1;
            pub const PLTRELSZ: $elf = 2;
            pub const PLTGOT: $elf = 3;
            pub const HASH: $elf = 4;
            pub const STRTAB: $elf = 5;
            pub const SYMTAB: $elf = 6;
            pub const RELA: $elf = 7;
            pub const RELASZ: $elf = 8;
            pub const RELAENT: $elf = 9;
            pub const STRSZ: $elf = 10;
            pub const SYMENT: $elf = 11;
            pub const INIT: $elf = 12;
            pub const FINI: $elf = 13;
            pub const SONAME: $elf = 14;
            pub const RPATH: $elf = 15;
            pub const SYMBOLIC: $elf = 16;
            pub const REL: $elf = 17;
            pub const RELSZ: $elf = 18;
            pub const RELENT: $elf = 19;
            pub const PLTREL: $elf = 20;
            pub const DEBUG: $elf = 21;
            pub const TEXTREL: $elf = 22;
            pub const JMPREL: $elf = 23;
            pub const BIND_NOW: $elf = 24;
            pub const LOPROC: $elf = 0x7000_0000;
            pub const HIPROC: $elf = 0x7fff_ffff;
        };
    }
}
