use crate::Elf64Xword;

pub struct ReloType {
    value: Elf64Xword,
}
impl ReloType {
    fn new(value: Elf64Xword) -> Self {
        Self {
            value: value & 0xffff_ffff,
        }
    }
    pub fn proc(value: Elf64Xword) -> Self {
        Self {
            value: value & 0xffff_ffff,
        }
    }
}

pub mod rel {
    use crate::{Elf64Addr, Elf64Xword};

    #[accessor]
    pub struct Rel {
        pub offset: Elf64Addr,
        pub info: Elf64Xword,
    }
    pub use rel_accessor::*;
    
    impl fields::Info {
        pub fn sym_idx(&self) -> usize {
            (self.raw() >> 32) as usize
        }
        pub fn relo_type(&self) -> super::ReloType {
            super::ReloType::new(self.raw() & 0xffff_ffff)
        }
    }
}
pub mod rela {
    use crate::{Elf64Addr, Elf64Sxword, Elf64Xword};

    #[accessor]
    pub struct Rela {
        pub offset: Elf64Addr,
        pub info: Elf64Xword,
        pub addend: Elf64Sxword,
    }
    pub use rela_accessor::*;

    impl fields::Info {
        pub fn sym_idx(&self) -> usize {
            (self.raw() >> 32) as usize
        }
        pub fn relo_type(&self) -> super::ReloType {
            super::ReloType::new(self.raw() & 0xffff_ffff)
        }
    }
}
