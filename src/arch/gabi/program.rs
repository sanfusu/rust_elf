pub mod p_type {
    pub const NULL: u32 = 0;
    pub const LOAD: u32 = 1;
    pub const DYNAMIC: u32 = 2;
    pub const INTERP: u32 = 3;
    pub const NOTE: u32 = 4;
    pub const SHLIB: u32 = 5;
    pub const PHDR: u32 = 6;
    pub const LOPROC: u32 = 0x7000_0000;
    pub const HIPROC: u32 = 0x7fff_ffff;
}

pub mod p_flags {
    pub const X: u32 = 0x1;
    pub const W: u32 = 0x2;
    pub const R: u32 = 0x4;
    pub const MASKPROC: u32 = 0xf000_0000;
}
