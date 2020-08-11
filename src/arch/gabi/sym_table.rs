pub mod st_bind {
    pub const LOCAL: u8 = 0;
    pub const GLOBAL: u8 = 1;
    pub const WEAK: u8 = 2;
    pub const LOPROC: u8 = 13;
    pub const HIPROC: u8 = 15;
}

pub mod st_type {
    pub const NOTYPE: u8 = 0;
    pub const OBJECT: u8 = 1;
    pub const FUNC: u8 = 2;
    pub const SECTION: u8 = 3;
    pub const FILE: u8 = 4;
    pub const LOPROC: u8 = 13;
    pub const HIPROC: u8 = 15;
}
