pub mod rel {
    use crate::arch::{elf64, gabi::relocation::rel};

    pub type Entry = rel::Entry<elf64::Elf>;
}
pub mod rela {
    use crate::arch::{elf64, gabi::relocation::rela};

    pub type Entry = rela::Entry<elf64::Elf>;
}
