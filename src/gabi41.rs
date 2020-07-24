pub mod ehdr;
pub mod shdr;
pub mod spsec;
pub mod symtable;
pub trait ElfBasicType {
    type Xword: Default + From<u32>;
    type Sxword: Default + From<i32>;
    type Addr: Default + From<u32>;
    type Half: Default + From<u16>;
    type Off: Default + From<u32>;
    type Sword: Default + From<i32>;
    type Word: Default + From<u32>;
}
