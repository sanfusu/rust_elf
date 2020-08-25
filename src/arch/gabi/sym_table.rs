#[allow(non_snake_case)]
pub(crate) mod BIND {
    pub const LOCAL: u8 = 0;
    pub const GLOBAL: u8 = 1;
    pub const WEAK: u8 = 2;
    pub const LOPROC: u8 = 13;
    pub const HIPROC: u8 = 15;
}

#[allow(non_snake_case)]
pub(crate) mod TYPE {
    pub const NOTYPE: u8 = 0;
    pub const OBJECT: u8 = 1;
    pub const FUNC: u8 = 2;
    pub const SECTION: u8 = 3;
    pub const FILE: u8 = 4;
    pub const LOPROC: u8 = 13;
    pub const HIPROC: u8 = 15;
}
