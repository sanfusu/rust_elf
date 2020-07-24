use super::{
    gabi41::ehdr::{EIVersionValue, EMachineValue, ETypeValue, EhdrGeneral, PreEIdent, EI_NIDENT},
    Elf,
};

#[repr(C)]
pub struct EIdent {
    pub preident: PreEIdent,
    pub pad: [u8; 9],

    /// e_ident 的大小
    pub nident: u8,
}
impl Default for EIdent {
    fn default() -> Self {
        EIdent {
            preident: PreEIdent {
                ..Default::default()
            },
            pad: [0; 9],
            nident: EI_NIDENT as u8,
        }
    }
}

pub type Ehdr = EhdrGeneral<Elf, EIdent>;

impl Default for Ehdr {
    fn default() -> Self {
        Ehdr {
            e_ident: Default::default(),
            e_type: ETypeValue::NONE as u16,
            e_machine: EMachineValue::EmNone as u16,
            e_version: EIVersionValue::Current as u32,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}
