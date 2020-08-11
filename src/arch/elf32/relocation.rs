pub mod rel {
    use crate::arch::{elf32, gabi::relocation::rel};

    pub type Entry = rel::Entry<elf32::Elf>;
}
pub mod rela {
    use crate::arch::{elf32, gabi::relocation::rela};

    pub type Entry = rela::Entry<elf32::Elf>;
}
