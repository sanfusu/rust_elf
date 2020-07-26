use super::{
    gabi41::ehdr::{EIVersionValue, EMachineValue, ETypeValue, EhdrGeneral, PreEIdent, EI_NIDENT},
    Elf,
};

#[derive(Ordinalize, Debug)]
#[repr(u8)]
#[non_exhaustive]
/// 该枚举类型只用与 elf64 用于标识字段 [`EIdent::osabi`]
pub enum EIOsAbiValue {
    /// System V ABI
    OsAbiSysv = 0,
    /// HP-UX 操作系统
    OsAbiHpUx = 1,
    /// Standalone (嵌入式) 应用
    OsAbiStandlone = 255,
}

#[repr(packed)]
#[derive(Debug)]
pub struct EIdent {
    pub preident: PreEIdent,
    /// 操作系统和 ABI 标识
    pub osabi: u8,
    /// 表示 ABI 版本，用于区别版本兼容性。该字段的值依赖于 [`EIdent::osabi`]
    pub abiversion: u8,
    pub pad: [u8; 7], // 填充字节
    pub nident: u8,   // eident64 的大小
}

impl Default for EIdent {
    fn default() -> Self {
        EIdent {
            preident: PreEIdent {
                ..Default::default()
            },
            osabi: EIOsAbiValue::OsAbiSysv as u8,
            abiversion: 0,
            pad: [0; 7],
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
